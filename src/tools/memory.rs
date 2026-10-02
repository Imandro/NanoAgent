use anyhow::Result;
use async_trait::async_trait;
use rusqlite::{params, Connection};
use serde_json::{json, Value};
use std::path::PathBuf;

use super::Tool;
use crate::providers::{FunctionDefinition, ToolDefinition};

pub struct MemoryTool {
    db_path: PathBuf,
}

impl MemoryTool {
    pub fn new(data_dir: &PathBuf) -> Self {
        let db_path = data_dir.join("memory.db");
        let _ = Self::init_db(&db_path);
        MemoryTool { db_path }
    }

    fn init_db(path: &PathBuf) -> Result<()> {
        let conn = Connection::open(path)?;
        conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS memories (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                category TEXT NOT NULL DEFAULT 'general',
                key TEXT NOT NULL,
                value TEXT NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
            );

            CREATE INDEX IF NOT EXISTS idx_memories_key ON memories(key);
            CREATE INDEX IF NOT EXISTS idx_memories_category ON memories(category);

            CREATE VIRTUAL TABLE IF NOT EXISTS memories_fts USING fts5(
                key, value, category,
                content='memories',
                content_rowid='id'
            );
            ",
        )?;
        Ok(())
    }

    fn get_conn(&self) -> Result<Connection> {
        Ok(Connection::open(&self.db_path)?)
    }
}

#[async_trait]
impl Tool for MemoryTool {
    fn name(&self) -> &str {
        "memory"
    }

    fn description(&self) -> &str {
        "Memoria a largo plazo para el agente. Guarda y recuerda información importante."
    }

    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            tool_type: "function".to_string(),
            function: FunctionDefinition {
                name: "memory".to_string(),
                description: self.description().to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "action": {
                            "type": "string",
                            "enum": ["store", "search", "get", "delete", "list_categories"],
                            "description": "Acción a realizar"
                        },
                        "key": {
                            "type": "string",
                            "description": "Clave de la memoria"
                        },
                        "value": {
                            "type": "string",
                            "description": "Valor a guardar"
                        },
                        "category": {
                            "type": "string",
                            "description": "Categoría (ej: user_preferences, facts, tasks)"
                        },
                        "query": {
                            "type": "string",
                            "description": "Texto de búsqueda semántica"
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
            "store" => {
                let key = args["key"]
                    .as_str()
                    .ok_or_else(|| anyhow::anyhow!("Falta 'key'"))?;
                let value = args["value"]
                    .as_str()
                    .ok_or_else(|| anyhow::anyhow!("Falta 'value'"))?;
                let category = args["category"].as_str().unwrap_or("general");
                self.store(key, value, category).await
            }
            "search" => {
                let query = args["query"]
                    .as_str()
                    .ok_or_else(|| anyhow::anyhow!("Falta 'query'"))?;
                let limit = args["limit"].as_u64().unwrap_or(10) as usize;
                self.search(query, limit).await
            }
            "get" => {
                let key = args["key"]
                    .as_str()
                    .ok_or_else(|| anyhow::anyhow!("Falta 'key'"))?;
                self.get(key).await
            }
            "delete" => {
                let key = args["key"]
                    .as_str()
                    .ok_or_else(|| anyhow::anyhow!("Falta 'key'"))?;
                self.delete(key).await
            }
            "list_categories" => self.list_categories().await,
            _ => Err(anyhow::anyhow!("Acción desconocida: {}", action)),
        }
    }
}

impl MemoryTool {
    async fn store(&self, key: &str, value: &str, category: &str) -> Result<String> {
        let conn = self.get_conn()?;

        // Upsert: update if exists, insert if not
        conn.execute(
            "INSERT INTO memories (key, value, category) VALUES (?1, ?2, ?3)
             ON CONFLICT(key) DO UPDATE SET value = ?2, category = ?3, updated_at = CURRENT_TIMESTAMP",
            params![key, value, category],
        )?;

        // Update FTS index
        conn.execute(
            "DELETE FROM memories_fts WHERE rowid IN (SELECT id FROM memories WHERE key = ?1)",
            params![key],
        )?;
        conn.execute(
            "INSERT INTO memories_fts(rowid, key, value, category) 
             SELECT id, key, value, category FROM memories WHERE key = ?1",
            params![key],
        )?;

        Ok(format!("Memoria '{}' guardada en categoría '{}'", key, category))
    }

