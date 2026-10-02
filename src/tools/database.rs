use anyhow::Result;
use async_trait::async_trait;
use rusqlite::Connection;
use serde_json::{json, Value};

use super::Tool;
use crate::providers::{FunctionDefinition, ToolDefinition};

pub struct QueryTool;

#[async_trait]
impl Tool for QueryTool {
    fn name(&self) -> &str {
        "sql_query"
    }

    fn description(&self) -> &str {
        "Ejecuta una consulta SQL en una base de datos SQLite local."
    }

    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            tool_type: "function".to_string(),
            function: FunctionDefinition {
                name: "sql_query".to_string(),
                description: self.description().to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "query": {
                            "type": "string",
                            "description": "Consulta SQL a ejecutar"
                        }
                    },
                    "required": ["query"]
                }),
            },
        }
    }

    async fn execute(&self, args: Value) -> Result<String> {
        let query = args["query"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Falta el parámetro 'query'"))?;

        let conn = Connection::open("agent_queries.db")?;

        let upper = query.trim().to_uppercase();
        if upper.starts_with("SELECT")
            || upper.starts_with("EXPLAIN")
            || upper.starts_with("PRAGMA")
        {
            let mut stmt = conn.prepare(query)?;
            let columns: Vec<String> = stmt
                .column_names()
                .into_iter()
                .map(|s| s.to_string())
                .collect();

            let rows = stmt.query_map([], |row| {
                let mut values = Vec::new();
                for (i, col) in columns.iter().enumerate() {
                    let val: String = row.get(i).unwrap_or_else(|_| "<NULL>".to_string());
                    values.push(format!("{}: {}", col, val));
                }
                Ok(values.join(" | "))
            })?;

            let mut result = Vec::new();
            result.push(columns.join(" | "));
            result.push("-".repeat(40));

            for row in rows {
                result.push(row?);
            }

            Ok(result.join("\n"))
        } else if upper.starts_with("CREATE")
            || upper.starts_with("INSERT")
            || upper.starts_with("UPDATE")
            || upper.starts_with("DELETE")
        {
            conn.execute_batch(query)?;
            Ok("Consulta ejecutada correctamente".to_string())
        } else {
            anyhow::bail!("Solo se permiten consultas SELECT, CREATE, INSERT, UPDATE, DELETE");
        }
    }
}
