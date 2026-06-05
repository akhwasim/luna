// Luna learner — habit detection and pattern analysis

use crate::memory::Memory;

const HABIT_THRESHOLD: usize = 5;
const SEQUENCE_LENGTH: usize = 3;
const HISTORY_LIMIT: usize = 500;

pub struct DetectedPattern {
    pub commands: Vec<String>,
    pub count: usize,
}

pub fn detect_habits(memory: &Memory) -> Option<DetectedPattern> {
    let history = memory.get_commands_for_pattern_detection(HISTORY_LIMIT);

    if history.len() < SEQUENCE_LENGTH * HABIT_THRESHOLD {
        return None;
    }

    let commands: Vec<String> = history
        .into_iter()
        .map(|(cmd, _)| cmd)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect();

    let mut pattern_counts: std::collections::HashMap<Vec<String>, usize> =
        std::collections::HashMap::new();

    for window in commands.windows(SEQUENCE_LENGTH) {
        let pattern: Vec<String> = window.to_vec();

        let unique: std::collections::HashSet<&String> = pattern.iter().collect();
        if unique.len() < pattern.len() {
            continue;
        }

        if pattern.iter().all(|c| c.starts_with("cd ")) {
            continue;
        }

        *pattern_counts.entry(pattern).or_insert(0) += 1;
    }

    pattern_counts
        .into_iter()
        .filter(|(_, count)| *count >= HABIT_THRESHOLD)
        .max_by_key(|(_, count)| *count)
        .map(|(commands, count)| DetectedPattern { commands, count })
}

pub fn check_and_suggest(memory: &Memory, input: &str) {
    let skip_commands = ["clear", "cls", "history", "exit", "quit"];
    if skip_commands.contains(&input.trim()) || input.starts_with("luna ") {
        return;
    }

    let all = memory.get_commands_for_pattern_detection(10);
    if all.len() % 5 != 0 {
        return;
    }

    if let Some(pattern) = detect_habits(memory) {
        let pattern_key = pattern.commands.join("→");

        if memory.get_workflow(&pattern_key).is_some() {
            return;
        }
        if memory.is_pattern_rejected(&pattern_key) {
            return;
        }

        let has_dangerous = pattern.commands.iter().any(|cmd| {
            matches!(
                crate::safety::check(cmd),
                crate::safety::RiskLevel::Critical(_) | crate::safety::RiskLevel::High(_)
            )
        });
        if has_dangerous {
            return;
        }

        println!();
        println!("  🌙 Luna noticed a pattern");
        println!("  ─────────────────────────────────");
        println!("  You often run these {} commands together ({} times):",
            pattern.commands.len(), pattern.count);
        for cmd in &pattern.commands {
            println!("    → {}", cmd);
        }
        println!();
        print!("  Save as a workflow? (y/n) ❯ ");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();

        let mut input_buf = String::new();
        std::io::stdin().read_line(&mut input_buf).unwrap();

        if input_buf.trim().to_lowercase() == "y" {
            print!("  Name this workflow ❯ ");
            std::io::Write::flush(&mut std::io::stdout()).unwrap();

            let mut name = String::new();
            std::io::stdin().read_line(&mut name).unwrap();
            let name = name.trim().to_string();

            if !name.is_empty() {
                memory.save_workflow(&name, &pattern.commands);
                println!();
                println!("  ✅ Saved. Run it anytime with: luna run {}", name);
                println!();
            }
        } else {
            memory.reject_pattern(&pattern_key);
            println!("  Got it. Luna won't suggest this again.");
            println!();
        }
    }
}

pub fn run_workflow(memory: &Memory, name: &str) {
    match memory.get_workflow(name) {
        Some(commands) => {
            println!();
            println!("  Running '{}' ({} commands)", name, commands.len());
            println!("  → {}", commands.join(" → "));
            print!("  Press Enter to run or 'n' to cancel ❯ ");
            std::io::Write::flush(&mut std::io::stdout()).unwrap();

            let mut confirm = String::new();
            std::io::stdin().read_line(&mut confirm).unwrap();

            if confirm.trim().to_lowercase() == "n" {
                println!("  Skipped.");
                println!();
                return;
            }

            memory.increment_workflow_use(name);
            println!();

            for cmd in &commands {
                match crate::safety::check(cmd) {
                    crate::safety::RiskLevel::Critical(reason) => {
                        println!("  🚨 CRITICAL — {}", reason);
                        println!("  $ {}", cmd);
                        print!("  Type 'I UNDERSTAND' to run this step ❯ ");
                        std::io::Write::flush(&mut std::io::stdout()).unwrap();
                        let mut c = String::new();
                        std::io::stdin().read_line(&mut c).unwrap();
                        if c.trim() != "I UNDERSTAND" {
                            println!("  Step blocked. Workflow stopped.");
                            println!();
                            return;
                        }
                    }
                    crate::safety::RiskLevel::High(reason) => {
                        println!("  ⚠️  HIGH RISK — {}", reason);
                        println!("  $ {}", cmd);
                        print!("  Run this step? (y/n) ❯ ");
                        std::io::Write::flush(&mut std::io::stdout()).unwrap();
                        let mut c = String::new();
                        std::io::stdin().read_line(&mut c).unwrap();
                        if c.trim().to_lowercase() != "y" {
                            println!("  Step blocked. Workflow stopped.");
                            println!();
                            return;
                        }
                    }
                    _ => {}
                }
                println!("  $ {}", cmd);
                crate::commands::run(cmd);
                println!();
            }
            println!();
        }
        None => {
            println!();
            println!("  luna: workflow '{}' not found", name);
            println!("  Run 'luna workflows' to see all saved workflows.");
            println!();
        }
    }
}

