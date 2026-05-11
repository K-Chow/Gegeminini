mod chat;
mod config;
mod constants;
mod gemini;
mod models;
mod request;
mod socket;
mod voice;
use crate::gemini::init_gemini_manager;
use crate::models::GeminiCommand;
use std::sync::{Arc, Mutex};
use tauri::Manager;
use tokio::sync::mpsc;

use crate::config::{init_data, init_db};

#[derive(Clone)] //Enable to clone app state
pub struct AppState {
    pub db: Arc<sled::Db>,
    pub gemini_tx: tokio::sync::mpsc::UnboundedSender<GeminiCommand>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let (gemini_tx, gemini_rx) = mpsc::unbounded_channel::<GeminiCommand>();

    tokio::spawn(init_gemini_manager(gemini_rx));

    tauri::Builder::default()
        .setup(move |app| {
            let db_arc = init_db(app).unwrap();

            app.manage(AppState {
                db: db_arc,
                gemini_tx: gemini_tx, // 这是从 main 函数作用域捕获的
            });

            let state = app.state::<AppState>();
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
