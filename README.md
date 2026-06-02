<div align="center">

# 🌙 Luna

**The terminal that gets smarter every time you use it.**
> *Your terminal has amnesia. I'm fixing it.*

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Built with Rust](https://img.shields.io/badge/Built%20with-Rust-orange.svg)](https://www.rust-lang.org)
[![Status: Building in Public](https://img.shields.io/badge/Status-Building%20in%20Public-brightgreen.svg)]()

*Building in public → [@Wasim Akhtar](https://x.com/akhwasim)*

</div>

---

## The Problem

Every terminal you've ever used has the same flaw.

Close it. Reopen it. It doesn't know who you are. It doesn't know what you were working on. It doesn't remember the error you spent three hours fixing last Tuesday, the fix that finally worked, or that you always run the same four commands at the start of every session.

It just blinks at you. Empty. Again.

That's not a missing feature. That's amnesia. And every terminal has it.

**Luna is the fix.**

---

## What Luna Is

Luna is a **standalone intelligent terminal** — a compiled Rust binary that replaces your shell entirely.

Not a plugin. Not a wrapper. Not a chatbot glued to bash.

It understands natural language, executes commands safely, remembers everything across sessions, and gets smarter about how you specifically work — every single day.

| Every other terminal | Luna |
|---|---|
| Forgets you the moment you close it | Remembers every command, every error, every session |
| Same suggestions for everyone | Learns your specific patterns and habits |
| Reacts to what you type | Anticipates what you need |
| Stateless | Accumulates intelligence over time |

---

## How Luna Works

```
You type a command
       ↓
Safety Engine — deterministic risk analysis before anything runs
       ↓
Execute — Luna runs the command natively
       ↓
Error Analysis — failure triggers automatic AI-powered fix suggestion
       ↓
Fix verified — suggestion passes through same Safety Engine
       ↓
Memory — command, result, directory, errors written to SQLite
       ↓
Learning — patterns detected across sessions over time
```

---

## What Luna Can Do Today

### Natural Language → Exact Command

```
🌙 ~/projects/api ❯ /luna find all files modified in the last 7 days larger than 1MB

  ─────────────────────────────────
  Find large recently modified files
  $ find . -type f -mtime -7 -size +1M -exec ls -lh {} \;
  Risk: MEDIUM ⚡  resource-heavy operation
  ─────────────────────────────────
  Execute? (y/n) ❯
```

### Context-Aware Suggestions

Luna detects your project type and adapts automatically.

```
🌙 ~/projects/api ❯ /luna how do I run this project
  ─────────────────────────────────
  $ cargo run
  Risk: LOW ✅
  ─────────────────────────────────
  Execute? (y/n) ❯
```

Switch to a Python project — she suggests `python3 main.py`.
Switch to Node — she suggests `npm start`.
No configuration. Luna reads the room.

### Automatic Error Recovery

```
🌙 ~/projects ❯ git psuh
git: 'psuh' is not a git command.

🌙 analyzing error...
  ─────────────────────────────────
  Typo in git command
  $ git push
  Risk: LOW ✅  fixes typo
  ─────────────────────────────────
  Apply fix? (y/n) ❯
```

### Persistent Memory

```
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

Every command. Every directory. Every result. Across every session. Forever.

---

## Safety Layer

Luna's safety system is **deterministic** — written entirely in Rust, not delegated to AI judgment.

### Risk Classification

```
LOW      → Read-only operations        (ls, cat, find, git status)
MEDIUM   → System-modifying operations (apt install, git push, mv)
HIGH     → Destructive operations      (rm -rf, kill -9, chmod -R)
CRITICAL → Potentially irreversible    (rm -rf /, mkfs, dd, fork bomb)
```

### The Inspection Pipeline

Every command — typed by you, suggested by AI, or generated as an error fix — passes through the same pipeline:

```
Input
  ↓
Root deletion check
  ↓
Remote execution check  (curl | bash, wget | sh)
  ↓
Shell payload extraction  (recursively inspects bash -c "...")
  ↓
Service modification check
  ↓
Pattern matching
  ↓
Risk verdict → CRITICAL / HIGH / MEDIUM / LOW
```

### CRITICAL — Requires Explicit Acknowledgement

```
🌙 ~/projects ❯ rm -rf /

  🚨 CRITICAL — Extremely dangerous command
  ─────────────────────────────────
  deletes entire root filesystem
  $ rm -rf /

  Type 'I UNDERSTAND' to proceed or Any key to cancel ❯
```

### Shell Wrapper Detection

Luna extracts and inspects the payload inside shell wrappers recursively:

```
🌙 ~/projects ❯ bash -c "rm -rf /"

  🚨 CRITICAL — Extremely dangerous command
  ─────────────────────────────────
  shell wrapper hides: deletes entire root filesystem

  Type 'I UNDERSTAND' to proceed or Any key to cancel ❯
```

### Verified Recovery

When Luna suggests an error fix, that suggestion passes through the same safety engine before being shown to you. AI output is never trusted automatically.

### Coverage

- Filesystem deletion (root, home, current directory)
- Shell wrapper bypasses (`bash -c`, `sudo sh -c`, `zsh -c`)
- Remote code execution (`curl | bash`, `wget | sh`, `bash <(curl ...)`)
- Command separator attacks (`ls ; rm -rf *`, `ls || rm -rf *`)
- Subshell attacks (`(rm -rf /)`)
- Double privilege escalation (`sudo sudo rm -rf /`)
- Disk operations (`mkfs`, `dd if=/dev/zero`, `fdisk`, `wipefs`)
- Fork bombs (`:(){ :|:& };:`)
- Service manipulation (`systemctl stop`, `kill -9 -1`, `killall -9`)
- System file overwrites (`> /etc/passwd`, `> /etc/shadow`)

---

## What's Coming

### Learning Brain

This is where Luna becomes genuinely different.

**Habit Detection**
Luna watches your command sequences over time. When she sees the same pattern repeat — `git add` → `git commit` → `git push`, every single day — she offers to save it as a named workflow. One command instead of five.

**Personalized Ranking**
When the AI generates suggestions, Luna reorders them based on your actual history. If you've run `cargo run` 47 times and `cargo build` 12 times, Luna ranks `cargo run` first. Not because it's generally better — because it's what you do.

**Error Clustering**
Luna groups your errors over time into categories: dependency issues, permission errors, syntax mistakes, typos. Then tells you which category dominates your failures and what usually fixes it.

The Learning Brain is designed to stay local,
lightweight, and privacy-preserving.
### Your AI, Your Rules

Luna will support multiple AI providers on first launch:
- **Groq** (default) — free tier, fast, no GPU needed
- **Your own API** — OpenAI, Anthropic, any OpenAI-compatible provider
- **Local model** — Ollama, fully offline, 100% private
- **Hybrid** — local for everyday tasks, cloud for complex ones

---

## Technical Stack

| Component | Technology | Why |
|---|---|---|
| Language | Rust (edition 2021) | Memory safety, zero runtime overhead |
| Input handling | reedline | Production terminal line editor |
| AI provider | Groq API | Free tier, fast inference |
| Memory | SQLite via rusqlite | Local, embedded, no server |
| HTTP client | reqwest + tokio | Async, non-blocking AI calls |
| Serialization | serde + serde_json | Structured AI responses |
| ML (planned) | linfa + candle | Pure Rust, no Python dependency |

---

## Build Status

```
Phase 1 — Shell Core          ██████████  Complete
Phase 2 — AI Brain            ██████████  Complete
Phase 3 — Memory Engine       ██████████  Complete
Phase 4 — Error Intelligence  ██████████  Complete
Phase 5 — Safety Layer        ██████████  Complete
Phase 6 — Learning Brain      ░░░░░░░░░░  In Progress
Phase 7 — UI & Themes         ░░░░░░░░░░  Coming
Phase 8 — Distribution        ░░░░░░░░░░  Coming
```

---

## Try It Now

```bash
git clone https://github.com/akhwasim/luna.git
cd luna
cargo build --release
./target/release/luna
```

Add your Groq API key (free at [console.groq.com](https://console.groq.com)):

```bash
mkdir -p ~/.luna
echo "GROQ_API_KEY=your_key_here" > ~/.luna/.env
```

**Requirements:** Rust 1.70+ · Linux or WSL (Ubuntu recommended)

---

## The One Rule

> *Luna never does anything you didn't see coming.*

Before anything executes, you see it. Before anything changes, you approve it. Destructive patterns are intercepted before they run. AI suggestions are verified before they're shown.

Safety isn't a feature. It's the architecture.

---

## Why This Exists

Every developer has used the same kind of terminal for thirty years. It executes what you type and forgets everything the moment you close it.

Luna is what a terminal would look like if it was designed today — with persistent memory, safety by default, and intelligence that accumulates over time rather than resetting every session.

---

## Building in Public

Every decision, every mistake, every breakthrough — documented on X as it happens.

*→ [@Wasim Akhtar](https://x.com/akhwasim)*

---

<div align="center">

*Built with 🦀 Rust · Your data stays yours · No tracking*

*A terminal that remembers. A terminal that learns. A terminal that gets you.*

</div>