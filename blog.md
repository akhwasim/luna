---
title: "Your Terminal Has Amnesia. I Spent My Semester Trying to Fix That."
published: true
description: "How a simple frustration with stateless terminals turned into Luna — a memory-aware AI terminal built in Rust."
tags: rust, ai, terminal, opensource,
---

## Your Terminal Has Amnesia. I Fixed it.

Every terminal I've ever used has the same problem.

You close it.

You open it again.

It has no idea what happened yesterday.

It doesn't remember the command that finally fixed that weird Cargo error after two hours of debugging. It doesn't remember that you always use `pnpm` instead of `npm`. It doesn't remember the project you spent all week working on.

Every session starts from zero.

After a while I realized something strange.

My terminal forgot everything.

I was the memory.

So I started building Luna.

---

## It didn't start as an AI project.

It started as a frustration.

I'm a Computer Science student, and like most developers, I spend a huge part of my day inside a terminal.

Not because terminals are exciting.

Because that's where software gets built.

After a few months I noticed I wasn't struggling with commands.

I was struggling with remembering everything around them.

```text
Google the same error.

Copy the solution.

Paste it.

Forget it.

Repeat three weeks later.
```

Or I'd switch to another project for a few days, come back, and think:

*"How did I solve this last time?"*

The terminal had no answer.

It never does.

History only tells you **what** you typed.

It never tells you **why**.

---

## So I asked a simple question.

What if the terminal remembered?

Not just command history.

Actually remembered.

Projects.

Errors.

Directories.

Commands that worked.

Commands that failed.

Patterns in how I work.

That question eventually became Luna.

---

## Before writing any AI code, I had to answer another question.

What exactly **is** Luna?

A shell?

A wrapper?

A plugin?

A chatbot?

I realized pretty quickly I didn't want another tool sitting on top of Bash.

I wanted something that actually felt like its own terminal.

So Luna became its own shell.

Not a plugin.

Not an extension.

A standalone binary written in Rust.

---

## How Luna actually works

At a very high level, Luna is surprisingly simple.

```text
                        You
                         │
                         ▼
              Natural Language Input
                         │
                         ▼
                    AI Provider
      (Groq • Gemini • Claude • OpenAI • Ollama...)
                         │
                         ▼
                   Safety Engine
                         │
              LOW ───────┴──────── HIGH
               │                     │
               ▼                     ▼
            Execute            Ask Permission
               │
               ▼
          SQLite Memory
               │
               ▼
   Future Context & Suggestions
```

Every feature in Luna comes from those four building blocks.

The AI generates commands.

The safety engine decides whether those commands are safe enough to execute.

The shell runs them.

SQLite remembers everything that happened so future suggestions become more useful instead of starting from zero again.

The language model is only one piece of the system.

It never gets the final decision.

---

## Building Luna happened in three phases.

Looking back, they seem obvious.

While building them, they definitely weren't.

### Phase 1 - Build a real shell.

Before AI could do anything useful, Luna had to behave like an actual terminal.

That meant implementing built-in commands, history, directory navigation, command execution, a REPL loop, and all the boring things people expect without ever thinking about them.

Ironically, none of this involved AI.

It was just engineering.

---

### Phase 2 - Natural language.

Only after the shell worked did I integrate AI.

The first demo felt magical.

```text
find files larger than 10MB
```

became

```bash
find . -type f -size +10M
```

without me writing the command.

For about five minutes I thought I was done.

Then I asked myself a much more uncomfortable question.

> Should I actually trust this?

That single question completely changed the direction of the project.

Generating shell commands turned out to be the easy part.

Trusting them wasn't.

---

### Phase 3 - Security

Security is my favorite part of the project.

Every time a command enter **Luna** shell it goes through security check and determine either command is safe to run or not. 

No matter where that command came from, either from **User**, **AI Generated** or from saved **workflows**.

Workflow command are checked both at creation and while running.

---

### Phase 4 - Give it memory.

Instead of treating every terminal session as a brand-new conversation, Luna stores context locally.

Directories.

Projects.

Commands.

Previous errors.

Successful fixes.

Frequently used workflows.

After using it for a while, something changed.

Instead of wondering

> "What was that command again?"

Luna usually already knew.

That was the first moment it actually felt different from every shell I'd used before.

Not smarter.

Just less forgetful.

---

## Why Rust?

People ask this surprisingly often.

The short answer is that Python would've been much faster to build.

The long answer is that I didn't want users thinking about Python versions, virtual environments, or dependency conflicts before they could even open the terminal.

I wanted this experience:

```bash
cargo build --release

./luna
```

That's it.

One binary.

No runtime.

No setup guide longer than the project itself.

Rust also forced me to think carefully about architecture instead of continuously patching things together.

Looking back, that probably made Luna a better project than if I'd optimized only for development speed.

---

At this point, I thought the difficult part was over.

I had a shell.

I had AI.

I had memory.

Then the real engineering started.

## The first version was... honestly pretty dumb.

