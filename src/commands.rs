use std::process::{Command, Stdio};
use std::io::Read;

// Luna commands — execution engine

pub struct CommandResult {
    pub success: bool,
    pub error_output: String,
}

pub fn run(input: &str) -> CommandResult {
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
}

fn run_in_shell(input: &str) -> CommandResult {
    match Command::new("/bin/sh")
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
            let _ = std::io::Write::flush(&mut std::io::stdout());

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
            let _ = std::io::Write::flush(&mut std::io::stdout());

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