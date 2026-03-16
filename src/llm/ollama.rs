use anyhow::{Context, Result};
use async_trait::async_trait;
use serde_json::json;

use super::LlmBackend;
use crate::config::OllamaConfig;

pub struct OllamaBackend {
    config: OllamaConfig,
    client: reqwest::Client,
}

impl OllamaBackend {
    pub fn new(config: OllamaConfig) -> Self {
        Self {
            config,
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl LlmBackend for OllamaBackend {
    async fn chat(&self, system: &str, user: &str) -> Result<String> {
        let url = format!("{}/api/chat", self.config.host.trim_end_matches('/'));

        let body = json!({
            "model": self.config.model,
            "messages": [
                {"role": "system", "content": system},
                {"role": "user", "content": user}
            ],
            "stream": false
        });

        let resp = self
            .client
            .post(&url)
            .json(&body)
            .send()
            .await
            .context("Failed to send request to Ollama")?;

        let status = resp.status();
        let text = resp
            .text()
            .await
            .context("Failed to read Ollama response")?;

        if !status.is_success() {
            anyhow::bail!("Ollama API error ({}): {}", status, text);
        }

        let parsed: serde_json::Value =
            serde_json::from_str(&text).context("Failed to parse Ollama response")?;

        parsed["message"]["content"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| anyhow::anyhow!("Unexpected Ollama response format"))
    }
}
