use reedline::{DefaultPrompt, DefaultPromptSegment, Reedline, Signal};
use crate::commands;

// Luna shell — input loop and prompt

pub fn run() {
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

                commands::run(&input);
            }

            // Ctrl+C — clear current line
            Ok(Signal::CtrlC) => {
                println!("(use 'exit' to quit)");
            }

            // Ctrl+D — exit
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