use anyhow::Result;
use async_trait::async_trait;
use serde_json::{json, Value};
use std::process::Command;

use super::Tool;
use crate::providers::{FunctionDefinition, ToolDefinition};

pub struct ProcessTool;

#[async_trait]
impl Tool for ProcessTool {
    fn name(&self) -> &str {
        "process"
    }

    fn description(&self) -> &str {
        "Gestiona procesos del sistema: listar, matar, o ejecutar en background."
    }

    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            tool_type: "function".to_string(),
            function: FunctionDefinition {
                name: "process".to_string(),
                description: self.description().to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "action": {
                            "type": "string",
                            "enum": ["list", "kill", "run_background"],
                            "description": "Acción a realizar"
                        },
                        "pid": {
                            "type": "integer",
                            "description": "PID del proceso (para kill)"
                        },
                        "name": {
                            "type": "string",
                            "description": "Nombre del proceso (para kill por nombre)"
                        },
                        "command": {
                            "type": "string",
                            "description": "Comando a ejecutar en background"
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
            "list" => self.list_processes().await,
            "kill" => {
                let pid = args["pid"].as_i64();
                let name = args["name"].as_str();
                self.kill_process(pid, name).await
            }
            "run_background" => {
                let cmd = args["command"]
                    .as_str()
                    .ok_or_else(|| anyhow::anyhow!("Falta el parámetro 'command'"))?;
                self.run_background(cmd).await
            }
            _ => Err(anyhow::anyhow!("Acción desconocida: {}", action)),
        }
    }
}

impl ProcessTool {
    async fn list_processes(&self) -> Result<String> {
        let output = if cfg!(target_os = "windows") {
            Command::new("tasklist")
                .args(["/FO", "CSV", "/NH"])
                .output()?
        } else {
            Command::new("ps").args(["aux", "--sort=-pcpu"]).output()?
        };

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let lines: Vec<&str> = stdout.lines().take(20).collect();
        Ok(format!("Top 20 procesos:\n{}", lines.join("\n")))
    }

    async fn kill_process(&self, pid: Option<i64>, name: Option<&str>) -> Result<String> {
        if let Some(pid) = pid {
            let output = if cfg!(target_os = "windows") {
                Command::new("taskkill")
                    .args(["/PID", &pid.to_string(), "/F"])
                    .output()?
            } else {
                Command::new("kill")
                    .args(["-9", &pid.to_string()])
                    .output()?
            };
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            Ok(format!("{} {}", stdout, stderr).trim().to_string())
        } else if let Some(name) = name {
            let output = if cfg!(target_os = "windows") {
                Command::new("taskkill")
                    .args(["/IM", name, "/F"])
                    .output()?
            } else {
                Command::new("pkill").args(["-9", name]).output()?
            };
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            Ok(format!("{} {}", stdout, stderr).trim().to_string())
        } else {
            Err(anyhow::anyhow!("Proporciona pid o name"))
        }
    }

    async fn run_background(&self, command: &str) -> Result<String> {
        #[cfg(target_os = "windows")]
        {
            Command::new("cmd")
                .args(["/C", "start", "/B", command])
                .spawn()?;
            Ok(format!("Comando ejecutado en background: {}", command))
        }

        #[cfg(not(target_os = "windows"))]
        {
            use std::os::unix::process::CommandExt;
            Command::new("sh")
                .args(["-c", &format!("nohup {} > /dev/null 2>&1 &", command)])
                .spawn()?;
            Ok(format!("Comando ejecutado en background: {}", command))
        }
    }
}
