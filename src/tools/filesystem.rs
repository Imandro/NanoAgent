use anyhow::Result;
use async_trait::async_trait;
use serde_json::{json, Value};
use std::fs;

use super::Tool;
use crate::providers::{FunctionDefinition, ToolDefinition};

pub struct ReadFileTool;

#[async_trait]
impl Tool for ReadFileTool {
    fn name(&self) -> &str {
        "read_file"
    }

    fn description(&self) -> &str {
        "Lee el contenido de un archivo de texto."
    }

    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            tool_type: "function".to_string(),
            function: FunctionDefinition {
                name: "read_file".to_string(),
                description: self.description().to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "path": {
                            "type": "string",
                            "description": "Ruta del archivo a leer"
                        }
                    },
                    "required": ["path"]
                }),
            },
        }
    }

    async fn execute(&self, args: Value) -> Result<String> {
        let path = args["path"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Falta el parámetro 'path'"))?;

        let content = fs::read_to_string(path)
            .map_err(|e| anyhow::anyhow!("Error leyendo '{}': {}", path, e))?;

        // Truncar si es muy largo
        if content.len() > 10000 {
            Ok(format!(
                "{}\n\n[... truncado, {} bytes totales]",
                &content[..10000],
                content.len()
            ))
        } else {
            Ok(content)
        }
    }
}

pub struct WriteFileTool;

#[async_trait]
impl Tool for WriteFileTool {
    fn name(&self) -> &str {
        "write_file"
    }

    fn description(&self) -> &str {
        "Escribe contenido a un archivo. Crea el archivo si no existe."
    }

    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            tool_type: "function".to_string(),
            function: FunctionDefinition {
                name: "write_file".to_string(),
                description: self.description().to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "path": {
                            "type": "string",
                            "description": "Ruta del archivo a escribir"
                        },
                        "content": {
                            "type": "string",
                            "description": "Contenido a escribir"
                        }
                    },
                    "required": ["path", "content"]
                }),
            },
        }
    }

    async fn execute(&self, args: Value) -> Result<String> {
        let path = args["path"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Falta el parámetro 'path'"))?;

        let content = args["content"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Falta el parámetro 'content'"))?;

        fs::write(path, content)
            .map_err(|e| anyhow::anyhow!("Error escribiendo '{}': {}", path, e))?;

        Ok(format!("Archivo '{}' escrito correctamente", path))
    }
}

pub struct ListDirTool;

#[async_trait]
impl Tool for ListDirTool {
    fn name(&self) -> &str {
        "list_dir"
    }

    fn description(&self) -> &str {
        "Lista los archivos y carpetas en un directorio."
    }

    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            tool_type: "function".to_string(),
            function: FunctionDefinition {
                name: "list_dir".to_string(),
                description: self.description().to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "path": {
                            "type": "string",
                            "description": "Ruta del directorio a listar"
                        }
                    },
                    "required": ["path"]
                }),
            },
        }
    }

    async fn execute(&self, args: Value) -> Result<String> {
        let path = args["path"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Falta el parámetro 'path'"))?;

        let entries = fs::read_dir(path)
            .map_err(|e| anyhow::anyhow!("Error leyendo directorio '{}': {}", path, e))?;

        let mut result = Vec::new();
        for entry in entries {
            let entry = entry?;
            let metadata = entry.metadata()?;
            let name = entry.file_name().to_string_lossy().to_string();

            if metadata.is_dir() {
                result.push(format!("[DIR]  {}", name));
            } else {
                result.push(format!("[FILE] {}", name));
            }
        }

        if result.is_empty() {
            Ok("(directorio vacío)".to_string())
        } else {
            Ok(result.join("\n"))
        }
    }
}