It could take plain English, send it to an LLM, and print back a shell command.

That sounds impressive until you actually use it.

The first problem wasn't generating commands.

The first problem was trusting them.

An AI can confidently suggest

```bash
rm -rf something
```

just as easily as

```bash
ls
```

One is harmless.

One can ruin your afternoon.

I knew very early that if Luna was going to exist, there had to be something sitting between

```text
AI

↓

Your filesystem
```

That became the safety engine.

Not an AI.

Not another model.

Just deterministic Rust code.

Every command, regardless of where it came from, passes through exactly the same classifier before it ever reaches the shell.

High-risk commands stop.

The user sees exactly what's about to happen.

Nothing executes silently.

That became the rule I never compromised on.

I trust AI enough to make suggestions.

I don't trust it enough to execute commands without asking.

---

## Then Rust reminded me who's in charge.

People like to joke that Rust makes you fight the compiler.

They're right.

But most of my problems weren't borrow checker problems.

They were architecture problems.

One evening every AI request suddenly started failing with

```text
Cannot start a runtime from within a runtime
```

I was convinced something was wrong with Groq.

Spent hours checking API keys.

Reading documentation.

Testing requests outside Luna.

Nothing.

Eventually I realized the problem wasn't Groq at all.

It was me.

I had accidentally created a Tokio runtime inside another Tokio runtime.

The fix ended up being surprisingly small.

Spawn a dedicated thread with its own runtime for AI requests.

Problem solved.

It was one of those bugs where the solution takes ten minutes.

Understanding the problem takes four hours.

---

## Prompt engineering wasn't nearly as glamorous as people make it sound.

One day Luna suggested

```bash
history
```

to inspect previous commands.

Except...

`history` wasn't even a shell command.

It was Luna's own built-in.

The model had no way of knowing that.

The bug wasn't in the AI.

The bug was in the instructions I gave it.

So I went back and rewrote the system prompt.

Again.

That became a recurring pattern throughout the project.

Whenever the AI did something strange, my first instinct slowly changed from

> "The model is wrong."

to

> "What assumptions did I accidentally teach it?"

---

## Then came my favorite category of bugs.

The ones where the software technically worked.

Until someone got creative.

For example,

```bash
bash -c "rm -rf /"
```

was caught by the safety engine.

Good.

Then I stopped for a second and thought

*"Wait..."*

What about

```bash
eval "rm -rf /"
```

It wasn't caught.

That wasn't a hypothetical vulnerability.

It was an actual hole in my classifier.

So I added `eval` to the recursive shell unwrapper.

Then another question appeared.

What about

```bash
sudo bash -c "..."
```

Already handled.

Good.

Then

```bash
"$($cmd)"
```

Command substitution.

Also handled.

Only because I'd already implemented recursive payload extraction.

Building the safety engine slowly became an exercise in asking myself one question over and over.

"What would someone do to get around this?"

Every answer became another test.

Another edge case.

Another rule.

Not because users are malicious.

Because software eventually reaches people who are more creative than its author.

---

## The most boring file in the repository is also my favorite.

It's around 250 lines of Rust.

No machine learning.

No APIs.

No fancy algorithms.

Just pattern matching.

But that file probably taught me more than the rest of the project combined.

At one point I noticed I was warning users every time they used

```bash
mv
```

or

```bash
chmod
```

Technically they *could* be dangerous.

Practically...

Nobody wants to confirm

```bash
mv old.txt new.txt
```

every five minutes.

Warnings people ignore are worse than warnings that don't exist.

So I removed them.

The safety engine became quieter.

And because of that,

it became more useful.

---

## I made exactly the same mistake somewhere else.

I wanted Luna's personalized command ranking to feel intelligent.

So I experimented with fuzzy matching.

Token similarity.

Partial matches.

Prefix scoring.

The results looked clever.

Until I actually used them.

Luna started recommending commands that were *kind of similar* to things I'd run before.

Not the exact command.

Just close enough.

It felt wrong immediately.

So I deleted all of it.

Now the ranking is intentionally simple.

If you've successfully run

```bash
cargo run
```

twenty times,

Luna assumes you probably like

```bash
cargo run
```

Not

```bash
cargo test
```

Not

```bash
cargo build
```

Just the thing you've repeatedly chosen yourself.

Sometimes software becomes better because it gets smarter.

Sometimes it becomes better because it stops pretending to be.

---

## SQLite was probably the easiest decision in the whole project.

People sometimes ask why I didn't use PostgreSQL.

Or Redis.

Or some embedded vector database.

Because Luna isn't a distributed system.

It's your terminal.

I didn't need clusters.

I didn't need replication.

I needed one file.

One database that starts instantly, survives restarts, and doesn't ask users to install anything else.

SQLite has been solving exactly that problem for over twenty years.

Sometimes boring technology is exactly the right technology.

---

## Somewhere around this point, something changed.

When I started Luna, I thought integrating AI would be the difficult part.

It wasn't.

