#!/bin/bash

# CommitCraft Cinematic Demo Recording Script
# Professional video recording with automatic terminal sizing and upload-ready outputs

set -e

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
WHITE='\033[1;37m'
RED='\033[0;31m'
NC='\033[0m'

# Recording settings
RECORDING_DIR="./recordings"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
OUTPUT_NAME="commitcraft_cinematic_${TIMESTAMP}"

print_banner() {
    clear
    echo -e "${PURPLE}"
    echo "🎬 CommitCraft Professional Recording Studio"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo -e "${NC}"
}

check_dependencies() {
    echo -e "${CYAN}🔍 Checking recording dependencies...${NC}"
    
    local missing_deps=()
    
    # Check for asciinema
    if ! command -v asciinema &> /dev/null; then
        missing_deps+=("asciinema")
    fi
    
    # Check for ffmpeg (for video conversion)
    if ! command -v ffmpeg &> /dev/null; then
        missing_deps+=("ffmpeg")
    fi
    
    if [ ${#missing_deps[@]} -ne 0 ]; then
        echo -e "${RED}❌ Missing dependencies: ${missing_deps[*]}${NC}"
        echo
        echo -e "${YELLOW}📦 Install missing dependencies:${NC}"
        
        if command -v apt &> /dev/null; then
            echo "sudo apt update && sudo apt install asciinema ffmpeg"
        elif command -v dnf &> /dev/null; then
            echo "sudo dnf install asciinema ffmpeg"
        elif command -v pacman &> /dev/null; then
            echo "sudo pacman -S asciinema ffmpeg"
        elif command -v brew &> /dev/null; then
            echo "brew install asciinema ffmpeg"
        else
            echo "Please install asciinema and ffmpeg for your system"
        fi
        
        echo
        echo -e "${BLUE}🌐 Or install asciinema via pip:${NC}"
        echo "pip install asciinema"
        
        exit 1
    fi
    
    echo -e "${GREEN}✅ All dependencies available!${NC}"
}

setup_recording_env() {
    echo -e "${CYAN}📁 Setting up recording environment...${NC}"
    
    # Create recordings directory
    mkdir -p "$RECORDING_DIR"
    
    # Verify demo script exists
    if [ ! -f "demo/cinematic-demo.sh" ]; then
        echo -e "${RED}❌ Cinematic demo script not found!${NC}"
        echo "Make sure you're in the project root directory"
        exit 1
    fi
    
    # Make demo script executable
    chmod +x demo/cinematic-demo.sh demo/quick-demo.sh
    
    echo -e "${GREEN}✅ Recording environment ready!${NC}"
}

auto_resize_terminal() {
    local width=$1
    local height=$2
    local format_name="$3"
    
    echo -e "${CYAN}📐 Auto-resizing terminal for ${format_name} (${width}x${height})...${NC}"
    
    # Store original size
    ORIGINAL_SIZE=$(stty size 2>/dev/null || echo "24 80")
    
    # Try different methods to resize terminal
    if command -v resize &> /dev/null; then
        resize -s $height $width >/dev/null 2>&1
        echo -e "${GREEN}✅ Terminal resized using 'resize'${NC}"
    elif [ -n "$TMUX" ]; then
        tmux resize-window -x $width -y $height >/dev/null 2>&1
        echo -e "${GREEN}✅ Terminal resized using tmux${NC}"
    elif [ -n "$TERM_PROGRAM" ] && [ "$TERM_PROGRAM" = "vscode" ]; then
        echo -e "${YELLOW}⚠️  VSCode terminal detected - manual resize may be needed${NC}"
    else
        echo -e "${BLUE}📏 Please resize your terminal to ${width}x${height} for optimal recording${NC}"
        echo -e "${CYAN}Press Enter when ready...${NC}"
        read
    fi
}

restore_terminal_size() {
    if [ -n "$ORIGINAL_SIZE" ]; then
        local height=$(echo $ORIGINAL_SIZE | cut -d' ' -f1)
        local width=$(echo $ORIGINAL_SIZE | cut -d' ' -f2)
        
        if command -v resize &> /dev/null; then
            resize -s $height $width >/dev/null 2>&1
        fi
    fi
}

convert_to_formats() {
    local cast_file="$1"
    local base_name="${cast_file%.cast}"
    
    echo
    echo -e "${CYAN}🎨 Converting to upload-ready formats...${NC}"
    
    # 1. High-quality GIF (for GitHub, social media)
    if command -v agg &> /dev/null; then
        echo -e "${BLUE}📹 Creating high-quality GIF...${NC}"
        agg --font-size 16 --line-height 1.4 --cols 100 --rows 30 \
            "$cast_file" "${base_name}.gif"
        echo -e "${GREEN}✅ Created: ${base_name}.gif${NC}"
    fi
    
    # 2. MP4 video (for YouTube, LinkedIn, Twitter)
    if command -v ffmpeg &> /dev/null && command -v agg &> /dev/null; then
        echo -e "${BLUE}🎬 Creating MP4 video...${NC}"
        
        # First create a high-res GIF, then convert to MP4
        agg --font-size 18 --line-height 1.4 --cols 120 --rows 35 \
            "$cast_file" "${base_name}_hd.gif"
        
        # Convert GIF to MP4 with optimal settings for social media
        ffmpeg -y -i "${base_name}_hd.gif" \
            -vf "fps=30,scale=1280:720:flags=lanczos" \
            -c:v libx264 -crf 18 -preset slow \
            -movflags +faststart \
            "${base_name}.mp4" >/dev/null 2>&1
        
        # Clean up temporary HD gif
        rm -f "${base_name}_hd.gif"
        
        echo -e "${GREEN}✅ Created: ${base_name}.mp4${NC}"
    fi
    
    # 3. WebM video (for web embedding)
    if command -v ffmpeg &> /dev/null && [ -f "${base_name}.mp4" ]; then
        echo -e "${BLUE}🌐 Creating WebM video...${NC}"
        ffmpeg -y -i "${base_name}.mp4" \
            -c:v libvpx-vp9 -crf 30 -b:v 0 \
            -movflags +faststart \
            "${base_name}.webm" >/dev/null 2>&1
        echo -e "${GREEN}✅ Created: ${base_name}.webm${NC}"
    fi
    
    echo
    echo -e "${PURPLE}🎉 Upload-ready formats created!${NC}"
}

show_recording_menu() {
    echo
    echo -e "${YELLOW}🎥 Professional Recording Options:${NC}"
    echo "1. 🎬 Full Cinematic Demo (8-10 min) - YouTube/Presentations"
    echo "2. ⚡ Quick Demo (3-4 min) - Social Media/Demos"
    echo "3. 📱 Social Media Format - Square 1:1 ratio"
    echo "4. 🖥️  Widescreen Format - 16:9 HD ratio"
    echo "5. 📺 4K Format - Ultra HD for premium content"
    echo "6. ❌ Exit"
    echo
    echo -n "Choose option (1-6): "
}

record_demo() {
    local demo_type="$1"
    local format="$2"
    local title="$3"
    
    case $format in
        "social")
            auto_resize_terminal 80 25 "Social Media (1:1)"
            OUTPUT_NAME="${OUTPUT_NAME}_social"
            ;;
        "widescreen")
            auto_resize_terminal 120 30 "Widescreen (16:9)"
            OUTPUT_NAME="${OUTPUT_NAME}_widescreen"
            ;;
        "4k")
            auto_resize_terminal 140 35 "4K Ultra HD"
            OUTPUT_NAME="${OUTPUT_NAME}_4k"
            ;;
        *)
            auto_resize_terminal 100 30 "Standard HD"
            ;;
    esac
    
    # Add demo type to filename
    if [ "$demo_type" = "quick" ]; then
        OUTPUT_NAME="${OUTPUT_NAME}_quick"
    fi
    
    echo
    echo -e "${PURPLE}🎬 Starting recording: $title${NC}"
    echo -e "${YELLOW}📁 Output: ${RECORDING_DIR}/${OUTPUT_NAME}.cast${NC}"
    echo
    echo -e "${GREEN}📋 Recording will auto-start in 3 seconds...${NC}"
    echo -e "${CYAN}• Perfect timing built-in${NC}"
    echo -e "${CYAN}• No user interaction needed${NC}"  
    echo -e "${CYAN}• Professional quality guaranteed${NC}"
    echo
    
    # Countdown
    for i in 3 2 1; do
        echo -n "$i... "
        sleep 1
    done
    echo -e "\n${GREEN}🎬 Recording started!${NC}\n"
    
    # Select demo script
    local demo_script
    if [ "$demo_type" = "quick" ]; then
        demo_script="./demo/quick-demo.sh"
    else
        demo_script="./demo/cinematic-demo.sh"
    fi
    
    # Start asciinema recording with optimal settings
    asciinema rec "${RECORDING_DIR}/${OUTPUT_NAME}.cast" \
        --title "$title" \
        --command "$demo_script" \
        --overwrite \
        --quiet
    
    # Restore terminal size
    restore_terminal_size
    
    echo
    echo -e "${GREEN}🎉 Recording completed successfully!${NC}"
    echo -e "${CYAN}📁 Raw recording: ${RECORDING_DIR}/${OUTPUT_NAME}.cast${NC}"
    
    # Auto-convert to upload formats
    convert_to_formats "${RECORDING_DIR}/${OUTPUT_NAME}.cast"
}

