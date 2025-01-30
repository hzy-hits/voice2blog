mod api;
mod audio;
mod config;
mod env;

use cpal::traits::StreamTrait;
use env::update_gitignore;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::signal::ctrl_c;
use tokio::sync::oneshot;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    update_gitignore()?;
    let config = config::Config::load()?;
    let config = Arc::new(config);
    let env_config = env::EnvConfig::load()?;

    let recorder = audio::AudioRecorder::new()?;

    let (tx, rx) = oneshot::channel();
    let recording_finished = Arc::new(AtomicBool::new(false));
    let processing_finished = Arc::new(AtomicBool::new(false));
    let processing_finished_clone = processing_finished.clone();

    let stream = recorder.start_recording()?;
    println!("Starting recording... (Press Ctrl+C to stop)");
    stream.play()?;

    let config_clone = config.clone();
    let recording_finished_clone = recording_finished.clone();

    // Start processing task
    tokio::spawn(async move {
        // Wait for recording completion signal
        let _ = rx.await;
        if recording_finished_clone.load(Ordering::SeqCst) {
            match process_recording(config_clone.as_ref(), &env_config).await {
                Ok(_) => {
                    println!("Processing complete! Program will exit automatically...");
                    processing_finished.store(true, Ordering::SeqCst);
                }
                Err(e) => {
                    eprintln!("Error during processing: {}", e);
                    std::process::exit(1);
                }
            }
        }
    });

    // Set up Ctrl+C handler
    let recorder_clone = recorder.clone();
    let tx = Arc::new(tokio::sync::Mutex::new(Some(tx)));

    tokio::spawn(async move {
        if let Ok(()) = ctrl_c().await {
            if let Err(e) = recorder_clone.save_recording("recording.wav") {
                eprintln!("Failed to save record file: {}", e);
                std::process::exit(1);
            }
            println!("\nThe record has been saved as recording.wav");
            recording_finished.store(true, Ordering::SeqCst);

            // send the recording finished signal
            if let Some(tx) = tx.try_lock().ok().and_then(|mut guard| guard.take()) {
                let _ = tx.send(());
            }
        }
    });

    // main loop, wait for processing to finish
    while !processing_finished_clone.load(Ordering::SeqCst) {
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }

    // wait for a short delay to ensure all messages are printed
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    Ok(())
}

async fn process_recording(
    config: &config::Config,
    env_config: &env::EnvConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    let api_client = api::ApiClient::new(config, env_config);

    // Transcribe audio
    println!("Starting audio transcription...");
    let transcript = api_client.transcribe_audio("recording.wav").await?;
    println!("Transcript: {}", transcript);
    println!("Audio transcription completed");

    // Generate Markdown
    println!("Generating Markdown document...");
    let markdown_content = api_client.generate_markdown(&transcript).await?;

    // Save Markdown
    std::fs::write("output.md", markdown_content)?;
    println!("Markdown file has been saved as output.md");

    Ok(())
}
