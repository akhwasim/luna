use serde::{Deserialize, Serialize};
use std::io::{self, Write};

const GROQ_API_URL: &str = "https://api.groq.com/openai/v1/chat/completions";

const SYSTEM_PROMPT: &str = "You are Luna, an AI terminal assistant. \
You ONLY help with terminal commands, file management, system operations, \
and development tasks. You do not engage in general conversation. \
Luna has these built-in commands: history, cd, pwd, exit. \
Never suggest 'history' as a command — if asked about recent commands, \
set command to empty string and tell user to type 'history' directly. \
Keep explanations under 12 words. Be precise, not chatty. \
Never use cargo for file operations, process management, or networking — \
use standard Linux tools only (find, ls, wc, lsof, ps, kill, fuser). \
To list source files in a Rust project use: find src -name '*.rs'. \
IMPORTANT: Always use the provided context to give accurate paths and directories. \
Always use correct Linux syntax. \
Prefer simple readable commands over complex pipelines unless necessary. \
If the intent is clear but details are missing make safe and minimal assumptions. \
Never assume destructive scope beyond what the user specified. \
Do not refuse unless the request is genuinely ambiguous or unsafe. \
Risk classification rules: \
low: read-only safe commands (ls, cat, find without -delete, wc). \
medium: resource-heavy or multi-file operations (du, large find). \
high: destructive or irreversible commands (rm, find -delete, mv, chmod, kill, fuser). \
Never suggest commands that operate on system-critical paths (/ /etc) unless explicitly requested. \
For file search and deletion always include -type f to match files only, never match directories unless explicitly requested, prefer find over rm for patterns, always use explicit patterns and scoped paths, mark as high risk. \
For process and port management prefer fuser for port-based termination (fuser -k PORT/tcp), avoid parsing lsof output with awk, prefer explicit tools over pipelines, use lsof only to show process info not to pipe into kill. \
For process action risk reasons always say terminates running process, never say deletes process. \
When multiple steps are required prefer a single safe command. Avoid suggesting incomplete workflows. \
For destructive actions always limit scope to user-specified paths. \
Risk reasons must describe the action not assumptions. \
Correct: deletes files permanently. Incorrect: may delete system files. \
Package installation with apt brew pip npm is always medium risk as it modifies the system. \
For installing system tools or utilities always use the system package manager (apt on Ubuntu/Debian), never use cargo install unless explicitly asked to install a Rust crate. \
Always respond in this exact JSON format: \
{ \"explanation\": \"short explanation\", \"command\": \"exact command or empty string\", \"risk\": \"low|medium|high\", \"reason\": \"short reason\" } \
If request is not terminal/system related respond: \
{ \"explanation\": \"I only help with terminal and system tasks.\", \"command\": \"\", \"risk\": \"low\", \"reason\": \"out of scope request\" }";

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct GroqRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
}

#[derive(Deserialize, Debug)]
struct GroqResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize, Debug)]
struct Choice {
    message: ResponseMessage,
}

#[derive(Deserialize, Debug)]
struct ResponseMessage {
    content: String,
}

#[derive(Deserialize, Debug)]
struct LunaResponse {
    explanation: String,
    command: String,
    risk: String,
    reason: String,
}

pub async fn ask(query: &str, api_key: &str, context: &str) {
    let client = reqwest::Client::new();

    let user_message = if context.is_empty() {
        query.to_string()
    } else {
        format!("{}\n\nContext:\n{}", query, context)
    };

    let request_body = GroqRequest {
        model: "llama-3.1-8b-instant".to_string(),
        messages: vec![
            Message {
                role: "system".to_string(),
                content: SYSTEM_PROMPT.to_string(),
            },
            Message {
                role: "user".to_string(),
                content: user_message,
            },
        ],
        temperature: 0.3,
    };

    print!("🌙 thinking...");
    io::stdout().flush().unwrap();

    let response = client
        .post(GROQ_API_URL)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await;

    match response {
        Ok(res) => {
            match res.json::<GroqResponse>().await {
                Ok(groq_res) => {
                    let content = &groq_res.choices[0].message.content;
                    let clean = content
                        .replace("```json", "")
                        .replace("```", "")
                        .trim()
                        .to_string();

                    match serde_json::from_str::<LunaResponse>(&clean) {
                        Ok(luna_res) => display_and_confirm(luna_res),
                        Err(_) => println!("\r🌙 {}", content),
                    }
                }
                Err(e) => eprintln!("\rluna: API error: {}", e),
            }
        }
        Err(e) => eprintln!("\rluna: Connection error: {}", e),
    }
}

fn display_and_confirm(res: LunaResponse) {
    let risk_label = match res.risk.as_str() {
        "high" => "HIGH ⚠️",
        "medium" => "MEDIUM ⚡",
        _ => "LOW ✅",
    };

    println!("\r");
    println!("  ─────────────────────────────────");
    println!("  {}", res.explanation);

    if res.command.is_empty() {
        println!("  ─────────────────────────────────");
        println!();
        return;
    }

    println!("  $ {}", res.command);
    println!("  Risk: {}  {}", risk_label, res.reason);
    println!("  ─────────────────────────────────");

    print!("  Execute? (y/n) ❯ ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    if input.trim().to_lowercase() == "y" {
        println!();
        crate::commands::run(&res.command);
    } else {
        println!("  Skipped.");
    }
    println!();
}