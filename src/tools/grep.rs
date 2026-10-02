use anyhow::Result;
use async_trait::async_trait;
use serde_json::{json, Value};
use std::path::Path;
use std::process::Command;

use super::Tool;
use crate::providers::{FunctionDefinition, ToolDefinition};

pub struct GrepTool;

#[async_trait]
impl Tool for GrepTool {
    fn name(&self) -> &str {
        "grep"
    }

    fn description(&self) -> &str {
        "Busca texto en archivos de código fuente. Soporta expresiones regulares."
    }

    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            tool_type: "function".to_string(),
            function: FunctionDefinition {
                name: "grep".to_string(),
                description: self.description().to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "pattern": {
                            "type": "string",
                            "description": "Patrón de búsqueda (regex)"
                        },
                        "path": {
                            "type": "string",
                            "description": "Directorio o archivo donde buscar"
                        },
                        "include": {
                            "type": "string",
                            "description": "Filtro de archivos (ej: *.rs, *.py)"
                        },
                        "case_insensitive": {
                            "type": "boolean",
                            "description": "Ignorar mayúsculas/minúsculas"
                        }
                    },
                    "required": ["pattern"]
                }),
            },
        }
    }

    async fn execute(&self, args: Value) -> Result<String> {
        let pattern = args["pattern"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Falta 'pattern'"))?;

        let path = args["path"].as_str().unwrap_or(".");
        let include = args["include"].as_str();
        let case_insensitive = args["case_insensitive"].as_bool().unwrap_or(false);

        // Intentar usar ripgrep primero
        let mut cmd = Command::new("rg");
        cmd.arg("--no-heading");
        cmd.arg("--line-number");

        if case_insensitive {
            cmd.arg("-i");
        }

        if let Some(inc) = include {
            cmd.arg("-g").arg(inc);
        }

        cmd.arg(pattern);
        cmd.arg(path);

        let output = cmd.output();

        // Si ripgrep no está disponible, usar grep o búsqueda manual
        match output {
            Ok(out) if out.status.success() => {
                let stdout = String::from_utf8_lossy(&out.stdout).to_string();
                let lines: Vec<&str> = stdout.lines().take(50).collect();
                let total = stdout.lines().count();

                if lines.is_empty() {
                    Ok("No se encontraron resultados".to_string())
                } else {
                    Ok(format!(
                        "Encontrados {} resultados (mostrando {}):\n{}",
                        total,
                        lines.len(),
                        lines.join("\n")
                    ))
                }
            }
            _ => {
                // Fallback: búsqueda manual con find + grep
                self.fallback_search(pattern, path, include, case_insensitive)
            }
        }
    }
}

impl GrepTool {
    fn fallback_search(
        &self,
        pattern: &str,
        path: &str,
        include: Option<&str>,
        case_insensitive: bool,
    ) -> Result<String> {
        let base = Path::new(path);
        if !base.exists() {
            anyhow::bail!("Path no existe: {}", path);
        }

        let mut results = Vec::new();
        self.search_recursive(base, pattern, include, case_insensitive, 0, &mut results)?;

        if results.is_empty() {
            Ok("No se encontraron resultados".to_string())
        } else {
            let total = results.len();
            let display: Vec<String> = results.into_iter().take(50).collect();
            Ok(format!(
                "Encontrados {} resultados (mostrando {}):\n{}",
                total,
                display.len(),
                display.join("\n")
            ))
        }
    }

    fn search_recursive(
        &self,
        dir: &Path,
        pattern: &str,
        include: Option<&str>,
        case_insensitive: bool,
        depth: usize,
        results: &mut Vec<String>,
    ) -> Result<()> {
        if depth > 10 {
            return Ok(());
        }

        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();

                if path.is_dir() {
                    let name = path.file_name().unwrap_or_default().to_string_lossy();
                    if name.starts_with('.')
                        || name == "target"
                        || name == "node_modules"
                        || name == "dist"
                        || name == "__pycache__"
                    {
                        continue;
                    }
                    self.search_recursive(
                        &path,
                        pattern,
                        include,
                        case_insensitive,
                        depth + 1,
                        results,
                    )?;
                } else if path.is_file() {
                    // Check include filter
                    if let Some(inc) = include {
                        let file_name = path.file_name().unwrap_or_default().to_string_lossy();
                        if !glob_match_simple(inc, &file_name) {
                            continue;
                        }
                    }

                    // Try to read as text
                    if let Ok(content) = std::fs::read_to_string(&path) {
                        for (line_num, line) in content.lines().enumerate() {
                            let matches = if case_insensitive {
                                line.to_lowercase().contains(&pattern.to_lowercase())
                            } else {
                                line.contains(pattern)
                            };

                            if matches {
                                results.push(format!(
                                    "{}:{}: {}",
                                    path.display(),
                                    line_num + 1,
                                    line.trim()
                                ));

                                if results.len() >= 100 {
                                    return Ok(());
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

fn glob_match_simple(pattern: &str, name: &str) -> bool {
    if pattern.contains('*') {
        let parts: Vec<&str> = pattern.split('*').collect();
        if parts.len() == 2 {
            let prefix = parts[0];
            let suffix = parts[1];
            return name.starts_with(prefix) && name.ends_with(suffix);
        }
    }
    name == pattern
}
