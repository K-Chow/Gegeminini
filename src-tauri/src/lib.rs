mod config;
mod models;
mod request;
use std::sync::Arc;

use crate::config::init_config;

pub struct AppState {
    pub db: Arc<sled::Db>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            init_config(app);
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            request::api_request,
            config::get_config
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
