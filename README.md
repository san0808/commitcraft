# 🎨 CommitCraft

A fast, intelligent CLI tool that generates conventional commit messages using AI. Built in Rust for performance and reliability.

## ✨ Features

- **🤖 Multi-AI Provider Support**: Works with OpenAI, Google Gemini, and Anthropic Claude
- **📋 Conventional Commits**: Follows [Conventional Commits v1.0.0](https://www.conventionalcommits.org/en/v1.0.0/) specification  
- **⚙️ Smart Configuration**: Interactive setup with TOML configuration file
- **🎯 Context-Aware**: Analyzes staged files and repository context
- **🔧 Developer-Friendly**: Built for fast iteration with dry-run, review, and verbose modes
- **📱 Model Aliases**: Quick switching between models with custom aliases

## 🚀 Quick Start

### Installation

#### Option 1: Build from Source
```bash
git clone https://github.com/your-username/commitcraft
cd commitcraft
cargo build --release
sudo cp target/release/commitcraft /usr/local/bin/commitcraft
```

#### Option 2: Install via Cargo
```bash
cargo install commitcraft
```

### Setup
```bash
# Run interactive setup
commitcraft setup

# Check your configuration
commitcraft config

# List available providers and models
commitcraft list
```

## 📖 Usage

### Basic Usage
```bash
# Stage your changes
git add .

# Generate and commit
commitcraft

# Dry run (generate message without committing)
commitcraft --dry-run

# Review message in editor before committing
commitcraft --review

# Skip confirmation prompt
commitcraft --force
```

### Advanced Usage
```bash
# Use specific provider and model
commitcraft --provider openai --model gpt-4o

# Use model alias
commitcraft --model smart

# Include verbose output and file context
commitcraft --verbose --include-files

# Combine options
commitcraft --provider anthropic --model fast --dry-run --verbose
```

## 🛠️ Configuration

Configuration is stored at `~/.config/commitcraft/config.toml`:

```toml
default_provider = "gemini"

[api_keys]
openai = "sk-..."
gemini = "your-api-key"
anthropic = "your-api-key"

[models]
openai = "gpt-4o-mini"
gemini = "gemini-1.5-flash-latest"
anthropic = "claude-3-haiku-20240307"

[aliases]
fast = "gemini-1.5-flash-latest"
smart = "gpt-4o"
```

## 🎯 Conventional Commits

This tool generates commit messages following the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) specification:

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

### Supported Types
- `feat`: New features
- `fix`: Bug fixes  
- `docs`: Documentation changes
- `style`: Code formatting
- `refactor`: Code restructuring
- `test`: Adding tests
- `chore`: Maintenance tasks
- `perf`: Performance improvements
- `ci`: CI/CD changes
- `build`: Build system changes

### Examples
```
feat(auth): add OAuth2 login support

Implement OAuth2 authentication flow with Google and GitHub providers.
Includes token refresh logic and secure storage.

feat!: remove deprecated API endpoints

BREAKING CHANGE: The v1 API endpoints have been removed. Use v2 instead.
```

## 🏗️ Project Structure

```
src/
├── main.rs           # Main application entry point
├── cli.rs            # Command-line interface definitions
├── config.rs         # Configuration management
├── git.rs            # Git operations (diff, commit, repo info)
└── providers/        # AI provider implementations
    ├── mod.rs        # Common traits and structures
    ├── openai.rs     # OpenAI GPT integration
    ├── gemini.rs     # Google Gemini integration
    └── anthropic.rs  # Anthropic Claude integration
```

## 🧪 Development

### Building
```bash
cargo build --release
```

### Testing
```bash
cargo test
```

### Manual Testing Checklist

1. **Setup Flow**
   ```bash
   cargo run -- setup
   ```

2. **Basic Generation**
   ```bash
   git add some-file.rs
   cargo run -- --dry-run
   ```

3. **Provider Testing**
   ```bash
   cargo run -- --provider openai --dry-run
   cargo run -- --provider gemini --dry-run
   cargo run -- --provider anthropic --dry-run
   ```

4. **Advanced Features**
   ```bash
   cargo run -- --verbose --include-files --dry-run
   cargo run -- config
   cargo run -- list
   ```

## 🔧 Troubleshooting

### Common Issues

**"Not inside a git repository"**
- Make sure you're in a git repository
- Run `git init` if needed

**"No staged files to commit"**
- Stage your changes with `git add .` or `git add <file>`

**"API key not found"**
- Run `commitcraft setup` to configure API keys
- Check your configuration with `commitcraft config`

**Build errors**
- Ensure you have Rust 1.70+ installed
- Run `rustup update` to update your toolchain

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## 📄 License

MIT License - see LICENSE file for details.

## 🙏 Acknowledgments

- [Conventional Commits](https://www.conventionalcommits.org/) specification
- Rust community for excellent crates
- AI providers (OpenAI, Google, Anthropic) for powerful models