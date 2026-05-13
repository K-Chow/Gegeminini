use crate::models::GeminiCommand;
use crate::AppState;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
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

    let mut writer = hound::WavWriter::create(file_path, spec).unwrap();

    let stream = device
        .build_input_stream(
            &config.into(),
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                let bytes: Vec<u8> = data.iter().flat_map(|&f| f.to_le_bytes()).collect();
                let _ = tx.send(GeminiCommand::SendAudio(bytes));
            },
            move |err| {
                eprintln!("An error occurred on the input audio stream: {}", err);
            },
            None,
        )
        .unwrap();

    stream.play().map_err(|e| e.to_string())?;

    std::thread::sleep(std::time::Duration::from_secs(5));
    drop(stream);

    Ok(())
}