show_upload_guide() {
    local base_name="$1"
    
    echo
    echo -e "${GREEN}🚀 Upload Guide - Your Content is Ready!${NC}"
    echo
    echo -e "${BLUE}📱 Social Media:${NC}"
    echo "• Twitter/X: Use .mp4 or .gif (< 15MB)"
    echo "• LinkedIn: Use .mp4 (best engagement)"  
    echo "• Instagram: Use .mp4 for Stories/Reels"
    echo "• TikTok: Use .mp4 (vertical crop recommended)"
    echo
    echo -e "${BLUE}📺 Video Platforms:${NC}"
    echo "• YouTube: Use .mp4 (perfect quality)"
    echo "• Vimeo: Use .mp4 or .webm"
    echo "• Loom: Use .mp4"
    echo
    echo -e "${BLUE}👨‍💻 Developer Platforms:${NC}"
    echo "• GitHub README: Use .gif (direct embed)"
    echo "• Documentation: Use .webm (smaller size)"
    echo "• Blog posts: Use .gif or .mp4"
    echo "• Presentations: Use .mp4"
    echo
    echo -e "${CYAN}📂 Files ready for upload:${NC}"
    ls -la "${RECORDING_DIR}/${base_name}".* 2>/dev/null | grep -E '\.(cast|gif|mp4|webm)$' || echo "No files found"
}

