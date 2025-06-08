#!/bin/bash
set -e

# CommitCraft Installation Script
# Usage: curl -sSL https://raw.githubusercontent.com/sanket08/commitcraft/main/install.sh | bash

REPO="sanket08/commitcraft"
BINARY_NAME="commitcraft"

# Detect OS and architecture
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case $ARCH in
    x86_64)
        ARCH="x86_64"
        ;;
    arm64|aarch64)
        ARCH="aarch64"
        ;;
    *)
        echo "❌ Unsupported architecture: $ARCH"
        exit 1
        ;;
esac

case $OS in
    linux)
        TARGET="$ARCH-unknown-linux-gnu"
        ;;
    darwin)
        TARGET="$ARCH-apple-darwin"
        ;;
    *)
        echo "❌ Unsupported OS: $OS"
        exit 1
        ;;
esac

echo "🚀 Installing CommitCraft for $OS-$ARCH..."

# Get latest release
LATEST_RELEASE=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" | grep -o '"tag_name": "[^"]*' | cut -d'"' -f4)
if [ -z "$LATEST_RELEASE" ]; then
    echo "❌ Failed to get latest release"
    exit 1
fi

DOWNLOAD_URL="https://github.com/$REPO/releases/download/$LATEST_RELEASE/${BINARY_NAME}-${TARGET}.tar.gz"

# Create temp directory
TMP_DIR=$(mktemp -d)
cd "$TMP_DIR"

echo "📦 Downloading $DOWNLOAD_URL..."
if ! curl -sSL "$DOWNLOAD_URL" -o "${BINARY_NAME}.tar.gz"; then
    echo "❌ Download failed. Trying cargo install fallback..."
    if command -v cargo >/dev/null 2>&1; then
        echo "🦀 Installing via cargo..."
        cargo install commitcraft
        echo "✅ CommitCraft installed successfully via cargo!"
        echo "🎯 Run 'commitcraft setup' to get started"
        exit 0
    else
        echo "❌ cargo not found. Please install Rust or download manually from:"
        echo "   https://github.com/$REPO/releases"
        exit 1
    fi
fi

# Extract and install
tar -xzf "${BINARY_NAME}.tar.gz"
chmod +x "$BINARY_NAME"

# Install to user bin directory
INSTALL_DIR="$HOME/.local/bin"
mkdir -p "$INSTALL_DIR"
mv "$BINARY_NAME" "$INSTALL_DIR/"

# Add to PATH if not already there
SHELL_RC=""
case "$SHELL" in
    */bash)
        SHELL_RC="$HOME/.bashrc"
        ;;
    */zsh)
        SHELL_RC="$HOME/.zshrc"
        ;;
    */fish)
        SHELL_RC="$HOME/.config/fish/config.fish"
        ;;
esac

if [ -n "$SHELL_RC" ] && [ -f "$SHELL_RC" ]; then
    if ! grep -q "$INSTALL_DIR" "$SHELL_RC"; then
        echo "export PATH=\"$INSTALL_DIR:\$PATH\"" >> "$SHELL_RC"
        echo "📝 Added $INSTALL_DIR to PATH in $SHELL_RC"
    fi
fi

# Cleanup
rm -rf "$TMP_DIR"

echo "✅ CommitCraft installed successfully to $INSTALL_DIR/$BINARY_NAME"
echo ""
echo "🎯 To get started:"
echo "   1. Restart your terminal or run: export PATH=\"$INSTALL_DIR:\$PATH\""
echo "   2. Run: commitcraft setup"
echo "   3. In any git repo: git add . && commitcraft"
echo ""
echo "📚 More info: https://github.com/$REPO" 