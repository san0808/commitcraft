# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Interactive command editing as default UX
- Shell completion scripts for bash, zsh, fish
- GitHub Actions CI/CD pipeline
- Automated binary releases for multiple platforms
- Enhanced error handling and user guidance
- Installation script for one-line setup
- Code coverage reporting
- Security audit in CI

### Changed
- Replaced clipboard approach with interactive command editing
- Enhanced README with installation badges and better documentation
- Improved configuration display with emojis and better formatting
- Updated model recommendations and aliases

### Fixed
- Windows shell command execution compatibility
- Cross-platform path handling in installation

## [1.0.0] - 2024-01-XX

### Added
- Multi-AI provider support (OpenAI, Google Gemini, Anthropic Claude)
- Conventional commits compliance and validation
- Interactive setup and configuration management
- Multiple execution modes (dry-run, review, force, verbose)
- Context-aware commit message generation
- Model aliases for quick switching
- Comprehensive error handling and fallbacks
- Git repository detection and operations
- TOML configuration file management
- Command-line interface with rich options

### Features
- **Providers**: OpenAI GPT-4o, Google Gemini 1.5, Anthropic Claude 3
- **Modes**: Interactive, legacy, dry-run, show-command
- **Configuration**: TOML-based with API key management
- **Git Integration**: Staged diff analysis and commit execution
- **UX**: Colored output, spinners, progress indicators

### Documentation
- Comprehensive README with examples
- Setup and configuration guides
- Troubleshooting section
- Contributing guidelines
- License and project structure

## [0.1.0] - Initial Development

### Added
- Basic CLI structure
- Initial provider implementations
- Git operations foundation
- Configuration management prototype 