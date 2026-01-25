mod request;
use std::sync::Arc;
use tauri::Manager;

pub struct AppState {
    pub db: Arc<sled::Db>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let app_data_dir = app.path().app_data_dir().expect("无法获取路径");
            std::fs::create_dir_all(&app_data_dir).ok();

            // 2. 打开（或创建）数据库文件夹
            let db_path = app_data_dir.join("gegeminini_sled");
            let db = sled::open(db_path).expect("无法初始化 sled");

            // 3. 注入到全局状态
            app.manage(AppState { db: Arc::new(db) });
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![request::api_request])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
