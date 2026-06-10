use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Provider {
    Groq,
    OpenAI,
    Ollama,
    Google,
    None,
}

impl Provider {
    pub fn default_base_url(&self) -> &'static str {
        match self {
            Provider::Groq   => "https://api.groq.com/openai/v1/chat/completions",
            Provider::OpenAI => "https://api.openai.com/v1/chat/completions",
            Provider::Ollama => "http://localhost:11434/api/chat",
            Provider::Google => "https://generativelanguage.googleapis.com/v1beta/models",
            Provider::None   => "",
        }
    }

    pub fn needs_key(&self) -> bool {
        !matches!(self, Provider::Ollama | Provider::None)
    }

    pub fn default_model(&self) -> &'static str {
        match self {
            Provider::Groq   => "llama-3.3-70b-versatile",
            Provider::OpenAI => "gpt-4o-mini",
            Provider::Ollama => "llama3.2",
            Provider::Google => "gemini-2.0-flash",
            Provider::None   => "",
        }
    }

    pub fn key_env_var(&self) -> &'static str {
        match self {
            Provider::Groq   => "GROQ_API_KEY",
            Provider::OpenAI => "OPENAI_API_KEY",
            Provider::Google => "GOOGLE_API_KEY",
            Provider::Ollama => "",
            Provider::None   => "",
        }
    }

    pub fn signup_url(&self) -> &'static str {
        match self {
            Provider::Groq   => "console.groq.com",
            Provider::OpenAI => "platform.openai.com",
            Provider::Google => "aistudio.google.com",
            Provider::Ollama => "",
            Provider::None   => "",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Provider::Groq   => "Groq (free, fast — recommended)",
            Provider::OpenAI => "OpenAI",
            Provider::Ollama => "Ollama (local, offline, no key needed)",
            Provider::Google => "Google Gemini",
            Provider::None   => "Skip for now",
        }
    }
}

impl fmt::Display for Provider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Provider::Groq   => write!(f, "groq"),
            Provider::OpenAI => write!(f, "openai"),
            Provider::Ollama => write!(f, "ollama"),
            Provider::Google => write!(f, "google"),
            Provider::None   => write!(f, "none"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiConfig {
    pub provider: Provider,
    pub model: String,
    pub api_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyConfig {
    pub level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppearanceConfig {
    pub theme: String,
    pub colors: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorConfig {
    pub habit_threshold: usize,
    pub history_limit: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LunaConfig {
    pub ai: AiConfig,
    pub safety: SafetyConfig,
    pub appearance: AppearanceConfig,
    pub behavior: BehaviorConfig,
}

impl LunaConfig {
        pub fn default_for(provider: Provider, api_key: String, theme: String) -> Self {
        let model = provider.default_model().to_string();
        LunaConfig {
            ai: AiConfig {
                provider,
                model,
                api_key,
                base_url: None,
            },
            safety: SafetyConfig {
                level: "balanced".to_string(),
            },
            appearance: AppearanceConfig {
                theme,
                colors: true,
            },
            behavior: BehaviorConfig {
                habit_threshold: 5,
                history_limit: 1000,
            },
        }
    }

    // Config wins, env var is the backward-compat fallback for ~/.luna/.env users
    pub fn resolved_api_key(&self) -> String {
        if !self.ai.api_key.is_empty() {
            return self.ai.api_key.clone();
        }
        let env_var = self.ai.provider.key_env_var();
        if env_var.is_empty() {
            return String::new();
        }
        std::env::var(env_var).unwrap_or_default()
    }

    pub fn resolved_base_url(&self) -> String {
        self.ai.base_url
            .clone()
            .unwrap_or_else(|| self.ai.provider.default_base_url().to_string())
    }
}

pub fn config_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_default();
    PathBuf::from(format!("{}/.luna/config.toml", home))
}

pub fn config_exists() -> bool {
    config_path().exists()
}

pub fn load() -> Result<LunaConfig, String> {
    let path = config_path();
    let content = std::fs::read_to_string(&path)
        .map_err(|e| format!("could not read config: {}", e))?;
    toml::from_str(&content)
        .map_err(|e| format!("config parse error: {}", e))
}

pub fn save(config: &LunaConfig) -> Result<(), String> {
    let path = config_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("could not create ~/.luna: {}", e))?;
    }
    let content = toml::to_string_pretty(config)
        .map_err(|e| format!("could not serialize config: {}", e))?;
    std::fs::write(&path, content)
        .map_err(|e| format!("could not write config: {}", e))
}