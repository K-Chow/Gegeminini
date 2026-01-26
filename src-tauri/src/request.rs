use crate::models::CommandResult;
use reqwest::Client;

#[tauri::command]
pub async fn api_request() -> Result<Vec<CommandResult>, String> {
    let api_key = "";
    Ok(vec![CommandResult {
        status: 200,
        message: format!("OK"),
    }])
}
