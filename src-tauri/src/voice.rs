use crate::models::GeminiCommand;
use crate::AppState;
use crate::AudioState;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::Manager;
use tokio::sync::oneshot;

#[tauri::command]
pub async fn trigger_recording(
    state: tauri::State<'_, AppState>,
    audio_state: tauri::State<'_, AudioState>,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    {
        if audio_state.stop_tx.lock().unwrap().is_some() {
            return Err("正在录音中...".into());
        }
    }

    let app_data_dir = app_handle.path().app_data_dir().expect("无法获取路径");
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let file_path = app_data_dir.join(format!("recording_{}.wav", timestamp));
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
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };

    let writer = hound::WavWriter::create(file_path, spec).unwrap();
    let writer_arc = Arc::new(Mutex::new(writer));
    let writer_clone = writer_arc.clone();

    let (stop_tx, stop_rx) = oneshot::channel::<()>();

    *audio_state.stop_tx.lock().unwrap() = Some(stop_tx);
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
            .map_err(|e| format!("无法构建音频流: {}", e))?;

        stream.play().map_err(|e| format!("无法启动录音: {}", e))?;

        let _ = stop_rx.blocking_recv();
        drop(stream);

        if let Ok(w) = Arc::try_unwrap(writer_arc) {
            if let Ok(writer) = w.into_inner() {
                writer
                    .finalize()
                    .map_err(|e| format!("WAV闭合失败: {}", e))?;
            }
        }

        Ok(())
    });

    Ok(())
}

#[tauri::command]
pub async fn stop_recording(audio_state: tauri::State<'_, AudioState>) -> Result<(), String> {
    if let Some(tx) = audio_state.stop_tx.lock().unwrap().take() {
        let _ = tx.send(());
    }
    Ok(())
}
