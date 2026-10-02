use anyhow::Result;
use async_trait::async_trait;
use serde_json::{json, Value};
use std::path::Path;

use super::Tool;
use crate::providers::{FunctionDefinition, ToolDefinition};

pub struct SearchTool;

#[async_trait]
impl Tool for SearchTool {
    fn name(&self) -> &str {
        "search_files"
    }

    fn description(&self) -> &str {
        "Busca archivos por nombre, patrón o contenido. Útil para encontrar archivos en el sistema."
    }

    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            tool_type: "function".to_string(),
            function: FunctionDefinition {
                name: "search_files".to_string(),
                description: self.description().to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "path": {
                            "type": "string",
                            "description": "Directorio base para buscar"
                        },
                        "pattern": {
                            "type": "string",
                            "description": "Patrón de nombre (ej: *.txt, *.rs)"
                        },
                        "contains": {
                            "type": "string",
                            "description": "Buscar archivos que contengan este texto"
                        },
                        "max_depth": {
                            "type": "integer",
                            "description": "Profundidad máxima de búsqueda (default: 5)"
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

        let pattern = args["pattern"].as_str();
        let contains = args["contains"].as_str();
        let max_depth = args["max_depth"].as_u64().unwrap_or(5) as usize;

        let base = Path::new(path);
        if !base.exists() {
            anyhow::bail!("El directorio '{}' no existe", path);
        }

        let mut results = Vec::new();
        self.search_recursive(base, pattern, contains, max_depth, 0, &mut results)?;

        if results.is_empty() {
            Ok("No se encontraron archivos que coincidan con la búsqueda".to_string())
        } else {
            let total = results.len();
            let display: Vec<String> = results.into_iter().take(50).collect();
            Ok(format!(
                "Encontrados {} archivos (mostrando primeros 50):\n{}",
                total,
                display.join("\n")
            ))
        }
    }
}

impl SearchTool {
    fn search_recursive(
        &self,
        dir: &Path,
        pattern: Option<&str>,
        contains: Option<&str>,
        max_depth: usize,
        current_depth: usize,
        results: &mut Vec<String>,
    ) -> Result<()> {
        if current_depth > max_depth {
            return Ok(());
        }

        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();

                if path.is_dir() {
                    // Skip hidden dirs and target dir
                    let name = path.file_name().unwrap_or_default().to_string_lossy();
                    if name.starts_with('.') || name == "target" || name == "node_modules" {
                        continue;
                    }
                    self.search_recursive(
                        &path,
                        pattern,
                        contains,
                        max_depth,
                        current_depth + 1,
                        results,
                    )?;
                } else if path.is_file() {
                    let matches = if let Some(pat) = pattern {
                        let file_name = path.file_name().unwrap_or_default().to_string_lossy();
                        glob_match(pat, &file_name)
                    } else {
                        true
                    };

                    let matches_content = if let Some(text) = contains {
                        if let Ok(content) = std::fs::read_to_string(&path) {
                            content.contains(text)
                        } else {
                            false
                        }
                    } else {
                        true
                    };

                    if matches && matches_content {
                        results.push(path.display().to_string());
                    }
                }
            }
        }

        Ok(())
    }
}

fn glob_match(pattern: &str, name: &str) -> bool {
    // Simple glob matching: *.txt, file?, etc.
    let pattern_lower = pattern.to_lowercase();
    let name_lower = name.to_lowercase();

    if pattern.contains('*') {
        let parts: Vec<&str> = pattern_lower.split('*').collect();
        if parts.len() == 2 {
            let prefix = parts[0];
            let suffix = parts[1];
            return name_lower.starts_with(prefix) && name_lower.ends_with(suffix);
        }
    } else if pattern.contains('?') {
        if name_lower.len() == pattern_lower.len() {
            return pattern_lower
                .chars()
                .zip(name_lower.chars())
                .all(|(p, n)| p == '?' || p == n);
        }
        return false;
    }

    name_lower == pattern_lower
}
