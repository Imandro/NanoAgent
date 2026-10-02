use anyhow::Result;
use async_trait::async_trait;
use serde_json::{json, Value};
use std::process::Command;

use super::Tool;
use crate::providers::{FunctionDefinition, ToolDefinition};

pub struct ScheduleTool;

#[async_trait]
impl Tool for ScheduleTool {
    fn name(&self) -> &str {
        "schedule"
    }

    fn description(&self) -> &str {
        "Programa tareas para ejecutar en el futuro. Soporta: delay (retraso), cron (expresión), o at (hora específica)."
    }

    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            tool_type: "function".to_string(),
            function: FunctionDefinition {
                name: "schedule".to_string(),
                description: self.description().to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "action": {
                            "type": "string",
                            "enum": ["add", "list", "remove"],
                            "description": "Acción a realizar"
                        },
                        "type": {
                            "type": "string",
                            "enum": ["delay", "cron", "at"],
                            "description": "Tipo de programación"
                        },
                        "command": {
                            "type": "string",
                            "description": "Comando a ejecutar"
                        },
                        "delay_seconds": {
                            "type": "integer",
                            "description": "Segundos de retraso (para type=delay)"
                        },
                        "cron": {
                            "type": "string",
                            "description": "Expresión cron (para type=cron, ej: '0 9 * * *')"
                        },
                        "at": {
                            "type": "string",
                            "description": "Hora de ejecución (para type=at, ej: '14:30')"
                        },
                        "id": {
                            "type": "string",
                            "description": "ID de la tarea (para remove)"
                        }
                    },
                    "required": ["action"]
                }),
            },
        }
    }

    async fn execute(&self, args: Value) -> Result<String> {
        let action = args["action"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Falta el parámetro 'action'"))?;

        match action {
            "add" => self.add_task(&args).await,
            "list" => self.list_tasks().await,
            "remove" => {
                let id = args["id"]
                    .as_str()
                    .ok_or_else(|| anyhow::anyhow!("Falta el parámetro 'id'"))?;
                self.remove_task(id).await
            }
            _ => Err(anyhow::anyhow!("Acción desconocida: {}", action)),
        }
    }
}

