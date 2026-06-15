use serde::{Deserialize, Serialize};
use std::io::{self, Write};
use crate::config::{LunaConfig, Provider};

const SYSTEM_PROMPT: &str = "You are Luna, an AI terminal assistant. \
You ONLY help with terminal commands, file management, system operations, \
and development tasks. You do not engage in general conversation. \
Luna has these built-in commands: history, cd, pwd, exit. \
Never suggest 'history' as a command — if asked about recent commands, \
set command to empty string and tell user to type 'history' directly. \
Keep explanations under 12 words. Be precise, not chatty. \
Never use language-specific build tools (cargo, npm, pip, gradle) for file operations, process management, or networking — \
use standard Linux tools only (find, ls, wc, lsof, ps, kill, fuser). \
To list source files use find with the file extension matching the project type from context. \
Derive the correct extension from project type — never hardcode a specific language. \
IMPORTANT: Always use the provided context to give accurate paths and directories. \
Always use correct Linux syntax. \
Prefer simple readable commands over complex pipelines unless necessary. \
If the intent is clear but details are missing make safe and minimal assumptions. \
Never assume destructive scope beyond what the user specified. \
If a destructive command requires a specific target and user did not specify one, always show the command with a placeholder. \
Example: user asks 'command to delete a folder' → command: 'rm -rf <folder-name>', explanation: 'replace folder-name with target'. \
Never return empty command just because target is missing. \
Do not refuse unless the request is genuinely ambiguous or unsafe. \
Risk classification rules: \
low: read-only safe commands (ls, cat, find without -delete, wc). \
medium: resource-heavy or multi-file operations (du, large find, package installs). \
high: destructive or irreversible commands (rm, find -delete, mv, chmod, kill, fuser). \
Never suggest commands that operate on system-critical paths (/ /etc) unless explicitly requested. \
For file search and deletion always include -type f to match files only, never match directories unless explicitly requested, prefer find over rm for patterns, always use explicit patterns and scoped paths, mark as high risk. \
For process and port management prefer fuser for port-based termination (fuser -k PORT/tcp), avoid parsing lsof output with awk, prefer explicit tools over pipelines, use lsof only to show process info not to pipe into kill. \
For process action risk reasons always say terminates running process, never say deletes process. \
When multiple steps are required prefer a single safe command. Avoid suggesting incomplete workflows. \
For destructive actions always limit scope to user-specified paths. \
Risk reasons must describe the action not assumptions. \
Luna can answer questions about the user's recent activity, errors, and commands using the provided context. \
Questions like 'what errors have I been getting', 'what was the last command I ran', 'what have I been working on' are valid. \
For memory questions always set command to empty string and put the answer in explanation. \
Example: 'what was the last command' → explanation: 'git status', command: ''. \
Example: 'what have I been working on' → explanation: 'Rust project, running cargo build and git commands', command: ''. \
Example: 'what directory am i in' → explanation: '/home/glitch/luna', command: ''. \
Never say 'check recent commands' or 'type history' — just state the answer directly from context. \
Risk reasons must describe the action not assumptions. \
Package installation with apt brew pip npm is always medium risk as it modifies the system. \
For installing system tools or utilities always use the system package manager (apt on Ubuntu/Debian, brew on macOS). \
Use language package managers only when the user specifically names one or the project type makes it obvious. \
Never assume a package manager — derive it from context. \
Docker kubectl and container tools are valid system operations, always suggest correct docker commands regardless of project type. \
Git commands are always standard git commands regardless of project type — never use language-specific tools for git operations. \
Examples of correct git commands: git init, git checkout -b <branch-name>, git add, git commit, git push. \
For generic how-to questions not tied to a specific path always use placeholders like <folder-name> <filename> <branch-name>. \
Never suggest --no-preserve-root in any command. This flag bypasses a critical Linux safety mechanism. \
Never suggest commands that could destroy the root filesystem. \
Always respond in this exact JSON format: \
{ \"explanation\": \"short explanation\", \"command\": \"best command\", \"risk\": \"low|medium|high\", \"reason\": \"short reason\", \"alternatives\": [\"second option\", \"third option\"] } \
The alternatives array should contain 2 other valid approaches if they exist, or empty array [] if not. \
If request is not terminal/system related respond: \
{ \"explanation\": \"I only help with terminal and system tasks.\", \"command\": \"\", \"risk\": \"low\", \"reason\": \"out of scope request\", \"alternatives\": [] }";

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct OpenAiRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
}

