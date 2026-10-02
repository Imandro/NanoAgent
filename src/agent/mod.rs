use anyhow::Result;
use colored::*;
use serde_json::Value;
use tokio::time::{sleep, Duration};

use crate::config::Config;
use crate::db::store::Store;
use crate::permissions::{PermissionLevel, Permissions};
use crate::providers::{LlmClient, Message, ToolCall};
use crate::skills::Profile;
use crate::tools::ToolRegistry;

const BASE_PROMPT: &str = "Eres NanoAgent, un asistente de IA para programacion.

CAPACIDADES PRINCIPALES:
- Leer, crear, modificar y eliminar archivos de codigo fuente
- Ejecutar comandos de terminal (build, test, run, etc.)
- Buscar en codigo con expresiones regulares (grep)
- Editar archivos de forma precisa con edit_file y patch_file
- Navegar la estructura del proyecto
- Ejecutar scripts y comandos del sistema

INSTRUCCIONES:
1. Analiza el codigo existente antes de hacer cambios
2. Usa herramientas de lectura para entender el contexto
3. Edita archivos de forma precisa (edit_file, patch_file)
4. Verifica los cambios ejecutando tests o builds
5. Si hay errores, investiga la causa antes de sugerir soluciones
6. Usa /help para ver comandos y /skills para gestionar tus skills

COMPORTAMIENTO:
- Se directo y eficiente
- Muestra fragmentos de codigo relevantes, no el archivo entero
- Explica brevemente que estas haciendo
- Si el usuario pide algo ambiguo, pregunta para clarificar

Limite de 15 pasos de herramientas por mensaje.";

fn build_system_prompt(profile: &crate::skills::Profile, working_dir: &str) -> String {
    let mut sections: Vec<String> = vec![BASE_PROMPT.to_string()];

    sections.push(format!(
        "IDIOMA Y ESTILO:\n{}",
        profile.language.instructions()
    ));
    sections.push(profile.verbosity.instructions().to_string());

    if let Some(name) = &profile.name {
        sections.push(format!(
            "El usuario se identifica como '{}'. Dirigete a el por su nombre cuando sea natural.",
            name
        ));
    }

    let active = profile.active();
    if !active.is_empty() {
        let mut block = String::from("SKILLS ACTIVAS:");
        for skill in &active {
            block.push_str(&format!("\n\n[{}]\n{}", skill.name, skill.instructions));
        }
        sections.push(block);
    } else {
        sections.push(
            "No hay skills configuradas. Puedes sugerirle al usuario usar /setup para activarlas."
                .to_string(),
        );
    }

    sections.push(format!("Directorio de trabajo actual: {}", working_dir));
    sections.join("\n\n")
}

pub struct Agent {
    llm: LlmClient,
    tools: ToolRegistry,
    store: Store,
    permissions: Permissions,
    conversation_id: i64,
    system_messages: Vec<Message>,
    working_dir: String,
    profile: Profile,
}

impl Agent {
    pub fn new(config: Config, permissions: Permissions, profile: Profile) -> Result<Self> {
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
            content: build_system_prompt(&profile, &working_dir),
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
            profile,
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

            let (content, tool_calls) = self
                .llm
                .chat(&messages, Some(&self.tools.definitions()))
                .await?;

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
                            println!(
                                "  {} {} {}",
                                "🚫".dimmed(),
                                tool_call.function.name.yellow(),
                                "BLOQUEADO".red()
                            );
                            messages.push(Message {
                                role: "tool".to_string(),
                                content: format!(
                                    "Herramienta '{}' bloqueada por permisos",
                                    tool_call.function.name
                                ),
                                tool_call_id: Some(tool_call.id.clone()),
                            });
                            continue;
                        }
                        PermissionLevel::Ask => {
                            println!(
                                "  {} {} {}",
                                "🔔".dimmed(),
                                tool_call.function.name.yellow(),
                                format!(
                                    "(requiere confirmacion, se ejecuta: {})",
                                    "si autorizaste en /setup"
                                )
                                .dimmed()
                            );
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
                                content: format!("[{}]: {}", tool_call.function.name, r),
                                tool_call_id: Some(tool_call.id.clone()),
                            });
                        }
                        Err(e) => {
                            println!("    {} {}", "Error:".red(), e.to_string().dimmed());

                            messages.push(Message {
                                role: "tool".to_string(),
                                content: format!("[Error en {}]: {}", tool_call.function.name, e),
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
            self.store
                .add_message(self.conversation_id, &assistant_msg)?;

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
            content: build_system_prompt(&self.profile, &self.working_dir),
            tool_call_id: None,
        };
    }

    pub fn profile(&self) -> &Profile {
        &self.profile
    }

    pub fn profile_mut(&mut self) -> &mut Profile {
        &mut self.profile
    }

    pub fn permissions_mut(&mut self) -> &mut Permissions {
        &mut self.permissions
    }

    pub fn refresh_system_prompt(&mut self) {
        self.system_messages[0] = Message {
            role: "system".to_string(),
            content: build_system_prompt(&self.profile, &self.working_dir),
            tool_call_id: None,
        };
    }
}
