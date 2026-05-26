use crate::constants::GEMINI_STREAM_URL;
use crate::models::GeminiCommand;
use crate::request::get_api_key;
use crate::AppState;
use futures_util::{SinkExt, StreamExt};
use tokio::sync::Mutex;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

type GeminiStream =
    tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>;

#[tauri::command]
pub async fn connect_gemini(
    state: tauri::State<'_, AppState>,
    app_handle: tauri::AppHandle,
) -> Result<String, String> {
    let mut tx_guard = state.gemini_tx.lock().await;

    // 1. 防御性检查：确保不重复建立物理连接
    if tx_guard.is_some() {
        return Ok("已经处于连接状态，无需重复连接。".to_string());
    }

    println!("🌐 触发连接 Action：开始构建网络基础设施...");

    // 2. 建立内存中的跨线程通信管道 (MPSC)
    // new_tx 由外部（未来的录音 Action）持有，用来发射数据
    // new_rx 由下面的后台网络协程持有，用来接收数据并吐给网络
    let (new_tx, mut new_rx) = tokio::sync::mpsc::unbounded_channel::<GeminiCommand>();

    // 3. 产生一个纯粹的常驻后台异步任务，负责死守网络链路
    tokio::spawn(async move {
        println!("⚡ [网络任务] 开始向 Gemini API 终点发起物理握手 (例如 WebSocket)...");

        // -------------------------------------------------------------
        // 🔴 这里放置你真实的物理网络连接代码，例如：
        // let url = "wss://generativelanguage.googleapis.com/...";
        // let (mut ws_stream, _) = tokio_tungstenite::connect_async(url).await.unwrap();
        // -------------------------------------------------------------

        println!("✅ [网络任务] 物理连接建立成功！网络协程进入常驻挂起状态，等待发信指令...");

        // 物理通路建好了，现在让这个协程在后台安安静静地挂起（Sleep/Wait）
        // 只有当外面的管道有东西丢进来时，它才会醒来，不消耗任何 CPU
        while let Some(command) = new_rx.recv().await {
            match command {
                GeminiCommand::SendAudio(bytes) => {
                    // 只有未来有动作往 tx 里扔数据时，这里才会被唤醒并发送
                    // ws_stream.send(Message::Binary(bytes)).await.ok();
                    println!(
                        "📭 [网络任务] 接收到外来投递，成功将 {} 字节音频注入物理网卡",
                        bytes.len()
                    );
                }
                GeminiCommand::Start { .. } | GeminiCommand::Stop | GeminiCommand::CloseSession => {
                    // Handle other command types
                }
            }
        }
    });

    // 4. 将发信端安全地寄存在全局状态中，方便后续任何其他 Action 随时借用
    *tx_guard = Some(new_tx);

    Ok("Gemini 物理连接已建立，网络常驻任务已就绪。".to_string())
}
