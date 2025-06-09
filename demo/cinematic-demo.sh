#!/bin/bash

# CommitCraft Cinematic Demo Script
# A professional, engaging demo with realistic typing effects and dramatic timing

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
DEMO_DIR="/tmp/commitcraft-cinematic-demo"
TYPING_SPEED=0.03        # Speed for realistic typing
FAST_TYPING=0.01         # Faster typing for long text
PAUSE_SHORT=1.5          # Short dramatic pause
PAUSE_MEDIUM=2.5         # Medium pause for effect
PAUSE_LONG=4             # Long pause for maximum drama
REVEAL_PAUSE=3           # Time to let viewers read important content

# Cinematic utility functions
typewriter() {
    local text="$1"
    local speed="${2:-$TYPING_SPEED}"
    local no_newline="${3:-false}"
    
    for (( i=0; i<${#text}; i++ )); do
        echo -n "${text:$i:1}"
        sleep "$speed"
    done
    
    if [[ "$no_newline" != "true" ]]; then
        echo
    fi
}

typewriter_with_color() {
    local color="$1"
    local text="$2"
    local speed="${3:-$TYPING_SPEED}"
    
    echo -n -e "$color"
    typewriter "$text" "$speed" true
    echo -e "$NC"
}

dramatic_pause() {
    local message="$1"
    local duration="${2:-$PAUSE_MEDIUM}"
    
    if [[ -n "$message" ]]; then
        echo -e "${YELLOW}$message${NC}"
    fi
    sleep "$duration"
}

loading_animation() {
    local message="$1"
    local duration="${2:-3}"
    
    echo -n -e "${CYAN}$message${NC}"
    
    for i in $(seq 1 $duration); do
        for spinner in '⠋' '⠙' '⠹' '⠸' '⠼' '⠴' '⠦' '⠧' '⠇' '⠏'; do
            echo -n -e "\b$spinner"
            sleep 0.1
        done
    done
    echo -e "\b✓"
}

thinking_dots() {
    local message="$1"
    local count="${2:-5}"
    
    echo -n -e "${BLUE}$message${NC}"
    for i in $(seq 1 $count); do
        echo -n "."
        sleep 0.5
    done
    echo
}

reveal_effect() {
    local text="$1"
    local color="${2:-$WHITE}"
    
    # Build up effect
    echo -n -e "${color}"
    for word in $text; do
        echo -n "$word "
        sleep 0.3
    done
    echo -e "${NC}"
}

command_prompt() {
    echo -n -e "${GREEN}$ ${NC}"
}

simulate_user_typing() {
    local command="$1"
    local speed="${2:-$TYPING_SPEED}"
    
    command_prompt
    typewriter "$command" "$speed"
    sleep 0.5  # Pause as if user is thinking
}

clear_with_style() {
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
    echo -e "${WHITE}Fast • Intelligent • Developer-Friendly${NC}"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo
}

# Demo sections with cinematic flair
cinematic_intro() {
    clear_with_style
    
    dramatic_pause "🎬 Lights, camera, action!" $PAUSE_MEDIUM
    
    echo -e "${BLUE}"
    typewriter "Tired of writing boring commit messages?"
    dramatic_pause "" $PAUSE_SHORT
    
    typewriter "Struggling with conventional commits format?"
    dramatic_pause "" $PAUSE_SHORT
    
    typewriter "Spending too much time on git history?"
    dramatic_pause "" $PAUSE_MEDIUM
    
    echo -e "${GREEN}"
    reveal_effect "✨ Meet CommitCraft - Your AI Commit Assistant! ✨"
    
    echo -e "${WHITE}"
    typewriter "🤖 Supports OpenAI, Gemini, and Anthropic"
    typewriter "📏 Perfect Conventional Commits every time"
    typewriter "⚡ 10x faster than manual writing"
    
    dramatic_pause "🚀 Let's see the magic in action..." $PAUSE_LONG
}

cinematic_installation() {
    clear_with_style
    echo -e "${CYAN}📦 GETTING STARTED${NC}"
    echo -e "${WHITE}Multiple ways to join the revolution${NC}"
    echo
    
    dramatic_pause "📊 Option 1: The Rust way" $PAUSE_SHORT
    simulate_user_typing "cargo install commitcraft"
    loading_animation "📦 Installing from crates.io " 3
    echo -e "${GREEN}✅ Installed successfully!${NC}"
    
    dramatic_pause "" $PAUSE_MEDIUM
    
    echo -e "${YELLOW}🚀 Option 2: One-line magic${NC}"
    simulate_user_typing "curl -sSL https://raw.githubusercontent.com/san0808/commitcraft/main/install.sh | bash"
    loading_animation "🌐 Downloading and installing " 3
    echo -e "${GREEN}✅ Ready to use!${NC}"
    
    dramatic_pause "⚡ Installation complete! Now let's configure..." $PAUSE_LONG
}

cinematic_setup() {
    clear_with_style
    echo -e "${CYAN}⚙️  CONFIGURATION${NC}"
    echo -e "${WHITE}Setting up your AI assistant${NC}"
    echo
    
    simulate_user_typing "commitcraft setup"
    loading_animation "🔧 Initializing setup wizard " 2
    echo
    
    dramatic_pause "🤖 Interactive setup starting..." $PAUSE_SHORT
    
    typewriter_with_color "$BLUE" "Which AI provider do you want to use by default?"
    typewriter_with_color "$CYAN" "(gemini, openai, anthropic)"
    dramatic_pause "" $PAUSE_SHORT
    
    simulate_user_typing "gemini" 0.15
    echo -e "${GREEN}✓ Gemini selected${NC}"
    
    dramatic_pause "" $PAUSE_SHORT
    
    typewriter_with_color "$BLUE" "🔑 Enter your Google AI (Gemini) API key:"
    simulate_user_typing "AIzaSyC_demo_key_hidden_for_security" $FAST_TYPING
    echo -e "${GREEN}✓ API key configured${NC}"
    
    dramatic_pause "" $PAUSE_SHORT
    
    typewriter_with_color "$BLUE" "🏷️  Set up model aliases? (fast, smart, etc.)"
    simulate_user_typing "Y"
    
    loading_animation "🎯 Creating smart aliases " 2
    
    echo -e "${GREEN}✅ Configuration complete!${NC}"
    echo -e "${CYAN}📝 Your AI assistant is ready to craft amazing commits!${NC}"
    
    dramatic_pause "🎉 Let's see it in action..." $PAUSE_LONG
}

cinematic_main_demo() {
    clear_with_style
    echo -e "${CYAN}💡 THE MAGIC MOMENT${NC}"
    echo -e "${WHITE}Watch AI transform your development workflow${NC}"
    echo
    
    # Setup demo environment
    dramatic_pause "📁 Setting up demo project..." $PAUSE_SHORT
    rm -rf "$DEMO_DIR"
    mkdir -p "$DEMO_DIR"
    cd "$DEMO_DIR"
    git init -q
    git config user.name "Demo Developer"
    git config user.email "demo@commitcraft.dev"
    
    # Create realistic development scenario
    echo -e "${YELLOW}🔨 Let's build a modern React component...${NC}"
    
    mkdir -p src/components
    
    echo -e "${BLUE}Creating ButtonComponent.jsx...${NC}"
    thinking_dots "Writing component logic" 3
    
    cat > src/components/ButtonComponent.jsx << 'EOF'
import React from 'react';
import PropTypes from 'prop-types';
import './ButtonComponent.css';

const ButtonComponent = ({ 
  children, 
  variant = 'primary', 
  size = 'medium',
  onClick, 
  disabled = false,
  loading = false 
}) => {
  const handleClick = (e) => {
    if (!disabled && !loading && onClick) {
      onClick(e);
    }
  };

  return (
    <button 
      className={`btn btn--${variant} btn--${size} ${loading ? 'btn--loading' : ''}`}
      onClick={handleClick}
      disabled={disabled || loading}
      aria-label={loading ? 'Loading...' : undefined}
    >
      {loading ? (
        <span className="btn__spinner" />
      ) : (
        children
      )}
    </button>
  );
};

ButtonComponent.propTypes = {
  children: PropTypes.node.isRequired,
  variant: PropTypes.oneOf(['primary', 'secondary', 'danger']),
  size: PropTypes.oneOf(['small', 'medium', 'large']),
  onClick: PropTypes.func,
  disabled: PropTypes.bool,
  loading: PropTypes.bool,
};

export default ButtonComponent;
EOF

    echo -e "${BLUE}Adding beautiful CSS styling...${NC}"
    thinking_dots "Crafting modern design" 3
    
    cat > src/components/ButtonComponent.css << 'EOF'
.btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
  font-weight: 600;
  border: none;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s ease-in-out;
  position: relative;
  overflow: hidden;
}

.btn--small {
  padding: 0.5rem 1rem;
  font-size: 0.875rem;
}

.btn--medium {
  padding: 0.75rem 1.5rem;
  font-size: 1rem;
}

.btn--large {
  padding: 1rem 2rem;
  font-size: 1.125rem;
}

.btn--primary {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  box-shadow: 0 4px 15px rgba(102, 126, 234, 0.4);
}

.btn--primary:hover:not(:disabled) {
  transform: translateY(-2px);
  box-shadow: 0 8px 25px rgba(102, 126, 234, 0.6);
}

.btn--secondary {
  background: #f8f9fa;
  color: #495057;
  border: 2px solid #dee2e6;
}

.btn--danger {
  background: linear-gradient(135deg, #ff6b6b 0%, #ee5a24 100%);
  color: white;
}

.btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
  transform: none !important;
}

.btn__spinner {
  width: 16px;
  height: 16px;
  border: 2px solid transparent;
  border-top: 2px solid currentColor;
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}
EOF

    echo -e "${GREEN}📁 Created professional React button component${NC}"
    echo -e "${CYAN}   ✓ Multiple variants and sizes${NC}"
    echo -e "${CYAN}   ✓ Loading states and accessibility${NC}"
    echo -e "${CYAN}   ✓ Modern CSS with animations${NC}"
    
    dramatic_pause "" $PAUSE_MEDIUM
    
    echo -e "${YELLOW}📋 Time to commit our work...${NC}"
    simulate_user_typing "git add ."
    git add .
    
    dramatic_pause "🎯 Now for the magic moment..." $PAUSE_MEDIUM
    
    simulate_user_typing "commitcraft"
    echo
    
    # ACCURATE CommitCraft output format
    echo "Using provider: gemini (gemini-1.5-flash-latest)"
    loading_animation "Generating commit message " 4
    
    echo "✓ Message generated successfully!"
    
    dramatic_pause "✨ AI has crafted something special..." $PAUSE_MEDIUM
    
    echo
    echo -e "${CYAN}📝 Generated commit message:${NC}"
    echo "─────────────────────────────────────────────────────────────"
    
    reveal_effect "feat(components): add comprehensive ButtonComponent with variants and loading states"
    echo
    typewriter "Implement reusable ButtonComponent featuring:" $FAST_TYPING
    typewriter "- Multiple variants (primary, secondary, danger)" $FAST_TYPING
    typewriter "- Three size options (small, medium, large)" $FAST_TYPING
    typewriter "- Loading state with spinner animation" $FAST_TYPING
    typewriter "- Accessibility support with ARIA labels" $FAST_TYPING
    typewriter "- Modern CSS with gradients and hover effects" $FAST_TYPING
    typewriter "- PropTypes validation for type safety" $FAST_TYPING
    
    echo "─────────────────────────────────────────────────────────────"
    
    dramatic_pause "🎉 Look at that perfect commit message!" $PAUSE_LONG
    
    # ACCURATE interactive flow format
    echo
    echo -e "${CYAN}Edit the command below (or press Enter to execute):${NC}"
    command_prompt
    typewriter_with_color "$WHITE" "git commit -m \"feat(components): add comprehensive ButtonComponent with variants and loading states\""
    
    dramatic_pause "⚡ Executing commit..." $PAUSE_MEDIUM
    
    echo
    echo "Executing: git commit -m \"feat(components): add comprehensive ButtonComponent with variants and loading states\""
    echo -e "${GREEN}✓ Commit successful!${NC}"
    
    # Actually commit
    git commit -q -m "feat(components): add comprehensive ButtonComponent with variants and loading states

Implement reusable ButtonComponent featuring:
- Multiple variants (primary, secondary, danger)
- Three size options (small, medium, large)  
- Loading state with spinner animation
- Accessibility support with ARIA labels
- Modern CSS with gradients and hover effects
- PropTypes validation for type safety"

    dramatic_pause "🚀 That was just the beginning..." $PAUSE_LONG
}

cinematic_advanced_demo() {
    clear_with_style
    echo -e "${CYAN}🎯 ADVANCED FEATURES${NC}"
    echo -e "${WHITE}Explore the full power of CommitCraft${NC}"
    echo
    
    echo -e "${YELLOW}🐛 Let's fix a critical security bug...${NC}"
    
    mkdir -p src/utils
    
    thinking_dots "Implementing security improvements" 4
    
    cat > src/utils/security.js << 'EOF'
// Security utilities for input validation and sanitization

export const sanitizeInput = (input) => {
  if (!input || typeof input !== 'string') {
    return '';
  }
  
  // Fix: Prevent XSS attacks with comprehensive sanitization
  return input
    .replace(/[<>]/g, '') // Remove potential HTML tags
    .replace(/javascript:/gi, '') // Remove javascript: protocol
    .replace(/on\w+=/gi, '') // Remove event handlers
    .replace(/script/gi, '') // Remove script tags
    .trim();
};

export const validateCSRFToken = (token, sessionToken) => {
  if (!token || !sessionToken) {
    return false;
  }
  
  // Enhanced CSRF protection
  return token === sessionToken && token.length >= 32;
};

export const rateLimit = {
  attempts: new Map(),
  
  isAllowed(ip, maxAttempts = 5, windowMs = 15 * 60 * 1000) {
    const now = Date.now();
    const attempts = this.attempts.get(ip) || [];
    
    // Clean old attempts
    const validAttempts = attempts.filter(time => now - time < windowMs);
    
    if (validAttempts.length >= maxAttempts) {
      return false;
    }
    
    validAttempts.push(now);
    this.attempts.set(ip, validAttempts);
    return true;
  }
};
EOF

    echo -e "${GREEN}🔒 Critical security improvements implemented!${NC}"
    
    dramatic_pause "" $PAUSE_SHORT
    
    simulate_user_typing "git add ."
    git add .
    
    dramatic_pause "🔍 Let's try dry-run mode first..." $PAUSE_SHORT
    
    simulate_user_typing "commitcraft --dry-run"
    
    echo "Using provider: gemini (gemini-1.5-flash-latest)"
    loading_animation "Generating commit message " 3
    echo "✓ Message generated successfully!"
    
    echo
    echo -e "${CYAN}Generated Commit Message:${NC}"
    echo "---"
    reveal_effect "fix(security): implement comprehensive input sanitization and CSRF protection"
    echo
    typewriter "Add robust security measures including XSS prevention," $FAST_TYPING
    typewriter "CSRF token validation, and rate limiting to protect" $FAST_TYPING
    typewriter "against common web vulnerabilities and attacks." $FAST_TYPING
    echo "---"
    
    dramatic_pause "✅ Perfect! No commit made yet." $PAUSE_MEDIUM
    
    echo
    echo -e "${BLUE}🎨 Let's try a different AI provider...${NC}"
    simulate_user_typing "commitcraft --provider openai --model gpt-4o"
    
    echo "Using provider: openai (gpt-4o)"
    loading_animation "Generating commit message " 3
    echo "✓ Message generated successfully!"
    
    echo -e "${GREEN}✓ Different style, same quality!${NC}"
    echo -e "${CYAN}💡 Each provider has its own writing personality${NC}"
    
    dramatic_pause "" $PAUSE_MEDIUM
    
    # Actually commit the security fix
    git commit -q -m "fix(security): implement comprehensive input sanitization and CSRF protection

Add robust security measures including XSS prevention,
CSRF token validation, and rate limiting to protect
against common web vulnerabilities and attacks."
    
    echo -e "${GREEN}🔒 Security fix committed!${NC}"
    
    dramatic_pause "📈 Let's see our beautiful commit history..." $PAUSE_LONG
}

cinematic_finale() {
    clear_with_style
    echo -e "${CYAN}🎉 THE TRANSFORMATION${NC}"
    echo -e "${WHITE}Your development workflow will never be the same${NC}"
    echo
    
    echo -e "${BLUE}📊 Look at this professional commit history:${NC}"
    simulate_user_typing "git log --oneline --graph"
    
    dramatic_pause "" $PAUSE_SHORT
    
    echo
    echo -e "${YELLOW}* $(git rev-parse --short HEAD)${NC} fix(security): implement comprehensive input sanitization and CSRF protection"
    echo -e "${YELLOW}* $(git log --format="%h" -n 1 HEAD~1)${NC} feat(components): add comprehensive ButtonComponent with variants and loading states"
    
    dramatic_pause "🎨 Beautiful, professional, meaningful commits!" $PAUSE_LONG
    
    echo -e "${GREEN}✨ Your new superpower workflow:${NC}"
    echo
    
    typewriter "1️⃣  Write amazing code" 0.08
    typewriter "2️⃣  git add ." 0.08
    typewriter "3️⃣  commitcraft  ← ✨ AI magic happens here" 0.08
    typewriter "4️⃣  git push & celebrate! 🎉" 0.08
    
    dramatic_pause "" $PAUSE_LONG
    
    echo -e "${PURPLE}🚀 Benefits you'll love:${NC}"
    echo
    
    reveal_effect "⚡ 10x faster than manual writing"
    reveal_effect "📏 Perfect conventional commits format"
    reveal_effect "🧠 Contextual, detailed messages"
    reveal_effect "🎯 Consistent team coding style"
    reveal_effect "🤖 Multiple AI providers"
    reveal_effect "🔧 Zero configuration hassle"
    
    dramatic_pause "" $PAUSE_LONG
    
    clear_with_style
    
    echo -e "${PURPLE}🎬 Ready to Transform Your Git Game?${NC}"
    echo
    
    typewriter_with_color "$CYAN" "🚀 Get started in 30 seconds:"
    echo
    
    typewriter_with_color "$GREEN" "cargo install commitcraft"
    typewriter_with_color "$GREEN" "commitcraft setup"
    typewriter_with_color "$GREEN" "# Start creating amazing commits!"
    
    echo
    
    typewriter_with_color "$WHITE" "📚 Learn more:"
    typewriter_with_color "$BLUE" "🌐 https://github.com/san0808/commitcraft"
    typewriter_with_color "$BLUE" "📖 Full documentation & examples"
    typewriter_with_color "$BLUE" "💬 Community support & discussions"
    
    dramatic_pause "" $PAUSE_MEDIUM
    
    echo
    reveal_effect "✨ Thank you for watching CommitCraft! ✨"
    reveal_effect "🎨 Happy committing, developers! 🎨"
    
    dramatic_pause "🎬 End scene." $PAUSE_LONG
}

# Cleanup
cleanup() {
    cd /tmp 2>/dev/null || cd /
    rm -rf "$DEMO_DIR" 2>/dev/null || true
}

# Main cinematic flow
main() {
    echo -e "${PURPLE}🎬 Starting CommitCraft Cinematic Demo...${NC}"
    
    dramatic_pause "🎭 Preparing the stage..." $PAUSE_MEDIUM
    
    cinematic_intro
    cinematic_installation
    cinematic_setup
    cinematic_main_demo
    cinematic_advanced_demo
    cinematic_finale
    
    cleanup
    
    echo -e "${GREEN}🎊 Cinematic demo completed!${NC}"
    echo -e "${CYAN}🎥 Perfect for recording and sharing!${NC}"
}

# Handle script interruption
trap cleanup EXIT

# Run the cinematic demo
main "$@" 