#[derive(Deserialize, Debug)]
struct OpenAiResponse {
    choices: Vec<OpenAiChoice>,
}

#[derive(Deserialize, Debug)]
struct OpenAiChoice {
    message: OpenAiMessage,
}

#[derive(Deserialize, Debug)]
struct OpenAiMessage {
    content: String,
}

#[derive(Serialize)]
struct GoogleRequest {
    contents: Vec<GoogleContent>,
}

#[derive(Serialize)]
struct GoogleContent {
    parts: Vec<GooglePart>,
}

#[derive(Serialize)]
struct GooglePart {
    text: String,
}

#[derive(Deserialize, Debug)]
struct GoogleResponse {
    candidates: Vec<GoogleCandidate>,
}

#[derive(Deserialize, Debug)]
struct GoogleCandidate {
    content: GoogleCandidateContent,
}

#[derive(Deserialize, Debug)]
struct GoogleCandidateContent {
    parts: Vec<GoogleCandidatePart>,
}

#[derive(Deserialize, Debug)]
struct GoogleCandidatePart {
    text: String,
}

#[derive(Serialize)]
struct AnthropicRequest {
    model: String,
    system: String,
    messages: Vec<Message>,
    max_tokens: u32,
}

#[derive(Deserialize, Debug)]
struct AnthropicResponse {
    content: Vec<AnthropicContent>,
}

