# Contributing to CommitCraft 🤝

Thank you for your interest in contributing to CommitCraft! This guide will help you get started.

## 🚀 Quick Start

1. **Fork the repository**
   ```bash
   git clone https://github.com/your-username/commitcraft.git
   cd commitcraft
   ```

2. **Set up development environment**
   ```bash
   # Install Rust (if not already installed)
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   
   # Build the project
   cargo build
   
   # Run tests
   cargo test
   ```

3. **Create a feature branch**
   ```bash
   git checkout -b feature/your-amazing-feature
   ```

## 🎯 How to Contribute

### 🐛 Bug Reports
- Use the issue tracker to report bugs
- Include steps to reproduce, expected vs actual behavior
- Provide system info (OS, Rust version, CommitCraft version)

### ✨ Feature Requests
- Check existing issues to avoid duplicates
- Clearly describe the use case and proposed solution
- Consider starting a discussion for major features

### 🔧 Code Contributions

#### Development Setup
```bash
# Test your changes
cargo test

# Test with real git repos
git add some-file.rs
cargo run -- --dry-run

# Test different providers
cargo run -- --provider openai --dry-run
cargo run -- setup
```

#### Code Style
- Run `cargo fmt` before committing
- Run `cargo clippy` to catch common issues
- Follow existing patterns and conventions
- Add tests for new functionality

#### Project Structure
```
src/
├── main.rs           # Application entry point
├── cli.rs            # Command-line interface
├── config.rs         # Configuration management
├── git.rs            # Git operations
└── providers/        # AI provider implementations
    ├── mod.rs        # Common traits
    ├── openai.rs     # OpenAI integration
    ├── gemini.rs     # Google Gemini integration
    └── anthropic.rs  # Anthropic Claude integration
```

## 🎨 Areas That Need Help

### 🔧 **High Priority**
- [ ] **Windows support testing and fixes**
- [ ] **Performance optimizations for large diffs**
- [ ] **Better error messages and user guidance**
- [ ] **Shell completion scripts (bash, zsh, fish)**

### 🚀 **New Features**
- [ ] **Additional AI providers** (Cohere, Llama, etc.)
- [ ] **Custom prompt templates**
- [ ] **Commit message templates and conventions**
- [ ] **Integration with git hooks**
- [ ] **Diff filtering and preprocessing**

### 📚 **Documentation**
- [ ] **Video tutorials and demos**
- [ ] **Provider-specific setup guides**
- [ ] **Advanced usage examples**
- [ ] **Troubleshooting guides**

### 🧪 **Testing**
- [ ] **Integration tests with real git repos**
- [ ] **Cross-platform testing**
- [ ] **Performance benchmarks**
- [ ] **AI provider response testing**

## 📝 Commit Message Guidelines

We use our own tool! But follow these conventions:
- `feat: add new feature`
- `fix: resolve issue with X`
- `docs: update README`
- `test: add tests for Y`
- `chore: update dependencies`

## 🔄 Pull Request Process

1. **Create descriptive PR title and description**
2. **Link to related issues**
3. **Ensure all tests pass**
4. **Update documentation if needed**
5. **Keep PRs focused and atomic**

### PR Checklist
- [ ] Tests pass (`cargo test`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] No clippy warnings (`cargo clippy`)
- [ ] Documentation updated if needed
- [ ] CHANGELOG.md updated for significant changes

## 🎓 Getting Help

- **Discord**: [Join our community](https://discord.gg/commitcraft)
- **Discussions**: Use GitHub Discussions for questions
- **Issues**: Check existing issues or create new ones

## 🏅 Recognition

Contributors will be:
- Listed in the README contributors section
- Mentioned in release notes for significant contributions
- Invited to join the core maintainer team for sustained contributions

## 📋 Development Workflow

```bash
# 1. Create feature branch
git checkout -b feature/awesome-feature

# 2. Make changes and test
cargo test
cargo run -- --dry-run

# 3. Commit using commitcraft itself!
git add .
cargo run

# 4. Push and create PR
git push origin feature/awesome-feature
```

## 🤔 Questions?

Don't hesitate to ask! We're here to help new contributors succeed.

---

**Happy coding! 🦀✨** 