Calling an LLM API took an afternoon.

Everything around the model took months.

Deciding when not to trust it.

Remembering useful context without remembering everything.

Making sure users stayed in control.

Building software around AI turned out to be far more interesting than calling AI itself.

That wasn't what I expected to learn when I started this project.

But it's probably the lesson that's going to stay with me the longest.

## Shipping was harder than building.

At some point, Luna reached a stage where it actually worked.

The shell worked.

The AI worked.

The memory worked.

The safety engine worked.

I thought I was almost finished.

I wasn't.

Because working software on your own machine isn't a product.

It's just software.

Making it usable by someone else turned out to be an entirely different project.

I had to think about installation.

Releases.

GitHub Actions.

Configuration files.

Auto-updates.

Backward compatibility.

Migration when configuration formats changed.

The installer alone took much longer than I expected.

The core logic is maybe fifty lines of Bash.

Everything around it—missing `PATH` entries, unsupported platforms, build-from-source fallback, release detection, user mistakes—added hundreds more.

None of it was exciting.

All of it mattered.

Building features gets attention.

Building a good experience doesn't.

---

## Luna today

Today Luna supports six AI providers.

- Groq
- OpenAI
- Anthropic Claude
- Google Gemini
- OpenRouter
- Ollama

That wasn't originally the plan.

Early on I only wanted one provider.

Then I realized something.

The AI ecosystem changes too quickly.

Models improve.

Pricing changes.

Rate limits appear.

Companies disappear.

I didn't want Luna to depend on one API.

I wanted users to decide.

If someone prefers cloud models, great.

If someone wants everything to stay local through Ollama, that's great too.

Luna doesn't care.

Switch providers.

Keep working.

That flexibility became part of the design rather than an extra feature.

---

## Looking back, Luna isn't really about AI.

This surprised me.

If someone asked what Luna is, the easy answer would be

> "An AI-powered terminal."

Technically that's true.

But I don't think that's what I built.

I think I built software that tries to answer a different question.

**What happens when AI becomes part of everyday tools?**

My conclusion is that the model itself isn't the difficult part.

The difficult part is everything surrounding it.

When should it remember?

When should it forget?

When should it ask for confirmation?

When should it stay completely out of the way?

Those questions don't have API endpoints.

They're product decisions.

They're engineering decisions.

And they're trust decisions.

Building Luna changed how I think about AI.

I used to think the interesting challenge was making models smarter.

Now I think the more interesting challenge is making them predictable.

---

## What I learned

If I had to summarize six months of building Luna into a few lessons, they'd be these.

### 1. Safety is a feature.

It's easy to get excited about what AI *can* do.

It's harder—and more important—to decide what it *shouldn't* do.

The safety engine isn't the flashiest part of Luna.

It's the part I'd miss most if it disappeared.

---

### 2. Memory changes everything.

The terminal already remembers commands.

That isn't enough.

Remembering context is different.

Context turns repeated work into assisted work.

The goal was never to replace the user.

The goal was to stop making the user repeat themselves.

---

### 3. Boring technology wins.

Rust.

SQLite.

Git.

GitHub Actions.

None of these are particularly exciting.

They are reliable.

Over and over again, I found myself replacing clever ideas with simpler ones.

Almost every time, the software became better.

---

### 4. Shipping teaches different lessons than building.

There were features I wanted in v0.1.0.

Themes.

Better workflow suggestions.

Smarter personalization.

Cross-platform support.

None of them shipped.

Not because they weren't useful.

Because software only becomes useful after people can actually use it.

Version 1 is never perfect.

Waiting for perfect means never shipping.

---

## What's next

Luna is still at the beginning.

There are dozens of ideas sitting in my notes.

Some are small.

Some probably require rewriting parts of the architecture.

The biggest things I want to improve are:

- Better workflow detection.
- Smarter error recovery.
- Cross-platform support beyond Linux and WSL.
- A memory system that understands projects instead of only commands.
- Better explanations for why Luna recommends something, not just what it recommends.

The project will probably change a lot over the next year.

I hope the core philosophy doesn't.

Keep users in control.

Remember useful things.

Never execute something surprising.

---

## Final thoughts

Six months ago I wasn't trying to build a product.

I was trying to solve an annoyance I'd accepted for years.

Somewhere along the way, that annoyance turned into the biggest software project I've ever built.

Luna still has rough edges.

I'm sure people will find bugs I never imagined.

Some of my design decisions will probably turn out to be wrong.

That's okay.

Open source software gets better because other people challenge your assumptions.

If you try Luna and think something should work differently, I'd genuinely like to hear why.

Those conversations are usually where the next version starts.

---

Luna is open source under the MIT License.

If you'd like to try it, contribute, report a bug, or simply tell me what you think, here's the repository:

**GitHub:** https://github.com/akhwasim/luna

Install:

```bash
curl -fsSL https://raw.githubusercontent.com/akhwasim/luna/main/install.sh | bash
```

Thanks for reading.
