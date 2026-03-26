use crate::chat::{get_messages, save_message};
use crate::constants::GEMINI_BASE_URL;
use crate::gemini::{build_gemini_request, parse_gemini_response};
use crate::models::{ApiConfigItem, GeminiStruct, Page, SysConfig};
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
    text: String,
) -> Result<Value, String> {
    let model = get_model(&state)?;
    let url = format!("{}/{}:generateContent", GEMINI_BASE_URL, model);
    let app = get_current_app(&state)?;

    let payload = build_gemini_request(&state.db, &app, &text)?;

    let result: Value = api_request(&state, url, payload.clone())
        .await
        .map_err(|e: String| e.to_string())?;

    let state_handle = state.inner().clone();

    let parsed_result = GeminiStruct {
        app: app.clone(),
        ..parse_gemini_response(&result)?
    };
    let response_result = json!(vec![&parsed_result]);

    spawn(async move {
        let _ = save_message(
            state_handle,
            vec![
                GeminiStruct {
                    app: app.clone(),
                    role: "USER".to_string(),
                    content: text,
                    content_type: "text".to_string(),
                    ..Default::default()
                },
                parsed_result,
            ],
        )
        .await
        .map_err(|e| e.to_string());
    });

    Ok(response_result)
}

#[tauri::command]
pub async fn get_model_list(
    state: tauri::State<'_, AppState>,
) -> Result<serde_json::Value, String> {
    let url = format!("{}/models?key={}", GEMINI_BASE_URL, get_api_key(&state)?);

    let result = fetcher(url).await?;

    Ok(result)
}