impl ScheduleTool {
    async fn add_task(&self, args: &Value) -> Result<String> {
        let task_type = args["type"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Falta el parámetro 'type'"))?;

        let command = args["command"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Falta el parámetro 'command'"))?;

        let id = format!("task_{}", chrono_simple_id());

        match task_type {
            "delay" => {
                let seconds = args["delay_seconds"]
                    .as_u64()
                    .ok_or_else(|| anyhow::anyhow!("Falta delay_seconds"))?;

                #[cfg(target_os = "windows")]
                {
                    // Use PowerShell Start-Sleep + invoke
                    let ps_cmd = format!(
                        "Start-Sleep -Seconds {}; {}",
                        seconds, command
                    );
                    Command::new("powershell")
                        .args(["-Command", &format!(
                            "Start-Job -ScriptBlock {{ {} }} | Out-Null", ps_cmd
                        )])
                        .spawn()?;
                }

                #[cfg(not(target_os = "windows"))]
                {
                    Command::new("sh")
                        .args(["-c", &format!("({}s) && {} &", seconds, command)])
                        .spawn()?;
                }

                Ok(format!("Tarea '{}' programada para ejecutarse en {} segundos", id, seconds))
            }
            "cron" => {
                let cron_expr = args["cron"]
                    .as_str()
                    .ok_or_else(|| anyhow::anyhow!("Falta cron expression"))?;

                // Check if crontab exists and add entry
                #[cfg(not(target_os = "windows"))]
                {
                    let entry = format!("{} {} # nano-agent:{}", 
                        cron_expr, command, id);
                    Command::new("sh")
                        .args(["-c", &format!("(crontab -l 2>/dev/null; echo '{}') | crontab -", entry)])
                        .spawn()?;
                    Ok(format!("Tarea '{}' programada con cron: {}", id, cron_expr))
                }

                #[cfg(target_os = "windows")]
                {
                    // Use Windows Task Scheduler
                    let result = Command::new("schtasks")
                        .args([
                            "/create",
                            "/tn", &format!("NanoAgent_{}", id),
                            "/tr", command,
                            "/sc", "daily",
                            "/st", "09:00",
                        ])
                        .output()?;

                    if result.status.success() {
                        Ok(format!("Tarea '{}' creada en Windows Task Scheduler", id))
                    } else {
                        let err = String::from_utf8_lossy(&result.stderr);
                        Err(anyhow::anyhow!("Error creando tarea: {}", err))
                    }
                }
            }
            "at" => {
                let time = args["at"]
                    .as_str()
                    .ok_or_else(|| anyhow::anyhow!("Falta hora 'at'"))?;

                #[cfg(not(target_os = "windows"))]
                {
                    Command::new("sh")
                        .args(["-c", &format!("echo '{}' | at {}", command, time)])
                        .spawn()?;
                    Ok(format!("Tarea '{}' programada para las {}", id, time))
                }

                #[cfg(target_os = "windows")]
                {
                    // Convert time to schtasks format
                    let result = Command::new("schtasks")
                        .args([
                            "/create",
                            "/tn", &format!("NanoAgent_{}", id),
                            "/tr", command,
                            "/sc", "daily",
                            "/st", time,
                        ])
                        .output()?;

                    if result.status.success() {
                        Ok(format!("Tarea '{}' programada para las {}", id, time))
                    } else {
                        let err = String::from_utf8_lossy(&result.stderr);
                        Err(anyhow::anyhow!("Error: {}", err))
                    }
                }
            }
            _ => Err(anyhow::anyhow!("Tipo de tarea desconocido: {}", task_type)),
        }
    }

    async fn list_tasks(&self) -> Result<String> {
        #[cfg(not(target_os = "windows"))]
        {
            let output = Command::new("crontab").args(["-l"]).output()?;
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            if stdout.contains("nano-agent:") {
                let tasks: Vec<&str> = stdout
                    .lines()
                    .filter(|l| l.contains("nano-agent:"))
                    .collect();
                Ok(format!("Tareas programadas:\n{}", tasks.join("\n")))
            } else {
                Ok("No hay tareas programadas".to_string())
            }
        }

        #[cfg(target_os = "windows")]
        {
            let output = Command::new("schtasks")
                .args(["/query", "/fo", "LIST"])
                .output()?;
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let tasks: Vec<&str> = stdout
                .split("\n\n")
                .filter(|s| s.contains("NanoAgent_"))
                .collect();
            if tasks.is_empty() {
                Ok("No hay tareas NanoAgent programadas".to_string())
            } else {
                Ok(format!("Tareas NanoAgent:\n{}", tasks.join("\n\n")))
            }
        }
    }

    async fn remove_task(&self, id: &str) -> Result<String> {
        #[cfg(not(target_os = "windows"))]
        {
            let output = Command::new("crontab").args(["-l"]).output()?;
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let new_crontab: String = stdout
                .lines()
                .filter(|l| !l.contains(&format!("nano-agent:{}", id)))
                .collect::<Vec<&str>>()
                .join("\n");

            Command::new("sh")
                .args(["-c", &format!("echo '{}' | crontab -", new_crontab)])
                .spawn()?;
            Ok(format!("Tarea '{}' eliminada", id))
        }

        #[cfg(target_os = "windows")]
        {
            let output = Command::new("schtasks")
                .args(["/delete", "/tn", &format!("NanoAgent_{}", id), "/f"])
                .output()?;
            if output.status.success() {
                Ok(format!("Tarea '{}' eliminada", id))
            } else {
                Err(anyhow::anyhow!("Error eliminando tarea"))
            }
        }
    }
}

fn chrono_simple_id() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}
