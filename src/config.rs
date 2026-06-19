// Luna's persistent configuration.
//
// On disk this is ~/.luna/config.toml. The schema is split into four
// top-level sections: [providers] (a map of every AI provider the user has
// ever configured), [ai] (which provider is currently active), [safety],
// [appearance], and [behavior].
//
// We support two on-disk formats:
//     added multi-AI. `load` detects the legacy format on read and
//     converts it on the fly, then writes the new format back so we don't
//     re-migrate on every launch.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Provider {
    Groq,
    OpenAI,
    Ollama,
    Google,
    Anthropic,
    OpenRouter,
    None,
}

impl Provider {
    pub fn default_base_url(&self) -> &'static str {
        match self {
            Provider::Groq       => "https://api.groq.com/openai/v1/chat/completions",
            Provider::OpenAI     => "https://api.openai.com/v1/chat/completions",
            Provider::Ollama     => "http://localhost:11434/api/chat",
            Provider::Google     => "https://generativelanguage.googleapis.com/v1beta/models",
            Provider::Anthropic  => "https://api.anthropic.com/v1/messages",
            Provider::OpenRouter => "https://openrouter.ai/api/v1/chat/completions",
            Provider::None       => "",
        }
    }

    pub fn needs_key(&self) -> bool {
        !matches!(self, Provider::Ollama | Provider::None)
    }

    pub fn default_model(&self) -> &'static str {
        match self {
            Provider::Groq       => "llama-3.3-70b-versatile",
            Provider::OpenAI     => "gpt-4o-mini",
            Provider::Ollama     => "llama3.2",
            Provider::Google     => "gemini-flash-latest",
            Provider::Anthropic  => "claude-sonnet-4-5",
            Provider::OpenRouter => "meta-llama/llama-3.1-8b-instruct",
            Provider::None       => "",
        }
    }

    pub fn key_env_var(&self) -> &'static str {
        match self {
            Provider::Groq       => "GROQ_API_KEY",
            Provider::OpenAI     => "OPENAI_API_KEY",
            Provider::Anthropic  => "ANTHROPIC_API_KEY",
            Provider::OpenRouter => "OPENROUTER_API_KEY",
            Provider::Google     => "GOOGLE_API_KEY",
            Provider::Ollama     => "",
            Provider::None       => "",
        }
    }

    pub fn signup_url(&self) -> &'static str {
        match self {
            Provider::Groq       => "console.groq.com",
            Provider::OpenAI     => "platform.openai.com",
            Provider::Anthropic  => "console.anthropic.com",
            Provider::OpenRouter => "openrouter.ai",
            Provider::Google     => "aistudio.google.com",
            Provider::Ollama     => "",
            Provider::None       => "",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Provider::Groq       => "Groq (free, fast — recommended)",
            Provider::OpenAI     => "OpenAI",
            Provider::Ollama     => "Ollama (local, offline, no key needed)",
            Provider::Google     => "Google Gemini",
            Provider::Anthropic  => "Anthropic Claude",
            Provider::OpenRouter => "OpenRouter (any model, free tier available)",
            Provider::None       => "Skip for now",
        }
    }
}

impl fmt::Display for Provider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Provider::Groq       => write!(f, "groq"),
            Provider::OpenAI     => write!(f, "openai"),
            Provider::Ollama     => write!(f, "ollama"),
            Provider::Google     => write!(f, "google"),
            Provider::Anthropic  => write!(f, "anthropic"),
            Provider::OpenRouter => write!(f, "openrouter"),
            Provider::None       => write!(f, "none"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub provider: Provider,
    pub model: String,
    pub api_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Providers(pub HashMap<String, ProviderConfig>);

// The currently active provider. We just store the key (e.g. "groq") —
// the actual config lives in the providers map.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveAi {
    pub active: String,
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
    pub providers: Providers,
    pub ai: ActiveAi,
    pub safety: SafetyConfig,
    pub appearance: AppearanceConfig,
    pub behavior: BehaviorConfig,
}

impl LunaConfig {
    /// Build a fresh config with one provider installed and active. Used
    /// by the first-time setup flow.
    pub fn default_with_active(provider: Provider, api_key: String, theme: String) -> Self {
        let key = provider.to_string();
        let model = provider.default_model().to_string();
        let mut map = HashMap::new();
        map.insert(
            key.clone(),
            ProviderConfig {
                provider,
                model,
                api_key,
                base_url: None,
            },
        );
        LunaConfig {
            providers: Providers(map),
            ai: ActiveAi { active: key },
            safety: SafetyConfig { level: "balanced".to_string() },
            appearance: AppearanceConfig { theme, colors: true },
            behavior: BehaviorConfig { habit_threshold: 5, history_limit: 1000 },
        }
    }

    /// Return a reference to the currently active provider config, or
    /// None if the active key doesn't resolve.
    pub fn active_provider(&self) -> Option<&ProviderConfig> {
        self.providers.0.get(&self.ai.active)
    }

  
    pub fn resolved_api_key(&self) -> String {
        let Some(p) = self.active_provider() else { return String::new() };
        if !p.api_key.is_empty() {
            return p.api_key.clone();
        }
        let env_var = p.provider.key_env_var();
        if env_var.is_empty() {
            return String::new();
        }
        std::env::var(env_var).unwrap_or_default()
    }

    /// The base URL to send AI requests to. Honors per-provider overrides
    /// in config, falls back to the provider's known default.
    pub fn resolved_base_url(&self) -> String {
        let Some(p) = self.active_provider() else {
            return Provider::None.default_base_url().to_string();
        };
        p.base_url
            .clone()
            .unwrap_or_else(|| p.provider.default_base_url().to_string())
    }
}

pub fn config_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_default();
    PathBuf::from(format!("{}/.luna/config.toml", home))
}

pub fn config_exists() -> bool {
    config_path().exists()
}

/// Read the config from disk. If it's in the legacy single-provider
/// format, migrate it to the new multi-provider format and write it back,
/// then return the new version.
pub fn load() -> Result<LunaConfig, String> {
    let path = config_path();
    let content = std::fs::read_to_string(&path)
        .map_err(|e| format!("could not read config: {}", e))?;

    if let Ok(cfg) = toml::from_str::<LunaConfig>(&content) {
        return Ok(cfg);
    }

    // Legacy: single [ai] block with provider, model, api_key.
    #[derive(Deserialize)]
    struct Legacy {
        ai: LegacyAi,
        safety: SafetyConfig,
        appearance: AppearanceConfig,
        behavior: BehaviorConfig,
    }
    #[derive(Deserialize)]
    struct LegacyAi {
        provider: Provider,
        model: String,
        api_key: String,
        #[serde(default)]
        base_url: Option<String>,
    }

    let legacy: Legacy = toml::from_str(&content)
        .map_err(|e| format!("config parse error: {}", e))?;

    let key = legacy.ai.provider.to_string();
    let mut map = HashMap::new();
    map.insert(
        key.clone(),
        ProviderConfig {
            provider: legacy.ai.provider,
            model: legacy.ai.model,
            api_key: legacy.ai.api_key,
            base_url: legacy.ai.base_url,
        },
    );

    let new_cfg = LunaConfig {
        providers: Providers(map),
        ai: ActiveAi { active: key },
        safety: legacy.safety,
        appearance: legacy.appearance,
        behavior: legacy.behavior,
    };

    let _ = save(&new_cfg);

    Ok(new_cfg)
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