use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// The AI provider to use (e.g., "gemini", "openai", "anthropic"). Overrides config default.
    #[arg(short, long)]
    pub provider: Option<String>,

    /// The specific model or alias to use (e.g., "fast", "gpt-4o"). Overrides config default.
    #[arg(short, long)]
    pub model: Option<String>,

    /// Generate and output the commit message without committing.
    #[arg(long)]
    pub dry_run: bool,

    /// Review the generated commit message in your editor before committing.
    #[arg(short, long)]
    pub review: bool,

    /// Commit without asking for confirmation.
    #[arg(short, long)]
    pub force: bool,

    /// Show verbose output including the diff being analyzed.
    #[arg(short, long)]
    pub verbose: bool,

    /// Include file names in the commit message context.
    #[arg(long)]
    pub include_files: bool,

    /// Show the git command without interactive editing (just print it).
    #[arg(short = 's', long)]
    pub show_command: bool,

    /// Use legacy commit flow (auto-commit with confirmation).
    #[arg(long)]
    pub legacy: bool,

    /// Skip interactive command editing and commit immediately.
    #[arg(short = 'y', long)]
    pub yes: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Run the interactive first-time setup to configure the tool.
    Setup,
    /// List all configured providers and models.
    List,
    /// Show current configuration.
    Config,
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn test_cli_parse_provider_and_model() {
        let args = vec!["prog", "--provider", "openai", "--model", "gpt-4o"];
        let cli = Cli::parse_from(args);
        assert_eq!(cli.provider, Some("openai".to_string()));
        assert_eq!(cli.model, Some("gpt-4o".to_string()));
    }

    #[test]
    fn test_cli_parse_dry_run_and_force() {
        let args = vec!["prog", "--dry-run", "--force"];
        let cli = Cli::parse_from(args);
        assert!(cli.dry_run);
        assert!(cli.force);
    }

    #[test]
    fn test_cli_parse_setup_command() {
        let args = vec!["prog", "setup"];
        let cli = Cli::parse_from(args);
        match cli.command {
            Some(Commands::Setup) => {}
            _ => panic!("Expected Setup command"),
        }
    }
}
