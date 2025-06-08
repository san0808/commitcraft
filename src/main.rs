use clap::Parser;
use colored::*;
use spinners::{Spinner, Spinners};
use question::{Question, Answer};

mod cli;
mod config;
mod git;
mod providers;

use cli::{Cli, Commands};
use providers::{
    AIProvider,
    openai::OpenAIProvider,
    gemini::GeminiProvider,
    anthropic::AnthropicProvider
};

#[tokio::main]
async fn main() {
    // Initialize logger
    env_logger::init();
    
    let cli_args = Cli::parse();

    // Handle commands
    match cli_args.command {
        Some(Commands::Setup) => {
            if let Err(e) = config::run_setup() {
                eprintln!("{} {}", "Error during setup:".red().bold(), e);
                std::process::exit(1);
            }
            return;
        },
        Some(Commands::Config) => {
            show_config();
            return;
        },
        Some(Commands::List) => {
            list_providers_and_models();
            return;
        },
        None => {}
    }

    // Load configuration
    let config = match config::load_config() {
        Ok(c) => c,
        Err(_) => {
            // Error message is already printed by load_config
            std::process::exit(1);
        }
    };

    // Check if in a git repository
    if !git::is_git_repository() {
        eprintln!("{}", "Error: Not inside a git repository.".red().bold());
        std::process::exit(1);
    }
    
    // Get staged diff
    let diff = match git::get_staged_diff() {
        Ok(d) => d,
        Err(e) => {
            eprintln!("{} {}", "Error:".red().bold(), e);
            std::process::exit(1);
        }
    };

    // Show verbose output if requested
    if cli_args.verbose {
        println!("{}", "Analyzing the following diff:".bold());
        println!("{}", "─".repeat(50));
        println!("{}", diff.dimmed());
        println!("{}", "─".repeat(50));
    }

    // Get file context if requested
    let file_context = if cli_args.include_files {
        match git::get_staged_files() {
            Ok(files) => Some(format!("Files modified: {}", files.join(", "))),
            Err(_) => None,
        }
    } else {
        None
    };

    // Get repository context
    let repo_context = match git::get_repo_info() {
        Ok((repo_name, branch)) => Some(format!("Repository: {} (branch: {})", repo_name, branch)),
        Err(_) => None,
    };

    // Enhance diff with context
    let enhanced_diff = if file_context.is_some() || repo_context.is_some() {
        let mut context = Vec::new();
        if let Some(repo) = repo_context {
            context.push(repo);
        }
        if let Some(files) = file_context {
            context.push(files);
        }
        format!("{}\n\n{}", context.join("\n"), diff)
    } else {
        diff
    };

    // Determine provider and model
    let provider_name = cli_args.provider.or(config.default_provider)
        .unwrap_or_else(|| "gemini".to_string());

    let model_name_or_alias = cli_args.model.unwrap_or_else(|| {
        match provider_name.as_str() {
            "openai" => config.models.openai,
            "gemini" => config.models.gemini,
            "anthropic" => config.models.anthropic,
            _ => None,
        }
        .unwrap_or_else(|| "default".to_string())
    });

    let model_name = config.aliases
        .get(&model_name_or_alias)
        .unwrap_or(&model_name_or_alias);

    // Get API key for the selected provider
    let api_key = match provider_name.as_str() {
        "openai" => config.api_keys.openai,
        "gemini" => config.api_keys.gemini,
        "anthropic" => config.api_keys.anthropic,
        _ => None,
    }
    .ok_or_else(|| {
        format!(
            "API key for provider '{}' not found. Please run 'commitcraft setup'.",
            provider_name
        )
    })
    .unwrap_or_else(|e| {
        eprintln!("{} {}", "Configuration Error:".red().bold(), e);
        std::process::exit(1);
    });

    // Instantiate the provider
    let provider: Box<dyn AIProvider> = match provider_name.as_str() {
        "openai" => Box::new(OpenAIProvider::new(api_key, model_name.to_string())),
        "gemini" => Box::new(GeminiProvider::new(api_key, model_name.to_string())),
        "anthropic" => Box::new(AnthropicProvider::new(api_key, model_name.to_string())),
        _ => {
            eprintln!("{} Unknown provider '{}'", "Error:".red().bold(), provider_name);
            std::process::exit(1);
        }
    };
    
    println!(
        "Using provider: {} ({})",
        provider_name.cyan(),
        model_name.cyan()
    );

    let mut sp = Spinner::new(Spinners::Dots, "Generating commit message...".into());

    let commit_msg = match provider.generate_commit_message(&enhanced_diff).await {
        Ok(msg) => {
            sp.stop_with_message("✓ Message generated successfully!".into());
            
            // Validate the generated commit message
            if let Err(validation_error) = msg.validate() {
                eprintln!("{} {}", "Warning:".yellow().bold(), validation_error);
                eprintln!("The generated message may not follow conventional commits format exactly.");
            }
            
            msg
        },
        Err(e) => {
            sp.stop_with_message("✗ Error generating message.".into());
            eprintln!("{} {}", "API Error:".red().bold(), e);
            std::process::exit(1);
        }
    };

    let commit_str = commit_msg.to_string();

    if cli_args.dry_run {
        println!("\n{}\n---\n{}\n---", "Generated Commit Message:".bold(), commit_str.green());
        return;
    }

    println!("\n{}\n---\n{}\n---", "Proposed Commit:".bold(), commit_str.green());
    
    if !cli_args.force {
        let answer = Question::new("Do you want to commit with this message? (Y/n)")
            .yes_no()
            .default(Answer::YES)
            .ask()
            .expect("Couldn't ask question.");

        if answer == Answer::NO {
            println!("{}", "Commit aborted by user.".yellow());
            std::process::exit(0);
        }
    }

    if let Err(e) = git::commit(&commit_str, cli_args.review) {
        eprintln!("{} {}", "Error during commit:".red().bold(), e);
        std::process::exit(1);
    }
}

