use crate::models::{ApiConfigItem, CommandResult, SysConfig};
use crate::AppState;
use sled::Batch;
use std::sync::Arc;
use tauri::{App, Manager};

pub async fn init_db(app: &mut App) -> Result<(), String> {
    let app_data_dir = app.path().app_data_dir().expect("无法获取路径");
    std::fs::create_dir_all(&app_data_dir).ok();

    // 2. 打开（或创建）数据库文件夹
    let db = sled::open(app_data_dir.join("gegeminini_sled")).expect("sled init failure");
    let db_arc = Arc::new(db);

    // 3. 注入到全局状态
    app.manage(AppState { db: db_arc.clone() });

    let key = format!("config:gemini");

    if db_arc.get(&key).unwrap().is_none() {
        db_arc.insert(&key, "").map_err(|e| e.to_string())?;
        db_arc.flush().map_err(|e| e.to_string())?;
    }

    Ok(())
}

#[tauri::command]
pub async fn get_api_config(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<ApiConfigItem>, String> {
    let mut list: Vec<ApiConfigItem> = Vec::new();

    for item in state.db.scan_prefix("config:") {
        let (_key, value) = item.map_err(|e: sled::Error| e.to_string())?;
        let config = bincode::deserialize(&value).map_err(|e| e.to_string())?;
        list.push(config);
    }

    list.sort_by(|a, b| b.app.cmp(&a.app));
    Ok(list)
}

#[tauri::command]
pub async fn save_api_config(
    state: tauri::State<'_, AppState>,
    configs: Vec<ApiConfigItem>,
) -> Result<CommandResult, String> {
    let mut batch = Batch::default();

    for config in configs {
        if config.app.is_empty() {
            return Err("App name cannot be empty".into());
        }

        let key = format!("config:api:{}", config.app);
        let value = bincode::serialize(&config).map_err(|e| e.to_string())?;

        batch.insert(key.as_str(), value);
    }

    state.db.apply_batch(batch).map_err(|e| e.to_string())?;
    state.db.flush().map_err(|e| e.to_string())?;

    Ok(CommandResult {
        status: 200,
        message: format!("OK"),
    })
}

#[tauri::command]
pub async fn save_sys_config(
    state: tauri::State<'_, AppState>,
    config: SysConfig,
) -> Result<CommandResult, String> {
    let data = bincode::serialize(&config).map_err(|e| e.to_string())?;

    state
        .db
        .insert("config:sys", data)
        .map_err(|e| e.to_string())?;

    Ok(CommandResult {
        status: 200,
        message: format!("OK"),
    })
}

#[tauri::command]
pub async fn get_sys_config(state: tauri::State<'_, AppState>) -> Result<SysConfig, String> {
    state
        .db
        .get("config:sys")
        .map_err(|e| e.to_string())? // 处理数据库读取错误
        .map(|bytes| bincode::deserialize(&bytes).map_err(|e| e.to_string())) // 尝试反序列化
        .unwrap_or_else(|| Ok(SysConfig::default())) // 如果是 None 则返回默认值
}
