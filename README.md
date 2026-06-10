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

Every terminal you have ever used has the same flaw.

Close it. Reopen it. It doesn't know who you are. It doesn't know what you were working on. It doesn't remember the error you spent three hours fixing last Tuesday, the fix that finally worked, or that you always run the same four commands at the start of every session.

It just blinks at you. Empty. Again.

That is not a missing feature. That is amnesia. And every terminal has it.

**Luna is the fix.**

---

## What Luna Is

Luna is a standalone intelligent terminal.

Not a plugin. Not a wrapper. Not a chatbot glued to bash.

It understands natural language, executes commands safely, remembers everything across sessions, detects your patterns, and gets smarter about how you specifically work, every single day.

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

---

## What Luna Can Do Today

### Natural Language to Exact Command

```
🌙 ~/projects ❯ /luna find all files modified in the last 7 days larger than 1MB

  ─────────────────────────────────
  Find large recently modified files
  $ find . -type f -mtime -7 -size +1M -exec ls -lh {} \;
  Risk: MEDIUM ⚡  resource-heavy operation
  ─────────────────────────────────
  Execute? (y/n/more) ❯
```

### Multiple Options on Demand

Type `more` to see alternatives. Luna ranks them based on your actual history.

```
  Execute? (y/n/more) ❯ more

  All options:
  ─────────────────────────────────
  1. $ cargo run  ⭐ based on your history
  2. $ cargo build && cargo run
  3. $ cargo test
  ─────────────────────────────────
  Choose (1-3) or n to skip ❯
```

### Context-Aware Suggestions

Luna detects what kind of project you are in and adapts automatically.

```
🌙 ~/projects/api ❯ /luna how do I run this project

  ─────────────────────────────────
  $ cargo run  ⭐ based on your history
  Risk: LOW ✅  builds and runs
  ─────────────────────────────────
  Execute? (y/n/more) ❯
```

Switch to a Python project and she suggests `python3 main.py`. Switch to Node and she suggests `npm start`. No configuration needed.

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
  Apply fix? (y/n/more) ❯
```

### Built-in Typo Correction

Common typos are caught instantly without any API call.

```
🌙 ~/projects ❯ gti status
luna: did you mean 'git'?
On branch main...

🌙 ~/projects ❯ cler
luna: did you mean 'clear'?
```

Arguments are preserved too. `gti status` becomes `git status`, not just `git`.

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

### Workflow Automation

Luna watches your command sequences across sessions. When she sees the same pattern repeat five times, she offers to automate it.

```
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

```
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

When you run it, Luna asks for missing input at the right moment:

```
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

### Your Patterns at a Glance

```
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

## Safety Layer

Luna's safety system is deterministic — written entirely in Rust, not delegated to AI judgment.

### Risk Classification

```
LOW      → Read-only operations        (ls, cat, find, git status)
MEDIUM   → System-modifying operations (apt install, git push, mv)
HIGH     → Destructive operations      (rm -rf, kill -9, chmod -R)
CRITICAL → Potentially irreversible    (rm -rf /, mkfs, dd, fork bomb)
```

### Every Command Gets Inspected

Every command, whether typed by you, suggested by AI, or generated as an error fix, passes through the same pipeline before anything executes.

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
Risk verdict
```

### CRITICAL Example

```
🌙 ~/projects ❯ rm -rf /

  🚨 CRITICAL — Extremely dangerous command
  ─────────────────────────────────
  deletes entire root filesystem
  $ rm -rf /

  Type 'I UNDERSTAND' to proceed or Any key to cancel ❯
```

### Shell Wrapper Detection

Dangerous commands hidden inside shell wrappers are still caught:

```
🌙 ~/projects ❯ bash -c "rm -rf /"

  🚨 CRITICAL — Extremely dangerous command
  ─────────────────────────────────
  shell wrapper hides: deletes entire root filesystem

  Type 'I UNDERSTAND' to proceed or Any key to cancel ❯
```

### Verified Recovery

When Luna suggests an error fix, that suggestion passes through the same safety engine before being shown. AI output is never trusted automatically.

### Workflow Safety

Saved workflows are never trusted automatically.

