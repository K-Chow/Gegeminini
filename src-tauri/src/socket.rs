use crate::constants::GEMINI_STREAM_URL;
use crate::request::get_api_key;
use futures_util::{SinkExt, StreamExt};
use std::env;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

type GeminiStream =
    tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>;

/// 建立与 Gemini Live API 的 WebSocket 连接并完成配置初始化
pub async fn connect_to_gemini(api_key: &str) -> Result<GeminiStream, String> {
    // 1. 构造 WebSocket URL (Gemini v1alpha 实时双向流端点)
    let url_str = format!(GEMINI_STREAM_URL, api_key);

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

#[tauri::command]
async fn start_session(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let api_key = get_api_key(&state)?;
    let ws_stream = connect_to_gemini(&api_key).await?;

    // 2. 拆分读写端
    let (ws_sender, mut ws_receiver) = ws_stream.split();

    // 3. 将发送端用 Arc<Mutex<...>> 包裹，存入你的全局 Tauri State
    let shared_sender = Arc::new(Mutex::new(ws_sender));

    // 4. 开启一个独立的后台异步任务，专门用来“监听”并播放 Gemini 发回的声音
    tokio::spawn(async move {
        while let Some(message) = ws_receiver.next().await {
            match message {
                Ok(Message::Text(text)) => {
                    // 这里接收到了来自 Gemini 的服务器消息 (BidiGenerateContentServerMessage)
                    // TODO: 解析里面的音频数据并喂给你的播放器 (Rodio/Sink)
                    println!("收到 Gemini 响应 (大小: {} 字符)", text.len());
                }
                Ok(Message::Close(_)) => {
                    println!("Gemini 关闭了连接");
                    break;
                }
                Err(e) => {
                    eprintln!("接收 Gemini 数据时出错: {}", e);
                    break;
                }
                _ => {}
            }
        }
    });

    // 5. 将发送端保存进状态中，后续 start_record 指令就可以利用它实时发送麦克风数据了
    let mut session_guard = state.session_lock.lock().unwrap();
    *session_guard = Some(shared_sender);

    Ok(())
}
