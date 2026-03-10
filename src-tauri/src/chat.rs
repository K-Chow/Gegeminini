use crate::AppState;
use crate::{models::ChatMessage, request::get_current_app};
use serde_json::json;
use tauri::{App, Manager};
use uuid::{Timestamp, Uuid};

pub async fn save_message(state: AppState, messages: Vec<ChatMessage>) -> Result<(), String> {
    for msg in &messages {
        let uuid = Uuid::new_v7(Timestamp::now(uuid::NoContext));
        state
            .db
            .insert(
                format!("message:{}:{}", msg.app, uuid),
                bincode::serialize(&msg).map_err(|e| e.to_string())?,
            )
            .map_err(|e| e.to_string())?;
    }
    state.db.flush().map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn get_messages(state: tauri::State<'_, AppState>) -> Result<serde_json::Value, String> {
    let app = get_current_app(&state)?;
    let prefix = format!("message:{}:", app);

    let mut messages = Vec::new();

    for item in state.db.scan_prefix(prefix) {
        let (_key, value) = item.map_err(|e| e.to_string())?;
        let message: ChatMessage = bincode::deserialize(&value).map_err(|e| e.to_string())?;
        messages.push(message);
    }

    Ok(json!(messages))
}
