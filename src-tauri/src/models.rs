use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiConfigItem {
    pub app: String,
    pub api_key: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SysConfig {
    pub theme: String,
    pub current_app: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Chat {
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