#[derive(Deserialize, Debug)]
struct AnthropicContent {
    text: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct LunaResponse {
    pub explanation: String,
    pub command: String,
    pub risk: String,
    pub reason: String,
    pub alternatives: Option<Vec<String>>,
}

fn sanitize_command(cmd: &str) -> String {
    let patterns: &[(&str, &str)] = &[
        ("git init && cd . &&", "git init &&"),
        ("&& cd . &&", " &&"),
        ("git init && git add . && git commit -m 'initial commit'", "git init"),
        ("git init && cd . && git add . && git commit -m 'initial commit'", "git init"),
        ("cargo new . --bin &&", "git init &&"),
        ("cargo new . &&", "git init &&"),
        ("cargo new --vcs git . &&", "git init &&"),
        ("cargo new .", "git init"),
        ("cargo add git", "git checkout -b <branch-name>"),
        ("cargo add feature", "git checkout -b <branch-name>"),
        ("cargo feature", "git checkout -b <branch-name>"),
        ("cargo tree | grep .rs", "find src -name '*.rs'"),
        ("cargo find", "find"),
    ];

    let mut result = cmd.to_string();
    for (wrong, correct) in patterns {
        if result.contains(wrong) {
            result = result.replace(wrong, correct);
        }
    }
    result
}

fn is_memory_query(query: &str) -> bool {
    let q = query.to_lowercase();
    q.contains("last command")
        || q.contains("what directory")
        || q.contains("where am i")
        || q.contains("current dir")
        || q.contains("what have i been")
        || q.contains("what was i")
        || q.contains("what errors")
        || q.contains("what did i")
        || q.contains("recently")
        || q.contains("working on")
}

fn extract_first_json(text: &str) -> String {
    let start = text.find('{');
    let end = text.rfind('}');
    match (start, end) {
        (Some(s), Some(e)) if e > s => text[s..=e].to_string(),
        _ => text.to_string(),
    }
}

pub async fn ask(query: &str, cfg: &LunaConfig, context: &str, recent: &[String]) {
    if matches!(cfg.active_provider().map(|p| &p.provider), Some(Provider::None)) {
        println!("\n  🌙 No AI provider configured.");
        println!("  Run `luna config` to set one up.\n");
        return;
    }

    let api_key = cfg.resolved_api_key();
    if api_key.is_empty() && cfg.active_provider().map(|p| p.provider.needs_key()).unwrap_or(false) {
        let provider_label = cfg.active_provider().map(|p| p.provider.to_string()).unwrap_or_else(|| "unknown".to_string());
        eprintln!("\n  ⚠️  Missing API key for {}.", provider_label);
        eprintln!("  Run `luna config` to update.\n");
        return;
    }

    let user_message = if context.is_empty() {
        query.to_string()
    } else {
        format!("{}\n\nContext:\n{}", query, context)
    };

    print!("🌙 thinking...");
    io::stdout().flush().unwrap();

    let res = dispatch_ask(&user_message, cfg).await;
    handle_response(res, query, Some(context), false, recent);
}


pub async fn fix_error(command: &str, error: &str, cfg: &LunaConfig, context: &str, recent: &[String]) {
    if matches!(cfg.active_provider().map(|p| &p.provider), Some(Provider::None)) {
        return;
    }
    if cfg.resolved_api_key().is_empty() && cfg.active_provider().map(|p| p.provider.needs_key()).unwrap_or(false) {
        return;
    }

    let user_message = format!(
        "Command that failed: {}\nError output: {}\n\nContext:\n{}\n\nThe command failed. Suggest a specific fix command. If the file or path does not exist, suggest how to create it or find the correct path.",
        command, error, context
    );

    print!("🌙 analyzing error...");
    io::stdout().flush().unwrap();

    let res = dispatch_ask(&user_message, cfg).await;
    println!("\r");
    println!("  🌙 Luna detected an error");
    println!("  ─────────────────────────────────");
    handle_response(res, "", None, true, recent);
}


async fn dispatch_ask(user_message: &str, cfg: &LunaConfig) -> Result<String, String> {
    let active = cfg.active_provider().map(|p| &p.provider);
    match active {
        Some(Provider::Groq) | Some(Provider::OpenAI) | Some(Provider::Ollama) | Some(Provider::OpenRouter) => {
            call_openai_format(user_message, cfg).await
        }
        Some(Provider::Google) => call_google_format(user_message, cfg).await,
        Some(Provider::Anthropic) => call_anthropic_format(user_message, cfg).await,
        Some(Provider::None) | None => Err("no provider".to_string()),
    }
}

async fn call_openai_format(user_message: &str, cfg: &LunaConfig) -> Result<String, String> {
    let client = reqwest::Client::new();
    let url = cfg.resolved_base_url();
    let body = OpenAiRequest {
        model: cfg.active_provider().map(|p| p.model.clone()).unwrap_or_default(),
        messages: vec![
            Message { role: "system".to_string(), content: SYSTEM_PROMPT.to_string() },
            Message { role: "user".to_string(), content: user_message.to_string() },
        ],
        temperature: 0.3,
    };

    let mut req = client.post(&url).header("Content-Type", "application/json").json(&body);
    let key = cfg.resolved_api_key();
    if !key.is_empty() {
        req = req.header("Authorization", format!("Bearer {}", key));
    }

    let res = req.send().await.map_err(|e| format!("connection error: {}", e))?;
    if !res.status().is_success() {
        let status = res.status();
        let body = res.text().await.unwrap_or_default();
        return Err(format!("http {} — {}", status, &body[..body.len().min(200)]));
    }
    let parsed: OpenAiResponse = res.json().await.map_err(|e| format!("api error: {}", e))?;
    Ok(parsed.choices.first()
        .map(|c| c.message.content.clone())
        .unwrap_or_default())
}

async fn call_google_format(user_message: &str, cfg: &LunaConfig) -> Result<String, String> {
    let client = reqwest::Client::new();
    let url = format!(
        "{}/{}:generateContent",
        cfg.resolved_base_url(),
        cfg.active_provider().map(|p| p.model.clone()).unwrap_or_default()
    );
    let body = GoogleRequest {
        contents: vec![GoogleContent {
            parts: vec![GooglePart { text: format!("{}\n\n{}", SYSTEM_PROMPT, user_message) }],
        }],
    };

    let res = client.post(&url)
        .header("Content-Type", "application/json")
        .header("X-goog-api-key", cfg.resolved_api_key())
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("connection error: {}", e))?;

