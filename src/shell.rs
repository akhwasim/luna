use reedline::{DefaultPrompt, DefaultPromptSegment, Reedline, Signal};
use crate::commands;
use crate::ai;
use crate::memory::Memory;

// Luna shell — input loop and prompt

fn load_env() {
    let home = std::env::var("HOME").unwrap_or_default();
    let env_path = format!("{}/.luna/.env", home);
    let _ = dotenvy::from_path(env_path);
}

pub fn run() {
    load_env();

    let memory = Memory::new().unwrap_or_else(|e| {
        eprintln!("luna: memory error: {}", e);
        std::process::exit(1);
    });

    println!("🌙 Luna v0.1");
    println!("Type 'exit' to quit\n");

    let mut line_editor = Reedline::create();

    loop {
        let prompt = build_prompt();

        match line_editor.read_line(&prompt) {
            Ok(Signal::Success(line)) => {
                let input = line.trim().to_string();

                if input.is_empty() {
                    continue;
                }

                if input == "exit" || input == "quit" {
                    println!("Goodbye. 🌙");
                    break;
                }

                // AI trigger
                if input.starts_with("\\luna ") || input.starts_with("/luna ") {
                    let query = input
                        .trim_start_matches("\\luna ")
                        .trim_start_matches("/luna ")
                        .to_string();

                    let api_key = std::env::var("GROQ_API_KEY")
                        .unwrap_or_default();

                    if api_key.is_empty() {
                        eprintln!("luna: GROQ_API_KEY not set in ~/.luna/.env");
                    } else {
                        let context = memory.context_for_ai();

                        std::thread::spawn(move || {
                            tokio::runtime::Runtime::new()
                                .unwrap()
                                .block_on(ai::ask(&query, &api_key, &context));
                        })
                        .join()
                        .unwrap();
                    }
                    continue;
                }

                // Built-ins that don't get saved to history
                if input == "history" {
                    memory.print_recent(10);
                    continue;
                }

                // Save and execute
                let cwd = std::env::current_dir()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();

                memory.save_command(&input, &cwd, true);
                commands::run(&input);
            }

            Ok(Signal::CtrlC) => {
                println!("(use 'exit' to quit)");
            }

            Ok(Signal::CtrlD) => {
                println!("Goodbye. 🌙");
                break;
            }

            Err(e) => {
                eprintln!("Error: {}", e);
                break;
            }
        }
    }
}

fn build_prompt() -> DefaultPrompt {
    let cwd = std::env::current_dir()
        .unwrap_or_default();

    let home = std::env::var("HOME").unwrap_or_default();
    let path_str = cwd.to_string_lossy();

    let display_path = if path_str.starts_with(&home) {
        path_str.replacen(&home, "~", 1)
    } else {
        path_str.to_string()
    };

    DefaultPrompt::new(
        DefaultPromptSegment::Basic(format!("🌙 {}", display_path)),
        DefaultPromptSegment::Basic("".to_string()),
    )
}