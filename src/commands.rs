
use std::process::Command;

// Luna commands — execution engine

pub fn run(input: &str) {
    let mut parts = input.split_whitespace();

    let command = match parts.next() {
        Some(cmd) => cmd,
        None => return,
    };

    let args: Vec<&str> = parts.collect();

    // Hand off to /bin/sh for shell features — pipes, redirects, globs
  
    if needs_shell(input) {
        run_in_shell(input);
        return;
    }

    match command {
        "cd" => cd(&args),
        "pwd" => pwd(),
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

// Pass command to /bin/sh 
fn run_in_shell(input: &str) {
    match Command::new("/bin/sh")
        .arg("-c")
        .arg(input)
        .spawn()
    {
        Ok(mut child) => {
            child.wait().unwrap();
            print!("\x1b[?1h\x1b=");
            let _ = std::io::Write::flush(&mut std::io::stdout());
        }
        Err(e) => {
            eprintln!("luna: {}", e);
        }
    }
}

fn cd(args: &[&str]) {
    let home = std::env::var("HOME").unwrap_or_default();

    let path = args.first().map(|s| *s).unwrap_or("~");

    // Expand ~ and ~/something
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

fn execute(command: &str, args: &[&str]) {
    match Command::new(command).args(args).spawn() {
        Ok(mut child) => {
            child.wait().unwrap();
            print!("\x1b[?1h\x1b=");
            let _ = std::io::Write::flush(&mut std::io::stdout());
        }
        Err(e) => {
            eprintln!("luna: {}: {}", command, e);
        }
    }
}