    if !res.status().is_success() {
        let status = res.status();
        let body = res.text().await.unwrap_or_default();
        return Err(format!("http {} — {}", status, &body[..body.len().min(200)]));
    }
    let parsed: GoogleResponse = res.json().await.map_err(|e| format!("api error: {}", e))?;
    Ok(parsed.candidates.first()
        .and_then(|c| c.content.parts.first())
        .map(|p| p.text.clone())
        .unwrap_or_default())
}

async fn call_anthropic_format(user_message: &str, cfg: &LunaConfig) -> Result<String, String> {
    let client = reqwest::Client::new();
    let url = cfg.resolved_base_url();
    let body = AnthropicRequest {
        model: cfg.active_provider().map(|p| p.model.clone()).unwrap_or_default(),
        system: SYSTEM_PROMPT.to_string(),
        messages: vec![
            Message { role: "user".to_string(), content: user_message.to_string() },
        ],
        max_tokens: 4096,
    };

    let res = client.post(&url)
        .header("Content-Type", "application/json")
        .header("x-api-key", cfg.resolved_api_key())
        .header("anthropic-version", "2023-06-01")
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("connection error: {}", e))?;

    if !res.status().is_success() {
        let status = res.status();
        let body = res.text().await.unwrap_or_default();
        return Err(format!("http {} — {}", status, &body[..body.len().min(200)]));
    }
    let parsed: AnthropicResponse = res.json().await.map_err(|e| format!("api error: {}", e))?;
    Ok(parsed.content.first().map(|c| c.text.clone()).unwrap_or_default())
}

/// Re-rank the AI's suggestions based on the user's command history.
///
/// The AI returns one "main" command plus 0-2 alternatives. We look at how
/// often the user has run each of those commands (or close variants) and
/// re-order them so the most-used one comes first.
///
/// Counting rules:
///   1. Exact match: `cargo run` 12 times → score 12
///   2. First-word match: `cargo run --release` 5 times → if the AI's
///      suggestion starts with `cargo`, count those 5 too. This handles
///      variants like `cargo run` vs `cargo run --release`.
///   3. We use the bigger of the two so a specific match beats a vague
///      first-word match when both apply.
fn rank_by_user_preference(
    luna_res: &LunaResponse,
    recent: &[String],
) -> LunaResponse {
    // Collect all options: main + alternatives, skipping empties.
    let mut options: Vec<String> = vec![luna_res.command.clone()];
    if let Some(alts) = &luna_res.alternatives {
        for alt in alts {
            options.push(alt.clone());
        }
    }
    options.retain(|c| !c.trim().is_empty());

    if options.len() <= 1 {
        // Nothing to re-rank.
        return luna_res.clone();
    }

    // Count occurrences of each option in recent history.
    let mut scored: Vec<(String, usize)> = options
        .into_iter()
        .map(|cmd| {
            let score = score_command(&cmd, recent);
            (cmd, score)
        })
        .collect();

    // Stable sort by score descending. Stable preserves the AI's original
    // order for ties, which feels right (the AI knew what it was doing).
    scored.sort_by(|a, b| b.1.cmp(&a.1));

    // Reassemble: highest score becomes the new main, the rest become
    // the new alternatives (in their re-ranked order).
    let mut new_main = String::new();
    let mut new_alts: Vec<String> = Vec::new();
    for (i, (cmd, _)) in scored.iter().enumerate() {
        if i == 0 {
            new_main = cmd.clone();
        } else {
            new_alts.push(cmd.clone());
        }
    }

    LunaResponse {
        explanation: luna_res.explanation.clone(),
        command: new_main,
        risk: luna_res.risk.clone(),
        reason: luna_res.reason.clone(),
        alternatives: if new_alts.is_empty() { None } else { Some(new_alts) },
    }
}

