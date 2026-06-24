<div align="center">

# 🌙 Luna

**The terminal that gets smarter every time you use it.**

> *Your terminal has amnesia. I'm fixing it.*

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Built with Rust](https://img.shields.io/badge/Built%20with-Rust-orange.svg)](https://www.rust-lang.org)
[![Status: Building in Public](https://img.shields.io/badge/Status-Building%20in%20Public-brightgreen.svg)]()

*Built by [Wasim Akhtar](https://github.com/akhwasim) · Building in public on [@akhwasim](https://x.com/akhwasim)*

</div>

---



## Installation

### Linux / WSL

Install Luna with a single command:

```bash
curl -fsSL https://raw.githubusercontent.com/akhwasim/luna/main/install.sh | bash
```

Restart your terminal and run:

```bash
luna
```

### Requirements

* Linux or WSL
* Internet connection for cloud AI providers
* Optional: Ollama for fully local/offline usage

### First Launch

On first launch, Luna will guide you through:

1. Choosing an AI provider
2. Entering an API key (if required)
3. Creating your configuration

Supported providers:

* Groq
* OpenAI
* Ollama
* Google Gemini
* Anthropic Claude
* OpenRouter
---

https://github.com/user-attachments/assets/5dd48d83-26c3-4ccf-a613-7391ccf3da04

---

## The Problem

Every terminal you have ever used has the same flaw.

Close it. Reopen it. It doesn't know who you are. It doesn't know what you were working on. It doesn't remember the error you spent three hours fixing last Tuesday, the fix that finally worked, or that you always run the same four commands at the start of every session.

It just blinks at you. Empty. Again.

That is not a missing feature. That is amnesia. And every terminal has it.

**Luna is the fix.**

---

## What is Luna?

Luna is a **standalone intelligent terminal** written in Rust. It understands natural language, executes commands safely, remembers everything across sessions, detects your patterns, and gets smarter about how *you specifically* work, every single day.

It is not a plugin. It is not a wrapper around bash. It is a standalone intelligent terminal built in Rust, with first-class support for six AI providers (cloud and local), persistent SQLite-backed memory, deterministic command safety, and a learning loop that ranks suggestions by your actual history.

| Every other terminal | Luna |
|---|---|
| Forgets you the moment you close it | Remembers every command, every error, every workflow, every session |
| Same suggestions for everyone | Ranks AI suggestions by *your* command history |
| Reacts to what you type | Anticipates what you need |
| One AI or none | Six providers — switch instantly without restart |
| No safety layer | Deterministic risk classification on every command |
| Stateless | Accumulates intelligence over time |

---

## How Luna Works

```
You type a command
       ↓
Safety Engine inspects it before anything runs
       ↓
Luna executes it
       ↓
If it fails, Luna analyzes the error and suggests a fix
       ↓
The fix is verified by the same Safety Engine
       ↓
Everything is saved to SQLite memory
       ↓
Patterns are detected across sessions over time
       ↓
Next session, Luna already knows where you left off
```


## Highlights

- **6 AI providers out of the box** — Groq, OpenAI, Ollama, Google Gemini, Anthropic Claude, OpenRouter. Switch between them mid-session with `luna model`. No restart. No re-pasting keys.
- **Personalized ranking** — when the AI returns multiple valid commands, Luna re-orders them so your most-used one comes first, marked with `⭐`.
- **Deterministic safety** — every command, whether typed by you, suggested by AI, or run as a workflow step, passes through the same risk classifier. Recursive shell-wrapper detection catches `bash -c "rm -rf /"` and friends.
- **Pattern learning** — Luna watches your command sequences. When you run the same three commands in the same order five times, it offers to save them as a workflow.
- **Auto error recovery** — when a command fails, Luna analyzes the error, suggests a fix, runs the fix through the same safety engine, and presents it for your approval.
- **Tab completion** — bash-like completion for `luna` subcommands, workflow names, `$PATH` commands, and file paths.
- **Zero telemetry** — your data stays on your machine in `~/.luna/`. No accounts. No cloud sync. No tracking.

---

## Quick Start

```bash
git clone https://github.com/akhwasim/luna.git
cd luna
cargo build --release
./target/release/luna
```

On first launch, Luna walks you through choosing an AI provider and pasting an API key (or selecting Ollama for fully local/offline). That's it — you're in.

**Requirements:** Rust 1.70+, Linux or WSL (Ubuntu recommended).

### Using Ollama (no API key, fully local)

```bash
# Install Ollama from https://ollama.com
ollama serve &
ollama pull llama3.2
```

Pick "Ollama" during Luna's first-launch setup. No key required. Works offline.

### Using Groq (fast, free tier)

Sign up free at [console.groq.com](https://console.groq.com), grab an API key, and paste it when Luna asks. Groq's free tier is generous and very fast.

---

## Architecture
```
User
  ↓
Safety Engine
  ↓
Execute
  ↓
Memory
  ↓
Learning
  ↓
Smarter Suggestions

```

Luna is a single Rust binary organized as eight modules. Each owns one responsibility and talks to the others through narrow APIs.

```
src/
├── main.rs       — entry point; gates first-launch setup, then hands off
├── shell.rs      — REPL loop, prompt, command dispatch, all input handling
├── commands.rs   — child-process execution (simple vs. shell)
├── ai.rs         — provider dispatch, request/response handling, display
├── memory.rs     — SQLite-backed history, errors, workflows, context
├── safety.rs     — deterministic risk classification (CRITICAL → LOW)
├── learner.rs    — habit detection, workflow creation and replay
├── stats.rs      — read-only analytics over the memory tables
├── config.rs     — ~/.luna/config.toml load/save + Provider enum + metadata
├── setup.rs      — first-launch wizard (provider, API key)
└── completer.rs  — bash-like tab completion
```

**The flow when you type something:**

```
You type a command
       ↓
Safety Engine classifies it (deterministic, in Rust)
       ↓
If CRITICAL: hard block unless you type "I UNDERSTAND"
If HIGH:     y/n prompt
If MEDIUM:   warning, then run
If LOW:      run silently
       ↓
Luna executes it
       ↓
Result saved to SQLite
       ↓
If it failed → AI suggests a fix (safety-checked again before showing)
       ↓
If it succeeded → learner checks for repeated patterns, suggests workflows
       ↓
Next prompt is ready
```

---

## The Multi-AI System

Luna ships with support for **six AI providers** out of the box. Switching is one command, no restart.

| Provider | Cost | Network | Use case |
|---|---|---|---|
| **Groq** | Free tier | Cloud | Fastest inference, generous free quota — recommended default |
| **OpenAI** | Pay per token | Cloud | Industry standard, GPT-4o, GPT-4o-mini |
| **Google Gemini** | Free tier | Cloud | Google's models, gemini-flash-latest |
| **Anthropic Claude** | Pay per token | Cloud | Claude Sonnet 4.5, best-in-class reasoning |
| **OpenRouter** | Free + paid tiers | Cloud | Aggregator — access dozens of models via one key |
| **Ollama** | Free, local | None | Fully offline, your machine, your data |

### Switching providers

```bash
🌙 ~/luna〉luna model

  Switch AI provider
  ─────────────────────────────────
  1. Groq (free, fast — recommended) (llama-3.3-70b-versatile) ← active
  2. OpenRouter (any model, free tier available) (meta-llama/...)
  3. Ollama (local, offline, no key needed) (llama3.2)

  a. Add a new provider
  q. Quit

❯ a

  Which provider do you want to add?
  1. Groq
  2. OpenAI
  3. Ollama
  4. Google Gemini
  5. Anthropic Claude
  6. OpenRouter

Choice ❯ 4

Enter your Google Gemini API key
(free at aistudio.google.com)

❯ AIza...

  ✅ Added and switched to google
  Luna is ready to use!
```

Your API keys are stored once in `~/.luna/config.toml` and remembered. Switch between providers as often as you want.

### How providers differ internally

Luna abstracts provider differences behind three transport functions in `ai.rs`:

- **OpenAI-format** (Groq, OpenAI, Ollama, OpenRouter) — same request/response shape, only the base URL differs
- **Google-format** (Gemini) — different request body, model name in URL path, header-based auth
- **Anthropic-format** (Claude) — `system` field at top level, `x-api-key` header, content-block responses

Adding a new provider is one match arm in `dispatch_ask` and one transport function. The display, safety, and ranking layers don't change.

---

## Personalized Ranking

This is Luna's "learning brain" made visible. When the AI returns multiple valid commands, Luna re-orders them by *your* history — not the AI's guess.

**Example:**

```bash
🌙 ~/luna〉/luna how do i run this project

🌙 thinking...
  ─────────────────────────────────
  Use cargo run
  $ cargo build && ./target/debug/luna ⭐ based on your history
  Risk: LOW ✅  executes project
  ─────────────────────────────────
  Execute? (y/n/more) ❯ more

  All options:
  ─────────────────────────────────
  1. $ cargo build && ./target/debug/luna        ← you use this one most
  2. $ cargo build --release && ...              ← less common for you
  3. $ cargo run                                  ← you rarely use this
  ─────────────────────────────────
  Choose (1-3) or n to skip ❯
```

The `⭐` marks the command Luna thinks you'll actually want, based on what you've run before. The other options are still there if you need them — just ranked by your history, not the AI's preference.

**How it works:** after parsing the AI's JSON response, Luna scores each suggested command by exact-match count in your recent history (last 50 commands). Highest score becomes the main suggestion. The display layer applies the `⭐` when the chosen command appears in your context.

The scoring is intentionally conservative — exact matches only, no fuzzy matching. False positives would teach Luna wrong patterns.

---

## Safety Layer

Luna's safety system is **deterministic** — written entirely in Rust, not delegated to AI judgment. The same logic runs on every command path: user-typed, AI-suggested, error-fix, and workflow steps.

### Risk levels

```
LOW       → Read-only operations        (ls, cat, find without -delete, git status)
MEDIUM    → System-modifying operations (apt install, git push, package managers)
HIGH      → Destructive operations      (rm -rf, kill -9, chmod -R, find -delete)
CRITICAL  → Potentially irreversible    (rm -rf /, mkfs, dd, fork bomb)
```

### Every command passes the same pipeline

```
Input
  ↓
Root deletion check       (rm -rf /, rm -rf ~/)
  ↓
Remote execution check    (curl | bash, wget | sh)
  ↓
Shell payload extraction  (recursively inspects bash -c "...", sudo sh -c, eval)
  ↓
Service modification check
  ↓
Pattern matching          (mkfs, dd, dd if=/dev/zero, fork bomb, etc.)
  ↓
Risk verdict
```

### CRITICAL example

```bash
🌙 ~/projects ❯ rm -rf /

  🚨 CRITICAL — Extremely dangerous command
  ─────────────────────────────────
  deletes entire root filesystem
  $ rm -rf /

  Type 'I UNDERSTAND' to proceed or Any key to cancel ❯
```

### Shell wrapper detection

Dangerous commands hidden inside shell wrappers are caught by recursively unwrapping:

```bash
🌙 ~/projects ❯ bash -c "rm -rf /"

  🚨 CRITICAL — Extremely dangerous command
  ─────────────────────────────────
  shell wrapper hides: deletes entire root filesystem

  Type 'I UNDERSTAND' to proceed or Any key to cancel ❯
```

The same logic catches `sudo sh -c "..."`, `eval "..."`, and any combination.

### Workflow safety

Saved workflows are never trusted automatically. Every command inside a workflow is re-inspected by the same Safety Engine at run time:

```
Create workflow
      ↓
 Safety Check
      ↓
     Save

 Run workflow
      ↓
 Safety Check (per step)
      ↓
   Execute
```

A saved workflow never receives a permanent safety pass.

---

## Pattern Learning & Workflows

Luna watches what you do and gets smarter. When you run the same sequence of commands repeatedly, it offers to save them as a one-command workflow.

```bash
🌙 Luna noticed a pattern
  ─────────────────────────────────
  You run these 3 commands together (5 times):
    → git add .
    → git commit -m
    → git push
  Save as a workflow? (y/n) ❯ y
  Name this workflow ❯ deploy
  ✅ Saved. Run with: luna run deploy
```

Or create one yourself at any time:

```bash
🌙 ~/projects ❯ luna create deploy

  Creating workflow 'deploy'
  ─────────────────────────────────
  Enter commands one by one. Empty line when done.

  Command 1 ❯ git add .
  Command 2 ❯ git commit -m
  Command 3 ❯ git push
  Command 4 ❯

  Note: 'git commit -m' will ask for input at runtime.
  ✅ Workflow 'deploy' saved with 3 commands.
```

When you run a workflow with placeholders, Luna asks for input at the right moment:

```bash
🌙 ~/projects ❯ luna run deploy

  Running 'deploy' (3 commands)
  → git add . → git commit -m → git push
  Some steps will ask for input when reached.
  Press Enter to run or 'n' to cancel ❯

  $ git add .

  Commit message ❯ fix login bug
  $ git commit -m "fix login bug"

  $ git push
```

### Pattern rejection

If you decline a workflow suggestion, Luna remembers that pattern and won't suggest it again. The `rejected_patterns` table in the database stores your "no" decisions permanently.

---

## Persistent Memory

Luna remembers everything. Every command you run, every error, every workflow, every AI suggestion you accepted or rejected. All in a local SQLite file at `~/.luna/memory.db`.

```bash
🌙 ~/projects ❯ history

  Recent commands
  ─────────────────────────────────
  /home/user/projects/api   │ cargo build
  /home/user/projects/api   │ git status
  /home/user/projects/api   │ cargo run
  /tmp                      │ mkdir test
  /home/user/projects/api   │ git push
  ─────────────────────────────────
```

### What gets stored

- **commands** — every command, with directory, success/fail, timestamp
- **errors** — every failed command + error text + (optional) AI's fix
- **workflows** — name, commands (JSON), use count
- **rejected_patterns** — patterns the user said "don't suggest this again"

The AI gets a context block built from your recent activity:

```
User's current directory: /home/user/projects/api
Project type in current directory: Rust (use cargo commands)
Last command run: cargo build
Recent commands (newest first): cargo build, git status, cargo run, ...
Recent errors: cargo test: error[E0425]: ...
```

This is what makes the AI's answers *specific to you* — it knows your project, your tools, your recent failures, and your patterns.

### Your patterns at a glance

```bash
🌙 ~/projects ❯ luna stats

  Your Terminal Patterns
  ─────────────────────────────────
  Commands run:     847
  Success rate:     84%

  Most used:
    67x  cargo build
    43x  git status
    31x  cargo run

  Error patterns:
  [████████░░] 87%  File not found
  Usually fixed by: check path with ls first

  [█░░░░░░░░░] 12%  Command typos
  Usually fixed by: check command spelling
  ─────────────────────────────────
```

---

## Commands Reference

### AI

```
/luna <question>       Ask Luna anything terminal-related
\luna <question>       Alias for /luna
```

### Built-in

```
history                 Show recent commands with directories
exit / quit             Leave Luna
```

### Luna commands

```
luna help               Show this help
luna config             Set up or change AI provider, key, and theme
luna model              Switch AI provider or add a new one
luna theme              List and switch themes (coming in v0.2.0)
luna stats              Show your patterns and error clusters
luna workflows          List all saved workflows
luna run <name>         Run a saved workflow
luna create <name>      Create a workflow interactively
luna delete <name>      Delete a saved workflow
```

### Workflow commands (luna workflow <action>)

```
luna workflow list      Same as 'luna workflows'
luna workflow create X  Same as 'luna create X'
luna workflow run X     Same as 'luna run X'
luna workflow delete X  Same as 'luna delete X'
```

---

## Configuration

Luna reads its configuration from `~/.luna/config.toml`. The file is created on first launch and updated whenever you change providers or settings.

```toml
[providers.groq]
provider = "groq"
model = "llama-3.3-70b-versatile"
api_key = "gsk_..."

[providers.ollama]
provider = "ollama"
model = "llama3.2"
api_key = ""

[ai]
active = "groq"

[safety]
level = "balanced"

[appearance]
colors = true

[behavior]
habit_threshold = 5
history_limit = 1000
```

Each provider you configure lives in its own `[providers.<name>]` block. The `[ai] active` field says which one is currently in use. Add a new provider with `luna model` → `a` (add), and it's saved automatically.


### Automatic migration

If you upgrade from a single-provider config (the format Luna used before multi-AI), the new version detects it on first load, converts it to the new multi-provider format, and writes it back. No user action required.

---

## Tab Completion

Luna ships with bash-like tab completion:

- `luna <Tab>` — shows all `luna` subcommands
- `luna run <Tab>` — shows your saved workflow names
- `<command> <Tab>` — completes file paths (with `~` expansion)
- `<partial> <Tab>` — completes commands from your `$PATH`

Press Tab once for suggestions, Tab again to cycle through them.

---

## Technical Stack

| Component | Technology | Why |
|---|---|---|
| Language | Rust 2021 edition | Memory safety, zero-cost abstractions, single binary |
| Input handling | reedline 0.36 | Production terminal line editor, Emacs bindings, menu support |
| AI providers | Groq, OpenAI, Ollama, Google, Anthropic, OpenRouter | Six first-class providers, switch instantly |
| Memory | SQLite via rusqlite (bundled) | Local, embedded, no server, no setup |
| HTTP client | reqwest 0.12 + tokio | Async, non-blocking, supports all 6 provider formats |
| Serialization | serde + serde_json + toml | Structured AI responses, clean config files |
| Configuration | `~/.luna/config.toml` | Plain text, version-controllable, easy to edit |

**Zero non-essential dependencies.** The whole binary is one Cargo project with seven crates. No telemetry libraries, no analytics, no auto-updaters.

---

## Build Phases

```
Phase 1 — Shell Core          ██████████  Complete
Phase 2 — AI Brain            ██████████  Complete
Phase 3 — Memory Engine       ██████████  Complete
Phase 4 — Error Intelligence  ██████████  Complete
Phase 5 — Safety Layer        ██████████  Complete
Phase 6 — Learning Brain      ██████████  Complete
Phase 7 — CLI Polish          ██████████  Complete  
Phase 8 — Distribution        ██████████  Complete
```

**Phase 7 shipped:** first-launch setup, multi-AI provider system, instant `luna model` switcher, `luna help`, tab completion, automatic config migration, workflow safety validation, no-restart configuration reload.

---

## Roadmap

### v0.2.0 — Polish & Visual

- **Themes** — dark (Catppuccin Mocha), moonlight, e-ink, light. Pure ANSI escape codes, no new dependencies. Live switching via `luna theme`.
- **README screenshots and demo GIF** — visual proof of the safety layer and personalized ranking.
- **Config polish** — `luna config` command to inspect and edit settings without touching the TOML directly.

### v0.3.0 — Smarter Learning

- **Learned autocorrect** — typos the AI fixes get saved to the database. Next time you mistype the same thing, instant fix from the local DB, no API call.
- **Full re-ranking of alternatives** — exact-match scoring is shipped; the next iteration will rank alternatives by token-overlap with your history.
- **Pattern confidence scores** — workflow suggestions show how confident Luna is, based on how many times you've run the sequence.

### v1.0.0 — Distribution

- **One-line install script** (`curl | bash`)
- **Install script**
- **GitHub Actions release workflow** — auto-build binaries for Linux on every tag
- **Clean-machine test suite** — automated install + smoke test on a fresh Ubuntu VM

---

## The One Rule

> *Luna never does anything you didn't see coming.*

Before anything executes, you see it. Before anything changes, you approve it. Destructive patterns are intercepted. AI suggestions are verified. Workflow steps are re-checked.

Safety isn't a feature. It's the architecture.

---

## Why This Exists

Every developer has used the same kind of terminal for thirty years. It executes what you type and forgets everything the moment you close it.

Luna is what a terminal would look like if it was designed today — with persistent memory, safety by default, pattern recognition, multi-provider AI, and intelligence that accumulates over time rather than resetting every session.

It's built by one person, in public, as a study in what a focused Rust project can become.

---

## Building in Public

Every decision, every bug, every breakthrough is documented on X as it happens. The CONTEXT.md file in the repo tracks the design, the known issues, and the architectural decisions for anyone who wants to follow along.

*→ [@Wasim Akhtar](https://x.com/akhwasim)*

---

## License

MIT — see [LICENSE](LICENSE).

You can read it, fork it, learn from it, ship products on top of it. Just don't blame me if your `rm -rf /` somehow gets past the safety engine.

---

## Contributing

Issues, bug reports, and PRs welcome. The codebase is intentionally small and well-organized so new contributors can get oriented in an afternoon. Start with the source top-down: `main.rs` → `shell.rs` → `commands.rs` / `ai.rs` / `safety.rs` / `memory.rs` / `learner.rs`.

If you add a new AI provider, the touch points are:
1. Add a variant to the `Provider` enum in `config.rs`
2. Add metadata (URL, model, env var, signup URL) to the `Provider` impl
3. Add it to the menu in `setup.rs::choose_provider`
4. If it uses a new wire format, add a `call_<name>_format` function in `ai.rs` and a match arm in `dispatch_ask`

If you add a new `luna` subcommand, the touch point is the `if/else` ladder in `shell.rs::run`. The new command also goes in `luna help`'s `print_help()` function.

---

<div align="center">

*Built with 🦀 Rust · Your data stays yours · No tracking · No accounts · No cloud*

**A terminal that remembers. A terminal that learns. A terminal that gets you.**

</div>
