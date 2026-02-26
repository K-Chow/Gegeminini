mod config;
mod constants;
mod models;
mod request;
use tauri::Manager;

use crate::config::{init_data, init_db};
use std::sync::Arc;

pub struct AppState {
    pub db: Arc<sled::Db>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let _db: Result<(), String> = init_db(app);
            let state = app.state();
            let _init_data = init_data(state);
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            request::send_message,
            request::get_model_list,
            config::get_api_config,
            config::save_api_config,
            config::set_sys_config,
            config::get_sys_config
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
