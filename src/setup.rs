use std::io::{self, Write};
use crate::config::{self, LunaConfig, Provider};

pub async fn run() {
    print_welcome();

    let provider = choose_provider();
    let api_key = if provider.needs_key() {
        prompt_api_key(&provider)
    } else {
        String::new()
    };

    let theme = choose_theme();
    let cfg = LunaConfig::default_for(provider, api_key, theme);

    match config::save(&cfg) {
        Ok(_) => println!("\n✅ Saved to ~/.luna/config.toml\n"),
        Err(e) => {
            eprintln!("\n⚠️  Could not save config: {}", e);
            eprintln!("   Luna will still run, but settings won't persist.\n");
        }
    }

    if matches!(cfg.ai.provider, Provider::Ollama) {
        check_ollama().await;
    }
}

fn print_welcome() {
    println!();
    println!("🌙 Welcome to Luna. Let's get you set up.");
    println!();
}

fn choose_provider() -> Provider {
    let providers = [
        Provider::Groq,
        Provider::OpenAI,
        Provider::Ollama,
        Provider::Google,
        Provider::None,
    ];

    println!("Choose your AI provider:");
    for (i, p) in providers.iter().enumerate() {
        println!("  {}. {}", i + 1, p.label());
    }
    println!();

    loop {
        let choice = read_line("Choice ❯ ");
        match choice.trim() {
            "1" => return Provider::Groq,
            "2" => return Provider::OpenAI,
            "3" => return Provider::Ollama,
            "4" => return Provider::Google,
            "5" => return Provider::None,
            _ => println!("  Please enter 1-5."),
        }
    }
}

fn prompt_api_key(provider: &Provider) -> String {
    let env_var = provider.key_env_var();
    let signup = provider.signup_url();
    println!("{} API key (free at {}) ❯ ", env_var, signup);
    io::stdout().flush().ok();
    let key = read_line("");
    key.trim().to_string()
}

fn choose_theme() -> String {
    let themes = ["dark", "moonlight", "eink", "light"];
    println!();
    println!("Choose theme:");
    for (i, t) in themes.iter().enumerate() {
        println!("  {}. {}", i + 1, t);
    }
    println!();

    loop {
        let choice = read_line("Choice ❯ ");
        match choice.trim() {
            "1" => return "dark".to_string(),
            "2" => return "moonlight".to_string(),
            "3" => return "eink".to_string(),
            "4" => return "light".to_string(),
            _ => println!("  Please enter 1-4."),
        }
    }
}

async fn check_ollama() {
    println!("Checking if Ollama is running...");
    let url = "http://localhost:11434/api/tags";
    match reqwest::get(url).await {
        Ok(_) => println!("✅ Ollama is running.\n"),
        Err(_) => {
            println!("⚠️  Ollama is not reachable at localhost:11434.");
            println!("   Start it with `ollama serve` in another terminal.");
            println!("   Luna will save your settings anyway.\n");
        }
    }
}

fn read_line(prompt: &str) -> String {
    if !prompt.is_empty() {
        print!("{}", prompt);
        io::stdout().flush().ok();
    }
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap_or(0);
    s
}