use serde::{Deserialize, Serialize};
use std::io::{self, Write};

const GROQ_API_URL: &str = "https://api.groq.com/openai/v1/chat/completions";

const SYSTEM_PROMPT: &str = "You are Luna, an AI terminal assistant. \
You ONLY help with terminal commands, file management, system operations, \
and development tasks. You do not engage in general conversation. \
Always respond in this exact JSON format: \
{ \"explanation\": \"brief explanation of what this does\", \"command\": \"the exact command to run\", \"risk\": \"low|medium|high\" } \
If the request is not terminal/system related, respond: \
{ \"explanation\": \"I only help with terminal and system tasks.\", \"command\": \"\", \"risk\": \"low\" }";

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
}

pub async fn ask(query: &str, api_key: &str) {
    let client = reqwest::Client::new();

    let request_body = GroqRequest {
        model: "llama-3.1-8b-instant".to_string(),
        messages: vec![
            Message {
                role: "system".to_string(),
                content: SYSTEM_PROMPT.to_string(),
            },
            Message {
                role: "user".to_string(),
                content: query.to_string(),
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
                    // Clean up markdown code blocks if present
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
                Err(e) => eprintln!("\r luna: API error: {}", e),
            }
        }
        Err(e) => eprintln!("\rluna: Connection error: {}", e),
    }
}

fn display_and_confirm(res: LunaResponse) {
    // Risk indicator
    let risk_label = match res.risk.as_str() {
        "high" => "⚠️  HIGH RISK",
        "medium" => "⚡ MEDIUM RISK",
        _ => "✅ LOW RISK",
    };

    println!("\r");
    println!("  🌙 Luna");
    println!("  ─────────────────────────────────");
    println!("  {}", res.explanation);
    println!();

    if res.command.is_empty() {
        return;
    }

    println!("  $ {}", res.command);
    println!("  {}", risk_label);
    println!();
    print!("  Run this? (y/n) ❯ ");
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