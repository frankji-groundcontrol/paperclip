use std::time::Duration;

use async_trait::async_trait;
use serde_json::{json, Value};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LlmUsage {
    pub input_tokens: i64,
    pub output_tokens: i64,
    pub total_tokens: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LlmAnswer {
    pub text: String,
    pub model: String,
    pub usage: LlmUsage,
}

#[async_trait]
pub trait LlmClient: Send + Sync {
    async fn respond(&self, model: &str, prompt: &str) -> anyhow::Result<LlmAnswer>;

    /// The model used when a caller does not specify one (honors OPENAI_MODEL).
    fn default_model(&self) -> &str {
        "gpt-5.4-mini"
    }
}

pub fn parse_responses_sse(body: &str) -> anyhow::Result<LlmAnswer> {
    let mut text = String::new();
    let mut completed: Option<LlmAnswer> = None;

    for line in body.split('\n') {
        let Some(data) = line.strip_prefix("data:") else {
            continue;
        };
        let data = data.strip_prefix(' ').unwrap_or(data).trim_end_matches('\r');
        if data == "[DONE]" {
            continue;
        }
        let Ok(event) = serde_json::from_str::<Value>(data) else {
            continue;
        };
        match event.get("type").and_then(Value::as_str) {
            Some("response.output_text.delta") => {
                if let Some(delta) = event.get("delta").and_then(Value::as_str) {
                    text.push_str(delta);
                }
            }
            Some("response.completed") => {
                let response = event
                    .get("response")
                    .ok_or_else(|| anyhow::anyhow!("missing response"))?;
                let usage = response
                    .get("usage")
                    .ok_or_else(|| anyhow::anyhow!("missing usage"))?;
                completed = Some(LlmAnswer {
                    text: text.clone(),
                    model: string_field(response, "model")?,
                    usage: LlmUsage {
                        input_tokens: int_field(usage, "input_tokens")?,
                        output_tokens: int_field(usage, "output_tokens")?,
                        total_tokens: int_field(usage, "total_tokens")?,
                    },
                });
            }
            Some("response.failed") | Some("response.error") => {
                let code = event
                    .get("response")
                    .and_then(|r| r.get("error"))
                    .and_then(|e| e.get("code"))
                    .and_then(Value::as_str)
                    .unwrap_or("unknown");
                anyhow::bail!("response failed: {code}")
            }
            Some("response.incomplete") => {
                anyhow::bail!("response incomplete")
            }
            _ => {}
        }
    }

    completed.ok_or_else(|| anyhow::anyhow!("no terminal event"))
}

/// Whether an LLM error is worth retrying (provider concurrency/rate limits).
pub fn is_retryable_llm_error(message: &str) -> bool {
    let m = message.to_ascii_lowercase();
    m.contains("rate_limit") || m.contains("concurrency limit") || m.contains("429")
}

fn string_field(value: &Value, name: &str) -> anyhow::Result<String> {
    value
        .get(name)
        .and_then(Value::as_str)
        .map(ToString::to_string)
        .ok_or_else(|| anyhow::anyhow!("missing {name}"))
}

fn int_field(value: &Value, name: &str) -> anyhow::Result<i64> {
    value
        .get(name)
        .and_then(Value::as_i64)
        .ok_or_else(|| anyhow::anyhow!("missing {name}"))
}

#[derive(Clone)]
pub struct OpenAiResponsesClient {
    base_url: String,
    api_key: String,
    default_model: String,
    client: reqwest::Client,
}

impl OpenAiResponsesClient {
    pub fn from_env() -> anyhow::Result<Self> {
        let base_url = std::env::var("OPENAI_BASE_URL")?;
        let api_key = std::env::var("OPENAI_API_KEY")?;
        let default_model = std::env::var("OPENAI_MODEL")
            .ok()
            .filter(|model| !model.trim().is_empty())
            .unwrap_or_else(|| "gpt-5.4-mini".to_string());
        let timeout_secs = std::env::var("OPENAI_TIMEOUT_SECS")
            .ok()
            .and_then(|value| value.parse().ok())
            .unwrap_or(120);
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(timeout_secs))
            .build()?;
        Ok(Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            api_key,
            default_model,
            client,
        })
    }

    pub fn default_model(&self) -> &str {
        &self.default_model
    }
}

#[async_trait]
impl LlmClient for OpenAiResponsesClient {
    fn default_model(&self) -> &str {
        &self.default_model
    }

    async fn respond(&self, model: &str, prompt: &str) -> anyhow::Result<LlmAnswer> {
        let model = if model.trim().is_empty() {
            self.default_model()
        } else {
            model
        };
        let payload = json!({
            "model": model,
            "input": [{ "role": "user", "content": prompt }],
            "stream": true,
        });
        // Retry provider concurrency/rate limits with exponential backoff.
        let max_attempts: u32 = 4;
        let mut last_err: Option<anyhow::Error> = None;
        for attempt in 0..max_attempts {
            if attempt > 0 {
                let backoff = Duration::from_millis(500u64 * (1u64 << attempt)); // 1s, 2s, 4s
                tokio::time::sleep(backoff).await;
            }
            let sent = self
                .client
                .post(format!("{}/v1/responses", self.base_url))
                .bearer_auth(&self.api_key)
                .json(&payload)
                .send()
                .await;
            let response = match sent {
                Ok(response) => response,
                Err(err) => {
                    last_err = Some(err.into());
                    continue;
                }
            };
            let status = response.status();
            if status.as_u16() == 429 {
                last_err = Some(anyhow::anyhow!("OpenAI request failed with {status}"));
                continue;
            }
            if !status.is_success() {
                anyhow::bail!("OpenAI request failed with {status}");
            }
            let body = response.text().await?;
            match parse_responses_sse(&body) {
                Ok(answer) => return Ok(answer),
                Err(err) => {
                    if is_retryable_llm_error(&err.to_string()) {
                        last_err = Some(err);
                        continue;
                    }
                    return Err(err);
                }
            }
        }
        Err(last_err.unwrap_or_else(|| anyhow::anyhow!("OpenAI request failed after retries")))
    }
}

#[derive(Debug, Clone, Default)]
pub struct DisabledLlmClient;

#[async_trait]
impl LlmClient for DisabledLlmClient {
    async fn respond(&self, _model: &str, _prompt: &str) -> anyhow::Result<LlmAnswer> {
        anyhow::bail!("OpenAI is not configured")
    }
}

