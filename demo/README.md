# 🎬 CommitCraft Demo Suite

Professional video recording system for creating engaging CommitCraft demonstrations. Perfect for social media, presentations, and marketing content.

## ✨ Features

- **🎯 100% Accurate**: Demos match real CommitCraft behavior exactly
- **🎨 Cinematic Effects**: Realistic typing, loading animations, dramatic pauses
- **📱 Multiple Formats**: Social media, YouTube, 4K, widescreen optimized
- **🤖 Auto Terminal Resize**: No manual configuration needed
- **🎬 Upload-Ready**: Automatically generates MP4, GIF, WebM formats
- **⚡ Zero Interaction**: Perfect timing built-in, no user input required

## 🚀 Quick Start

### Option 1: Professional Recording Studio
```bash
./demo/record-cinematic.sh
```

**Automatic Features:**
- ✅ Auto-resizes terminal for optimal recording
- ✅ Creates multiple upload-ready formats (MP4, GIF, WebM)
- ✅ Professional quality settings
- ✅ Upload guide for different platforms

### Option 2: Manual Demo Testing
```bash
# Quick 3-minute demo with effects
./demo/quick-demo.sh

# Full 8-10 minute cinematic experience  
./demo/cinematic-demo.sh
```

## 📋 Prerequisites

### Required Dependencies
```bash
# Install recording tools
sudo apt install asciinema ffmpeg      # Ubuntu/Debian
sudo dnf install asciinema ffmpeg      # Fedora
sudo pacman -S asciinema ffmpeg        # Arch
brew install asciinema ffmpeg          # macOS

# For high-quality GIF conversion
cargo install --git https://github.com/asciinema/agg
```

### Alternative Installation
```bash
# Python-based asciinema
pip install asciinema
```

## 🎥 Recording Options

### 1. Full Cinematic Demo (8-10 minutes)
**Perfect for:** YouTube, presentations, detailed showcases
- Complete feature walkthrough
- Professional storytelling flow
- Multiple code examples
- Advanced features demonstration

### 2. Quick Demo (3-4 minutes)  
**Perfect for:** Social media, quick demos, sales pitches
- Core features focus
- Fast-paced with effects
- High engagement value

### 3. Format Options
- **📱 Social Media**: Square 1:1 ratio for Instagram/Twitter
- **🖥️ Widescreen**: 16:9 HD for YouTube/presentations  
- **📺 4K Ultra HD**: Premium quality for top-tier content
- **🎯 Standard HD**: Balanced quality for general use

## 📁 Output Files

Each recording automatically generates:

```
recordings/
├── commitcraft_cinematic_20241201_143022.cast    # Raw asciinema
├── commitcraft_cinematic_20241201_143022.gif     # GitHub/Social
├── commitcraft_cinematic_20241201_143022.mp4     # YouTube/Twitter  
└── commitcraft_cinematic_20241201_143022.webm    # Web embedding
```

## 🌍 Platform Upload Guide

### 📱 Social Media
- **Twitter/X**: Use `.mp4` or `.gif` (< 15MB)
- **LinkedIn**: Use `.mp4` (best engagement)
- **Instagram**: Use `.mp4` for Stories/Reels
- **TikTok**: Use `.mp4` (crop to vertical)

### 📺 Video Platforms  
- **YouTube**: Use `.mp4` (perfect quality)
- **Vimeo**: Use `.mp4` or `.webm`
- **Loom**: Use `.mp4`

### 👨‍💻 Developer Content
- **GitHub README**: Use `.gif` (direct embed)
- **Documentation**: Use `.webm` (smaller size)
- **Blog posts**: Use `.gif` or `.mp4`
- **Presentations**: Use `.mp4`

## 🎯 Accuracy Guarantee

Our demos show **exactly** what users will experience:

### ✅ Real CommitCraft Output Format
```bash
Using provider: gemini (gemini-1.5-flash-latest)
Generating commit message...
✓ Message generated successfully!

📝 Generated commit message:
─────────────────────────────────────────
feat(components): add comprehensive ButtonComponent
─────────────────────────────────────────

Edit the command below (or press Enter to execute):
$ git commit -m "feat(components): add comprehensive ButtonComponent"

Executing: git commit -m "feat(components): add comprehensive ButtonComponent"
✓ Commit successful!
```

### ✅ Real CLI Commands
- `commitcraft` (default interactive flow)
- `commitcraft --dry-run` (preview only)
- `commitcraft --provider openai --model gpt-4o`
- `commitcraft setup` (configuration)

### ✅ Real Features Shown
- Interactive command editing (new default UX)
- Multiple AI providers (OpenAI, Gemini, Anthropic)
- Dry-run mode for previews
- Perfect conventional commits format
- Setup wizard simulation

## 🎨 Cinematic Features

### Realistic Typing Effects
```bash
typewriter() {
    for (( i=0; i<${#text}; i++ )); do
        echo -n "${text:$i:1}"
        sleep 0.03  # Realistic typing speed
    done
}
```

### Professional Loading Animations
- Spinner animations during AI processing
- Progress indicators for different stages
- Dramatic pauses for maximum impact

### Storytelling Flow
1. **Hook** (0-30s): Problem introduction
2. **Solution** (30s-2m): CommitCraft introduction  
3. **Demo** (2m-8m): Live feature showcase
4. **Benefits** (8m-10m): Value proposition
5. **CTA** (10m+): Getting started guide

## 🛠️ Customization

### Adjust Timing
```bash
# In demo scripts, modify these variables:
TYPING_SPEED=0.03        # Character typing speed
PAUSE_SHORT=1.5          # Quick pauses
PAUSE_MEDIUM=2.5         # Medium dramatic pauses  
PAUSE_LONG=4             # Long pauses for impact
```

### Terminal Sizing
```bash
# Automatic sizing in record-cinematic.sh:
auto_resize_terminal 100 30 "Standard HD"    # width x height
auto_resize_terminal 80 25 "Social Media"    # Square format
auto_resize_terminal 120 30 "Widescreen"     # 16:9 format
```

## 🎬 Pro Recording Tips

### For Maximum Engagement
1. **Use social media format** for Instagram/Twitter
2. **MP4 files** get highest engagement rates  
3. **Keep quick demos under 3 minutes** for social media
4. **Use full cinematic** for YouTube/presentations

### Technical Quality
- **4K recordings** for premium content
- **Standard HD** for general use  
- **WebM files** for documentation (smaller size)
- **GIF files** for GitHub README embeds

### Content Strategy
- **Quick demo first** to hook viewers
- **Full cinematic** for detailed education
- **Multiple formats** for different platforms
- **A/B test** different versions

## 🚀 Distribution Strategy

### Phase 1: Social Media Teasers
- Post quick demos on Twitter/LinkedIn
- Use engaging captions with problems/solutions
- Include clear CTAs to GitHub

### Phase 2: Educational Content
- Upload full demos to YouTube
- Create blog posts with embedded videos
- Add to documentation sites

### Phase 3: Community Sharing
- Share in developer communities
- Submit to awesome lists
- Include in conference presentations

---

## 📞 Support

Need help with demos? 
- 📖 Check [main README](../README.md)
- 🐛 [Open an issue](https://github.com/san0808/commitcraft/issues)
- 💬 [Join discussions](https://github.com/san0808/commitcraft/discussions)

**🎉 Ready to create amazing CommitCraft content!** 