use crate::constants::{GEMINI_BASE_URL, GEMINI_VERSION};
use crate::models::{ApiConfigItem, SysConfig};
use crate::AppState;
use reqwest::Client;

async fn api_request(
    app_state: &AppState,
    url: String,
    payload: serde_json::Value,
) -> Result<serde_json::Value, String> {
    let current_app: String = app_state
        .db
        .get("config:system")
        .map_err(|e| e.to_string())?
        .and_then(|bytes| bincode::deserialize(&bytes).ok())
        .map(|config: SysConfig| config.current_app)
        .ok_or_else(|| format!("sys config is not found"))?;

    let api_key = app_state
        .db
        .get(format!("config:app:{}", current_app))
        .map_err(|e| e.to_string())?
        .and_then(|bytes| bincode::deserialize(&bytes).ok())
        .map(|item: ApiConfigItem| item.api_key)
        .ok_or_else(|| format!("{} api key is not found", current_app))?;

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

#[tauri::command]
pub async fn send_message(
    state: tauri::State<'_, AppState>,
    contents: serde_json::Value,
) -> Result<serde_json::Value, String> {
    let url = format!(
        "{}/models/{}:generateContent",
        GEMINI_BASE_URL, GEMINI_VERSION
    );

    let payload = serde_json::json!({
      "contents": contents
    });

    let result = api_request(&state, url, payload).await?;

    Ok(result)
}

#[tauri::command]
pub async fn get_model_list(
    state: tauri::State<'_, AppState>,
) -> Result<serde_json::Value, String> {
    let client = Client::new();

    let url = format!("{}/", GEMINI_BASE_URL);

    let payload = serde_json::json!({});

    let result = api_request(&state, url, payload).await?;

    Ok(result)
}
