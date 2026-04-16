<div align="center">

# 🌙 Luna

**Your terminal has amnesia. I'm fixing it.**

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Built with Rust](https://img.shields.io/badge/Built%20with-Rust-orange.svg)](https://www.rust-lang.org)
[![Status: Building in Public](https://img.shields.io/badge/Status-Building%20in%20Public-brightgreen.svg)]()

*Building in public. Follow the journey → [@Wasim Akhtar](https://x.com/akhwasim)*

</div>

---

## The Problem

Every terminal you've ever used has the same flaw.

Close it. Reopen it. It doesn't know who you are. It doesn't know what
you were working on. It doesn't remember the error you spent three hours
debugging last Tuesday, or the fix that finally worked, or that you
always use `pnpm` instead of `npm`.

It just blinks at you. Empty. Again.

That's not a missing feature. That's a fundamental design flaw.

**Luna is the fix.**

---

## What Luna Is

Luna is a standalone terminal shell --- not a plugin, not a wrapper, not
a chatbot glued to bash.

A compiled Rust binary that:

-   Understands natural language
-   Translates it into precise terminal commands
-   Ensures safe, controlled execution
-   (Soon) remembers everything you do

---

## What Luna Can Do (Right Now)

Luna already works as a **controlled, safety-first execution layer for
the terminal**

-   Natural language → terminal command
-   Explains what will run before execution
-   Assigns a **risk score** to every command
-   Requires explicit confirmation (`y/n`)
-   Rejects non-terminal queries (strict scope control)

### Example

``` bash
\luna "find files larger than 10mb"

🌙 Luna
─────────────────────────────────
Find files larger than 10MB in the current directory:

$ find . -type f -size +10M

Risk: LOW
─────────────────────────────────
Execute? (y/n)
```

#### Out-of-scope example

``` bash
\luna "what is the capital of france"

Out of scope: Luna only handles terminal-related tasks.
```

---

## What Luna Is Not

-   ❌ Not an AI chatbot inside a terminal
-   ❌ Not a cloud service that syncs your data somewhere
-   ❌ Not an Electron-based terminal
-   ❌ Not a plugin for your existing shell

---

## Current Status

    Phase 1 — Shell Core     ██████████  Completed
    Phase 2 — AI Brain       ██████████  Completed
    Phase 3 — Memory         ░░░░░░░░░░  In Progress
    Phase 4 — Intelligence   ░░░░░░░░░░  Coming
    Phase 5 — Distribution   ░░░░░░░░░░  Coming

---

## Try It Now

``` bash
git clone https://github.com/akhwasim/luna.git
cd luna
cargo build --release
./target/release/luna
```

> Requires Rust 1.70+ 
> Linux or WSL (Ubuntu recommended)

---

## Why Rust

No runtime. No garbage collector. No heavy dependencies.

Luna starts in milliseconds and stays lightweight --- exactly what a
terminal should do.

---

## Roadmap

-   Persistent memory across sessions
-   Learning from command history + errors
-   Context-aware suggestions
-   Smarter risk analysis

---

## The Architecture Has One Rule

> *Luna never does anything you didn't see coming.*

Before anything executes, you see it. Before anything changes, you
approve it. Destructive patterns are intercepted before they run.
Nothing happens silently.

Safety isn't a feature. It's the foundation everything else is built on.

---

## Building in Public

Every decision, every mistake, every breakthrough --- documented on X as
it happens.

This README will change. The code will change. Follow along if you want
to see how something like this actually gets built.

*→ [@Wasim Akhtar](https://x.com/akhwasim)*

---

<div align="center">

*Built with 🦀 Rust · No cloud · No tracking · Just a terminal with a memory*

</div>
