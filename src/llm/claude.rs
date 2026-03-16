use anyhow::{Context, Result};
use async_trait::async_trait;
use serde_json::json;

use super::LlmBackend;
use crate::config::ClaudeConfig;

pub struct ClaudeBackend {
    config: ClaudeConfig,
    client: reqwest::Client,
}

impl ClaudeBackend {
    pub fn new(config: ClaudeConfig) -> Self {
        Self {
            config,
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl LlmBackend for ClaudeBackend {
    async fn chat(&self, system: &str, user: &str) -> Result<String> {
        let base = self
            .config
            .base_url
            .as_deref()
            .unwrap_or("https://api.anthropic.com");
        let url = format!("{}/v1/messages", base.trim_end_matches('/'));

        let body = json!({
            "model": self.config.model,
            "max_tokens": 1024,
            "system": system,
            "messages": [
                {"role": "user", "content": user}
            ]
        });

        let resp = self
            .client
            .post(&url)
            .header("x-api-key", &self.config.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .context("Failed to send request to Claude")?;

        let status = resp.status();
        let text = resp
            .text()
            .await
            .context("Failed to read Claude response")?;

        if !status.is_success() {
            anyhow::bail!("Claude API error ({}): {}", status, text);
        }

        let parsed: serde_json::Value =
            serde_json::from_str(&text).context("Failed to parse Claude response")?;

        parsed["content"][0]["text"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| anyhow::anyhow!("Unexpected Claude response format"))
    }
}
