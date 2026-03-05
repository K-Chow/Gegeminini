use crate::models::ChatMessage;
use crate::AppState;
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
