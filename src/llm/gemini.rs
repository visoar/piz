use anyhow::{Context, Result};
use async_trait::async_trait;
use serde_json::json;

use super::LlmBackend;
use crate::config::GeminiConfig;

pub struct GeminiBackend {
    config: GeminiConfig,
    client: reqwest::Client,
}

impl GeminiBackend {
    pub fn new(config: GeminiConfig) -> Self {
        Self {
            config,
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl LlmBackend for GeminiBackend {
    async fn chat(&self, system: &str, user: &str) -> Result<String> {
        let base = self
            .config
            .base_url
            .as_deref()
            .unwrap_or("https://generativelanguage.googleapis.com");
        let url = format!(
            "{}/v1beta/models/{}:generateContent?key={}",
            base.trim_end_matches('/'),
            self.config.model,
            self.config.api_key
        );

        let body = json!({
            "system_instruction": {
                "parts": [{"text": system}]
            },
            "contents": [
                {
                    "role": "user",
                    "parts": [{"text": user}]
                }
            ],
            "generationConfig": {
                "temperature": 0.1
            }
        });

        let resp = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .context("Failed to send request to Gemini")?;

        let status = resp.status();
        let text = resp
            .text()
            .await
            .context("Failed to read Gemini response")?;

        if !status.is_success() {
            anyhow::bail!("Gemini API error ({}): {}", status, text);
        }

        let parsed: serde_json::Value =
            serde_json::from_str(&text).context("Failed to parse Gemini response")?;

        parsed["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| anyhow::anyhow!("Unexpected Gemini response format"))
    }
}
