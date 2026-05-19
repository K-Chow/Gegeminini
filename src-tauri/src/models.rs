use futures_util::stream::SplitSink;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::{oneshot, Mutex};
use tokio_tungstenite::{tungstenite::Message, MaybeTlsStream, WebSocketStream};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ApiConfigItem {
    pub app: String,
    pub api_key: String,
    pub model: String,
}
impl Default for ApiConfigItem {
    fn default() -> Self {
        Self {
            app: "gemini".to_string(),
            api_key: "".to_string(),
            model: "models/gemini-2.5-flash-lite".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SysConfig {
    pub theme: String,
    pub current_app: String,
}

impl Default for SysConfig {
    fn default() -> Self {
        Self {
            theme: "light".to_string(),
            current_app: "gemini".to_string(),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default)]
pub struct GeminiStruct {
    pub id: String,
    pub app: String,
    pub role: String,
    pub content: String,
    pub content_type: String,
    pub timestamp: i64,
    pub model: Option<String>,
    pub input_tokens: Option<i32>,
    pub output_tokens: Option<i32>,
    pub finish_reason: Option<String>,
}

#[derive(Serialize)]
pub struct CommandResult {
    pub status: u16,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct _Part {
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct _Content {
    pub parts: Vec<_Part>,
}

#[derive(Serialize)]
pub struct _GeminiRequest {
    pub contents: Vec<_Content>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Page {
    pub size: usize,
    pub number: usize,
}

impl Default for Page {
    fn default() -> Self {
        Self {
            size: 20,
            number: 1,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct List<T> {
    pub items: Vec<T>,
    pub total: usize,
    pub page: usize,
    pub size: usize,
}

impl Default for List<serde_json::Value> {
    fn default() -> Self {
        Self {
            items: vec![],
            total: 0,
            page: 1,
            size: 20,
        }
    }
}

type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

pub type GeminiSink = SplitSink<WsStream, Message>;

pub struct GeminiSession {
    pub ws_sender: Arc<Mutex<GeminiSink>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum GeminiCommand {
    Start { api_key: String },
    SendAudio(Vec<u8>),
    Stop,
}

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<sled::Db>,
    pub gemini_tx: tokio::sync::mpsc::UnboundedSender<GeminiCommand>,
}

pub struct AudioState {
    pub stop_tx: std::sync::Mutex<Option<oneshot::Sender<()>>>,
}
