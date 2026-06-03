use reedline::{DefaultPrompt, DefaultPromptSegment, Reedline, Signal};
use crate::commands;
use crate::ai;
use crate::memory::Memory;
use crate::safety;
use crate::learner;

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

                // Handle luna workflow commands
                if input == "luna workflow" || input == "luna workflow list" {
                    learner::list_workflows(&memory);
                    continue;
                }

                if input.starts_with("luna workflow create ") {
                    let name = input.trim_start_matches("luna workflow create ").trim();
                    learner::create_workflow_interactive(&memory, name);
                    continue;
                }

                if input.starts_with("luna create ") {
                    let name = input.trim_start_matches("luna create ").trim();
                    learner::create_workflow_interactive(&memory, name);
                    continue;
                }

                if input.starts_with("luna workflow run ") {
                    let name = input.trim_start_matches("luna workflow run ").trim();
                    learner::run_workflow(&memory, name);
                    continue;
                }

                if input.starts_with("luna workflow delete ") {
                    let name = input.trim_start_matches("luna workflow delete ").trim();
                    learner::delete_workflow(&memory, name);
                    continue;
                }

                if input.starts_with("luna delete ") {
                    let name = input.trim_start_matches("luna delete ").trim();
                    learner::delete_workflow(&memory, name);
                    continue;
                }

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

                // Built-in history
                if input == "history" {
                    memory.print_recent(10);
                    continue;
                }

                if input == "luna workflows" {
                    learner::list_workflows(&memory);
                    continue;
                }

                if input == "luna stats" {
                    println!("\n  luna stats coming soon.\n");
                    continue;
                }

                if input.starts_with("luna run ") {
                    let name = input.trim_start_matches("luna run ").trim();
                    learner::run_workflow(&memory, name);
                    continue;
                }

                // Safety check before execution
                match safety::check(&input) {
                safety::RiskLevel::Critical(reason) => {
                    println!();
                    println!("  🚨 CRITICAL — Extremely dangerous command");
                    println!("  ─────────────────────────────────");
                    println!("  {}", reason);
                    println!("  $ {}", input);
                    println!();
                    print!("  Type 'I UNDERSTAND' to proceed or Any key to cancel ❯ ");
                    std::io::Write::flush(&mut std::io::stdout()).unwrap();
                    let mut confirm = String::new();
                    std::io::stdin().read_line(&mut confirm).unwrap();
                    if confirm.trim() != "I UNDERSTAND" {
                        println!("  Blocked.");
                        println!();
                        continue;
                    }
                    println!();
                }
                safety::RiskLevel::High(reason) => {
                    println!();
                    println!("  ⚠️  HIGH RISK");
                    println!("  ─────────────────────────────────");
                    println!("  {}", reason);
                    println!("  $ {}", input);
                    print!("  Run anyway? (y/n) ❯ ");
                    std::io::Write::flush(&mut std::io::stdout()).unwrap();
                    let mut confirm = String::new();
                    std::io::stdin().read_line(&mut confirm).unwrap();
                    if confirm.trim().to_lowercase() != "y" {
                        println!("  Blocked.");
                        println!();
                        continue;
                    }
                    println!();
                }
                safety::RiskLevel::Medium(reason) => {
                    println!();
                    println!("  ⚡ MEDIUM — {}", reason);
                    println!();
                }
                safety::RiskLevel::Safe => {}
                }

                // Save and execute
                let cwd = std::env::current_dir()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();

                let result = commands::run(&input);
                memory.save_command(&input, &cwd, result.success);

                learner::check_and_suggest(&memory, &input);

                // Error detection
                if !result.success && !result.error_output.is_empty() {
                    let is_permission_error = result.error_output.contains("Permission denied")
                        || result.error_output.contains("Operation not permitted")
                        || result.error_output.contains("Interactive authentication required");
                    let already_has_sudo = input.starts_with("sudo ");

                    // Never suggest sudo escalation if original command didn't use sudo
                    if is_permission_error && !already_has_sudo {
                        println!();
                        println!("  luna: permission denied");
                        println!("  If you intended to run with elevated privileges, add sudo.");
                        println!();
                        memory.save_error(&input, &result.error_output, None);
                    } else {
                        let api_key = std::env::var("GROQ_API_KEY")
                            .unwrap_or_default();

                        if !api_key.is_empty() {
                            let context = memory.context_for_ai();
                            let failed_cmd = input.clone();
                            let error_out = result.error_output.clone();

                            memory.save_error(&failed_cmd, &error_out, None);

                            std::thread::spawn(move || {
                                tokio::runtime::Runtime::new()
                                    .unwrap()
                                    .block_on(ai::fix_error(
                                        &failed_cmd,
                                        &error_out,
                                        &api_key,
                                        &context,
                                    ));
                            })
                            .join()
                            .unwrap();
                        }
                    }
                }
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