fn show_config() {
    match config::load_config() {
        Ok(config) => {
            println!("{}", "Current Configuration:".bold().green());
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            
            // Default provider
            match &config.default_provider {
                Some(provider) => println!("Default Provider: {}", provider.cyan()),
                None => println!("Default Provider: {}", "Not set".yellow()),
            }
            
            // API Keys (masked)
            println!("\n{}:", "API Keys".bold());
            println!("  OpenAI: {}", if config.api_keys.openai.is_some() { "✓ Configured".green() } else { "✗ Not set".red() });
            println!("  Gemini: {}", if config.api_keys.gemini.is_some() { "✓ Configured".green() } else { "✗ Not set".red() });
            println!("  Anthropic: {}", if config.api_keys.anthropic.is_some() { "✓ Configured".green() } else { "✗ Not set".red() });
            
            // Default Models
            println!("\n{}:", "Default Models".bold());
            if let Some(model) = &config.models.openai {
                println!("  OpenAI: {}", model.cyan());
            }
            if let Some(model) = &config.models.gemini {
                println!("  Gemini: {}", model.cyan());
            }
            if let Some(model) = &config.models.anthropic {
                println!("  Anthropic: {}", model.cyan());
            }
            
            // Aliases
            if !config.aliases.is_empty() {
                println!("\n{}:", "Model Aliases".bold());
                for (alias, model) in &config.aliases {
                    println!("  {} → {}", alias.yellow(), model.cyan());
                }
            }
        },
        Err(e) => {
            eprintln!("{} {}", "Error loading configuration:".red().bold(), e);
            std::process::exit(1);
        }
    }
}

fn list_providers_and_models() {
    println!("{}", "Available Providers and Models:".bold().green());
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    println!("\n{}", "OpenAI:".bold().blue());
    println!("  • gpt-4o (latest GPT-4 Omni)");
    println!("  • gpt-4o-mini (faster, cheaper GPT-4 Omni)");
    println!("  • gpt-4-turbo (previous generation)");
    
    println!("\n{}", "Google Gemini:".bold().blue());
    println!("  • gemini-1.5-pro-latest (most capable)");
    println!("  • gemini-1.5-flash-latest (fast and efficient)");
    println!("  • gemini-1.0-pro (stable version)");
    
    println!("\n{}", "Anthropic Claude:".bold().blue());
    println!("  • claude-3-5-sonnet-latest (most capable)");
    println!("  • claude-3-haiku-20240307 (fastest)");
    println!("  • claude-3-opus-20240229 (most powerful)");
    
    println!("\n{}", "Usage Examples:".bold().yellow());
    println!("  commitcraft --provider openai --model gpt-4o");
    println!("  commitcraft --provider gemini --model gemini-1.5-flash-latest");
    println!("  commitcraft --provider anthropic --model claude-3-haiku-20240307");
    
    if let Ok(config) = config::load_config() {
        if !config.aliases.is_empty() {
            println!("\n{}", "Your Configured Aliases:".bold().yellow());
            for (alias, model) in &config.aliases {
                println!("  commitcraft --model {} (→ {})", alias.green(), model.cyan());
            }
        }
    }
}
