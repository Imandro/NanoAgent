use anyhow::Result;
use async_trait::async_trait;
use serde_json::{json, Value};
use std::fs;

use super::Tool;
use crate::providers::{FunctionDefinition, ToolDefinition};

pub struct EditFileTool;

#[async_trait]
impl Tool for EditFileTool {
    fn name(&self) -> &str {
        "edit_file"
    }

    fn description(&self) -> &str {
        "Edita un archivo reemplazando texto específico. Útil para modificar código existente."
    }

    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            tool_type: "function".to_string(),
            function: FunctionDefinition {
                name: "edit_file".to_string(),
                description: self.description().to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "path": {
                            "type": "string",
                            "description": "Ruta del archivo a editar"
                        },
                        "old_text": {
                            "type": "string",
                            "description": "Texto exacto a reemplazar (debe coincidir exactamente)"
                        },
                        "new_text": {
                            "type": "string",
                            "description": "Nuevo texto que reemplazará al anterior"
                        }
                    },
                    "required": ["path", "old_text", "new_text"]
                }),
            },
        }
    }

    async fn execute(&self, args: Value) -> Result<String> {
        let path = args["path"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Falta 'path'"))?;

        let old_text = args["old_text"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Falta 'old_text'"))?;

        let new_text = args["new_text"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Falta 'new_text'"))?;

        let content = fs::read_to_string(path)
            .map_err(|e| anyhow::anyhow!("Error leyendo '{}': {}", path, e))?;

        if !content.contains(old_text) {
            anyhow::bail!(
                "No se encontró el texto en '{}'. Verifica que el texto coincida exactamente.",
                path
            );
        }

        let new_content = content.replacen(old_text, new_text, 1);
        fs::write(path, &new_content)?;

        // Calcular stats del cambio
        let old_lines = old_text.lines().count();
        let new_lines = new_text.lines().count();
        let diff = new_lines as i64 - old_lines as i64;

        Ok(format!(
            "Editado '{}' ({} líneas -> {} líneas, {:+} líneas)",
            path, old_lines, new_lines, diff
        ))
    }
}

pub struct PatchFileTool;

#[async_trait]
impl Tool for PatchFileTool {
    fn name(&self) -> &str {
        "patch_file"
    }

    fn description(&self) -> &str {
        "Aplica un patch con múltiples ediciones a un archivo. Cada línea con + es agregada, - es eliminada."
    }

    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            tool_type: "function".to_string(),
            function: FunctionDefinition {
                name: "patch_file".to_string(),
                description: self.description().to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "path": {
                            "type": "string",
                            "description": "Ruta del archivo a editar"
                        },
                        "operations": {
                            "type": "array",
                            "items": {
                                "type": "object",
                                "properties": {
                                    "type": {
                                        "type": "string",
                                        "enum": ["replace", "insert", "delete"],
                                        "description": "Tipo de operación"
                                    },
                                    "find": {
                                        "type": "string",
                                        "description": "Texto a buscar (para replace/delete)"
                                    },
                                    "content": {
                                        "type": "string",
                                        "description": "Contenido nuevo (para replace/insert)"
                                    },
                                    "line": {
                                        "type": "integer",
                                        "description": "Número de línea (para insert)"
                                    }
                                },
                                "required": ["type"]
                            },
                            "description": "Lista de operaciones a realizar"
                        }
                    },
                    "required": ["path", "operations"]
                }),
            },
        }
    }

    async fn execute(&self, args: Value) -> Result<String> {
        let path = args["path"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Falta 'path'"))?;

        let operations = args["operations"]
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("Falta 'operations'"))?;

        let mut content = fs::read_to_string(path)
            .map_err(|e| anyhow::anyhow!("Error leyendo '{}': {}", path, e))?;

        let mut changes = 0;

        for op in operations {
            let op_type = op["type"]
                .as_str()
                .ok_or_else(|| anyhow::anyhow!("Operación sin 'type'"))?;

            match op_type {
                "replace" => {
                    let find = op["find"]
                        .as_str()
                        .ok_or_else(|| anyhow::anyhow!("replace requiere 'find'"))?;
                    let replace_with = op["content"]
                        .as_str()
                        .unwrap_or("");

                    if content.contains(find) {
                        content = content.replacen(find, replace_with, 1);
                        changes += 1;
                    }
                }
                "insert" => {
                    let line = op["line"]
                        .as_u64()
                        .ok_or_else(|| anyhow::anyhow!("insert requiere 'line'"))? as usize;
                    let insert_content = op["content"]
                        .as_str()
                        .unwrap_or("");

                    let lines: Vec<&str> = content.lines().collect();
                    let mut new_lines = Vec::new();
                    for (i, l) in lines.iter().enumerate() {
                        if i == line - 1 {
                            new_lines.push(insert_content);
                        }
                        new_lines.push(l);
                    }
                    content = new_lines.join("\n");
                    changes += 1;
                }
                "delete" => {
                    let find = op["find"]
                        .as_str()
                        .ok_or_else(|| anyhow::anyhow!("delete requiere 'find'"))?;

                    content = content.replace(find, "");
                    changes += 1;
                }
                _ => {
                    anyhow::bail!("Operación desconocida: {}", op_type);
                }
            }
        }

        fs::write(path, &content)?;

        Ok(format!(
            "Archivo '{}' actualizado con {} operaciones",
            path, changes
        ))
    }
}
