# Rust YouTube Downloader & Wallpaper Setter

A powerful Rust CLI tool that downloads YouTube videos and automatically converts them into stunning live wallpapers for macOS. Built with performance and user experience in mind.

![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)
![macOS](https://img.shields.io/badge/macOS-10.15+-blue.svg)
![License](https://img.shields.io/badge/License-MIT-green.svg)

## ✨ Features

- **🎥 Smart Video Download**: Intelligent format selection using yt-dlp
- **🎨 Live Wallpapers**: Transform any YouTube video into a dynamic macOS wallpaper
- **⚡ High Performance**: Hardware-accelerated HEVC conversion to 4K 60fps
- **🔧 Multiple Modes**: Interactive, download-only, and wallpaper installation modes
- **🛡️ Robust Error Handling**: Comprehensive error detection and user guidance
- **📊 Progress Tracking**: Real-time download and conversion progress
- **🔍 Dependency Checking**: Automatic verification of required tools
- **🧹 Auto Cleanup**: Automatic removal of temporary files

## 🚀 Quick Start

### Prerequisites

Ensure you have the following installed:

- **Rust** (1.70 or later)
- **yt-dlp** (YouTube downloader)
- **ffmpeg** (video conversion)
- **macOS** (10.15 or later)

#### Install Dependencies (macOS)

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Homebrew (if not already installed)
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Install yt-dlp and ffmpeg
brew install yt-dlp ffmpeg
```

### Building and Running

```bash
# Clone the repository
git clone <repository-url>
cd rust-downloader

# Build the project
cargo build --release

# Run in interactive mode
cargo run --release

# Download a video directly
cargo run --release -- "https://youtu.be/VIDEO_ID"

# Download and install as wallpaper
cargo run --release -- --wallpaper "https://youtu.be/VIDEO_ID"
```

## 📖 Usage

### Interactive Mode (Recommended for first-time users)

```bash
cargo run --release
```

This starts an interactive session where you'll be prompted to:
1. Enter a YouTube URL
2. Choose whether to install as wallpaper
3. Select output directory (optional)

### Direct Commands

#### Download Only
```bash
cargo run --release -- download "https://youtu.be/VIDEO_ID"
```

#### Download and Install as Wallpaper
```bash
cargo run --release -- wallpaper "https://youtu.be/VIDEO_ID"
```

#### Check Dependencies
```bash
cargo run --release -- check
```

### Command-Line Arguments

| Argument | Description | Example |
|----------|-------------|---------|
| `URL` | YouTube video URL | `https://youtu.be/dQw4w9WgXcQ` |
| `--wallpaper` | Enable wallpaper installation (requires sudo) | `--wallpaper` |
| `--download-only` | Disable wallpaper installation | `--download-only` |
| `--output, -o` | Custom output directory | `--output ./my_videos` |
| `--help, -h` | Show help information | `--help` |

### Supported URL Formats

- `https://www.youtube.com/watch?v=VIDEO_ID`
- `https://youtu.be/VIDEO_ID`
- `https://www.youtube.com/embed/VIDEO_ID`
- `https://www.youtube.com/v/VIDEO_ID`

## 🔧 Advanced Features

### Utility Commands

#### Cleanup Utility
Fixes permission issues and cleans up temporary files:
```bash
cargo run --bin cleanup
```

#### Refresh Utility
Refreshes wallpaper animation (useful if wallpaper becomes static after screen lock):
```bash
cargo run --bin refresh
```

### Configuration

The application uses a default configuration that can be customized through command-line arguments or environment variables.

## 📁 Output Structure

```
output/
├── video_title_YYYY-MM-DD/
│   ├── final_wallpaper.mov    # 4K 60fps HEVC video
│   └── logs/                  # Download and conversion logs
```

## 🎯 What Happens During Download

1. **Environment Check**: Verifies yt-dlp and ffmpeg are available
2. **Video Analysis**: Analyzes available formats and selects optimal quality
3. **Download**: Downloads video in best available quality
4. **Conversion**: Converts to 4K 60fps HEVC .mov format for macOS
5. **Cleanup**: Removes original files to save space
6. **Wallpaper Installation** (if enabled): Sets as live wallpaper

## 🐛 Troubleshooting

### Common Issues

#### "yt-dlp not found"
```bash
# Install yt-dlp
brew install yt-dlp
# or
pip install yt-dlp
```

#### "ffmpeg not found"
```bash
# Install ffmpeg
brew install ffmpeg
```

#### "Permission denied" errors
```bash
# Run cleanup utility
cargo run --bin cleanup
```

#### Wallpaper appears static after screen lock
```bash
# Refresh wallpaper animation
cargo run --bin refresh
```

#### Network/Connection issues
- Check internet connection
- Try downloading a different video to isolate the issue
- Ensure YouTube video is not region-locked or private

### Getting Help

1. Run `cargo run --release -- check` to diagnose dependency issues
2. Check the logs in the output directory
3. Try downloading a different video to isolate the issue

## 🔒 Privacy & Security

- Videos are downloaded directly from YouTube
- No data is stored or transmitted to third-party servers
- All processing happens locally on your machine
- Temporary files are automatically cleaned up

## ⚠️ Important Notes

- **Sudo Requirement**: Wallpaper installation requires administrator privileges
- **macOS Only**: Wallpaper features are designed specifically for macOS
- **Video Quality**: The tool automatically selects the best available quality
- **Storage**: Final videos are optimized for quality while maintaining reasonable file sizes
- **Power Management**: Live wallpapers may pause on battery power to conserve energy

## 🚦 Performance Tips

- **Plug In**: Keep your Mac plugged in for uninterrupted wallpaper animation
- **SSD Storage**: Use SSD storage for faster I/O operations
- **Network**: Stable internet connection ensures smooth downloads
- **Storage Space**: Ensure sufficient free space (videos can be several GB)

## 📋 Requirements

- **Operating System**: macOS 10.15 (Catalina) or later
- **Rust**: 1.70 or later
- **Memory**: 4GB RAM minimum, 8GB recommended
- **Storage**: 10GB free space for temporary files
- **Network**: Stable internet connection

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Development Setup

```bash
# Clone the repository
git clone <repository-url>
cd rust-downloader

# Install development dependencies
cargo build

# Run tests
cargo test

# Run with debug logging
RUST_LOG=debug cargo run
```

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🙏 Acknowledgments

- [yt-dlp](https://github.com/yt-dlp/yt-dlp) for excellent YouTube downloading capabilities
- [ffmpeg](https://ffmpeg.org/) for powerful video processing
- The Rust community for amazing crates and tooling

## 📞 Support

If you encounter any issues or have questions:

1. Check the [troubleshooting section](#-troubleshooting)
2. Search existing issues in the repository
3. Create a new issue with detailed information about your problem

---

**Made with ❤️ using Rust**

*Transform your desktop with the power of live YouTube wallpapers!*
