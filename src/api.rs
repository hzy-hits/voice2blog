use reqwest::multipart;
use serde::Deserialize;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Deserialize)]
pub struct WhisperResponse {
    text: String,
}

#[derive(Debug, Deserialize)]
pub struct LLMResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: Message,
}

#[derive(Debug, Deserialize)]
struct Message {
    content: String,
}
pub struct ApiClient<'a> {
    client: reqwest::Client,
    config: &'a crate::config::Config,
    env_config: &'a crate::env::EnvConfig,
}

impl<'a> ApiClient<'a> {
    pub fn new(config: &'a crate::config::Config, env_config: &'a crate::env::EnvConfig) -> Self {
        Self {
            client: reqwest::Client::new(),
            config,
            env_config,
        }
    }

    pub async fn transcribe_audio(
        &self,
        audio_path: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {

        // read the file content
        let mut file = File::open(audio_path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        // create a file part
        let part = multipart::Part::bytes(buffer)
            .file_name(audio_path.to_string())
            .mime_str("audio/wav")?;


        // create a form
        let form = multipart::Form::new()
            .part("file", part)
            .text("model", self.config.whisper_api.model.clone());


        // send the request
        let response = self
            .client
            .post(&self.config.whisper_api.url)
            .header(
                "Authorization",
                format!("Bearer {}", self.env_config.whisper_api_key),
            )
            .multipart(form)
            .send()
            .await?;

        // check the response status
        if !response.status().is_success() {
            return Err(format!(
                "API request failed with status: {}, body: {}",
                response.status(),
                response.text().await?
            )
            .into());
        }

        let whisper_response: WhisperResponse = response.json().await?;
        Ok(whisper_response.text)
    }

    pub async fn generate_markdown(
        &self,
        transcript: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let llm_prompt = self
            .config
            .llm_api
            .user_prompt_template
            .replace("{}", transcript);

        let response = self
            .client
            .post(&self.config.llm_api.url)
            .header(
                "Authorization",
                format!("Bearer {}", self.env_config.llm_api_key),
            )
            .json(&serde_json::json!({
                "model": self.config.llm_api.model,
                "messages": [
                    {
                        "role": "system",
                        "content": self.config.llm_api.system_prompt
                    },
                    {
                        "role": "user",
                        "content": llm_prompt
                    }
                ]
            }))
            .send()
            .await?;


        // check the response status
        if !response.status().is_success() {
            return Err(format!(
                "API request failed with status: {}, body: {}",
                response.status(),
                response.text().await?
            )
            .into());
        }

        let llm_response: LLMResponse = response.json().await?;
        Ok(llm_response.choices[0].message.content.clone())
    }
}
