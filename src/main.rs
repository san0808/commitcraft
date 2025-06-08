use clap::Parser;
use colored::*;
use question::{Answer, Question};
use rustyline::DefaultEditor;
use spinners::{Spinner, Spinners};
use std::process::Command;

mod cli;
mod config;
mod git;
mod providers;

use cli::{Cli, Commands};
use providers::{
    anthropic::AnthropicProvider, gemini::GeminiProvider, openai::OpenAIProvider, AIProvider,
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
        }
        Some(Commands::Config) => {
            show_config();
            return;
        }
        Some(Commands::List) => {
            list_providers_and_models();
            return;
        }
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
    let provider_name = cli_args
        .provider
        .or(config.default_provider)
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

    let model_name = config
        .aliases
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
            eprintln!(
                "{} Unknown provider '{}'",
                "Error:".red().bold(),
                provider_name
            );
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
                eprintln!(
                    "The generated message may not follow conventional commits format exactly."
                );
            }

            msg
        }
        Err(e) => {
            sp.stop_with_message("✗ Error generating message.".into());
            eprintln!("{} {}", "API Error:".red().bold(), e);
            std::process::exit(1);
        }
    };

    let commit_str = commit_msg.to_string();

    // Handle different modes
    if cli_args.dry_run {
        println!(
            "\n{}\n---\n{}\n---",
            "Generated Commit Message:".bold(),
            commit_str.green()
        );
        return;
    }

    if cli_args.show_command {
        let git_command = format_git_command(&commit_str, cli_args.review);
        println!("\n{}", "Generated git command:".bold());
        println!("{}", git_command.cyan());
        return;
    }

    if cli_args.legacy {
        // Use the old confirmation-based flow
        legacy_commit_flow(&commit_str, cli_args.force, cli_args.review);
        return;
    }

    if cli_args.yes {
        // Skip interactive editing, commit immediately
        if let Err(e) = execute_git_commit(&commit_str, cli_args.review) {
            eprintln!("{} {}", "Error during commit:".red().bold(), e);
            std::process::exit(1);
        }
        return;
    }

    // Default: Interactive command editing
    interactive_commit_flow(&commit_str, cli_args.review);
}

/// Format the git commit command with proper escaping
fn format_git_command(message: &str, review: bool) -> String {
    let review_flag = if review { " -e" } else { "" };

    // For multi-line messages, we need to handle them properly
    if message.contains('\n') {
        // Use heredoc-style for multi-line messages
        format!("git commit{} -F- <<'EOF'\n{}\nEOF", review_flag, message)
    } else {
        // Simple single-line message
        format!(
            "git commit{} -m \"{}\"",
            review_flag,
            message.replace('"', "\\\"")
        )
    }
}

/// Interactive command editing flow (new default)
fn interactive_commit_flow(commit_message: &str, review: bool) {
    println!("\n{}", "📝 Generated commit message:".bold());
    println!("{}", "─".repeat(50));
    println!("{}", commit_message.green());
    println!("{}", "─".repeat(50));

    let git_command = if commit_message.contains('\n') {
        // For multi-line, show a simplified version for editing
        let title = commit_message.lines().next().unwrap_or(commit_message);
        format!(
            "git commit{} -m \"{}\"",
            if review { " -e" } else { "" },
            title
        )
    } else {
        format_git_command(commit_message, review)
    };

    println!(
        "\n{}",
        "Edit the command below (or press Enter to execute):".bold()
    );

    let mut rl = match DefaultEditor::new() {
        Ok(editor) => editor,
        Err(e) => {
            eprintln!(
                "{} Failed to create interactive editor: {}",
                "Error:".red().bold(),
                e
            );
            println!("Falling back to legacy mode...");
            legacy_commit_flow(commit_message, false, review);
            return;
        }
    };

    match rl.readline_with_initial("$ ", (&git_command, "")) {
        Ok(edited_command) => {
            let edited_command = edited_command.trim();
            if edited_command.is_empty() {
                println!("{}", "Commit cancelled.".yellow());
                return;
            }

            // Execute the edited command
            println!("\n{} {}", "Executing:".bold(), edited_command.cyan());
            match execute_shell_command(edited_command) {
                Ok(output) => {
                    println!("{}", "✓ Commit successful!".green().bold());
                    if !output.trim().is_empty() {
                        println!("{}", output);
                    }
                }
                Err(e) => {
                    eprintln!("{} {}", "Error:".red().bold(), e);
                    std::process::exit(1);
                }
            }
        }
        Err(rustyline::error::ReadlineError::Interrupted) => {
            println!("{}", "Commit cancelled.".yellow());
        }
        Err(e) => {
            eprintln!("{} Failed to read input: {}", "Error:".red().bold(), e);
            std::process::exit(1);
        }
    }
}

