use dotenv::dotenv;
use std::env;

#[derive(Debug, Clone)]
pub struct EnvConfig {
    pub whisper_api_key: String,
    pub llm_api_key: String,
    pub recording_filename: String,
    pub output_filename: String,
}

impl EnvConfig {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        dotenv().ok();

        Ok(Self {
            whisper_api_key: env::var("WHISPER_API_KEY")
                .map_err(|_| "WHISPER_API_KEY not found in environment")?,
            llm_api_key: env::var("LLM_API_KEY")
                .map_err(|_| "LLM_API_KEY not found in environment")?,
            recording_filename: env::var("RECORDING_FILENAME")
                .unwrap_or_else(|_| "recording.wav".to_string()),
            output_filename: env::var("OUTPUT_FILENAME")
                .unwrap_or_else(|_| "output.md".to_string()),
        })
    }
}

pub fn update_gitignore() -> std::io::Result<()> {
    use std::fs::OpenOptions;
    use std::io::Write;

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(".gitignore")?;

    writeln!(file, "\n# Environment Variables")?;
    writeln!(file, ".env")?;
    writeln!(file, ".env.local")?;
    writeln!(file, "*.wav")?;
    writeln!(file, "*.md")?;

    Ok(())
}
