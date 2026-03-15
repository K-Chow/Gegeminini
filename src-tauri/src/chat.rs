use crate::AppState;
use crate::{
    models::{ChatMessage, List, Page},
    request::get_current_app,
};
use serde_json::json;
use uuid::{Timestamp, Uuid};

pub async fn save_message(state: AppState, messages: Vec<ChatMessage>) -> Result<(), String> {
    let app = get_current_app(&state)?;
    let mut batch = sled::Batch::default();
    let base_time = chrono::Utc::now().timestamp_nanos_opt().unwrap();
    for (index, msg) in messages.into_iter().enumerate() {
        let base_nanos = base_time + index as i64;
        let secs = (base_nanos / 1_000_000_000) as u64;
        let nanos = (base_nanos % 1_000_000_000) as u32;
        let uuid = Uuid::new_v7(Timestamp::from_unix(uuid::NoContext, secs, nanos));
        let key = format!("message:{}:{}", app, uuid);
        let msg = ChatMessage {
            id: uuid.to_string(),
            timestamp: base_nanos,
            ..msg
        };
        let value = bincode::serialize(&msg).map_err(|e| e.to_string())?;
        batch.insert(key.as_bytes(), value);
    }
    state.db.apply_batch(batch).map_err(|e| e.to_string())?;
    state.db.flush().map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn get_messages(
    state: tauri::State<'_, AppState>,
    page: Option<Page>,
) -> Result<serde_json::Value, String> {
    let app = get_current_app(&state)?;
    let prefix = format!("message:{}:", app);

    let page = page.unwrap_or_default();
    let skip = (page.number.max(1) - 1) * page.size;
    let count: usize = page.size;

    let mut messages = Vec::new();

    let total = state.db.scan_prefix(prefix.as_bytes()).count();

    for item in state.db.scan_prefix(prefix).rev().skip(skip).take(count) {
        let (_key, value) = item.map_err(|e| e.to_string())?;
        let message: ChatMessage = bincode::deserialize(&value).map_err(|e| e.to_string())?;
        messages.push(message);
    }
    println!(
        "get messages: skip {}, count {}, actual {}",
        skip,
        count,
        messages.len()
    );
    messages.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
    Ok(json!(List {
        items: messages,
        total: total,
        page: page.number,
        size: page.size
    }))
}
