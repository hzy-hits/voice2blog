use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use hound::{WavSpec, WavWriter};
use std::sync::{Arc, Mutex};
#[derive(Debug, Clone)]
pub struct AudioRecorder {
    recording: Arc<Mutex<Vec<i16>>>,
    spec: WavSpec,
}

impl AudioRecorder {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // use the same configuration as Whisper
        let spec = WavSpec {
            channels: 1,
            sample_rate: 16000,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };

        Ok(Self {
            recording: Arc::new(Mutex::new(Vec::new())),
            spec,
        })
    }

    pub fn start_recording(&self) -> Result<impl StreamTrait, Box<dyn std::error::Error>> {
        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .ok_or("Failed to get input device")?;

        // use the same configuration as Whisper
        let config = cpal::StreamConfig {
            channels: 1,
            sample_rate: cpal::SampleRate(16000),
            buffer_size: cpal::BufferSize::Default,
        };

        let recording = self.recording.clone();

        let stream = device.build_input_stream(
            &config,
            move |data: &[f32], _: &_| {
                let mut buffer = recording.lock().unwrap();
                for &sample in data {
                    // transform floating point audio samples to 16-bit integers
                    let sample = (sample * i16::MAX as f32) as i16;
                    buffer.push(sample);
                }
            },
            |err| eprintln!("Recording error: {}", err),
            None,
        )?;

        Ok(stream)
    }

    pub fn save_recording(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let buffer = self.recording.lock().unwrap();

        // build a WAV writer
        let mut writer = WavWriter::create(filename, self.spec)?;

        // write samples to the WAV file
        for &sample in buffer.iter() {
            writer.write_sample(sample)?;
        }

        writer.finalize()?;

        // check the file size and provide feedback
        if let Ok(metadata) = std::fs::metadata(filename) {
            println!(
                "
                Recording saved as a WAV file, size: {:.2} MB",
                metadata.len() as f64 / 1024.0 / 1024.0
            );
        }

        Ok(())
    }
}
