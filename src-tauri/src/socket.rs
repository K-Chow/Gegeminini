use futures_util::{SinkExt, StreamExt};
use std::env;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

type GeminiStream =
    tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>;

/// 建立与 Gemini Live API 的 WebSocket 连接并完成配置初始化
pub async fn connect_to_gemini(api_key: &str) -> Result<GeminiStream, String> {
    // 1. 构造 WebSocket URL (Gemini v1alpha 实时双向流端点)
    let url_str = format!(
        "wss://generativelanguage.googleapis.com/ws/google.ai.generativelanguage.v1alpha.GenerativeService.BidiGenerateContent?key={}",
        api_key
    );

    println!("正在连接到 Gemini Live API...");

    // 2. 建立 wss 安全连接
    let (mut ws_stream, response) = connect_async(&url_str)
        .await
        .map_err(|e| format!("WebSocket 连接失败: {}", e))?;

    println!("连接成功！HTTP 状态码: {}", response.status());

    // 3. 构造第一帧初始化配置 (Setup Frame)
    let setup_msg = LiveClientMessage {
        setup: Some(GeminiSetup {
            model: "models/gemini-2.5-flash".to_string(), // 使用 native-audio 支持最好的模型
            generation_config: GenerationConfig {
                response_modalities: vec!["AUDIO".to_string()], // 只要音频返回
                speech_config: SpeechConfig {
                    voice_config: VoiceConfig {
                        prebuilt_voice_config: PrebuiltVoiceConfig {
                            voice_name: "Puck".to_string(), // 可换 Puck, Aoede, Fenrir 等音色
                        },
                    },
                },
            },
        }),
        client_content: None,
    };

    // 4. 将 Setup 配置转换为 JSON 并发送
    let setup_json = serde_json::to_string(&setup_msg).unwrap();
    ws_stream
        .send(Message::Text(setup_json))
        .await
        .map_err(|e| format!("发送初始化配置失败: {}", e))?;

    println!("初始化握手消息已发送，会话已建立。");

    Ok(ws_stream)
}
