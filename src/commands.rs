use std::process::{Command, Stdio};
use std::io::Read;
// Remembers PATH from just before a venv activation, so `deactivate`
// can restore it. TEMPORARY: this whole source/deactivate special-case
// goes away once Luna moves to a persistent shell session(bash)
static PREV_PATH: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None);

// Luna commands — execution engine

pub struct CommandResult {
    pub success: bool,
    pub error_output: String,
}

pub fn run(input: &str) -> CommandResult {
    if let Some(result) = try_source_chain(input) {
        return result;
    }

    let mut parts = input.split_whitespace();

    let command = match parts.next() {
        Some(cmd) => cmd,
        None => return CommandResult { success: true, error_output: String::new() },
    };

    let args: Vec<&str> = parts.collect();

    if needs_shell(input) {
        return run_in_shell(input);
    }

    match command {
        "cd" => {
            cd(&args);
            CommandResult { success: true, error_output: String::new() }
        }
        "pwd" => {
            pwd();
            CommandResult { success: true, error_output: String::new() }
        }
        "source" | "." => {
            match args.first() {
                Some(file) => source_file(file),
                None => {
                    eprintln!("luna: source: filename argument required");
                    CommandResult { success: false, error_output: "source: filename argument required".to_string() }
                }
            }
        }
        "deactivate" => {
            deactivate();
            CommandResult { success: true, error_output: String::new() }
        }
        _ => execute(command, &args),
    }
}

fn needs_shell(input: &str) -> bool {
    input.contains('|')
        || input.contains('>')
        || input.contains('<')
        || input.contains("&&")
        || input.contains("||")
        || input.contains(';')
        || input.contains('*')
        || input.contains('?')
        || input.contains('$')
        || input.contains('"')
        || input.contains('\'')
}

pub fn run_in_shell(input: &str) -> CommandResult {
    match Command::new("bash")
        .arg("-c")
        .arg(input)
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(mut child) => {
            let mut stderr = String::new();
            if let Some(mut err) = child.stderr.take() {
                err.read_to_string(&mut stderr).ok();
            }
            
            let status = child.wait().unwrap();
            print!("\x1b[?1h\x1b=");
            let _ = std::io::Write::flush(&mut std::io::stdout()).unwrap();

            if !stderr.is_empty() {
                eprint!("{}", stderr);
            }

            CommandResult {
                success: status.success(),
                error_output: stderr.trim().to_string(),
            }
        }
        Err(e) => {
            eprintln!("luna: {}", e);
            CommandResult { success: false, error_output: e.to_string() }
        }
    }
}

fn cd(args: &[&str]) {
    let home = std::env::var("HOME").unwrap_or_default();
    let path = args.first().map(|s| *s).unwrap_or("~");

    let expanded = if path == "~" {
        home.clone()
    } else if path.starts_with("~/") {
        format!("{}/{}", home, &path[2..])
    } else {
        path.to_string()
    };

    if let Err(e) = std::env::set_current_dir(&expanded) {
        eprintln!("luna: cd: {}: {}", expanded, e);
    }
}



fn pwd() {
    match std::env::current_dir() {
        Ok(path) => println!("{}", path.display()),
        Err(e) => eprintln!("luna: pwd: {}", e),
    }
}

fn source_file(file: &str) -> CommandResult {
    if !std::path::Path::new(file).exists() {
        let msg = format!("source: no such file: {}", file);
        eprintln!("luna: {}", msg);
        return CommandResult { success: false, error_output: msg };
    }

    let is_venv_activate = file.contains("activate");
    let already_in_venv = std::env::var("VIRTUAL_ENV").is_ok();

    let escaped = file.replace('\'', "'\\''");
    let output = Command::new("/bin/sh")
        .arg("-c")
        .arg(format!(". '{}' && env", escaped))
        .output();

    let output = match output {
        Ok(o) => o,
        Err(e) => {
            eprintln!("luna: source: {}", e);
            return CommandResult { success: false, error_output: e.to_string() };
        }
    };

    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr).to_string();
        eprint!("{}", err);
        return CommandResult { success: false, error_output: err };
    }

    // activation — don't clobber the original baseline if the user
    // sources activate twice in a row.
    if is_venv_activate && !already_in_venv {
        if let Ok(mut guard) = PREV_PATH.lock() {
            *guard = Some(std::env::var("PATH").unwrap_or_default());
        }
    }

    let dump = String::from_utf8_lossy(&output.stdout);
    for line in dump.lines() {
        if let Some((key, value)) = line.split_once('=') {

            if matches!(key, "SHLVL" | "_" | "PWD" | "OLDPWD") {
                continue;
            }
            if std::env::var(key).ok().as_deref() != Some(value) {
                // SAFETY: called synchronously from the main REPL loop,
                // between prompts — no other thread reads/writes env
                // at this point.
                unsafe { std::env::set_var(key, value); }
            }
        }
    }

    CommandResult { success: true, error_output: String::new() }
}


// Handles `source <file>` and `source <file> && <rest>` (also `.`).
// Without this, `&&` chains skip source_file() entirely via needs_shell()
// and go straight to run_in_shell — which runs fine as one block, but the
// activated env dies with that subprocess instead of persisting in Luna.
fn try_source_chain(input: &str) -> Option<CommandResult> {
    let trimmed = input.trim();
    if !(trimmed.starts_with("source ") || trimmed.starts_with(". ")) {
        return None;
    }

    if let Some(idx) = trimmed.find("&&") {
        let (first_part, rest) = trimmed.split_at(idx);
        let rest = rest[2..].trim();

        let file = first_part.trim().split_whitespace().nth(1);
        let result = match file {
            Some(f) => source_file(f),
            None => CommandResult { success: false, error_output: "source: filename argument required".to_string() },
        };

        if !result.success || rest.is_empty() {
            return Some(result);
        }

        return Some(run(rest));
    }

    let file = trimmed.split_whitespace().nth(1);
    Some(match file {
        Some(f) => source_file(f),
        None => CommandResult { success: false, error_output: "source: filename argument required".to_string() },
    })
}


fn deactivate() {
    if std::env::var("VIRTUAL_ENV").is_err() {
        eprintln!("luna: deactivate: not currently in a virtual environment");
        return;
    }

    if let Ok(mut guard) = PREV_PATH.lock() {
        if let Some(path) = guard.take() {
            // SAFETY: same as above — main-thread only, no concurrent env access.
            unsafe { std::env::set_var("PATH", path); }
        }
    }

    // SAFETY: same as above.
    unsafe {
        std::env::remove_var("VIRTUAL_ENV");
        std::env::remove_var("PYTHONHOME");
    }

    println!("luna: deactivated virtual environment");
}

fn execute(command: &str, args: &[&str]) -> CommandResult {
    match Command::new(command)
        .args(args)
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(mut child) => {
            let mut stderr = String::new();
            if let Some(mut err) = child.stderr.take() {
                err.read_to_string(&mut stderr).ok();
            }
            let status = child.wait().unwrap();
            print!("\x1b[?1h\x1b=");
            let _ = std::io::Write::flush(&mut std::io::stdout()).unwrap();

            if !stderr.is_empty() {
                eprint!("{}", stderr);
            }

            CommandResult {
                success: status.success(),
                error_output: stderr.trim().to_string(),
            }
        }
        Err(e) => {
            eprintln!("luna: {}: {}", command, e);
            CommandResult { success: false, error_output: e.to_string() }
        }
    }
}