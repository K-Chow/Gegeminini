use crate::models::{CommandResult, Config};
use crate::AppState;
use std::sync::Arc;
use tauri::{App, Manager};

pub async fn init_config(app: &mut App) -> Result<(), String> {
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
pub async fn get_config(state: tauri::State<'_, AppState>) -> Result<Vec<Config>, String> {
    let mut list = Vec::new();

    for item in state.db.scan_prefix("config:") {
        let (_key, value) = item.map_err(|e| e.to_string())?;
        let config: Config = bincode::deserialize(&value).map_err(|e| e.to_string())?;
        list.push(config);
    }

    list.sort_by(|a, b| b.app.cmp(&a.app));
    Ok(list)
}

#[tauri::command]
pub async fn save_config() -> Result<Vec<CommandResult>, String> {
    let result = vec![CommandResult {
        status: 200,
        message: format!("OK"),
    }];

    Ok(result)
}
