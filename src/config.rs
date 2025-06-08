use colored::*;
use question::{Answer, Question};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    pub default_provider: Option<String>,
    #[serde(default)]
    pub api_keys: ApiKeys,
    #[serde(default)]
    pub models: Models,
    #[serde(default)]
    pub aliases: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ApiKeys {
    pub openai: Option<String>,
    pub gemini: Option<String>,
    pub anthropic: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Models {
    pub openai: Option<String>,
    pub gemini: Option<String>,
    pub anthropic: Option<String>,
}

impl Default for Models {
    fn default() -> Self {
        Self {
            openai: Some("gpt-4o-mini".to_string()),
            gemini: Some("gemini-1.5-flash-latest".to_string()),
            anthropic: Some("claude-3-haiku-20240307".to_string()),
        }
    }
}

fn get_config_path() -> Result<PathBuf, String> {
    let config_dir = directories::ProjectDirs::from("com", "commitcraft", "commitcraft")
        .ok_or("Could not determine config directory.")?;
    Ok(config_dir.config_dir().to_path_buf())
}

pub fn load_config() -> Result<Config, String> {
    let config_path = get_config_path()?.join("config.toml");
    if !config_path.exists() {
        println!("{}", "Configuration file not found.".yellow());
        println!(
            "Please run '{}' to get started.",
            "commitcraft setup".bold().cyan()
        );
        return Err("Config not found".to_string());
    }

    let content = fs::read_to_string(&config_path)
        .map_err(|e| format!("Failed to read config file: {}", e))?;

    toml::from_str(&content).map_err(|e| format!("Failed to parse config file: {}", e))
}

pub fn run_setup() -> Result<(), String> {
    println!("{}", "Welcome to CommitCraft setup!".bold().green());
    println!("Let's configure your AI providers.");

    let mut config = load_config().unwrap_or_default();

    // Ask for default provider
    let provider_answer = Question::new(
        "Which AI provider do you want to use by default? (gemini, openai, anthropic)",
    )
    .ask()
    .ok_or("Failed to ask question".to_string())?;
    let provider_choice = match provider_answer {
        Answer::RESPONSE(s) => s,
        _ => return Err("Invalid answer type for provider choice".to_string()),
    }
    .to_lowercase();

    config.default_provider = Some(provider_choice.clone());

    println!("\nNow, let's add API keys. You can leave any of them blank.");

    // Ask for API keys
    let openai_key_answer = Question::new("Enter your OpenAI API key (starts with 'sk-'):")
        .ask()
        .ok_or("Failed to ask question".to_string())?;
    let openai_key = match openai_key_answer {
        Answer::RESPONSE(s) => s,
        _ => String::new(), // Default to empty if not a string response, or handle error
    };
    if !openai_key.is_empty() {
        config.api_keys.openai = Some(openai_key);
    }

    let gemini_key_answer = Question::new("Enter your Google AI (Gemini) API key:")
        .ask()
        .ok_or("Failed to ask question".to_string())?;
    let gemini_key = match gemini_key_answer {
        Answer::RESPONSE(s) => s,
        _ => String::new(),
    };
    if !gemini_key.is_empty() {
        config.api_keys.gemini = Some(gemini_key);
    }

    let anthropic_key_answer = Question::new("Enter your Anthropic (Claude) API key:")
        .ask()
        .ok_or("Failed to ask question".to_string())?;
    let anthropic_key = match anthropic_key_answer {
        Answer::RESPONSE(s) => s,
        _ => String::new(),
    };
    if !anthropic_key.is_empty() {
        config.api_keys.anthropic = Some(anthropic_key);
    }

    // Setup aliases
    let setup_aliases = Question::new(
        "Do you want to set up model aliases? (e.g., 'fast' -> 'gemini-1.5-flash-latest')",
    )
    .yes_no()
    .default(Answer::YES)
    .ask()
    .ok_or("Failed to ask question".to_string())?;

    if let Answer::YES = setup_aliases {
        config
            .aliases
            .insert("fast".to_string(), "gemini-1.5-flash-latest".to_string());
        config
            .aliases
            .insert("smart".to_string(), "gpt-4o".to_string());
        println!("Default aliases 'fast' and 'smart' have been added.");
    }

    save_config(&config)?;

    println!(
        "\n{}",
        "Setup complete! Your configuration has been saved."
            .bold()
            .green()
    );
    Ok(())
}

fn save_config(config: &Config) -> Result<(), String> {
    let config_path = get_config_path()?.join("config.toml");
    let config_dir = config_path.parent().unwrap();

    fs::create_dir_all(config_dir)
        .map_err(|e| format!("Failed to create config directory: {}", e))?;

    let toml_string =
        toml::to_string_pretty(config).map_err(|e| format!("Failed to serialize config: {}", e))?;

    fs::write(&config_path, toml_string).map_err(|e| format!("Failed to write config file: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn test_models_default() {
        let models = Models::default();
        assert_eq!(models.openai, Some("gpt-4o-mini".to_string()));
        assert_eq!(models.gemini, Some("gemini-1.5-flash-latest".to_string()));
        assert_eq!(
            models.anthropic,
            Some("claude-3-haiku-20240307".to_string())
        );
    }

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert!(config.default_provider.is_none());
        assert!(config.api_keys.openai.is_none());
        assert!(config.api_keys.gemini.is_none());
        assert!(config.api_keys.anthropic.is_none());
        assert!(config.models.openai.is_some());
        assert!(config.models.gemini.is_some());
        assert!(config.models.anthropic.is_some());
        assert!(config.aliases.is_empty());
    }

    #[test]
    fn test_config_serialization() {
        let mut config = Config::default();
        config.default_provider = Some("openai".to_string());
        config.api_keys.openai = Some("sk-test".to_string());
        config.models.openai = Some("gpt-4".to_string());
        config
            .aliases
            .insert("fast".to_string(), "gpt-4o-mini".to_string());
        let toml = toml::to_string(&config).unwrap();
        let deserialized: Config = toml::from_str(&toml).unwrap();
        assert_eq!(deserialized.default_provider, Some("openai".to_string()));
        assert_eq!(deserialized.api_keys.openai, Some("sk-test".to_string()));
        assert_eq!(deserialized.models.openai, Some("gpt-4".to_string()));
        assert_eq!(
            deserialized.aliases.get("fast"),
            Some(&"gpt-4o-mini".to_string())
        );
    }

    #[test]
    fn test_api_keys_default() {
        let keys = ApiKeys::default();
        assert!(keys.openai.is_none());
        assert!(keys.gemini.is_none());
        assert!(keys.anthropic.is_none());
    }
}
