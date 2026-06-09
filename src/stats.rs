// Luna stats — command patterns and error clustering

use crate::memory::Memory;

pub fn show(memory: &Memory) {
    let total = get_total_commands(memory);
    let success_rate = get_success_rate(memory);
    let most_used = get_most_used(memory, 5);
    let error_clusters = cluster_errors(memory);
    let total_errors = get_total_errors(memory);

    println!();
    println!("  Your Terminal Patterns");
    println!("  ─────────────────────────────────");
    println!("  Commands run:     {}", total);
    println!("  Success rate:     {}%", success_rate);
    println!("  Errors logged:    {}", total_errors);
    println!();

    if !most_used.is_empty() {
        println!("  Most used commands:");
        for (cmd, count) in &most_used {
            println!("  {:>4}x  {}", count, cmd);
        }
        println!();
    }

    if !error_clusters.is_empty() {
        println!("  Error patterns:");
        println!("  ─────────────────────────────────");
        for (category, count, fix) in &error_clusters {
            let pct = if total_errors > 0 {
                (count * 100) / total_errors
            } else {
                0
            };
            let bar = make_bar(pct);
            println!("  {} {}%  {} ({} times)", bar, pct, category, count);
            println!("  Usually fixed by: {}", fix);
            println!();
        }
    }

    println!("  ─────────────────────────────────");
    println!();
}

fn get_total_commands(memory: &Memory) -> usize {
    memory.get_stat("SELECT COUNT(*) FROM commands")
}

fn get_total_errors(memory: &Memory) -> usize {
    memory.get_stat("SELECT COUNT(*) FROM errors")
}

fn get_success_rate(memory: &Memory) -> usize {
    let total = get_total_commands(memory);
    if total == 0 {
        return 100;
    }
    let success = memory.get_stat(
        "SELECT COUNT(*) FROM commands WHERE success = 1"
    );
    (success * 100) / total
}

fn get_most_used(memory: &Memory, limit: usize) -> Vec<(String, usize)> {
    memory.get_command_counts(limit)
}

fn cluster_errors(memory: &Memory) -> Vec<(String, usize, String)> {
    let errors = memory.get_all_errors();
    let total = errors.len();

    if total == 0 {
        return Vec::new();
    }

    let mut dependency = 0usize;
    let mut permission = 0usize;
    let mut typo = 0usize;
    let mut network = 0usize;
    let mut not_found = 0usize;
    let mut other = 0usize;

    for (cmd, error) in &errors {
        let e = error.to_lowercase();
        let c = cmd.to_lowercase();

        if e.contains("no such command") || e.contains("not a git command")
            || e.contains("command not found") || e.contains("unknown command")
            || e.contains("similar name exists") {
            typo += 1;
        } else if e.contains("permission denied") || e.contains("operation not permitted")
            || e.contains("authentication required") {
            permission += 1;
        } else if e.contains("no such file or directory") && !c.contains("rm") {
            not_found += 1;
        } else if e.contains("module not found") || e.contains("missing crate")
            || e.contains("cannot find") || e.contains("unresolved import") {
            dependency += 1;
        } else if e.contains("connection refused") || e.contains("timeout")
            || e.contains("network unreachable") {
            network += 1;
        } else {
            other += 1;
        }
    }

    let mut clusters = vec![
        ("Typos / wrong command".to_string(),  typo,       "check command spelling".to_string()),
        ("File not found".to_string(),          not_found,  "check path with ls first".to_string()),
        ("Permission denied".to_string(),       permission, "add sudo".to_string()),
        ("Missing dependency".to_string(),      dependency, "install missing package".to_string()),
        ("Network issues".to_string(),          network,    "check connection".to_string()),
        ("Other".to_string(),                   other,      "check error message".to_string()),
    ];

    clusters.retain(|(_, count, _)| *count > 0);
    clusters.sort_by(|a, b| b.1.cmp(&a.1));
    clusters
}

fn make_bar(pct: usize) -> String {
    let filled = pct / 10;
    let empty = 10 - filled.min(10);
    format!("[{}{}]", "█".repeat(filled), "░".repeat(empty))
}