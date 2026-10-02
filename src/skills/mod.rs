pub mod catalog;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

pub use catalog::{find, suggest_for, Skill, SKILLS};

const PROFILE_VERSION: u32 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Autonomy {
    Conservador,
    Balanceado,
    Autonomo,
}

impl Autonomy {
    pub fn label(&self) -> &'static str {
        match self {
            Autonomy::Conservador => "Conservador",
            Autonomy::Balanceado => "Balanceado",
            Autonomy::Autonomo => "Autonomo",
        }
    }

    pub fn from_index(i: usize) -> Self {
        match i {
            0 => Autonomy::Conservador,
            1 => Autonomy::Balanceado,
            _ => Autonomy::Autonomo,
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Autonomy::Conservador => "Pide confirmacion antes de escribir o ejecutar",
            Autonomy::Balanceado => "Escribe solo, confirma los comandos de shell",
            Autonomy::Autonomo => "Actua sin preguntar y verifica el resultado",
        }
    }

    pub fn skill_id(&self) -> &'static str {
        match self {
            Autonomy::Conservador => "conservador",
            Autonomy::Balanceado => "balanceado",
            Autonomy::Autonomo => "autonomo",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Verbosity {
    Conciso,
    Normal,
    Detallado,
}

impl Verbosity {
    pub fn label(&self) -> &'static str {
        match self {
            Verbosity::Conciso => "Conciso",
            Verbosity::Normal => "Normal",
            Verbosity::Detallado => "Detallado",
        }
    }

    pub fn from_index(i: usize) -> Self {
        match i {
            0 => Verbosity::Conciso,
            1 => Verbosity::Normal,
            _ => Verbosity::Detallado,
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Verbosity::Conciso => "Solo lo esencial, sin rodeos",
            Verbosity::Normal => "Explica lo necesario, omite lo obvio",
            Verbosity::Detallado => "Explica decisiones y casos limite",
        }
    }

    pub fn instructions(&self) -> &'static str {
        match self {
            Verbosity::Conciso => "BREVEDAD: Maximo unas pocas lineas. Sin preambulos ni resumen final. Si el usuario ya sabe la respuesta, no se la repitas.",
            Verbosity::Normal => "BREVEDAD: Explica lo necesario y omite lo obvio. No repitas el codigo que ya es legible.",
            Verbosity::Detallado => "PROFUNDIDAD: Explica el razonamiento cuando la decision no sea obvia. Incluye casos limite y alternativas descartadas.",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Language {
    Espanol,
    Ingles,
    SegunMensaje,
}

impl Language {
    pub fn label(&self) -> &'static str {
        match self {
            Language::Espanol => "Espanol",
            Language::Ingles => "Ingles",
            Language::SegunMensaje => "Segun el mensaje",
        }
    }

    pub fn from_index(i: usize) -> Self {
        match i {
            0 => Language::Espanol,
            1 => Language::Ingles,
            _ => Language::SegunMensaje,
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Language::Espanol => "Responde siempre en espanol",
            Language::Ingles => "Responde siempre en ingles",
            Language::SegunMensaje => "Sigue el idioma del mensaje",
        }
    }

    pub fn instructions(&self) -> &'static str {
        match self {
            Language::Espanol => {
                "IDIOMA: Responde siempre en espanol, aunque el usuario escriba en otro idioma."
            }
            Language::Ingles => "IDIOMA: Responde siempre en ingles.",
            Language::SegunMensaje => "IDIOMA: Responde en el idioma en el que el usuario escribe.",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub version: u32,
    pub name: Option<String>,
    pub skills: Vec<String>,
    pub autonomy: Autonomy,
    pub verbosity: Verbosity,
    pub language: Language,
    pub installed_at: String,
}

impl Default for Profile {
    fn default() -> Self {
        Profile {
            version: PROFILE_VERSION,
            name: None,
            skills: Vec::new(),
            autonomy: Autonomy::Balanceado,
            verbosity: Verbosity::Normal,
            language: Language::SegunMensaje,
            installed_at: String::new(),
        }
    }
}

impl Profile {
    pub fn path(data_dir: &Path) -> PathBuf {
        data_dir.join("profile.json")
    }

    pub fn exists(data_dir: &Path) -> bool {
        Self::path(data_dir).exists()
    }

    pub fn load(data_dir: &Path) -> Option<Self> {
        let content = std::fs::read_to_string(Self::path(data_dir)).ok()?;
        serde_json::from_str(&content).ok()
    }

    pub fn save(&self, data_dir: &Path) -> Result<()> {
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(Self::path(data_dir), content)?;
        Ok(())
    }

    pub fn is_configured(&self) -> bool {
        !self.skills.is_empty()
    }

    pub fn activate(&mut self, id: &str) -> Result<()> {
        let skill = find(id).ok_or_else(|| anyhow::anyhow!("Skill desconocida: {}", id))?;
        if !self.skills.iter().any(|s| s == id) {
            self.skills.push(skill.id.to_string());
        }
        Ok(())
    }

    pub fn deactivate(&mut self, id: &str) -> bool {
        let before = self.skills.len();
        self.skills.retain(|s| s != id);
        before != self.skills.len()
    }

    pub fn toggle(&mut self, id: &str) -> Result<bool> {
        if self.skills.iter().any(|s| s == id) {
            self.deactivate(id);
            Ok(false)
        } else {
            self.activate(id)?;
            Ok(true)
        }
    }

    pub fn active(&self) -> Vec<&'static Skill> {
        self.skills.iter().filter_map(|id| find(id)).collect()
    }

    pub fn inactive(&self) -> Vec<&'static Skill> {
        SKILLS
            .iter()
            .filter(|s| !self.skills.iter().any(|id| id == s.id))
            .collect()
    }

    pub fn is_active(&self, id: &str) -> bool {
        self.skills.iter().any(|s| s == id)
    }

    pub fn permission_overrides(&self) -> Vec<(&'static str, crate::permissions::PermissionLevel)> {
        let mut out = Vec::new();
        for skill in self.active() {
            for (tool, level) in skill.permission_hints {
                out.push((*tool, level.clone()));
            }
        }
        out
    }

    pub fn suggest(&self, input: &str) -> Option<&'static Skill> {
        let s = suggest_for(input)?;
        if self.is_active(s.id) {
            None
        } else {
            Some(s)
        }
    }

    pub fn stamp(&mut self) {
        if self.installed_at.is_empty() {
            self.installed_at = chrono_like_now();
        }
    }
}

fn chrono_like_now() -> String {
    let dur = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    format!("{}", dur.as_secs())
}