/// Score a single command against the recent history list. Returns a
/// count where higher = more frequently used (or more similar to a
/// frequently-used command).
fn score_command(cmd: &str, recent: &[String]) -> usize {
    let first_word = cmd.split_whitespace().next().unwrap_or("");
    if first_word.is_empty() {
        return 0;
    }

    let mut exact = 0usize;
    let mut first_word_matches = 0usize;
    for r in recent {
        let r_trim = r.trim();
        if r_trim.is_empty() {
            continue;
        }
        if r_trim == cmd {
            exact += 1;
        }
        // First-word match: does this recent command start with the same
        // first word as the suggested command?
        if r_trim.split_whitespace().next() == Some(first_word) {
            first_word_matches += 1;
        }
    }

    // Return the larger of the two. Exact matches are the strongest signal
    // but first-word matches are still meaningful (catches variants).
    exact.max(first_word_matches)
}

fn handle_response(
    res: Result<String, String>,
    query: &str,
    context: Option<&str>,
    from_error: bool,
    recent: &[String],
) {
    let content = match res {
        Ok(c) => c,
        Err(e) => {
            eprintln!("\rluna: {}", e);
            return;
        }
    };

    let clean = content.replace("```json", "").replace("```", "").trim().to_string();
    let json_str = extract_first_json(&clean);

    match serde_json::from_str::<LunaResponse>(&json_str) {
        Ok(mut luna_res) => {
            // Re-rank based on user history. Skipped for error-fix paths
            // (the AI is suggesting one specific fix, not a menu of
            // choices) and skipped for memory queries (command is empty
            // and ranking doesn't apply).
            if !from_error && !luna_res.command.is_empty() && luna_res.command != luna_res.explanation {
                luna_res = rank_by_user_preference(&luna_res, recent);
            }

            if !from_error && is_memory_query(query) {
                luna_res.explanation = if luna_res.command.is_empty() {
                    luna_res.explanation
                } else {
                    format!("{} — {}", luna_res.explanation, luna_res.command)
                };
                luna_res.command = String::new();
            }
            display_and_confirm(luna_res, context);
        }
        Err(_) => println!("\r🌙 {}", content),
    }
}

