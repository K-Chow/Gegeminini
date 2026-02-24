use crate::constants::{GEMINI_BASE_URL, GEMINI_VERSION};
use crate::models::{ApiConfigItem, SysConfig};
use crate::AppState;
use reqwest::Client;

#[tauri::command]
pub async fn api_request(
    state: tauri::State<'_, AppState>,
    contents: serde_json::Value,
) -> Result<serde_json::Value, String> {
    let current_app: String = state
        .db
        .get("config:system")
        .map_err(|e| e.to_string())?
        .and_then(|bytes| bincode::deserialize(&bytes).ok())
        .map(|config: SysConfig| config.current_app)
        .ok_or_else(|| format!("sys config is not found"))?;

    let api_key = state
        .db
        .get(format!("config:app:{}", current_app))
        .map_err(|e| e.to_string())?
        .and_then(|bytes| bincode::deserialize(&bytes).ok())
        .map(|item: ApiConfigItem| item.api_key)
        .ok_or_else(|| format!("{} api key is not found", current_app))?;

    let client = Client::new();

    let url = format!(
        "{}/models/{}:generateContent",
        GEMINI_BASE_URL, GEMINI_VERSION
    );

    let payload = serde_json::json!({
      "contents": contents
    });

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

pub async fn get_model_list(api_key: &str) -> Result<serde_json::Value, String> {
    let client = Client::new();

    let url = format!("{}/", GEMINI_BASE_URL);

    let response = client
        .post(url)
        .header("x-goog-api-key", api_key)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let result = response.json().await.map_err(|e| e.to_string())?;

    Ok(result)
}
