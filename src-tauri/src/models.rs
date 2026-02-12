use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ApiConfigItem {
    pub app: String,
    pub api_key: String,
}
impl Default for ApiConfigItem {
    fn default() -> Self {
        Self {
            app: "gemini".to_string(),
            api_key: "".to_string(),
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

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct _Chat {
    pub app: String,
    pub role: String,
    pub content: String,
    pub content_type: String,
    pub timestamp: String,
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
