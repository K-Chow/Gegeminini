use crate::models::GeminiCommand;
use crate::AppState;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{Arc, Mutex};
use tauri::async_runtime::spawn;
use tauri::Manager;

#[tauri::command]
pub async fn trigger_recording(
    state: tauri::State<'_, AppState>,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    let app_data_dir = app_handle.path().app_data_dir().expect("无法获取路径");
    let file_path = app_data_dir.join("recording.wav");
    let tx = state.gemini_tx.clone();
    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .ok_or("No input device available")?;
    let config = device
        .default_input_config()
        .map_err(|e| format!("Failed to get default input config: {}", e))?;

    let spec = hound::WavSpec {
        channels: config.channels(),
        sample_rate: config.sample_rate().0,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Float,
    };

    let writer = hound::WavWriter::create(file_path, spec).unwrap();
    let writer_arc = Arc::new(Mutex::new(writer));
    let writer_clone = writer_arc.clone();

    std::thread::spawn(move || -> Result<(), String> {
        let stream = device
            .build_input_stream(
                &config.into(),
                move |data: &[f32], _: &cpal::InputCallbackInfo| {
                    if let Ok(mut w) = writer_clone.lock() {
                        for &sample in data {
                            w.write_sample(sample).ok();
                        }
                    }
                    let bytes: Vec<u8> = data.iter().flat_map(|&f| f.to_le_bytes()).collect();
                    let _ = tx.send(GeminiCommand::SendAudio(bytes));
                },
                move |err| {
                    eprintln!("An error occurred on the input audio stream: {}", err);
                },
                None,
            )
            .map_err(|e| format!("无法构建音频流: {}", e))?; // <-- 关键解包

        stream.play().map_err(|e| format!("无法启动录音: {}", e))?;

        std::thread::sleep(std::time::Duration::from_secs(5));

        // 4. 5秒后，主动销毁流，释放硬件
        drop(stream);

        // 5. 闭合 WAV 文件
        if let Ok(w) = Arc::try_unwrap(writer_arc) {
            if let Ok(writer) = w.into_inner() {
                writer
                    .finalize()
                    .map_err(|e| format!("WAV闭合失败: {}", e))?;
                println!("WAV 文件已成功保存并闭合！");
            }
        }

        Ok(())
    });

    Ok(())
}

#[tauri::command]
pub async fn stop_recording() -> Result<(), String> {
    Ok(())
}
