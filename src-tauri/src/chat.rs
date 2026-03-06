use crate::AppState;
use crate::{models::ChatMessage, request::get_current_app};
use serde_json::json;
use sled::Batch;
use std::sync::Arc;
use tauri::{App, Manager};

#[tauri::command]
pub async fn save_message(
    state: tauri::State<'_, AppState>,
    messages: ChatMessage,
) -> Result<(), String> {
    let mut batch = Batch::default();
    state
        .db
        .insert(
            format!("message:{}:{}", messages.app, messages.id),
            bincode::serialize(&messages).map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())?;
    state.db.flush().map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn get_messages(state: tauri::State<'_, AppState>) -> Result<serde_json::Value, String> {
    let app = get_current_app(&state)?;
    let result = state
        .db
        .get(format!("message:{}:", app))
        .map_err(|e| e.to_string())?
        .map(|bytes| bincode::deserialize(&bytes).map_err(|e| e.to_string()))
        .unwrap_or_else(|| Ok(json!({})));
    Ok(json!(result))
}
