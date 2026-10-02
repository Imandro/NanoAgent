use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::config::{Config, Provider};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub id: String,
    pub function: FunctionCall,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    #[serde(rename = "type")]
    pub tool_type: String,
    pub function: FunctionDefinition,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionDefinition {
    pub name: String,
    pub description: String,
    #[serde(rename = "parameters")]
    pub parameters: serde_json::Value,
}

#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    max_tokens: u32,
    temperature: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<ToolDefinition>>,
}

#[derive(Debug, Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: ResponseMessage,
}

#[derive(Debug, Deserialize)]
struct ResponseMessage {
    content: Option<String>,
    #[serde(default)]
    tool_calls: Vec<ToolCall>,
}

pub struct LlmClient {
    http: Client,
    config: Config,
}

impl LlmClient {
    pub fn new(config: Config) -> Self {
        let http = Client::builder()
            .timeout(std::time::Duration::from_secs(120))
            .build()
            .expect("No se pudo crear HTTP client");

        LlmClient { http, config }
    }

    pub async fn chat(
        &self,
        messages: &[Message],
        tools: Option<&[ToolDefinition]>,
    ) -> Result<(Option<String>, Vec<ToolCall>)> {
        let url = format!("{}/chat/completions", self.config.provider.base_url());

        let mut request = ChatRequest {
            model: self.config.model.clone(),
            messages: messages.to_vec(),
            max_tokens: self.config.max_tokens,
            temperature: self.config.temperature,
            tools: tools.map(|t| t.to_vec()),
        };

        // Ollama no soporta max_tokens igual
        if self.config.provider == Provider::Ollama {
            request.max_tokens = 0;
        }

        let mut req = self.http.post(&url);

        // Agregar API key si existe
        if let Some(ref key) = self.config.api_key {
            req = req.bearer_auth(key);
        }

        // OpenRouter requiere header adicional
        if self.config.provider == Provider::OpenRouter {
            req = req.header("HTTP-Referer", "https://github.com/ai-agent");
            req = req.header("X-Title", "AI Agent");
        }

        let response = req
            .json(&request)
            .send()
            .await
            .context("Error al conectar con la API")?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            anyhow::bail!("Error API ({}): {}", status, error_text);
        }

        let chat_response: ChatResponse = response
            .json()
            .await
            .context("Error al parsear respuesta de la API")?;

        let choice = chat_response
            .choices
            .into_iter()
            .next()
            .context("No se recibió respuesta del modelo")?;

        Ok((choice.message.content, choice.message.tool_calls))
    }
}