/// Execute a shell command and return output
fn execute_shell_command(command: &str) -> Result<String, String> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .map_err(|e| format!("Failed to execute command: {}", e))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

/// Legacy commit flow (old behavior)
fn legacy_commit_flow(commit_message: &str, force: bool, review: bool) {
    println!(
        "\n{}\n---\n{}\n---",
        "Proposed Commit:".bold(),
        commit_message.green()
    );

    if !force {
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

    if let Err(e) = execute_git_commit(commit_message, review) {
        eprintln!("{} {}", "Error during commit:".red().bold(), e);
        std::process::exit(1);
    }
}

/// Execute git commit with the given message
fn execute_git_commit(message: &str, review: bool) -> Result<(), String> {
    git::commit(message, review)
}

fn show_config() {
    match config::load_config() {
        Ok(config) => {
            println!("{}", "📋 Current Configuration".bold().cyan());
            println!("{}", "─".repeat(50));

            // Provider info
            if let Some(provider) = &config.default_provider {
                println!("🤖 Default Provider: {}", provider.green());
            } else {
                println!("🤖 Default Provider: {}", "Not set".yellow());
            }

            // API keys (masked)
            println!("\n🔑 API Keys:");
            println!(
                "  OpenAI:    {}",
                if config.api_keys.openai.is_some() {
                    "✓ Configured".green()
                } else {
                    "✗ Not set".red()
                }
            );
            println!(
                "  Gemini:    {}",
                if config.api_keys.gemini.is_some() {
                    "✓ Configured".green()
                } else {
                    "✗ Not set".red()
                }
            );
            println!(
                "  Anthropic: {}",
                if config.api_keys.anthropic.is_some() {
                    "✓ Configured".green()
                } else {
                    "✗ Not set".red()
                }
            );

            // Models
            println!("\n🎯 Default Models:");
            if let Some(model) = &config.models.openai {
                println!("  OpenAI:    {}", model.cyan());
            }
            if let Some(model) = &config.models.gemini {
                println!("  Gemini:    {}", model.cyan());
            }
            if let Some(model) = &config.models.anthropic {
                println!("  Anthropic: {}", model.cyan());
            }

            // Aliases
            if !config.aliases.is_empty() {
                println!("\n🏷️  Model Aliases:");
                for (alias, model) in &config.aliases {
                    println!("  {} → {}", alias.yellow(), model.cyan());
                }
            }

            println!("\n💡 Run '{}' to reconfigure", "commitcraft setup".bold());
        }
        Err(e) => {
            eprintln!("{} {}", "Error loading config:".red().bold(), e);
            println!(
                "Run '{}' to set up configuration.",
                "commitcraft setup".bold().cyan()
            );
        }
    }
}

fn list_providers_and_models() {
    println!("{}", "🤖 Available Providers & Models".bold().cyan());
    println!("{}", "─".repeat(50));

    println!("\n{}:", "OpenAI".bold().green());
    println!("  • gpt-4o (latest, most capable)");
    println!("  • gpt-4o-mini (fast and efficient)");
    println!("  • gpt-4-turbo");
    println!("  • gpt-3.5-turbo");

    println!("\n{}:", "Google Gemini".bold().blue());
    println!("  • gemini-1.5-pro-latest (most capable)");
    println!("  • gemini-1.5-flash-latest (fast, default)");
    println!("  • gemini-1.0-pro");

    println!("\n{}:", "Anthropic Claude".bold().purple());
    println!("  • claude-3-5-sonnet-20241022 (latest, most capable)");
    println!("  • claude-3-haiku-20240307 (fast, default)");
    println!("  • claude-3-opus-20240229 (most powerful)");

    println!("\n{}:", "Usage Examples".bold().yellow());
    println!("  commitcraft --provider openai --model gpt-4o");
    println!("  commitcraft --provider gemini --model gemini-1.5-pro-latest");
    println!("  commitcraft --provider anthropic --model claude-3-5-sonnet-20241022");

    println!("\n💡 Set up aliases with '{}'", "commitcraft setup".bold());
}
