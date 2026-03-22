use crate::AppState;
use crate::{
    models::{GeminiStruct, List, Page},
    request::get_current_app,
};
use uuid::{Timestamp, Uuid};

pub async fn save_message(state: AppState, messages: Vec<GeminiStruct>) -> Result<(), String> {
    let app = get_current_app(&state)?;
    let mut batch = sled::Batch::default();
    let base_time = chrono::Utc::now().timestamp_nanos_opt().unwrap();
    for (index, msg) in messages.into_iter().enumerate() {
        let base_nanos = base_time + index as i64;
        let secs = (base_nanos / 1_000_000_000) as u64;
        let nanos = (base_nanos % 1_000_000_000) as u32;
        let uuid = Uuid::new_v7(Timestamp::from_unix(uuid::NoContext, secs, nanos));
        let key = format!("message:{}:{}", app, uuid);
        let msg = GeminiStruct {
            id: uuid.to_string(),
            timestamp: base_nanos,
            ..msg
        };
        let value = bincode::serialize(&msg).map_err(|e| e.to_string())?;
        batch.insert(key.as_bytes(), value);
    }
    state.db.apply_batch(batch).map_err(|e| e.to_string())?;
    state.db.flush().map_err(|e| e.to_string())?;
    increment_total(&state.db, &format!("total:{}", app));
    Ok(())
}

fn increment_total(db: &sled::Db, key: &str) -> Result<(), String> {
    let result = db
        .update_and_fetch(key, |old_bytes| {
            let current = old_bytes
                .and_then(|bytes| bincode::deserialize::<usize>(bytes).ok())
                .unwrap_or(0);
            let new_total = current + 1;
            bincode::serialize(&new_total).ok()
        })
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn get_messages(db: &sled::Db, prefix: &str, page: &Page) -> Result<Vec<GeminiStruct>, String> {
    let skip = (page.number.max(1) - 1) * page.size;
    let count: usize = page.size;
    let mut messages = Vec::new();
    for item in db.scan_prefix(prefix).rev().skip(skip).take(count) {
        let (_key, value) = item.map_err(|e| e.to_string())?;
        let message = bincode::deserialize::<GeminiStruct>(&value).map_err(|e| e.to_string())?;
        messages.push(message);
    }

    messages.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
    Ok(messages)
}

pub fn get_total(db: &sled::Db, key: &str) -> usize {
    let result = db.get(key).ok().flatten(); // 简化处理，把 Err 和 None 都看作没有数据

    result
        .and_then(|bytes| bincode::deserialize::<usize>(&bytes).ok())
        .unwrap_or(0)
}

#[tauri::command]
pub async fn response_messages(
    state: tauri::State<'_, AppState>,
    page: Option<Page>,
) -> Result<List<GeminiStruct>, String> {
    let app = get_current_app(&state)?;
    let prefix = format!("message:{}:", app);
    let page = page.unwrap_or_default();
    let messages = get_messages(&state.db, &prefix, &page)?;
    let total = get_total(&state.db, &prefix);

    Ok(List {
        items: messages,
        total: total,
        page: page.number,
        size: page.size,
    })
}