main() {
    print_banner
    check_dependencies
    setup_recording_env
    
    while true; do
        show_recording_menu
        read -r choice
        
        case $choice in
            1)
                record_demo "full" "standard" "CommitCraft - Full Cinematic Demo"
                ;;
            2)
                record_demo "quick" "standard" "CommitCraft - Quick Demo"
                ;;
            3)
                echo
                echo -e "${CYAN}📱 Social Media Format${NC}"
                echo "1. Full Cinematic Demo"
                echo "2. Quick Demo"
                read -p "Choose (1-2): " demo_choice
                
                if [ "$demo_choice" = "1" ]; then
                    record_demo "full" "social" "CommitCraft - Social Media Demo"
                else
                    record_demo "quick" "social" "CommitCraft - Quick Social Demo"
                fi
                ;;
            4)
                echo
                echo -e "${CYAN}🖥️  Widescreen Format${NC}"
                echo "1. Full Cinematic Demo"
                echo "2. Quick Demo"
                read -p "Choose (1-2): " demo_choice
                
                if [ "$demo_choice" = "1" ]; then
                    record_demo "full" "widescreen" "CommitCraft - Widescreen Demo"
                else
                    record_demo "quick" "widescreen" "CommitCraft - Quick Widescreen Demo"
                fi
                ;;
            5)
                echo
                echo -e "${CYAN}📺 4K Ultra HD Format${NC}"
                echo "1. Full Cinematic Demo"
                echo "2. Quick Demo"
                read -p "Choose (1-2): " demo_choice
                
                if [ "$demo_choice" = "1" ]; then
                    record_demo "full" "4k" "CommitCraft - 4K Ultra HD Demo"
                else
                    record_demo "quick" "4k" "CommitCraft - Quick 4K Demo"
                fi
                ;;
            6)
                echo -e "${GREEN}👋 Professional recordings completed!${NC}"
                exit 0
                ;;
            *)
                echo -e "${RED}❌ Invalid option. Please choose 1-6.${NC}"
                continue
                ;;
        esac
        
        # Show upload guide
        show_upload_guide "${OUTPUT_NAME%.cast}"
        
        echo
        echo -e "${PURPLE}🎬 Record another demo?${NC}"
        read -p "Continue? (y/N): " continue_choice
        
        if [[ ! "$continue_choice" =~ ^[Yy]$ ]]; then
            break
        fi
        
        print_banner
    done
    
    echo
    echo -e "${GREEN}🎊 All recordings saved in: ${RECORDING_DIR}/${NC}"
    echo -e "${CYAN}📋 View files: ls -la ${RECORDING_DIR}/commitcraft_*${NC}"
    echo
    echo -e "${PURPLE}🚀 Ready to share your amazing CommitCraft content worldwide!${NC}"
    echo -e "${YELLOW}💡 Pro tip: The .mp4 files are perfect for maximum reach!${NC}"
}

# Handle script interruption
cleanup() {
    restore_terminal_size
    echo
    echo -e "${YELLOW}⚠️  Recording interrupted${NC}"
    exit 1
}

trap cleanup INT TERM

# Run the recording studio
main "$@" 