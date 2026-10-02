use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Provider {
    Groq,
    Together,
    OpenRouter,
    Ollama,
}

impl Provider {
    pub fn base_url(&self) -> &str {
        match self {
            Provider::Groq => "https://api.groq.com/openai/v1",
            Provider::Together => "https://api.together.xyz/v1",
            Provider::OpenRouter => "https://openrouter.ai/api/v1",
            Provider::Ollama => "http://localhost:11434/v1",
        }
    }

    pub fn default_model(&self) -> &str {
        match self {
            Provider::Groq => "llama-3.1-8b-instant",
            Provider::Together => "meta-llama/Meta-Llama-3.1-8B-Instruct-Turbo",
            Provider::OpenRouter => "meta-llama/llama-3.1-8b-instruct:free",
            Provider::Ollama => "llama3.2:1b",
        }
    }

    pub fn env_key(&self) -> &str {
        match self {
            Provider::Groq => "GROQ_API_KEY",
            Provider::Together => "TOGETHER_API_KEY",
            Provider::OpenRouter => "OPENROUTER_API_KEY",
            Provider::Ollama => "", // No necesita API key
        }
    }
}

impl std::fmt::Display for Provider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Provider::Groq => write!(f, "Groq"),
            Provider::Together => write!(f, "Together"),
            Provider::OpenRouter => write!(f, "OpenRouter"),
            Provider::Ollama => write!(f, "Ollama (local)"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub provider: Provider,
    pub api_key: Option<String>,
    pub model: String,
    pub max_tokens: u32,
    pub temperature: f32,
    pub data_dir: PathBuf,
}

impl Config {
    pub fn load() -> Result<Self> {
        // Cargar .env si existe
        let _ = dotenvy::dotenv();

        let data_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("ai-agent");

        // Crear directorio de datos si no existe
        std::fs::create_dir_all(&data_dir)
            .context("No se pudo crear directorio de configuración")?;

        // Determinar proveedor desde variable de entorno o usar Groq por defecto
        let provider = std::env::var("AI_PROVIDER")
            .ok()
            .and_then(|p| match p.to_lowercase().as_str() {
                "groq" => Some(Provider::Groq),
                "together" => Some(Provider::Together),
                "openrouter" => Some(Provider::OpenRouter),
                "ollama" => Some(Provider::Ollama),
                _ => None,
            })
            .unwrap_or(Provider::Groq);

        // Obtener API key del proveedor seleccionado
        let api_key = if provider.env_key().is_empty() {
            None
        } else {
            std::env::var(provider.env_key())
                .ok()
                .filter(|k| !k.is_empty())
        };

        let model =
            std::env::var("AI_MODEL").unwrap_or_else(|_| provider.default_model().to_string());

        Ok(Config {
            provider,
            api_key,
            model,
            max_tokens: 1024,
            temperature: 0.7,
            data_dir,
        })
    }

    pub fn db_path(&self) -> PathBuf {
        self.data_dir.join("agent.db")
    }

    pub fn is_configured(&self) -> bool {
        // Ollama no necesita API key
        self.provider == Provider::Ollama || self.api_key.is_some()
    }
}
