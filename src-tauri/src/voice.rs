use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

#[tauri::command]
pub async fn trigger_recording(file_path: String) -> Result<(), String> {
    // Here you would implement the logic to start recording audio using cpal and hound.
    // This is a placeholder implementation.
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
                for &sample in data {
                    writer.write_sample(sample).unwrap();
                }
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
