mod chat;
mod config;
mod constants;
mod gemini;
mod models;
mod request;
mod socket;
mod voice;
use crate::models::GeminiSession;
use std::sync::{Arc, Mutex};
use tauri::Manager;

use crate::config::{init_data, init_db};

struct LiveSession(Mutex<Option<GeminiSession>>);

#[derive(Clone)] //Enable to clone app state
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
            config::get_sys_config,
            config::delete_data,
            voice::trigger_recording,
            chat::response_messages,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
