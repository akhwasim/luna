use reedline::{DefaultPrompt, DefaultPromptSegment, Reedline, Signal};
use crate::commands;
use crate::ai;
use crate::memory::Memory;
use crate::safety;
use crate::learner;
use crate::stats;
use crate::config::{self, LunaConfig, ProviderConfig, Provider};
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

    println!("🌙 Luna v0.1");
    println!("Type 'exit' to quit\n");

    let mut line_editor = Reedline::create();

    'main: loop {
        let prompt = build_prompt();

        let cfg = match config::load() {
            Ok(c) => c,
            Err(e) => {
                eprintln!("luna: config error: {}", e);
                eprintln!("run `luna config` to set up.");
                continue;
            }
        };
        let cfg_for_command = cfg.clone();

        match line_editor.read_line(&prompt) {
            Ok(Signal::Success(line)) => {
                let input = line.trim().to_string();

                if input == "luna help" || input == "help" || input == "luna ?" || input == "luna --help" {
                    print_help();
                    continue;
                }

                if input == "luna model" || input == "luna models" {
                    run_luna_model(&cfg);
                    continue;
                }

                 if input == "luna config" {
                    std::thread::spawn(move || {
                        tokio::runtime::Runtime::new()
                            .unwrap()
                            .block_on(async {
                                setup::run_wizard().await;
                            });
                    })
                    .join()
                    .unwrap();
                    println!("  ✅ Configuration updated. Luna is ready to use!\n");
                    continue;
                }

                if input == "luna workflow" || input == "luna workflow list" {
                    learner::list_workflows(&memory);
                    continue;
                }

                if let Some(correction) = suggest_luna_subcommand(&input) {
                    println!();
                    println!("  luna: did you mean '{}'?", correction);
                    continue;
                }
                
                fn suggest_luna_subcommand(input: &str) -> Option<&'static str> {
                    let luna_typos: &[(&str, &str)] = &[
                        ("luna modle",      "luna model"),
                        ("luna hlep",       "luna help"),
                        ("luna configg",    "luna config"),
                        ("luna sttats",     "luna stats"),
                        ("luna wokrflow",   "luna workflow"),
                        ("luna wrokflow",   "luna workflow"),
                        ("luna runn",       "luna run"),
                        ("luna craete",     "luna create"),
                        ("luna delte",      "luna delete"),
                    ];
                    for (wrong, correct) in luna_typos {
                        if input == *wrong || input.starts_with(&format!("{} ", wrong)) {
                            return Some(correct);
                        }
                    }
                    None
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
                    let recent = memory.recent_commands(50);
                    let cfg_clone = cfg_for_command.clone();
                    let query_clone = query.clone();
                    let recent_clone = recent.clone();

                    std::thread::spawn(move || {
                        tokio::runtime::Runtime::new()
                            .unwrap()
                            .block_on(ai::ask(&query_clone, &cfg_clone, &context, &recent_clone));
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
                        let recent = memory.recent_commands(50);
                        let failed_cmd = input.clone();
                        let error_out = result.error_output.clone();
                        let cfg_for_ai = cfg_for_command.clone();
                        let recent_clone = recent.clone();

                        memory.save_error(&failed_cmd, &error_out, None);

                        std::thread::spawn(move || {
                            tokio::runtime::Runtime::new()
                                .unwrap()
                                .block_on(ai::fix_error(
                                    &failed_cmd,
                                    &error_out,
                                    &cfg_for_ai,
                                    &context,
                                    &recent_clone,
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

//Luna help command

fn print_help() {
    println!();
    println!("  🌙 Luna — your terminal that remembers");
    println!("  ─────────────────────────────────");
    println!();
    println!("  Built-in commands:");
    println!("    exit / quit              leave Luna");
    println!("    history                  last 10 commands with directories");
    println!();
    println!("  AI:");
    println!("    /luna <question>         ask Luna anything terminal-related");
    println!("    \\luna <question>         alias for /luna");
    println!();
    println!("  Luna commands:");
    println!("    luna help                show this help");
    println!("    luna config              set up or change AI provider, key, and theme");
    println!("    luna theme               list and switch themes");
    println!("    luna model               switch AI provider or add a new one");
    println!("    luna stats               show your patterns and error clusters");
    println!("    luna workflows           list all saved workflows");
    println!("    luna run <name>          run a saved workflow");
    println!("    luna create <name>       create a workflow interactively");
    println!("    luna delete <name>       delete a saved workflow");
    println!();
    println!("  Workflow commands (luna workflow <action>):");
    println!("    luna workflow list       same as 'luna workflows'");
    println!("    luna workflow create X   same as 'luna create X'");
    println!("    luna workflow run X      same as 'luna run X'");
    println!("    luna workflow delete X   same as 'luna delete X'");
    println!();
    println!("  Examples:");
    println!("    /luna find all .rs files modified today");
    println!("    luna run deploy");
    println!("    luna theme moonlight");
    println!();

}

fn run_luna_model(cfg: &LunaConfig) {
    let configured: Vec<(String, ProviderConfig)> = cfg
        .providers
        .0
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();

    if configured.is_empty() {
        println!("\n  No providers configured. Run `luna config` to set one up.\n");
        return;
    }

    println!();
    println!("  Switch AI provider");
    println!("  ─────────────────────────────────");
    for (i, (key, p)) in configured.iter().enumerate() {
        let active = if *key == cfg.ai.active { " ← active" } else { "" };
        let key_marker = if p.api_key.is_empty() && p.provider.needs_key() {
            " (no key set)"
        } else {
            ""
        };
        println!("  {}. {} ({}){}{}", i + 1, p.provider.label(), p.model, key_marker, active);
    }
    println!("  a. Add a new provider");
    println!("  q. Quit");
    println!();

    loop {
        let choice = read_line_stripped("❯ ");
        if choice == "q" || choice.is_empty() {
            println!();
            return;
        }
        if choice == "a" {
            add_new_provider();
            return;
        }
        if let Ok(n) = choice.parse::<usize>() {
            if n >= 1 && n <= configured.len() {
                let (key, _) = &configured[n - 1];
                switch_provider(key);
                return;
            }
        }
        println!("  Please enter 1-{}, 'a', or 'q'.", configured.len());
    }
}

fn switch_provider(new_key: &str) {
    let cfg = match config::load() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("\n  ⚠️  Could not load config: {}\n", e);
            return;
        }
    };
    let mut new_cfg = cfg.clone();
    new_cfg.ai.active = new_key.to_string();
    match config::save(&new_cfg) {
        Ok(_) => {
            let model = new_cfg.providers.0.get(new_key).map(|p| p.model.clone()).unwrap_or_default();
            println!();
            println!("  ✅ Switched to {} ({})", new_key, model);
            println!("  Luna is ready to use!.\n");
        }
        Err(e) => eprintln!("\n  ⚠️  Could not save: {}\n", e),
    }
}

fn add_new_provider() {
    let providers = [
        Provider::Groq,
        Provider::OpenAI,
        Provider::Ollama,
        Provider::Google,
        Provider::Anthropic,
        Provider::OpenRouter,
    ];

    println!();
    println!("  Which provider do you want to add?");
    for (i, p) in providers.iter().enumerate() {
        println!("  {}. {}", i + 1, p.label());
    }
    println!();

    let provider = loop {
        let choice = read_line_stripped("Choice ❯ ");
        match choice.trim() {
            "1" => break Provider::Groq,
            "2" => break Provider::OpenAI,
            "3" => break Provider::Ollama,
            "4" => break Provider::Google,
            "5" => break Provider::Anthropic,
            "6" => break Provider::OpenRouter,
            "q" => {
                println!();
                return;
            }
            _ => println!("  Please enter 1-6 or 'q' to cancel."),
        }
    };

    let api_key = if provider.needs_key() {
        prompt_api_key_for_add(&provider)
    } else {
        String::new()
    };

    let cfg = match config::load() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("\n  ⚠️  Could not load config: {}\n", e);
            return;
        }
    };

    let key_name = provider.to_string();
    let model = provider.default_model().to_string();
    let mut new_cfg = cfg.clone();
    new_cfg.providers.0.insert(
        key_name.clone(),
        ProviderConfig {
            provider,
            model,
            api_key,
            base_url: None,
        },
    );
    new_cfg.ai.active = key_name.clone();

    match config::save(&new_cfg) {
        Ok(_) => {
            println!();
            println!("  ✅ Added and switched to {}", key_name);
            println!("  Luna is ready to use!.\n");
        }
        Err(e) => eprintln!("\n  ⚠️  Could not save: {}\n", e),
    }
}

fn prompt_api_key_for_add(provider: &Provider) -> String {
    let signup = provider.signup_url();
    let label = provider.label();
    let name = label.split(" (").next().unwrap_or(label);    println!();
    println!("Enter your {} API key", name);
    if !signup.is_empty() {
        println!("(free at {})", signup);
    }
    println!();
    read_line_stripped("❯ ")
}

fn read_line_stripped(prompt: &str) -> String {
    print!("{}", prompt);
    use std::io::Write;
    std::io::stdout().flush().ok();
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap_or(0);
    s.trim().to_string()
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