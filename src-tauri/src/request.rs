use crate::chat::{get_messages, save_message};
use crate::constants::GEMINI_BASE_URL;
use crate::models::{ApiConfigItem, ChatMessage, Page, SysConfig};
use crate::AppState;
use reqwest::Client;
use serde_json::{json, Value};
use tauri::async_runtime::spawn;

pub fn get_current_app(app_state: &AppState) -> Result<String, String> {
    let current_app: String = app_state
        .db
        .get("config:system")
        .map_err(|e: sled::Error| e.to_string())?
        .and_then(|bytes| bincode::deserialize(&bytes).ok())
        .map(|config: SysConfig| config.current_app)
        .ok_or_else(|| format!("sys config is not found"))?;
    Ok(current_app)
}

pub fn get_model(app_state: &AppState) -> Result<String, String> {
    let current_app: String = get_current_app(app_state)?;
    let model = app_state
        .db
        .get(format!("config:app:{}", current_app))
        .map_err(|e| e.to_string())?
        .and_then(|bytes| bincode::deserialize(&bytes).ok())
        .map(|item: ApiConfigItem| item.model)
        .ok_or_else(|| format!("{} model is not found", current_app))?;
    Ok(model)
}

fn get_api_key(app_state: &AppState) -> Result<String, String> {
    let current_app: String = get_current_app(app_state)?;

    let api_key = app_state
        .db
        .get(format!("config:app:{}", current_app))
        .map_err(|e| e.to_string())?
        .and_then(|bytes| bincode::deserialize(&bytes).ok())
        .map(|item: ApiConfigItem| item.api_key)
        .ok_or_else(|| format!("{} api key is not found", current_app))?;
    Ok(api_key)
}

async fn api_request(
    app_state: &AppState,
    url: String,
    payload: serde_json::Value,
) -> Result<serde_json::Value, String> {
    let api_key = get_api_key(app_state)?;

    let client = Client::new();
    let response = client
        .post(url)
        .header("x-goog-api-key", api_key)
        .json(&payload)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let result = response.json().await.map_err(|e| e.to_string())?;

    Ok(result)
}

async fn fetcher(url: String) -> Result<Value, String> {
    let client = Client::new();
    let response = client.get(url).send().await.map_err(|e| e.to_string())?;

    let result = response.json().await.map_err(|e| e.to_string())?;

    Ok(result)
}

#[tauri::command]
pub async fn send_message(
    state: tauri::State<'_, AppState>,
    contents: Value,
) -> Result<Value, String> {
    let model = get_model(&state)?;
    let url = format!("{}/{}:generateContent", GEMINI_BASE_URL, model);
    let app = get_current_app(&state)?;

    let history_message = get_messages(&state.db, &format!("message:{}", app), &Page::default())?;

    let mut all_contents = Vec::new();

    for item in history_message {
        all_contents.push(json!({
          "role": item.role,
          "parts":[{
            "text": item.content
          }]
        }))
    }

    let payload = json!({
      "contents": &contents
    });

    let result = api_request(&state, url, payload.clone())
        .await
        .map_err(|e| e.to_string())?;

    let state_handle = state.inner().clone();
    let save_payload = payload.to_string();
    let save_result = result.clone().to_string();

    spawn(async move {
        let _ = save_message(
            state_handle,
            vec![
                ChatMessage {
                    id: "".to_string(),
                    app: app.clone(),
                    role: "USER".to_string(),
                    content: save_payload,
                    content_type: "application/json".to_string(),
                    timestamp: 0,
                },
                ChatMessage {
                    id: "".to_string(),
                    app: app,
                    role: "MODEL".to_string(),
                    content: save_result,
                    content_type: "application/json".to_string(),
                    timestamp: 0,
                },
            ],
        )
        .await
        .map_err(|e| e.to_string());
    });

    Ok(result)
}

#[tauri::command]
pub async fn get_model_list(
    state: tauri::State<'_, AppState>,
) -> Result<serde_json::Value, String> {
    let url = format!("{}/models?key={}", GEMINI_BASE_URL, get_api_key(&state)?);

    let result = fetcher(url).await?;

    Ok(result)
}