    async fn search(&self, query: &str, limit: usize) -> Result<String> {
        let conn = self.get_conn()?;

        let mut stmt = conn.prepare(
            "SELECT m.key, m.value, m.category, m.updated_at 
             FROM memories m
             JOIN memories_fts f ON m.id = f.rowid
             WHERE memories_fts MATCH ?1
             ORDER BY rank
             LIMIT ?2",
        )?;

        let results: Vec<String> = stmt
            .query_map(params![query, limit as i64], |row| {
                let key: String = row.get(0)?;
                let value: String = row.get(1)?;
                let category: String = row.get(2)?;
                let updated: String = row.get(3)?;
                Ok(format!("[{}] {} = {} (actualizado: {})", category, key, value, updated))
            })?
            .collect::<Result<Vec<_>, _>>()?;

        if results.is_empty() {
            // Fallback: LIKE search
            let mut stmt = conn.prepare(
                "SELECT key, value, category, updated_at FROM memories 
                 WHERE key LIKE ?1 OR value LIKE ?1 OR category LIKE ?1
                 LIMIT ?2",
            )?;

            let pattern = format!("%{}%", query);
            let results: Vec<String> = stmt
                .query_map(params![pattern, limit as i64], |row| {
                    let key: String = row.get(0)?;
                    let value: String = row.get(1)?;
                    let category: String = row.get(2)?;
                    let updated: String = row.get(3)?;
                    Ok(format!("[{}] {} = {} (actualizado: {})", category, key, value, updated))
                })?
                .collect::<Result<Vec<_>, _>>()?;

            if results.is_empty() {
                Ok("No se encontraron memorias que coincidan".to_string())
            } else {
                Ok(format!("Resultados (búsqueda simple):\n{}", results.join("\n")))
            }
        } else {
            Ok(format!("Resultados de búsqueda:\n{}", results.join("\n")))
        }
    }

    async fn get(&self, key: &str) -> Result<String> {
        let conn = self.get_conn()?;

        let result: Option<(String, String, String)> = conn
            .query_row(
                "SELECT value, category, updated_at FROM memories WHERE key = ?1",
                params![key],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
            )
            .ok();

        match result {
            Some((value, category, updated)) => {
                Ok(format!("[{}] {} = {} (actualizado: {})", category, key, value, updated))
            }
            None => Ok(format!("No se encontró memoria con clave '{}'", key)),
        }
    }

    async fn delete(&self, key: &str) -> Result<String> {
        let conn = self.get_conn()?;

        // Delete from FTS first
        conn.execute(
            "DELETE FROM memories_fts WHERE rowid IN (SELECT id FROM memories WHERE key = ?1)",
            params![key],
        )?;

        let deleted = conn.execute("DELETE FROM memories WHERE key = ?1", params![key])?;

        if deleted > 0 {
            Ok(format!("Memoria '{}' eliminada", key))
        } else {
            Ok(format!("No se encontró memoria con clave '{}'", key))
        }
    }

    async fn list_categories(&self) -> Result<String> {
        let conn = self.get_conn()?;

        let mut stmt = conn.prepare(
            "SELECT category, COUNT(*) as cnt FROM memories GROUP BY category ORDER BY cnt DESC",
        )?;

        let categories: Vec<String> = stmt
            .query_map([], |row| {
                let cat: String = row.get(0)?;
                let cnt: i64 = row.get(1)?;
                Ok(format!("  {} ({} entradas)", cat, cnt))
            })?
            .collect::<Result<Vec<_>, _>>()?;

        if categories.is_empty() {
            Ok("No hay memorias guardadas aún".to_string())
        } else {
            Ok(format!("Categorías:\n{}", categories.join("\n")))
        }
    }
}
