# 🎨 CommitCraft

[![Crates.io](https://img.shields.io/crates/v/commitcraft.svg)](https://crates.io/crates/commitcraft)
[![Downloads](https://img.shields.io/crates/d/commitcraft.svg)](https://crates.io/crates/commitcraft)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/san0808/commitcraft/workflows/CI%2FCD%20Pipeline/badge.svg)](https://github.com/san0808/commitcraft/actions)

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

#### Option 1: Quick Install (Linux/macOS)
```bash
curl -sSL https://raw.githubusercontent.com/san0808/commitcraft/main/install.sh | bash
```

#### Option 2: Install via Cargo
```bash
cargo install commitcraft
```

#### Option 3: Download Binary
Download the latest binary from [GitHub Releases](https://github.com/san0808/commitcraft/releases)

#### Option 4: Build from Source
```bash
git clone https://github.com/san0808/commitcraft
cd commitcraft
cargo build --release
sudo cp target/release/commitcraft /usr/local/bin/commitcraft
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

### ✨ **New Interactive Mode (Default)**
```bash
# Stage your changes
git add .

# Generate and edit commit command interactively
commitcraft
✓ Message generated successfully!

📝 Generated commit message:
──────────────────────────────────────────────────
feat(auth): add OAuth2 login support

Implement OAuth2 authentication flow with Google and GitHub providers.
Includes token refresh logic and secure storage.
──────────────────────────────────────────────────

Edit the command below (or press Enter to execute):
$ git commit -m "feat(auth): add OAuth2 login support"█
# ↑ You can edit this command or just press Enter to execute
```

### 🚀 **Quick Modes**
```bash
# Skip interactive editing, commit immediately
commitcraft -y

# Just show the git command (no execution)
commitcraft --show-command

# Dry run (generate message without committing)
commitcraft --dry-run

# Use legacy confirmation flow
commitcraft --legacy
```

### 🔧 **Advanced Usage**
```bash
# Use specific provider and model
commitcraft --provider openai --model gpt-4o

# Use model alias
commitcraft --model smart

# Include verbose output and file context
commitcraft --verbose --include-files

# Review in editor before committing
commitcraft --review

# Combine options
commitcraft --provider anthropic --model fast --verbose --yes
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

## 🎬 Demos & Recordings

### Watch CommitCraft in Action

Experience CommitCraft's power through our professional demo videos:

#### 🎯 Quick Demo (3 minutes)
Perfect for social media and quick showcases:
```bash
./demo/quick-demo.sh
```

#### 🎨 Full Cinematic Demo (8-10 minutes)  
Complete feature walkthrough for presentations:
```bash
./demo/cinematic-demo.sh
```

**Automatic Features:**
- ✅ Generates MP4, GIF, WebM formats for any platform  
- ✅ Professional quality with realistic typing effects
- ✅ Upload guides for YouTube, Twitter, LinkedIn, GitHub

See [`demo/README.md`](demo/README.md) for complete documentation.

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