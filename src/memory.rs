use rusqlite::{Connection, Result, params};
use std::path::PathBuf;

// Luna memory — persistent command history and context

pub struct Memory {
    conn: Connection,
}

impl Memory {
    pub fn new() -> Result<Self> {
        let db_path = get_db_path();

        // Create ~/.luna directory if it doesn't exist
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent).ok();
        }

        let conn = Connection::open(&db_path)?;

        let memory = Memory { conn };
        memory.init()?;
        Ok(memory)
    }

    fn init(&self) -> Result<()> {
        self.conn.execute_batch("
            CREATE TABLE IF NOT EXISTS commands (
                id        INTEGER PRIMARY KEY AUTOINCREMENT,
                command   TEXT NOT NULL,
                directory TEXT NOT NULL,
                success   INTEGER NOT NULL DEFAULT 1,
                timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
            );

            CREATE TABLE IF NOT EXISTS errors (
                id        INTEGER PRIMARY KEY AUTOINCREMENT,
                command   TEXT NOT NULL,
                error     TEXT NOT NULL,
                fix       TEXT,
                timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
            );
        ")
    }

    pub fn save_command(&self, command: &str, directory: &str, success: bool) {
        let _ = self.conn.execute(
            "INSERT INTO commands (command, directory, success) VALUES (?1, ?2, ?3)",
            params![command, directory, success as i32],
        );
    }

    pub fn save_error(&self, command: &str, error: &str, fix: Option<&str>) {
        let _ = self.conn.execute(
            "INSERT INTO errors (command, error, fix) VALUES (?1, ?2, ?3)",
            params![command, error, fix],
        );
    }

    // Get last N commands for AI context
    pub fn recent_commands(&self, limit: usize) -> Vec<String> {
        let mut stmt = self.conn.prepare(
            "SELECT command FROM commands ORDER BY timestamp DESC LIMIT ?1"
        ).unwrap();

        stmt.query_map(params![limit as i64], |row| {
            row.get(0)
        })
        .unwrap()
        .filter_map(|r| r.ok())
        .collect()
    }

    // Get last N commands as formatted context string for AI
    pub fn context_for_ai(&self) -> String {
        let recent = self.recent_commands(10);

        if recent.is_empty() {
            return String::new();
        }

        let cwd = std::env::current_dir()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        format!(
            "Current directory: {}\nRecent commands: {}",
            cwd,
            recent.join(", ")
        )
    }
}


fn get_db_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_default();
    PathBuf::from(format!("{}/.luna/memory.db", home))
}

