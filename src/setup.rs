use std::io::{self, Write};
use crate::config::{self, LunaConfig, Provider};

/// Top-level entry point. If config exists, hand off to the shell. If not,
/// walk the wizard and then hand off. Either way, the user never sees a
/// "restart required" gap.
pub async fn run() {
    if config::config_exists() {
        crate::shell::run();
        return;
    }
    run_wizard().await;
    crate::shell::run();
}

/// The wizard itself. Used by `run()` on first launch and by
/// `shell::luna config` for re-setup. Walks the user through provider +
/// key + theme, then saves.
pub async fn run_wizard() {
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
        Ok(_) => {
            println!();
            println!("  ✅ Configuration saved");
            println!();
        }
        Err(e) => {
            eprintln!("\n  ⚠️  Could not save config: {}", e);
            eprintln!("     Luna will still run, but settings won't persist.\n");
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
    Provider::Anthropic,
    Provider::OpenRouter,
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
            "5" => return Provider::Anthropic,
            "6" => return Provider::OpenRouter,
            "7" => return Provider::None,
            _ => println!("  Please enter 1-7."),
        }
    }
}

fn prompt_api_key(provider: &Provider) -> String {
    let signup = provider.signup_url();
    let name = match provider {
        Provider::Groq       => "Groq",
        Provider::OpenAI     => "OpenAI",
        Provider::Google     => "Google Gemini",
        Provider::Anthropic  => "Anthropic",
        Provider::OpenRouter => "OpenRouter",
        _ => unreachable!(),
    };
    println!();
    println!("Enter your {} API key", name);
    if !signup.is_empty() {
        println!("(free at {})", signup);
    }
    println!();
    let key = read_line("❯ ");
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