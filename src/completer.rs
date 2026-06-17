// Bash-like tab completion for Luna.

use reedline::{Completer, Suggestion, Span};
use std::fs;
use std::path::Path;
use crate::memory::Memory;

const LUNA_SUBCOMMANDS: &[(&str, &str)] = &[
    ("luna stats",      "show your patterns and error clusters"),
    ("luna workflows",  "list saved workflows"),
    ("luna run ",       "run a saved workflow"),
    ("luna create ",    "create a new workflow"),
    ("luna delete ",    "delete a saved workflow"),
    ("luna workflow ",  "explicit workflow namespace"),
];

const BUILTIN_COMMANDS: &[&str] = &[
    "exit", "quit", "history", "cd", "pwd", "clear",
    "luna", "/luna", "\\luna",
];

pub struct LunaCompleter {
    workflow_names: Vec<String>,
    path_commands: Vec<String>,
}

impl LunaCompleter {
    pub fn new(memory: &Memory, path_commands: Vec<String>) -> Self {
        let workflow_names = memory
            .list_workflows()
            .into_iter()
            .map(|(name, _)| name)
            .collect();
        LunaCompleter { workflow_names, path_commands }
    }
}

impl Completer for LunaCompleter {
    fn complete(&mut self, line: &str, pos: usize) -> Vec<Suggestion> {
        if pos > line.len() {
            return Vec::new();
        }
        let typed = &line[..pos];

        // "luna run <partial>" → workflow names
        if let Some(partial) = typed.strip_prefix("luna run ") {
            return complete_workflows(&self.workflow_names, partial, typed.len());
        }

        // "luna <partial>" → subcommands
        if typed.starts_with("luna ") || typed == "luna" {
            return complete_luna_subcommands(typed);
        }

        // No space yet → completing the first word (a command)
        if !typed.contains(' ') {
            return complete_command(typed, &self.path_commands);
        }

        // Otherwise → completing a file/dir argument
        complete_path(typed)
    }
}

fn complete_luna_subcommands(typed: &str) -> Vec<Suggestion> {
    LUNA_SUBCOMMANDS
        .iter()
        .filter(|(cmd, _)| cmd.starts_with(typed) && *cmd != typed)
        .map(|(cmd, help)| Suggestion {
            value: cmd.to_string(),
            description: Some(help.to_string()),
            style: None,
            extra: None,
            span: Span::new(0, typed.len()),
            append_whitespace: false,
        })
        .collect()
}

fn complete_workflows(workflow_names: &[String], partial: &str, typed_len: usize) -> Vec<Suggestion> {
    workflow_names
        .iter()
        .filter(|name| name.starts_with(partial))
        .map(|name| Suggestion {
            value: name.clone(),
            description: Some("saved workflow".to_string()),
            style: None,
            extra: None,
            span: Span::new(typed_len - partial.len(), typed_len),
            append_whitespace: true,
        })
        .collect()
}

fn complete_command(typed: &str, path_commands: &[String]) -> Vec<Suggestion> {
    let mut seen = std::collections::HashSet::new();
    let mut results = Vec::new();

    for cmd in BUILTIN_COMMANDS {
        if cmd.starts_with(typed) && *cmd != typed && seen.insert(cmd.to_string()) {
            results.push(Suggestion {
                value: cmd.to_string(),
                description: None,
                style: None,
                extra: None,
                span: Span::new(0, typed.len()),
                append_whitespace: true,
            });
        }
    }

    for cmd in path_commands {
        if cmd.starts_with(typed) && cmd != typed && seen.insert(cmd.clone()) {
            results.push(Suggestion {
                value: cmd.clone(),
                description: None,
                style: None,
                extra: None,
                span: Span::new(0, typed.len()),
                append_whitespace: true,
            });
        }
    }

    results.sort_by(|a, b| a.value.cmp(&b.value));
    results
}

fn complete_path(typed: &str) -> Vec<Suggestion> {

    let arg_start = typed.rfind(' ').map(|i| i + 1).unwrap_or(0);
    let partial = &typed[arg_start..];

    let (search_dir, prefix) = match partial.rfind('/') {
        Some(i) => (&partial[..=i], &partial[i + 1..]),
        None => ("", partial),
    };

    let expanded_dir = expand_home(search_dir);
    let read_dir = if expanded_dir.is_empty() { ".".to_string() } else { expanded_dir };

    let entries = match fs::read_dir(&read_dir) {
        Ok(e) => e,
        Err(_) => return Vec::new(),
    };

    let mut results: Vec<Suggestion> = entries
        .filter_map(|e| e.ok())
        .filter_map(|entry| {
            let name = entry.file_name().to_string_lossy().to_string();
            if !name.starts_with(prefix) || (prefix.is_empty() && name.starts_with('.')) {
                return None;
            }

            let is_dir = entry.path().is_dir();
            let value = if is_dir {
                format!("{}{}/", search_dir, name)
            } else {
                format!("{}{}", search_dir, name)
            };

            Some(Suggestion {
                value,
                description: None,
                style: None,
                extra: None,
                span: Span::new(arg_start, typed.len()),
                append_whitespace: !is_dir,
            })
        })
        .collect();

    results.sort_by(|a, b| a.value.cmp(&b.value));
    results
}

fn expand_home(path: &str) -> String {
    if path.starts_with("~/") {
        let home = std::env::var("HOME").unwrap_or_default();
        format!("{}/{}", home, &path[2..])
    } else if path == "~" {
        std::env::var("HOME").unwrap_or_default()
    } else {
        path.to_string()
    }
}

pub fn build_path_commands() -> Vec<String> {
    let path_var = std::env::var("PATH").unwrap_or_default();
    let mut commands = std::collections::HashSet::new();

    for dir in path_var.split(':') {
        let dir_path = Path::new(dir);
        if let Ok(entries) = fs::read_dir(dir_path) {
            for entry in entries.filter_map(|e| e.ok()) {
                if let Some(name) = entry.file_name().to_str() {
                    commands.insert(name.to_string());
                }
            }
        }
    }

    commands.into_iter().collect()
}