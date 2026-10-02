use anyhow::{Context, Result};
use rusqlite::{params, Connection};
use std::path::Path;

use crate::providers::Message;

pub struct Store {
    conn: Connection,
}

impl Store {
    pub fn open(db_path: &Path) -> Result<Self> {
        let conn = Connection::open(db_path)
            .with_context(|| format!("No se pudo abrir base de datos: {:?}", db_path))?;

        conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS conversations (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            );

            CREATE TABLE IF NOT EXISTS messages (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                conversation_id INTEGER NOT NULL,
                role TEXT NOT NULL,
                content TEXT NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (conversation_id) REFERENCES conversations(id)
            );

            CREATE INDEX IF NOT EXISTS idx_messages_conv ON messages(conversation_id);
            ",
        )?;

        Ok(Store { conn })
    }

    pub fn new_conversation(&self) -> Result<i64> {
        self.conn
            .execute("INSERT INTO conversations DEFAULT VALUES", [])
            .context("Error creando conversación")?;

        Ok(self.conn.last_insert_rowid())
    }

    pub fn add_message(&self, conversation_id: i64, msg: &Message) -> Result<()> {
        self.conn
            .execute(
                "INSERT INTO messages (conversation_id, role, content) VALUES (?1, ?2, ?3)",
                params![conversation_id, msg.role, msg.content],
            )
            .context("Error guardando mensaje")?;

        Ok(())
    }

    pub fn get_messages(&self, conversation_id: i64, limit: usize) -> Result<Vec<Message>> {
        let mut stmt = self.conn
            .prepare(
                "SELECT role, content FROM messages 
                 WHERE conversation_id = ?1 
                 ORDER BY id DESC 
                 LIMIT ?2",
            )
            .context("Error preparando consulta")?;

            let messages = stmt
            .query_map(params![conversation_id, limit as i64], |row| {
                Ok(Message {
                    role: row.get(0)?,
                    content: row.get(1)?,
                    tool_call_id: None,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        // Invertir para que estén en orden cronológico
        Ok(messages.into_iter().rev().collect())
    }

    pub fn list_conversations(&self) -> Result<Vec<(i64, String)>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, created_at FROM conversations ORDER BY id DESC LIMIT 20")
            .context("Error listando conversaciones")?;

        let convs = stmt
            .query_map([], |row| {
                Ok((row.get(0)?, row.get(1)?))
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(convs)
    }

    pub fn delete_conversation(&self, id: i64) -> Result<()> {
        self.conn
            .execute("DELETE FROM messages WHERE conversation_id = ?1", params![id])?;
        self.conn
            .execute("DELETE FROM conversations WHERE id = ?1", params![id])?;
        Ok(())
    }
}
