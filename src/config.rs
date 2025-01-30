use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub whisper_api: WhisperConfig,
    pub llm_api: LlmConfig,
}

#[derive(Debug, Deserialize)]
pub struct WhisperConfig {
    pub url: String,
    pub model: String,
}

#[derive(Debug, Deserialize)]
pub struct LlmConfig {
    pub url: String,
    pub model: String,
    pub system_prompt: String,
    pub user_prompt_template: String,
}
impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let whisper_config: WhisperConfig =
            serde_json::from_str(&fs::read_to_string("config/whisper.json")?)?;
        let llm_config: LlmConfig = serde_json::from_str(&fs::read_to_string("config/llm.json")?)?;

        Ok(Config {
            whisper_api: whisper_config,
            llm_api: llm_config,
        })
    }
}
