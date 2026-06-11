// Luna is a multi-module Rust binary. This file is the entry point and
// intentionally holds no logic of its own — it only wires modules together
// and decides the boot sequence.
//
// Module map:
//   shell     — REPL loop, prompt, input handling, all command dispatch
//   commands  — executes a single command string in a child process
//   ai        — talks to the configured LLM provider, displays suggestions
//   memory    — SQLite-backed history of commands, errors, workflows
//   safety    — deterministic risk classifier (runs before any command)
//   learner   — habit detection, workflow creation and replay
//   stats     — read-only analytics over the memory tables
//   config    — ~/.luna/config.toml load/save + Provider enum + metadata
//   setup     — first-launch wizard (provider, API key, theme)

mod shell;
mod commands;
mod ai;
mod memory;
mod safety;
mod learner;
mod stats;
mod config;
mod setup;

#[tokio::main]
async fn main() {
    // Boot sequence: if no config file exists, this is a fresh install.
    // Run the setup wizard which writes ~/.luna/config.toml, then fall
    // through to the shell. If config already exists, we skip setup
    // entirely so repeat launches go straight to the prompt.
    //
    // The user can re-trigger setup at any time from inside the shell
    // with `luna config` — they don't have to delete the file by hand.
    if !config::config_exists() {
        setup::run().await;
    }

    shell::run();
}