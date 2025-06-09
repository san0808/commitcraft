#!/bin/bash

# CommitCraft Enhanced Quick Demo Script  
# Professional 3-minute demo with typing effects

set -e

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
WHITE='\033[1;37m'
RED='\033[0;31m'
NC='\033[0m'

# Demo settings
DEMO_DIR="/tmp/commitcraft-quick-demo"
TYPING_SPEED=0.02        # Quick but visible typing
PAUSE_SHORT=1.0          # Quick pauses
PAUSE_MEDIUM=1.5         # Medium pause
REVEAL_PAUSE=2.0         # Time to read important content

# Enhanced utility functions
typewriter() {
    local text="$1"
    local speed="${2:-$TYPING_SPEED}"
    
    for (( i=0; i<${#text}; i++ )); do
        echo -n "${text:$i:1}"
        sleep "$speed"
    done
    echo
}

typewriter_colored() {
    local color="$1"
    local text="$2"
    local speed="${3:-$TYPING_SPEED}"
    
    echo -n -e "$color"
    for (( i=0; i<${#text}; i++ )); do
        echo -n "${text:$i:1}"
        sleep "$speed"
    done
    echo -e "$NC"
}

loading_spinner() {
    local message="$1"
    local duration="${2:-2}"
    
    echo -n -e "${CYAN}$message${NC}"
    
    for i in $(seq 1 $((duration * 10))); do
        for spinner in '⠋' '⠙' '⠹' '⠸' '⠼' '⠴' '⠦' '⠧' '⠇' '⠏'; do
            echo -n -e "\b$spinner"
            sleep 0.1
        done
    done
    echo -e "\b✓"
}

pause() {
    sleep "${1:-$PAUSE_MEDIUM}"
}

type_command() {
    echo -n -e "${GREEN}$ ${NC}"
    typewriter "$1" 0.03
    pause 0.5
}

reveal_text() {
    local text="$1"
    local color="${2:-$WHITE}"
    
    echo -n -e "${color}"
    for word in $text; do
        echo -n "$word "
        sleep 0.2
    done
    echo -e "${NC}"
}

clear_and_title() {
    clear
    echo -e "${PURPLE}"
    echo " ██████╗ ██████╗ ███╗   ███╗███╗   ███╗██╗████████╗ ██████╗██████╗  █████╗ ███████╗████████╗"
    echo "██╔════╝██╔═══██╗████╗ ████║████╗ ████║██║╚══██╔══╝██╔════╝██╔══██╗██╔══██╗██╔════╝╚══██╔══╝"
    echo "██║     ██║   ██║██╔████╔██║██╔████╔██║██║   ██║   ██║     ██████╔╝███████║█████╗     ██║   "
    echo "██║     ██║   ██║██║╚██╔╝██║██║╚██╔╝██║██║   ██║   ██║     ██╔══██╗██╔══██║██╔══╝     ██║   "
    echo "╚██████╗╚██████╔╝██║ ╚═╝ ██║██║ ╚═╝ ██║██║   ██║   ╚██████╗██║  ██║██║  ██║██║        ██║   "
    echo " ╚═════╝ ╚═════╝ ╚═╝     ╚═╝╚═╝     ╚═╝╚═╝   ╚═╝    ╚═════╝╚═╝  ╚═╝╚═╝  ╚═╝╚═╝        ╚═╝   "
    echo -e "${NC}"
    echo -e "${CYAN}🎨 AI-Powered Conventional Commit Messages${NC}"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo
}

demo_intro() {
    clear_and_title
    
    typewriter_colored "$BLUE" "Stop writing boring commit messages!"
    pause $PAUSE_SHORT
    
    typewriter_colored "$BLUE" "Let AI craft perfect conventional commits for you."
    pause $PAUSE_SHORT
    
    reveal_text "✨ Meet CommitCraft - Your AI Assistant! ✨" "$GREEN"
    
    typewriter_colored "$WHITE" "⚡ 10x faster • 📏 Perfect format • 🤖 Multiple AI providers"
    
    pause $REVEAL_PAUSE
}

demo_setup() {
    clear_and_title
    echo -e "${CYAN}🚀 Quick Setup${NC}"
    echo
    
    type_command "cargo install commitcraft"
    loading_spinner "📦 Installing " 2
    echo -e "${GREEN}✅ Installed!${NC}"
    
    pause $PAUSE_SHORT
    
    type_command "commitcraft setup"
    loading_spinner "⚙️  Configuring " 2
    
    typewriter_colored "$BLUE" "Provider: Gemini ✓"
    typewriter_colored "$BLUE" "API Key: Configured ✓"
    
    echo -e "${GREEN}✅ Ready to generate amazing commits!${NC}"
    pause $REVEAL_PAUSE
}

demo_basic_usage() {
    clear_and_title
    echo -e "${CYAN}💡 AI Magic in Action${NC}"
    echo
    
    # Setup demo environment
    rm -rf "$DEMO_DIR"
    mkdir -p "$DEMO_DIR"
    cd "$DEMO_DIR"
    git init -q
    git config user.name "Demo Developer"
    git config user.email "demo@commitcraft.dev"
    
    typewriter_colored "$YELLOW" "Creating a React Button component..."
    pause $PAUSE_SHORT
    
    mkdir -p src/components
    
    cat > src/components/Button.jsx << 'EOF'
import React from 'react';
import './Button.css';

const Button = ({ children, variant = 'primary', onClick, disabled }) => {
  return (
    <button 
      className={`btn btn-${variant}`}
      onClick={onClick}
      disabled={disabled}
    >
      {children}
    </button>
  );
};

export default Button;
EOF

    cat > src/components/Button.css << 'EOF'
.btn {
  padding: 0.75rem 1.5rem;
  border: none;
  border-radius: 0.5rem;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s ease;
}

.btn-primary {
  background: #3b82f6;
  color: white;
}

.btn-primary:hover {
  background: #2563eb;
}

.btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}
EOF
    
    echo -e "${GREEN}📁 Created Button component with styling${NC}"
    pause $PAUSE_SHORT
    
    type_command "git add ."
    git add .
    pause $PAUSE_SHORT
    
    type_command "commitcraft"
    echo
    
    # ACCURATE CommitCraft output
    echo "Using provider: gemini (gemini-1.5-flash-latest)"
    loading_spinner "Generating commit message " 3
    echo "✓ Message generated successfully!"
    
    echo
    echo -e "${CYAN}📝 Generated commit message:${NC}"
    echo "──────────────────────────────────────────────────"
    
    reveal_text "feat(components): add reusable Button component with variant styles"
    echo
    typewriter "Implement Button React component with:" 0.01
    typewriter "- Primary variant with hover effects" 0.01
    typewriter "- Disabled state handling" 0.01
    typewriter "- Responsive CSS styling" 0.01
    typewriter "- TypeScript-ready prop interface" 0.01
    
    echo "──────────────────────────────────────────────────"
    echo
    echo -e "${CYAN}Edit the command below (or press Enter to execute):${NC}"
    echo "$ git commit -m \"feat(components): add reusable Button component with variant styles\""
    
    pause $REVEAL_PAUSE
    echo "Executing: git commit -m \"feat(components): add reusable Button component with variant styles\""
    echo -e "${GREEN}✓ Commit successful!${NC}"
    
    # Actually commit
    git commit -q -m "feat(components): add reusable Button component with variant styles

Implement Button React component with:
- Primary variant with hover effects  
- Disabled state handling
- Responsive CSS styling
- TypeScript-ready prop interface"

    pause $PAUSE_MEDIUM
}

demo_advanced_features() {
    clear_and_title
    echo -e "${BLUE}🚀 Advanced Features${NC}"
    echo
    
    # Create a bug fix
    typewriter_colored "$YELLOW" "Making a bug fix..."
    mkdir -p src/utils
    cat > src/utils/helpers.js << 'EOF'
// Utility functions
export const formatDate = (date) => {
  if (!date) return '';
  
  // Fix: Handle invalid date objects
  const dateObj = new Date(date);
  if (isNaN(dateObj.getTime())) {
    return '';
  }
  
  return dateObj.toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'short', 
    day: 'numeric'
  });
};

export const validateEmail = (email) => {
  const regex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
  return regex.test(email);
};
EOF
    
    git add .
    echo -e "${GREEN}🐛 Fixed date validation bug${NC}"
    pause $PAUSE_SHORT
    
    echo
    typewriter_colored "$PURPLE" "🔍 Dry-run mode (preview only):"
    type_command "commitcraft --dry-run"
    
    echo "Using provider: gemini (gemini-1.5-flash-latest)"
    loading_spinner "Generating commit message " 2
    echo "✓ Message generated successfully!"
    
    echo
    echo "Generated Commit Message:"
    echo "---"
    reveal_text "fix(utils): handle invalid date objects in formatDate function"
    echo
    typewriter "Add null check and NaN validation to prevent crashes when" 0.01
    typewriter "invalid date values are passed to formatDate helper." 0.01
    echo "---"
    pause $PAUSE_MEDIUM
    
    echo
    typewriter_colored "$PURPLE" "🤖 Different AI provider:"
    type_command "commitcraft --provider openai --model gpt-4o"
    echo "Using provider: openai (gpt-4o)"
    echo -e "${GREEN}✅ Same quality, different writing style!${NC}"
    pause $PAUSE_SHORT
    
    # Commit the fix
    git commit -q -m "fix(utils): handle invalid date objects in formatDate function

Add null check and NaN validation to prevent crashes when
invalid date values are passed to formatDate helper."
}

demo_workflow() {
    clear_and_title
    echo -e "${BLUE}🔄 Perfect Git Workflow${NC}"
    echo
    
    typewriter_colored "$GREEN" "Your new workflow:"
    typewriter "1️⃣  Make changes" 0.04
    typewriter "2️⃣  git add ." 0.04
    typewriter "3️⃣  commitcraft  ← ✨ AI magic!" 0.04
    typewriter "4️⃣  git push" 0.04
    echo
    pause $PAUSE_SHORT
    
    typewriter_colored "$BLUE" "📊 Beautiful commit history:"
    type_command "git log --oneline"
    echo
    git log --oneline
    pause $REVEAL_PAUSE
    
    echo
    typewriter_colored "$GREEN" "✨ Benefits:"
    typewriter "• ⚡ 10x faster than writing manually" 0.03
    typewriter "• 📏 Perfect conventional commits format" 0.03
    typewriter "• 🧠 Detailed, contextual messages" 0.03
    typewriter "• 🎯 Consistent team style" 0.03
    pause $REVEAL_PAUSE
}

demo_finale() {
    clear_and_title
    echo -e "${PURPLE}🎉 Ready to Transform Your Commits?${NC}"
    echo
    
    typewriter_colored "$CYAN" "Get Started:"
    typewriter "🚀 cargo install commitcraft" 0.03
    typewriter "⚙️  commitcraft setup" 0.03
    typewriter "✨ Start generating amazing commits!" 0.03
    echo
    
    typewriter_colored "$GREEN" "Resources:"
    typewriter "📖 https://github.com/san0808/commitcraft" 0.02
    typewriter "📚 Full documentation & examples" 0.02
    typewriter "🎯 Works with OpenAI, Gemini & Anthropic" 0.02
    echo
    
    reveal_text "Thank you for watching! Happy committing! 🎨" "$WHITE"
    pause $REVEAL_PAUSE
}

# Cleanup
cleanup() {
    cd /tmp 2>/dev/null || cd /
    rm -rf "$DEMO_DIR" 2>/dev/null || true
}

# Main demo flow
main() {
    echo -e "${BLUE}🎬 Starting CommitCraft Enhanced Demo...${NC}"
    pause 1
    
    demo_intro
    demo_setup  
    demo_basic_usage
    demo_advanced_features
    demo_workflow
    demo_finale
    
    cleanup
    echo -e "${GREEN}🎊 Enhanced demo completed!${NC}"
}

# Handle script interruption
trap cleanup EXIT

# Run the demo
main "$@" 