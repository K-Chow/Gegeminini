mod chat;
mod config;
mod constants;
mod gemini;
mod models;
mod request;
mod socket;
mod voice;
use crate::models::{AppState, AudioState, GeminiCommand};
use std::sync::Arc;
use tauri::Manager;
use tokio::sync::{mpsc, Mutex};

use crate::config::{init_data, init_db}; //Enable to clone app state
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let (gemini_tx, gemini_rx) = mpsc::unbounded_channel::<GeminiCommand>();

    tauri::Builder::default()
        .setup(move |app| {
            let db_arc = init_db(app).unwrap();

            app.manage(AudioState {
                stop_tx: std::sync::Mutex::new(None),
                stop_rx: std::sync::Mutex::new(None),
            });

            app.manage(AppState {
                db: db_arc,
                gemini_tx: Arc::new(Mutex::new(Some(gemini_tx))), // 这是从 main 函数作用域捕获的
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
            voice::stop_recording,
            chat::response_messages,
            socket::connect_gemini,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
