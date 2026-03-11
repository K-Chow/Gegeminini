use serde::{Deserialize, Serialize};

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

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChatMessage {
    pub app: String,
    pub role: String,
    pub content: String,
    pub content_type: String,
    pub timestamp: i64,
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