Every command inside a workflow is inspected by the same Safety Engine used everywhere else.

```
Create workflow
      ↓
 Safety Check
      ↓
     Save

 Run workflow
      ↓
 Safety Check
      ↓
   Execute
```

Example:

```
🌙 ~/projects ❯ luna create cleanup

  → ls
  → rm -rf /
  → git status

  🚨 CRITICAL command in workflow: 'rm -rf /'
  Reason: deletes your entire home or root filesystem
```

Even if a workflow is approved and saved, Luna checks every step again when the workflow is executed.

A saved workflow never receives a permanent safety pass.

---

### What Gets Caught

- Filesystem deletion targeting root, home, current directory, or wildcards
- Shell wrapper bypasses using bash -c, sudo sh -c, zsh -c
- Remote code execution via curl, wget piped to shell
- Command separator attacks like `ls ; rm -rf *`
- Subshell attacks like `(rm -rf /)`
- Double privilege escalation
- Disk operations including mkfs, dd, fdisk, wipefs, shred on devices
- Fork bombs
- Service manipulation via systemctl, kill -9 -1, killall
- System file overwrites targeting /etc/passwd, /etc/shadow
- Firewall removal via iptables -F and ufw disable

---

## Technical Stack

| Component | Technology | Why |
|---|---|---|
| Language | Rust edition 2021 | Memory safety, zero runtime overhead |
| Input handling | reedline | Production terminal line editor |
| AI provider | Groq API | Free tier, fast inference |
| Memory | SQLite via rusqlite | Local, embedded, no server needed |
| HTTP client | reqwest + tokio | Async, non-blocking AI calls |
| Serialization | serde + serde_json | Structured AI responses |

---

## Build Status

```
Phase 1 — Shell Core          ██████████  Complete
Phase 2 — AI Brain            ██████████  Complete
Phase 3 — Memory Engine       ██████████  Complete
Phase 4 — Error Intelligence  ██████████  Complete
Phase 5 — Safety Layer        ██████████  Complete
Phase 6 — Learning Brain      ██████████  Complete
Phase 7 — CLI Polish          ░░░░░░░░░░  In Progress
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

Add your Groq API key, free at [console.groq.com](https://console.groq.com):

```bash
mkdir -p ~/.luna
echo "GROQ_API_KEY=your_key_here" > ~/.luna/.env
```

**Requirements:** Rust 1.70+ and Linux or WSL (Ubuntu recommended)

---

## Commands

```
/luna <question>       Ask Luna anything terminal-related
history                Show recent commands with directories
luna stats             Show your patterns and error clusters
luna workflows         List all saved workflows
luna run <name>        Run a saved workflow
luna create <name>     Create a workflow interactively
luna delete <name>     Delete a workflow
```

---

## What's Coming

**Multiple AI providers** — Groq is the default but Luna will support OpenAI, Anthropic, and Ollama for fully local offline use. Choose on first launch, switch anytime.

**Themes** — dark, moonlight, e-ink, and light themes using pure terminal colors. No extra dependencies.

**Config file** — `~/.luna/config.toml` for AI provider, safety level, theme, and behavior settings.

**Learning metrics** — measure whether Luna is actually becoming more useful over time through suggestion acceptance, workflow acceptance, common error categories, and successful fixes.

---

## The One Rule

> *Luna never does anything you didn't see coming.*

Before anything executes, you see it. Before anything changes, you approve it. Destructive patterns are intercepted. AI suggestions are verified. Nothing runs silently.

Safety isn't a feature. It's the architecture.

---

## Why This Exists

Every developer has used the same kind of terminal for thirty years. It executes what you type and forgets everything the moment you close it.

Luna is what a terminal would look like if it was designed today — with persistent memory, safety by default, pattern recognition, and intelligence that accumulates over time rather than resetting every session.

---

## Building in Public

Every decision, every mistake, every breakthrough is documented on X as it happens.

*→ [@Wasim Akhtar](https://x.com/akhwasim)*

---

<div align="center">

*Built with 🦀 Rust · Your data stays yours · No tracking*

*A terminal that remembers. A terminal that learns. A terminal that gets you.*

</div>