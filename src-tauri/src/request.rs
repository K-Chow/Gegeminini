use reqwest::Client;

#[tauri::command]
pub async fn api_request() -> Result<String, String> {
    let api_key = "";
    Ok(format!(""))
}
