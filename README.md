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

Close it. Reopen it. It doesn't know who you are. It doesn't know what you were working on. It doesn't remember the error you spent three hours debugging last Tuesday, or the fix that finally worked, or that you always use `pnpm` instead of `npm`.

It just blinks at you. Empty. Again.

That's not a missing feature. That's a fundamental design flaw. And nobody has fixed it.

**Luna is the fix.**

---

## What Luna Is

Luna is a standalone terminal shell — not a plugin, not a wrapper, not a chatbot glued to bash. A compiled Rust binary that replaces your terminal entirely.

The rest is coming. Watch the repo.

---

## What Luna Is Not

- ❌ Not an AI chatbot inside a terminal
- ❌ Not a cloud service that syncs your data somewhere
- ❌ Not an Electron app pretending to be native
- ❌ Not a plugin for your existing shell

---

## The Architecture Has One Rule

> *Luna never does anything you didn't see coming.*

Before anything executes, you see it. Before anything changes, you approve it. Destructive patterns are intercepted before they run. Nothing happens silently.

Safety isn't a feature. It's the foundation everything else is built on.

---

## Current Status

```
   Phase 1 — Shell Core     ██████████  Completed
   Phase 2 — AI Brain       ██████████  completed
   Phase 3 — Memory         ░░░░░░░░░░  Coming
   Phase 4 — Intelligence   ░░░░░░░░░░  Coming
   Phase 5 — Distribution   ░░░░░░░░░░  Coming
```

---

## Try It Now

```bash
git clone https://github.com/akhwasim/luna.git
cd luna
cargo build --release
./target/release/luna
```

> Requires Rust 1.70+ and Linux or WSL (Ubuntu recommended)

---

## Why Rust

No runtime. No garbage collector. No 200MB of dependencies just to print a prompt.

Luna starts in milliseconds and uses the kind of memory that makes other terminals embarrassing. That's not a goal — it's a consequence of the language.

---

## Building in Public

Every decision, every mistake, every breakthrough — documented on X as it happens.

This README will change. The code will change. Follow along if you want to see how something like this actually gets built.

*→ [@Wasim Akhtar](https://x.com/akhwasim)*

---

<div align="center">

*Built with 🦀 Rust · No cloud · No tracking · Just a terminal with a memory*

</div>