fn display_and_confirm(res: LunaResponse, memory_ref: Option<&str>) {
    let clean_command = sanitize_command(&res.command);

    let risk_label = match res.risk.as_str() {
        "high" => "HIGH ⚠️",
        "medium" => "MEDIUM ⚡",
        _ => "LOW ✅",
    };

    println!("\r");
    println!("  ─────────────────────────────────");
    println!("  {}", res.explanation);

    if clean_command.is_empty() {
        println!("  ─────────────────────────────────");
        println!();
        return;
    }

    // Personalized ranking: a command is "preferred" if EITHER:
 
    let is_preferred = memory_ref
        .map(|ctx| {
            if ctx.contains(&clean_command) {
                return true;
            }
  
            let first_word = clean_command
                .split_whitespace()
                .next()
                .unwrap_or("");
            if first_word.is_empty() {
                return false;
            }

            ctx.split(|c: char| c == ',' || c == ' ' || c == '\n')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .any(|tok| tok.split_whitespace().next() == Some(first_word))
        })
        .unwrap_or(false);


    let preferred_label = if is_preferred { " ⭐ based on your history" } else { "" };

    match crate::safety::check(&clean_command) {
        crate::safety::RiskLevel::Critical(reason) => {
            println!("  🚨 CRITICAL — Luna's suggestion was flagged as dangerous");
            println!("  ─────────────────────────────────");
            println!("  {}", reason);
            println!("  $ {}", clean_command);
            println!();
            print!("  Type 'I UNDERSTAND' to proceed or any key to cancel ❯ ");
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
            let mut confirm = String::new();
            std::io::stdin().read_line(&mut confirm).unwrap();
            if confirm.trim() != "I UNDERSTAND" {
                println!("  Blocked.");
                println!();
                return;
            }
            println!();
        }
        crate::safety::RiskLevel::High(_) => {
            println!("  $ {}{}", clean_command, preferred_label);
            println!("  Risk: HIGH ⚠️  {}", res.reason);
            println!("  ─────────────────────────────────");
            print!("  Execute? (y/n/more) ❯ ");
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let input = input.trim().to_lowercase();
            if input == "more" {
                show_alternatives(&res.alternatives, risk_label, &clean_command);
                return;
            }
            if input != "y" {
                println!("  Skipped.");
                println!();
                return;
            }
            println!();
            crate::commands::run(&clean_command);
            return;
        }
        _ => {}
    }

    let looks_like_template = clean_command.contains('<')
        || clean_command.contains("file_name")
        || clean_command.contains("folder_name")
        || clean_command.contains("image_name")
        || clean_command.contains("image-name")
        || clean_command.contains("your_");

    if looks_like_template {
        println!("  $ {}{}", clean_command, preferred_label);
        println!("  Risk: {}  {}", risk_label, res.reason);
        println!("  ─────────────────────────────────");
        println!();
        return;
    }

    println!("  $ {}{}", clean_command, preferred_label);
    println!("  Risk: {}  {}", risk_label, res.reason);
    println!("  ─────────────────────────────────");

    print!("  Execute? (y/n/more) ❯ ");
    std::io::Write::flush(&mut std::io::stdout()).unwrap();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim().to_lowercase();

    if input == "more" {
        show_alternatives(&res.alternatives, risk_label, &clean_command);
        return;
    }

    if input == "y" {
        println!();
        crate::commands::run(&clean_command);
    } else {
        println!("  Skipped.");
    }
    println!();
}

fn show_alternatives(alternatives: &Option<Vec<String>>, risk_label: &str, original: &str) {
    match alternatives {
        Some(alts) if !alts.is_empty() => {
            println!();
            println!("  All options:");
            println!("  ─────────────────────────────────");
            println!("  1. $ {}", original);
            for (i, alt) in alts.iter().enumerate() {
                println!("  {}. $ {}", i + 2, alt);
            }
            println!("  ─────────────────────────────────");
            print!("  Choose (1-{}) or n to skip ❯ ", alts.len() + 1);
            std::io::Write::flush(&mut std::io::stdout()).unwrap();

            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();

            if input.to_lowercase() == "n" {
                println!("  Skipped.");
                return;
            }

            if let Ok(n) = input.parse::<usize>() {
                let cmd = if n == 1 {
                    original.to_string()
                } else if n >= 2 && n <= alts.len() + 1 {
                    alts[n - 2].clone()
                } else {
                    println!("  Invalid option.");
                    return;
                };

                println!();
                println!("  $ {}", cmd);
                println!("  Risk: {}", risk_label);
                println!("  ─────────────────────────────────");
                print!("  Execute? (y/n) ❯ ");
                std::io::Write::flush(&mut std::io::stdout()).unwrap();
                let mut confirm = String::new();
                std::io::stdin().read_line(&mut confirm).unwrap();
                if confirm.trim().to_lowercase() == "y" {
                    println!();
                    crate::commands::run(&cmd);
                } else {
                    println!("  Skipped.");
                }
            } else {
                println!("  Skipped.");
            }
        }
        _ => {
            println!();
            println!("  No alternatives available.");
            println!();
        }
    }
}