pub fn create_workflow_interactive(memory: &Memory, name: &str) {
    if name.is_empty() {
        println!();
        println!("  Usage: luna create <name>");
        println!("  Example: luna create deploy");
        println!();
        return;
    }

    println!();
    println!("  Creating workflow '{}'", name);
    println!("  ─────────────────────────────────");
    println!("  Enter commands one by one. Empty line when done.");
    println!();

    let mut commands: Vec<String> = Vec::new();
    let mut i = 1;

    loop {
        print!("  Command {} ❯ ", i);
        std::io::Write::flush(&mut std::io::stdout()).unwrap();

        let mut cmd = String::new();
        std::io::stdin().read_line(&mut cmd).unwrap();
        let cmd = cmd.trim().to_string();

        if cmd.is_empty() || cmd.to_lowercase() == "done" {
            break;
        }

        commands.push(cmd);
        i += 1;
    }

    if commands.is_empty() {
        println!("  No commands entered. Workflow not saved.");
        println!();
        return;
    }

    println!();
    let mut blocked = false;
    for cmd in &commands {
        match crate::safety::check(cmd) {
            crate::safety::RiskLevel::Critical(reason) => {
                println!("  🚨 CRITICAL command in workflow: '{}'", cmd);
                println!("  Reason: {}", reason);
                print!("  This is extremely dangerous. Type 'I UNDERSTAND' to include or 'n' to cancel ❯ ");
                std::io::Write::flush(&mut std::io::stdout()).unwrap();
                let mut confirm = String::new();
                std::io::stdin().read_line(&mut confirm).unwrap();
                if confirm.trim() != "I UNDERSTAND" {
                    println!("  Workflow not saved.");
                    blocked = true;
                    break;
                }
            }
            crate::safety::RiskLevel::High(reason) => {
                println!("  ⚠️  HIGH RISK command in workflow: '{}'", cmd);
                println!("  Reason: {}", reason);
                print!("  Include it anyway? (y/n) ❯ ");
                std::io::Write::flush(&mut std::io::stdout()).unwrap();
                let mut confirm = String::new();
                std::io::stdin().read_line(&mut confirm).unwrap();
                if confirm.trim().to_lowercase() != "y" {
                    println!("  Workflow not saved.");
                    blocked = true;
                    break;
                }
            }
            _ => {}
        }
    }

    if blocked {
        println!();
        return;
    }

    let mut has_placeholders = false;
    for cmd in &commands {
        if let Some(label) = needs_runtime_input(cmd) {
            if !has_placeholders {
                println!("  Luna detected commands that need input at runtime:");
                has_placeholders = true;
            }
            println!("  '{}' will ask for: {}", cmd, label);
        }
    }

    memory.save_workflow(name, &commands);
    println!();
    println!("  ✅ Workflow '{}' saved with {} commands.", name, commands.len());
    println!("  Run with: luna run {}", name);
    println!();
}

pub fn delete_workflow(memory: &Memory, name: &str) {
    match memory.get_workflow(name) {
        Some(_) => {
            memory.remove_workflow(name);
            println!();
            println!("  ✅ Workflow '{}' deleted.", name);
            println!();
        }
        None => {
            println!();
            println!("  luna: workflow '{}' not found.", name);
            println!();
        }
    }
}

fn needs_runtime_input(cmd: &str) -> Option<String> {
    if cmd.trim() == "git commit -m" || cmd.contains("git commit -m \"\"") {
        return Some("Commit message".to_string());
    }
    if cmd.contains("git checkout -b") && !cmd.contains(" ") {
        return Some("Branch name".to_string());
    }
    if cmd.contains("git merge") && cmd.trim() == "git merge" {
        return Some("Branch name".to_string());
    }
    if cmd.contains("docker build -t") && cmd.trim().ends_with("-t") {
        return Some("Image name".to_string());
    }
    None
}

pub fn list_workflows(memory: &Memory) {
    let workflows = memory.list_workflows();

    if workflows.is_empty() {
        println!();
        println!("  No workflows saved yet.");
        println!("  Luna will suggest workflows when she detects patterns.");
        println!("  Or create one with: luna create <name>");
        println!();
        return;
    }

    println!();
    println!("  Saved workflows");
    println!("  ─────────────────────────────────");
    for (name, commands) in &workflows {
        println!("  {} → {}", name, commands.join(" → "));
    }
    println!();
    println!("  Run:    luna run <name>");
    println!("  Delete: luna delete <name>");
    println!();
}