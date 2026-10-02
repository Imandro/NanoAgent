use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PermissionLevel {
    Auto, // Ejecutar sin preguntar
    Ask,  // Preguntar antes de ejecutar
    Deny, // No ejecutar nunca
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolPermission {
    pub level: PermissionLevel,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permissions {
    pub tools: HashMap<String, ToolPermission>,
}

impl Default for Permissions {
    fn default() -> Self {
        let mut tools = HashMap::new();

        // Herramientas seguras - auto approve
        tools.insert(
            "read_file".to_string(),
            ToolPermission {
                level: PermissionLevel::Auto,
                description: "Leer archivos".to_string(),
            },
        );
        tools.insert(
            "list_dir".to_string(),
            ToolPermission {
                level: PermissionLevel::Auto,
                description: "Listar directorios".to_string(),
            },
        );
        tools.insert(
            "search_files".to_string(),
            ToolPermission {
                level: PermissionLevel::Auto,
                description: "Buscar archivos".to_string(),
            },
        );
        tools.insert(
            "memory".to_string(),
            ToolPermission {
                level: PermissionLevel::Auto,
                description: "Memoria a largo plazo".to_string(),
            },
        );
        tools.insert(
            "process".to_string(),
            ToolPermission {
                level: PermissionLevel::Auto,
                description: "Gestionar procesos del sistema".to_string(),
            },
        );
        tools.insert(
            "grep".to_string(),
            ToolPermission {
                level: PermissionLevel::Auto,
                description: "Buscar en archivos".to_string(),
            },
        );
        tools.insert(
            "edit_file".to_string(),
            ToolPermission {
                level: PermissionLevel::Ask,
                description: "Editar archivos".to_string(),
            },
        );
        tools.insert(
            "patch_file".to_string(),
            ToolPermission {
                level: PermissionLevel::Ask,
                description: "Aplicar parches".to_string(),
            },
        );

        // Herramientas que modifican - ask
        tools.insert(
            "write_file".to_string(),
            ToolPermission {
                level: PermissionLevel::Ask,
                description: "Crear/modificar archivos".to_string(),
            },
        );
        tools.insert(
            "shell".to_string(),
            ToolPermission {
                level: PermissionLevel::Ask,
                description: "Ejecutar comandos".to_string(),
            },
        );
        tools.insert(
            "http_request".to_string(),
            ToolPermission {
                level: PermissionLevel::Ask,
                description: "Peticiones HTTP".to_string(),
            },
        );
        tools.insert(
            "schedule".to_string(),
            ToolPermission {
                level: PermissionLevel::Ask,
                description: "Programar tareas".to_string(),
            },
        );
        tools.insert(
            "sql_query".to_string(),
            ToolPermission {
                level: PermissionLevel::Ask,
                description: "Ejecutar SQL".to_string(),
            },
        );

        Permissions { tools }
    }
}

impl Permissions {
    pub fn load(data_dir: &PathBuf) -> Self {
        let path = data_dir.join("permissions.json");
        if path.exists() {
            if let Ok(content) = std::fs::read_to_string(&path) {
                if let Ok(perms) = serde_json::from_str(&content) {
                    return perms;
                }
            }
        }
        let perms = Self::default();
        let _ = perms.save(data_dir);
        perms
    }

    pub fn save(&self, data_dir: &PathBuf) -> Result<()> {
        let path = data_dir.join("permissions.json");
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    pub fn apply_overrides(&mut self, overrides: &[(&str, PermissionLevel)]) -> Vec<String> {
        let mut changed = Vec::new();
        for (tool, level) in overrides {
            let before = self.check(tool).clone();
            if &before != level {
                self.set_level(tool, level.clone());
                let label = match level {
                    PermissionLevel::Auto => "auto",
                    PermissionLevel::Ask => "ask",
                    PermissionLevel::Deny => "deny",
                };
                changed.push(format!("{} -> {}", tool, label));
            }
        }
        changed
    }

    pub fn check(&self, tool_name: &str) -> &PermissionLevel {
        self.tools
            .get(tool_name)
            .map(|p| &p.level)
            .unwrap_or(&PermissionLevel::Ask)
    }

    pub fn set_level(&mut self, tool_name: &str, level: PermissionLevel) {
        if let Some(perm) = self.tools.get_mut(tool_name) {
            perm.level = level;
        } else {
            self.tools.insert(
                tool_name.to_string(),
                ToolPermission {
                    level,
                    description: tool_name.to_string(),
                },
            );
        }
    }
}
