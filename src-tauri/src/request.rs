use crate::models::{ApiConfigItem, CommandResult, SysConfig};
use crate::AppState;
use reqwest::Client;

#[tauri::command]
pub async fn api_request(state: tauri::State<'_, AppState>) -> Result<Vec<CommandResult>, String> {
    let current_app = state
        .db
        .get("config:sys")
        .map_err(|e| e.to_string())?
        .and_then(|bytes| bincode::deserialize(&bytes).ok())
        .map(|config: SysConfig| config.current_app)
        .ok_or_else(|| "".to_string())?;

    let api_key = state
        .db
        .get(format!("config:api:{}", current_app))
        .map_err(|e| e.to_string())?
        .and_then(|bytes| bincode::deserialize(&bytes).ok())
        .map(|item: ApiConfigItem| item.api_key)
        .ok_or_else(|| "".to_string())?;

    let client = Client::new();

    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key={}",
        api_key
    );

    let resule = client
        .post(url)
        .json("{}")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    Ok(vec![CommandResult {
        status: 200,
        message: format!("OK"),
    }])
}
