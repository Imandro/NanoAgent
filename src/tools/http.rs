use anyhow::Result;
use async_trait::async_trait;
use reqwest::Client;
use serde_json::{json, Value};

use super::Tool;
use crate::providers::{FunctionDefinition, ToolDefinition};

pub struct HttpTool;

#[async_trait]
impl Tool for HttpTool {
    fn name(&self) -> &str {
        "http_request"
    }

    fn description(&self) -> &str {
        "Realiza una petición HTTP. Útil para acceder a APIs, sitios web, etc."
    }

    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            tool_type: "function".to_string(),
            function: FunctionDefinition {
                name: "http_request".to_string(),
                description: self.description().to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "url": {
                            "type": "string",
                            "description": "URL a la que hacer la petición"
                        },
                        "method": {
                            "type": "string",
                            "enum": ["GET", "POST", "PUT", "DELETE"],
                            "description": "Método HTTP (default: GET)"
                        },
                        "body": {
                            "type": "string",
                            "description": "Body de la petición (para POST/PUT)"
                        }
                    },
                    "required": ["url"]
                }),
            },
        }
    }

    async fn execute(&self, args: Value) -> Result<String> {
        let url = args["url"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Falta el parámetro 'url'"))?;

        let method = args["method"].as_str().unwrap_or("GET");
        let body = args["body"].as_str();

        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()?;

        let mut req = match method.to_uppercase().as_str() {
            "GET" => client.get(url),
            "POST" => client.post(url),
            "PUT" => client.put(url),
            "DELETE" => client.delete(url),
            _ => client.get(url),
        };

        if let Some(body) = body {
            req = req
                .header("Content-Type", "application/json")
                .body(body.to_string());
        }

        let response = req.send().await?;
        let status = response.status();
        let content = response.text().await?;

        // Truncar respuesta larga
        let display = if content.len() > 5000 {
            format!(
                "Status: {}\n{}\n\n[... truncado, {} bytes totales]",
                status,
                &content[..5000],
                content.len()
            )
        } else {
            format!("Status: {}\n{}", status, content)
        };

        Ok(display)
    }
}
