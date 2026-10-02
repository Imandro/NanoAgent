use anyhow::Result;
use colored::*;
use serde_json::Value;
use tokio::time::{sleep, Duration};

use crate::config::Config;
use crate::db::store::Store;
use crate::permissions::{PermissionLevel, Permissions};
use crate::providers::{LlmClient, Message, ToolCall};
use crate::tools::ToolRegistry;

const SYSTEM_PROMPT: &str = "Eres NanoAgent, un asistente de IA para programación inspirado en Claude Code y OpenCode.

CAPACIDADES PRINCIPALES:
- Leer, crear, modificar y eliminar archivos de código fuente
- Ejecutar comandos de terminal (build, test, run, etc.)
- Buscar en código con expresiones regulares (grep)
- Editar archivos de forma precisa con diff/patch
- Navegar la estructura del proyecto
- Ejecutar scripts y comandos del sistema

INSTRUCCIONES:
1. Analiza el código existente antes de hacer cambios
2. Usa herramientas de lectura para entender el contexto
3. Edita archivos de forma precisa (edit_file, patch_file)
4. Verifica los cambios ejecutando tests o builds
5. Responde en español con explicaciones concisas
6. Sé conciso: muestra el código relevante, no todo el archivo
7. Si hay errores, investiga la causa antes de sugerir soluciones

COMPORTAMIENTO:
- Sé directo y eficiente
- Muestra fragmentos de código relevantes
- Explica brevemente qué estás haciendo
- Si el usuario pide algo ambiguo, pregunta para clarificar
- Usa /help para ver comandos disponibles

Límite de 15 pasos de herramientas por mensaje.";

pub struct Agent {
    llm: LlmClient,
    tools: ToolRegistry,
    store: Store,
    permissions: Permissions,
    conversation_id: i64,
    system_messages: Vec<Message>,
    working_dir: String,
}

impl Agent {
    pub fn new(config: Config, permissions: Permissions) -> Result<Self> {
        let db_path = config.db_path();
        let store = Store::open(&db_path)?;
        let conversation_id = store.new_conversation()?;

        let llm = LlmClient::new(config.clone());
        let tools = ToolRegistry::new(&config.data_dir);

        let working_dir = std::env::current_dir()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|_| ".".to_string());

        let system_messages = vec![Message {
            role: "system".to_string(),
            content: format!("{}\n\nDirectorio de trabajo actual: {}", SYSTEM_PROMPT, working_dir),
            tool_call_id: None,
        }];

        Ok(Agent {
            llm,
            tools,
            store,
            permissions,
            conversation_id,
            system_messages,
            working_dir,
        })
    }

    pub async fn process_message(&mut self, user_input: &str) -> Result<String> {
        let user_msg = Message {
            role: "user".to_string(),
            content: user_input.to_string(),
            tool_call_id: None,
        };
        self.store.add_message(self.conversation_id, &user_msg)?;

        let mut messages = self.system_messages.clone();
        let history = self.store.get_messages(self.conversation_id, 25)?;
        messages.extend(history);

        let max_tool_steps = 15;
        let mut step = 0;

        loop {
            step += 1;
            if step > max_tool_steps {
                return Ok("Límite de pasos alcanzado.".to_string());
            }

            let (content, tool_calls) = self.llm.chat(&messages, Some(&self.tools.definitions())).await?;

            if !tool_calls.is_empty() {
                if let Some(ref c) = content {
                    messages.push(Message {
                        role: "assistant".to_string(),
                        content: c.clone(),
                        tool_call_id: None,
                    });
                }

                for tool_call in &tool_calls {
                    // Verificar permisos
                    let perm = self.permissions.check(&tool_call.function.name);
                    match perm {
                        PermissionLevel::Deny => {
                            println!("  {} {} {}", "🚫".dimmed(), tool_call.function.name.yellow(), "BLOQUEADO".red());
                            messages.push(Message {
                                role: "tool".to_string(),
                                content: format!("Herramienta '{}' bloqueada por permisos", tool_call.function.name),
                                tool_call_id: Some(tool_call.id.clone()),
                            });
                            continue;
                        }
                        PermissionLevel::Ask => {
                            // En modo no-interactivo, auto-approve
                        }
                        PermissionLevel::Auto => {}
                    }

                    // Delay para evitar rate limits
                    sleep(Duration::from_millis(500)).await;

                    let result = self.execute_tool_call(tool_call).await;

                    println!(
                        "  {} {} {}",
                        "⚡".dimmed(),
                        tool_call.function.name.yellow(),
                        "->".dimmed()
                    );

                    match &result {
                        Ok(r) => {
                            let display = if r.len() > 300 {
                                format!("{}...", &r[..300])
                            } else {
                                r.clone()
                            };
                            println!("    {}", display.dimmed());

                            messages.push(Message {
                                role: "tool".to_string(),
                                content: format!(
                                    "[{}]: {}",
                                    tool_call.function.name, r
                                ),
                                tool_call_id: Some(tool_call.id.clone()),
                            });
                        }
                        Err(e) => {
                            println!("    {} {}", "Error:".red(), e.to_string().dimmed());

                            messages.push(Message {
                                role: "tool".to_string(),
                                content: format!(
                                    "[Error en {}]: {}",
                                    tool_call.function.name, e
                                ),
                                tool_call_id: Some(tool_call.id.clone()),
                            });
                        }
                    }
                }
                continue;
            }

            let response = content.unwrap_or_else(|| "Sin respuesta".to_string());

            let assistant_msg = Message {
                role: "assistant".to_string(),
                content: response.clone(),
                tool_call_id: None,
            };
            self.store.add_message(self.conversation_id, &assistant_msg)?;

            return Ok(response);
        }
    }

    async fn execute_tool_call(&self, tool_call: &ToolCall) -> Result<String> {
        let args: Value = serde_json::from_str(&tool_call.function.arguments)
            .unwrap_or(Value::Object(serde_json::Map::new()));

        self.tools.execute(&tool_call.function.name, args).await
    }

    pub fn conversation_id(&self) -> i64 {
        self.conversation_id
    }

    pub fn new_conversation(&mut self) -> Result<()> {
        self.conversation_id = self.store.new_conversation()?;
        Ok(())
    }

    pub fn compact_conversation(&mut self) -> Result<()> {
        // Crear nueva conversación y copiar solo los últimos mensajes
        let old_id = self.conversation_id;
        self.conversation_id = self.store.new_conversation()?;

        // Copiar últimos 5 mensajes
        let messages = self.store.get_messages(old_id, 5)?;
        for msg in &messages {
            self.store.add_message(self.conversation_id, msg)?;
        }

        Ok(())
    }

    pub fn list_tools(&self) -> Vec<(&str, &str, bool)> {
        self.tools.list_tools()
    }

    pub fn set_working_dir(&mut self, dir: &str) {
        self.working_dir = dir.to_string();
        self.system_messages[0] = Message {
            role: "system".to_string(),
            content: format!("{}\n\nDirectorio de trabajo actual: {}", SYSTEM_PROMPT, self.working_dir),
            tool_call_id: None,
        };
    }
}
