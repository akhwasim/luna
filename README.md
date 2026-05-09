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

## The Problem With Every Terminal You've Ever Used

Close it. Reopen it.

It doesn't know who you are. It doesn't know what you were working on. It doesn't remember the error you spent three hours fixing last Tuesday, or the fix that actually worked, or that you always run the same four commands every time you start a session.

It just blinks at you. Empty. Again.

That's not a missing feature. That's amnesia. And every terminal has it.

**Luna is the cure.**

---

## What Luna Actually Is

Luna is not an AI assistant bolted onto a terminal.
Luna is not autocomplete with a ChatGPT wrapper.
Luna is not another plugin for your existing shell.

Luna is a **standalone intelligent terminal** — a compiled Rust binary that replaces your shell entirely and gets smarter about how YOU work, every single day.

The difference is fundamental:

| Every other terminal | Luna |
|---|---|
| Forgets you the moment you close it | Remembers every command, every error, every session |
| Generic suggestions for everyone | Learns your specific patterns and habits |
| Reacts to what you type | Anticipates what you need |
| Stateless | Accumulates intelligence over time |

This is not a tool. This is the closest thing to JARVIS that lives in your terminal.

---

## How Luna Thinks

```
You type a command
       ↓
Luna executes it
       ↓
Luna remembers — command, directory, result, errors
       ↓
Luna notices patterns across sessions
       ↓
Luna gets smarter about you specifically
       ↓
Next session — Luna already knows where you left off
```

Over days and weeks of use, Luna becomes more valuable to you than to anyone else — because she's learned how YOU work, not how the average developer works.

---

## What Luna Can Do Right Now

### Understand Natural Language

```
🌙 ~/projects/api ❯ /luna find all files modified in the last 7 days larger than 1MB

  ─────────────────────────────────
  Find large recently modified files
  $ find . -type f -mtime -7 -size +1M -exec ls -lh {} \;
  Risk: MEDIUM ⚡  resource-heavy operation
  ─────────────────────────────────
  Execute? (y/n) ❯
```

### Know Your Context

Luna knows what project you're in, what you've been doing, and adapts automatically.

```
🌙 ~/projects/api ❯ /luna how do I run this project

  ─────────────────────────────────
  Run the project
  $ cargo run
  Risk: LOW ✅  builds and runs
  ─────────────────────────────────
  Execute? (y/n) ❯
```

Switch to a Python project — she suggests `python3 main.py`.
Switch to Node — she suggests `npm start`.
She reads the room.

### Catch Errors Before You Do

A command fails. Luna catches it, analyzes it, and suggests a fix — before you've even processed what went wrong.

```
🌙 ~/projects ❯ git psuh
git: 'psuh' is not a git command.

🌙 analyzing error...
  🌙 Luna detected an error
  ─────────────────────────────────
  Typo in git command
  $ git push
  Risk: LOW ✅  fixes typo
  ─────────────────────────────────
  Apply fix? (y/n) ❯
```

### Remember Everything

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

Not just commands — directories, timestamps, what succeeded, what failed.
All of it. Across every session. Forever.

### Never Run Blind

Every command gets a risk score. Every risky action requires your confirmation. Nothing happens silently.

```
  $ rm -rf /tmp/old_build
  Risk: HIGH ⚠️  deletes files permanently
  ─────────────────────────────────
  Execute? (y/n) ❯
```

### Know Her Lane

```
🌙 ~/projects ❯ /luna what is the capital of France

  ─────────────────────────────────
  I only help with terminal and system tasks.
  ─────────────────────────────────
```

Luna is a specialist. She has one job. She does it exceptionally well.

---

## What's Coming

This is where Luna stops being impressive and starts being indispensable.

### Learning Brain
- Detects that you always run `git add && git commit && git push` together → offers to automate it
- Notices you've made the same typo 3 times → suggests an alias or auto-corrects it permanently
- Remembers that last time you were in this project, you were fixing a specific error
- Clusters your errors over time — "your most common issue is permission errors, usually fixed with sudo"

### Cross-Session Intelligence
```
🌙 ~/projects/api ❯ cd .

Luna: Last time you were here you were debugging
      a connection timeout in main.rs.
      cargo build was failing. Want to continue?
```

### Your AI, Your Rules
Luna will support multiple AI providers — choose on first launch and change anytime:
- **Groq** (default) — free, fast, no GPU needed
- **Your own API** — OpenAI, Anthropic, any provider
- **Local model** — Ollama, fully offline, 100% private
- **Hybrid** — local by default, cloud for complex tasks

### Safety Layer
- Blocks genuinely dangerous commands before they run
- Detects production repositories and elevates risk automatically
- Dry run mode — see what would happen without doing it

---

## Current Build Status

```
Phase 1 — Shell Core          ██████████  Complete
Phase 2 — AI Brain            ██████████  Complete
Phase 3 — Memory Engine       ██████████  Complete
Phase 4 — Error Intelligence  ██████████  Complete
Phase 5 — Safety Layer        ░░░░░░░░░░  In Progress
Phase 6 — Learning Brain      ░░░░░░░░░░  Coming
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

## Why Rust

No runtime. No garbage collector. No 200MB of dependencies sitting idle.

Luna starts in milliseconds and uses the kind of memory that makes other terminals embarrassing. That's not a goal — it's a consequence of the language.

A terminal assistant written in Python would use 200MB of RAM just sitting there. Luna uses less than 10MB. Always.

---

## The One Rule

> *Luna never does anything you didn't see coming.*

Before anything executes, you see it. Before anything changes, you approve it. Destructive patterns are intercepted. Nothing runs silently.

Safety isn't a feature. It's the architecture.

---

## Why This Exists

Every developer has a terminal they've used for years. It knows nothing about them. It will know nothing about them in ten years.

That's the problem. Luna is the answer.

For the developers who live in the terminal and are tired of it being the dumbest tool in their stack.

---

## Building in Public

Every decision, every mistake, every breakthrough — documented on X as it happens.

*→ [@Wasim Akhtar](https://x.com/akhwasim)*

---

<div align="center">

*Built with 🦀 Rust · Your data stays yours · No tracking*

*A terminal that remembers. A terminal that learns. A terminal that gets you.*

</div>