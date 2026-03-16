use anyhow::{Context, Result};
use async_trait::async_trait;
use serde_json::json;

use super::LlmBackend;
use crate::config::OpenAiConfig;

pub struct OpenAiBackend {
    config: OpenAiConfig,
    client: reqwest::Client,
}

impl OpenAiBackend {
    pub fn new(config: OpenAiConfig) -> Self {
        Self {
            config,
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl LlmBackend for OpenAiBackend {
    async fn chat(&self, system: &str, user: &str) -> Result<String> {
        let base_url = self
            .config
            .base_url
            .as_deref()
            .unwrap_or("https://api.openai.com");
        let url = format!("{}/v1/chat/completions", base_url.trim_end_matches('/'));

        let body = json!({
            "model": self.config.model,
            "messages": [
                {"role": "system", "content": system},
                {"role": "user", "content": user}
            ],
            "temperature": 0.1
        });

        let resp = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .context("Failed to send request to OpenAI")?;

        let status = resp.status();
        let text = resp
            .text()
            .await
            .context("Failed to read OpenAI response")?;

        if !status.is_success() {
            anyhow::bail!("OpenAI API error ({}): {}", status, text);
        }

        let parsed: serde_json::Value =
            serde_json::from_str(&text).context("Failed to parse OpenAI response")?;

        parsed["choices"][0]["message"]["content"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| anyhow::anyhow!("Unexpected OpenAI response format"))
    }
}
