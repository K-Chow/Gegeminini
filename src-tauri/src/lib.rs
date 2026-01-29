mod config;
mod constants;
mod models;
mod request;
use crate::config::init_db;
use std::sync::Arc;

pub struct AppState {
    pub db: Arc<sled::Db>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let _db = init_db(app);
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            request::api_request,
            config::get_api_config,
            config::save_api_config,
            config::save_sys_config,
            config::get_sys_config
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
