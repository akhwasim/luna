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

        CREATE TABLE IF NOT EXISTS workflows (
            name      TEXT PRIMARY KEY,
            commands  TEXT NOT NULL,
            use_count INTEGER DEFAULT 0,
            created   DATETIME DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS autocorrect (
            wrong     TEXT PRIMARY KEY,
            correct   TEXT NOT NULL,
            count     INTEGER DEFAULT 1
        );

        CREATE TABLE IF NOT EXISTS rejected_patterns (
            pattern TEXT PRIMARY KEY,
            created DATETIME DEFAULT CURRENT_TIMESTAMP
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
        let mut stmt = match self.conn.prepare(
            "SELECT command FROM commands ORDER BY timestamp DESC LIMIT ?1"
        ) {
            Ok(s) => s,
            Err(_) => return Vec::new(),
        };

        match stmt.query_map(params![limit as i64], |row| row.get(0)) {
            Ok(rows) => rows.filter_map(|r| r.ok()).collect(),
            Err(_) => Vec::new(),
        }
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
        let mut stmt = match self.conn.prepare(
            "SELECT command, directory FROM commands ORDER BY timestamp DESC LIMIT ?1"
        ) {
            Ok(s) => s,
            Err(_) => {
                println!("  No history available.");
                return;
            }
        };

        let rows: Vec<(String, String)> = match stmt.query_map(
            params![limit as i64],
            |row| Ok((row.get(0)?, row.get(1)?))
        ) {
            Ok(r) => r.filter_map(|x| x.ok()).collect(),
            Err(_) => {
                println!("  No history available.");
                return;
            }
        };

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

    pub fn save_workflow(&self, name: &str, commands: &[String]) {
    let commands_json = serde_json::to_string(commands).unwrap_or_default();
    let _ = self.conn.execute(
        "INSERT OR REPLACE INTO workflows (name, commands) VALUES (?1, ?2)",
        params![name, commands_json],
    );
    }

    pub fn get_workflow(&self, name: &str) -> Option<Vec<String>> {
        let mut stmt = self.conn.prepare(
            "SELECT commands FROM workflows WHERE name = ?1"
        ).ok()?;

        let result: Option<String> = stmt.query_row(
            params![name],
            |row| row.get(0)
        ).ok();

        result.and_then(|s| serde_json::from_str(&s).ok())
    }

    pub fn list_workflows(&self) -> Vec<(String, Vec<String>)> {
        let mut stmt = match self.conn.prepare(
            "SELECT name, commands FROM workflows ORDER BY use_count DESC"
        ) {
            Ok(s) => s,
            Err(_) => return Vec::new(),
        };

        stmt.query_map(params![], |row| {
            let name: String = row.get(0)?;
            let cmds: String = row.get(1)?;
            Ok((name, cmds))
        })
        .unwrap()
        .filter_map(|r| r.ok())
        .filter_map(|(name, cmds)| {
            serde_json::from_str::<Vec<String>>(&cmds)
                .ok()
                .map(|c| (name, c))
        })
        .collect()
    }

    pub fn increment_workflow_use(&self, name: &str) {
        let _ = self.conn.execute(
            "UPDATE workflows SET use_count = use_count + 1 WHERE name = ?1",
            params![name],
        );
    }

    // Phase 8: per-command frequency lookup for future "favorite commands"

    #[allow(dead_code)]
    pub fn get_command_frequency(&self, command: &str) -> usize {
        let mut stmt = match self.conn.prepare(
            "SELECT COUNT(*) FROM commands WHERE command = ?1"
        ) {
            Ok(s) => s,
            Err(_) => return 0,
        };

        stmt.query_row(params![command], |row| {
            row.get::<_, i64>(0)
        })
        .unwrap_or(0) as usize
    }

    pub fn get_commands_for_pattern_detection(&self, limit: usize) -> Vec<(String, String)> {
        let mut stmt = match self.conn.prepare(
            "SELECT command, directory FROM commands 
            WHERE success = 1 
            ORDER BY timestamp DESC 
            LIMIT ?1"
        ) {
            Ok(s) => s,
            Err(_) => return Vec::new(),
        };

        stmt.query_map(params![limit as i64], |row| {
            Ok((row.get(0)?, row.get(1)?))
        })
        .unwrap()
        .filter_map(|r| r.ok())
        .collect()
    }

    pub fn is_pattern_rejected(&self, pattern_key: &str) -> bool {
    let mut stmt = match self.conn.prepare(
        "SELECT COUNT(*) FROM rejected_patterns WHERE pattern = ?1"
    ) {
        Ok(s) => s,
        Err(_) => return false,
    };
    stmt.query_row(params![pattern_key], |row| {
        row.get::<_, i64>(0)
    }).unwrap_or(0) > 0
    }

    pub fn reject_pattern(&self, pattern_key: &str) {
        let _ = self.conn.execute(
            "INSERT OR IGNORE INTO rejected_patterns (pattern) VALUES (?1)",
            params![pattern_key],
        );
    }

    pub fn remove_workflow(&self, name: &str) {
    let _ = self.conn.execute(
        "DELETE FROM workflows WHERE name = ?1",
        params![name],
    );
    }

    pub fn get_stat(&self, query: &str) -> usize {
    let mut stmt = match self.conn.prepare(query) {
        Ok(s) => s,
        Err(_) => return 0,
    };
    stmt.query_row([], |row| row.get::<_, i64>(0))
        .unwrap_or(0) as usize
    }

    pub fn get_command_counts(&self, limit: usize) -> Vec<(String, usize)> {
        let mut stmt = match self.conn.prepare(
            "SELECT command, COUNT(*) as cnt FROM commands 
            GROUP BY command 
            ORDER BY cnt DESC 
            LIMIT ?1"
        ) {
            Ok(s) => s,
            Err(_) => return Vec::new(),
        };

        stmt.query_map(params![limit as i64], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)? as usize))
        })
        .unwrap()
        .filter_map(|r| r.ok())
        .collect()
    }

    pub fn get_all_errors(&self) -> Vec<(String, String)> {
        let mut stmt = match self.conn.prepare(
            "SELECT command, error FROM errors ORDER BY timestamp DESC"
        ) {
            Ok(s) => s,
            Err(_) => return Vec::new(),
        };

        stmt.query_map(params![], |row| {
            Ok((row.get(0)?, row.get(1)?))
        })
        .unwrap()
        .filter_map(|r| r.ok())
        .collect()
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