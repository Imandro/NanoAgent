use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;

pub mod shell;
pub mod filesystem;
pub mod http;
pub mod database;
pub mod process;
pub mod search;
pub mod schedule;
pub mod memory;
pub mod editor;
pub mod grep;

use crate::providers::ToolDefinition;

#[async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn definition(&self) -> ToolDefinition;
    async fn execute(&self, args: Value) -> Result<String>;
}

pub struct ToolRegistry {
    tools: Vec<Box<dyn Tool>>,
}

impl ToolRegistry {
    pub fn new(data_dir: &std::path::Path) -> Self {
        let mut registry = ToolRegistry { tools: vec![] };

        // Lectura (seguras - auto)
        registry.tools.push(Box::new(filesystem::ReadFileTool));
        registry.tools.push(Box::new(filesystem::ListDirTool));
        registry.tools.push(Box::new(search::SearchTool));
        registry.tools.push(Box::new(grep::GrepTool));
        registry.tools.push(Box::new(memory::MemoryTool::new(&data_dir.to_path_buf())));
        registry.tools.push(Box::new(process::ProcessTool));

        // Escritura (peligrosas - ask)
        registry.tools.push(Box::new(filesystem::WriteFileTool));
        registry.tools.push(Box::new(editor::EditFileTool));
        registry.tools.push(Box::new(editor::PatchFileTool));
        registry.tools.push(Box::new(shell::ShellTool));
        registry.tools.push(Box::new(http::HttpTool));
        registry.tools.push(Box::new(database::QueryTool));
        registry.tools.push(Box::new(schedule::ScheduleTool));

        registry
    }

    pub fn definitions(&self) -> Vec<ToolDefinition> {
        self.tools.iter().map(|t| t.definition()).collect()
    }

    pub async fn execute(&self, name: &str, args: Value) -> Result<String> {
        let tool = self
            .tools
            .iter()
            .find(|t| t.name() == name)
            .ok_or_else(|| anyhow::anyhow!("Herramienta desconocida: {}", name))?;

        tool.execute(args).await
    }

    pub fn list_tools(&self) -> Vec<(&str, &str, bool)> {
        self.tools.iter().map(|t| {
            let safe = matches!(t.name(), 
                "read_file" | "list_dir" | "search_files" | "grep" | "memory" | "process"
            );
            (t.name(), t.description(), safe)
        }).collect()
    }
}
