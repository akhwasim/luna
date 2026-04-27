use rusqlite::{Connection, Result, params};
use std::path::PathBuf;

// Luna memory — persistent command history and context

pub struct Memory {
    conn: Connection,
}

impl Memory {
    pub fn new() -> Result<Self> {
        let db_path = get_db_path();

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

    pub fn recent_commands(&self, limit: usize) -> Vec<String> {
        let mut stmt = self.conn.prepare(
            "SELECT command FROM commands ORDER BY timestamp DESC LIMIT ?1"
        ).unwrap();

        stmt.query_map(params![limit as i64], |row| row.get(0))
            .unwrap()
            .filter_map(|r| r.ok())
            .collect()
    }

    pub fn recent_errors(&self, limit: usize) -> Vec<String> {
        let mut stmt = match self.conn.prepare(
            "SELECT command, error FROM errors ORDER BY timestamp DESC LIMIT ?1"
        ) {
            Ok(s) => s,
            Err(_) => return Vec::new(),
        };

        stmt.query_map(params![limit as i64], |row| {
            let cmd: String = row.get(0)?;
            let err: String = row.get(1)?;
            Ok(format!("{}: {}", cmd, err))
        })
        .unwrap()
        .filter_map(|r| r.ok())
        .collect()
    }

    pub fn context_for_ai(&self) -> String {
    let recent = self.recent_commands(10);
    let recent_errors = self.recent_errors(3);

    let cwd = std::env::current_dir()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    let home = std::env::var("HOME").unwrap_or_default();
    let luna_dir = format!("{}/luna", home);
    let project_type = detect_project_type();

    let last_command = recent.first()
        .cloned()
        .unwrap_or_else(|| "none".to_string());

    let recent_str = if recent.is_empty() {
        "none".to_string()
    } else {
        recent.join(", ")
    };

    let errors_str = if recent_errors.is_empty() {
        "none".to_string()
    } else {
        recent_errors.join(", ")
    };

    format!(
        "User's current directory: {}\nLuna project location: {}\nProject type in current directory: {}\nLast command run: {}\nRecent commands (newest first): {}\nRecent errors: {}",
        cwd, luna_dir, project_type, last_command, recent_str, errors_str
    )
}

    pub fn print_recent(&self, limit: usize) {
        let mut stmt = self.conn.prepare(
            "SELECT command, directory FROM commands ORDER BY timestamp DESC LIMIT ?1"
        ).unwrap();

        let rows: Vec<(String, String)> = stmt.query_map(
            params![limit as i64],
            |row| Ok((row.get(0)?, row.get(1)?))
        )
        .unwrap()
        .filter_map(|r| r.ok())
        .collect();

        if rows.is_empty() {
            println!("  No history yet.");
            return;
        }

        println!();
        println!("  Recent commands");
        println!("  ─────────────────────────────────");
        for (cmd, dir) in rows.iter().rev() {
            println!("  {} │ {}", dir, cmd);
        }
        println!();
    }
}

fn detect_project_type() -> &'static str {
    let mut dir = std::env::current_dir().unwrap_or_default();

    for _ in 0..4 {
        if dir.join("Cargo.toml").exists() {
            return "Rust (use cargo commands)";
        } else if dir.join("package.json").exists() {
            return "Node.js (use npm/yarn/pnpm commands)";
        } else if dir.join("requirements.txt").exists()
            || dir.join("pyproject.toml").exists()
        {
            return "Python (use pip/python commands)";
        } else if dir.join("go.mod").exists() {
            return "Go (use go commands)";
        }

        match dir.parent() {
            Some(parent) => dir = parent.to_path_buf(),
            None => break,
        }
    }

    "Unknown"
}

fn get_db_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_default();
    PathBuf::from(format!("{}/.luna/memory.db", home))
}