use reedline::{DefaultPrompt, DefaultPromptSegment, Reedline, Signal};
use crate::commands;
use crate::ai;
use crate::memory::Memory;
use crate::safety;
use crate::learner;
use crate::stats;
use crate::config;
use crate::setup;

fn load_env() {
    let home = std::env::var("HOME").unwrap_or_default();
    let env_path = format!("{}/.luna/.env", home);
    let _ = dotenvy::from_path(env_path);
}

fn suggest_builtin(cmd: &str) -> Option<&'static str> {
    let typos = [
        ("cler",   "clear"),
        ("clera",  "clear"),
        ("celar",  "clear"),
        ("clean",  "clear"),
        ("cleat",  "clear"),
        ("exot",   "exit"),
        ("exut",   "exit"),
        ("ecit",   "exit"),
        ("exiy",   "exit"),
        ("exiit",  "exit"),
        ("dc",     "cd"),
        ("cs",     "cd"),
        ("pdw",    "pwd"),
        ("pws",    "pwd"),
        ("vm",     "vim"),
        ("vom",    "vim"),
        ("gut",    "git"),
        ("gti",    "git"),
        ("got",    "git"),
        ("gig",    "git"),
        ("py",     "python3"),
        ("pyhton", "python3"),
        ("pytohn", "python3"),
        ("pythno", "python3"),
        ("ndoe",   "node"),
        ("nod",    "node"),
        ("dokcer", "docker"),
        ("dcoker", "docker"),
        ("sl",     "ls"),
        ("kl",     "ls"),
    ];

    for (wrong, correct) in &typos {
        if cmd == *wrong {
            return Some(correct);
        }
    }
    None
}

pub fn run() {
    load_env();

    let memory = Memory::new().unwrap_or_else(|e| {
        eprintln!("luna: memory error: {}", e);
        std::process::exit(1);
    });

    let cfg = match config::load() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("luna: config error: {}", e);
            eprintln!("run `luna config` to set up.");
            std::process::exit(1);
        }
    };

    println!("🌙 Luna v0.1");
    println!("Type 'exit' to quit\n");

    let mut line_editor = Reedline::create();

    'main: loop {
        let prompt = build_prompt();
        let cfg_for_command = cfg.clone();

        match line_editor.read_line(&prompt) {
            Ok(Signal::Success(line)) => {
                let input = line.trim().to_string();

                if input == "luna config" {
                    std::thread::spawn(move || {
                        tokio::runtime::Runtime::new()
                            .unwrap()
                            .block_on(async {
                                setup::run().await;
                            });
                    })
                    .join()
                    .unwrap();
                    println!("Restart Luna for changes to take effect.\n");
                    continue;
                }

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

                if input.starts_with("\\luna ") || input.starts_with("/luna ") {
                    let query = input
                        .trim_start_matches("\\luna ")
                        .trim_start_matches("/luna ")
                        .to_string();

                    let context = memory.context_for_ai();
                    let cfg_clone = cfg_for_command.clone();
                    let query_clone = query.clone();

                    std::thread::spawn(move || {
                        tokio::runtime::Runtime::new()
                            .unwrap()
                            .block_on(ai::ask(&query_clone, &cfg_clone, &context));
                    })
                    .join()
                    .unwrap();
                    continue;
                }

                if input == "history" {
                    memory.print_recent(10);
                    continue;
                }

                if input == "luna workflows" {
                    learner::list_workflows(&memory);
                    continue;
                }

                if input == "luna stats" {
                    stats::show(&memory);
                    continue;
                }

                if input.starts_with("luna run ") {
                    let name = input.trim_start_matches("luna run ").trim();
                    learner::run_workflow(&memory, name);
                    continue;
                }

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

                let cwd = std::env::current_dir()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();

                let result = commands::run(&input);
                memory.save_command(&input, &cwd, result.success);

                learner::check_and_suggest(&memory, &input);

                if !result.success && !result.error_output.is_empty() {
                    let first_word = input.split_whitespace().next().unwrap_or("");
                    if let Some(correction) = suggest_builtin(first_word) {
                        println!();
                        println!("  luna: did you mean '{}'?", correction);
                        println!();
                        if correction == "exit" || correction == "quit" {
                            println!("  Goodbye. 🌙");
                            break 'main;
                        }
                        if correction == "clear" {
                            print!("\x1B[2J\x1B[1;1H");
                            let _ = std::io::Write::flush(&mut std::io::stdout());
                        } else {
                            let rest = input[first_word.len()..].trim();
                            let corrected = if rest.is_empty() {
                                correction.to_string()
                            } else {
                                format!("{} {}", correction, rest)
                            };
                            commands::run(&corrected);
                        }
                        continue;
                    }

                    let is_permission_error = result.error_output.contains("Permission denied")
                        || result.error_output.contains("Operation not permitted")
                        || result.error_output.contains("Interactive authentication required");
                    let already_has_sudo = input.starts_with("sudo ");

                    if is_permission_error && !already_has_sudo {
                        println!();
                        println!("  luna: permission denied");
                        println!("  If you intended to run with elevated privileges, add sudo.");
                        println!();
                        memory.save_error(&input, &result.error_output, None);
                    } else {
                        let context = memory.context_for_ai();
                        let failed_cmd = input.clone();
                        let error_out = result.error_output.clone();
                        let cfg_for_ai = cfg_for_command.clone();

                        memory.save_error(&failed_cmd, &error_out, None);

                        std::thread::spawn(move || {
                            tokio::runtime::Runtime::new()
                                .unwrap()
                                .block_on(ai::fix_error(
                                    &failed_cmd,
                                    &error_out,
                                    &cfg_for_ai,
                                    &context,
                                ));
                        })
                        .join()
                        .unwrap();
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
    let cwd = std::env::current_dir().unwrap_or_default();
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