# 🛡️ Health & Speed Checker

[![Build Status](https://github.com/yourusername/health-speed-checker/actions/workflows/ci.yml/badge.svg)](https://github.com/yourusername/health-speed-checker/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Version](https://img.shields.io/github/v/release/yourusername/health-speed-checker)](https://github.com/yourusername/health-speed-checker/releases)

A privacy-first, local-only PC health and performance analyzer that answers two simple questions: **"Am I safe?"** and **"Why is my PC slow?"**

![Health & Speed Checker Screenshot](docs/images/screenshot.png)

## ✨ Features

- 🔒 **100% Local** - No cloud, no telemetry, no account required
- 🛡️ **Security Scanning** - OS updates, firewall status, open ports, vulnerable apps
- ⚡ **Performance Analysis** - CPU/memory usage, startup bloat, process monitoring
- 🔧 **One-Click Fixes** - Auto-remediation with system restore points
- 📊 **Trend Tracking** - See how your system health changes over time
- 🚀 **Quick Scan Mode** - Get results in 5 seconds
- 📱 **Cross-Platform** - Windows, macOS, and Linux support

## 🚀 Quick Start

### Download Pre-built Binary

Download the latest release for your platform from the [Releases](https://github.com/yourusername/health-speed-checker/releases) page.

### Run from Command Line

```bash
# Quick scan (5 seconds)
health-checker scan --quick

# Full scan
health-checker scan

# Fix a specific issue
health-checker fix windows_update_pending

# View status
health-checker status
```

### Desktop Application

Simply double-click the downloaded application to launch the GUI.

## 📦 Installation

### Windows

```powershell
# Download the installer
curl -LO https://github.com/yourusername/health-speed-checker/releases/latest/download/health-checker-windows.msi

# Run the installer
msiexec /i health-checker-windows.msi
```

### macOS

```bash
# Intel Macs
brew install health-speed-checker

# Or download directly
curl -LO https://github.com/yourusername/health-speed-checker/releases/latest/download/health-checker-macos-amd64.tar.gz
tar xzf health-checker-macos-amd64.tar.gz
sudo mv health-checker /usr/local/bin/
```

### Linux

```bash
# Debian/Ubuntu
wget https://github.com/yourusername/health-speed-checker/releases/latest/download/health-checker-linux.deb
sudo dpkg -i health-checker-linux.deb

# Or use the AppImage
wget https://github.com/yourusername/health-speed-checker/releases/latest/download/health-checker.AppImage
chmod +x health-checker.AppImage
./health-checker.AppImage
```

### Portable Version (No Installation)

Download the portable ZIP file, extract, and run directly from any location (even USB drives).

## 🛠️ Building from Source

### Prerequisites

- Rust 1.75+ ([install](https://rustup.rs/))
- Node.js 18+ ([install](https://nodejs.org/))
- Platform-specific dependencies:
  - **Windows**: Windows SDK
  - **macOS**: Xcode Command Line Tools
  - **Linux**: `libwebkit2gtk-4.0-dev`, `libgtk-3-dev`

### Build Steps

```bash
# Clone the repository
git clone https://github.com/yourusername/health-speed-checker.git
cd health-speed-checker

# Build the Rust agent
cd agent
cargo build --release

# Build the Tauri desktop app
cd ../ui
npm install
npm run tauri build

# The binary will be in:
# - Windows: target/release/health-checker.exe
# - macOS/Linux: target/release/health-checker
```

## 🎯 CLI Usage

```bash
# SCANNING
health-checker scan                    # Full system scan
health-checker scan --quick            # Quick 5-second scan
health-checker scan --security         # Security only
health-checker scan --performance      # Performance only
health-checker scan --output json      # JSON output

# FIXING ISSUES
health-checker fix <issue-id>          # Fix specific issue
health-checker fix --top-3             # Fix top 3 issues

# REPORTING
health-checker report list             # List past scans
health-checker report show <scan-id>   # Show scan details
health-checker report export <scan-id> --format pdf

# CONFIGURATION
health-checker config set telemetry=off
health-checker config set auto-scan=daily
health-checker config show

# DAEMON MODE
health-checker daemon start            # Run in background
health-checker daemon stop
health-checker daemon status
```

## 🏗️ Architecture

```
┌─────────────┐     IPC      ┌──────────────┐
│  Tauri UI   │ ◄──────────► │  Rust Agent  │
│  (React)    │              │              │
└─────────────┘              └──────┬───────┘
                                    │
                            ┌───────▼───────┐
                            │  Checkers     │
                            │  - Firewall   │
                            │  - Ports      │
                            │  - Updates    │
                            │  - Processes  │
                            └───────────────┘
```

### Core Components

- **Checker System**: Modular plugins for different scan types
- **Scoring Engine**: Weighted algorithm for health/speed scores
- **Fix Executor**: Safe remediation with automatic restore points
- **Event Bus**: Real-time progress updates during scans

## 🔒 Privacy & Security

- **No telemetry by default** - Opt-in only, and even then it's anonymized
- **Local database** - All data stored in `~/.healthchecker/`
- **Open source** - Audit the code yourself
- **Signed binaries** - Verified publisher on Windows/macOS
- **Minimal permissions** - Only requests what's needed

## 📊 What Gets Checked?

### Security Checks
- ✅ Operating system updates
- ✅ Firewall status
- ✅ Open network ports
- ✅ Known vulnerabilities (CVE database)
- ✅ Suspicious processes

### Performance Checks
- ✅ CPU usage and top processes
- ✅ Memory usage and leaks
- ✅ Startup programs
- ✅ Disk usage
- ✅ System resource bottlenecks

## 🎨 Screenshots

<table>
  <tr>
    <td><img src="docs/images/dashboard.png" alt="Dashboard" /></td>
    <td><img src="docs/images/scan-progress.png" alt="Scan Progress" /></td>
  </tr>
  <tr>
    <td><img src="docs/images/results.png" alt="Results" /></td>
    <td><img src="docs/images/fixes.png" alt="Fixes" /></td>
  </tr>
</table>

## 🤝 Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details.

```bash
# Fork the repo, then:
git clone https://github.com/yourusername/health-speed-checker.git
cd health-speed-checker
git checkout -b feature/your-feature
# Make changes
cargo test
git commit -m "Add your feature"
git push origin feature/your-feature
# Open a Pull Request
```

## 📝 License

MIT License - see [LICENSE](LICENSE) for details.

## 🙏 Acknowledgments

- Built with [Tauri](https://tauri.app/) for a lightweight desktop experience
- Uses [Rust](https://www.rust-lang.org/) for performance and safety
- Icons from [Lucide](https://lucide.dev/)

## 🐛 Reporting Issues

Found a bug? Please [open an issue](https://github.com/yourusername/health-speed-checker/issues/new) with:
- Your OS version
- Steps to reproduce
- Expected vs actual behavior
- Relevant logs (found in `~/.healthchecker/logs/`)

## 💰 Support the Project

If you find this useful, consider:
- ⭐ Starring the repository
- 🐦 Sharing on social media
- ☕ [Buying us a coffee](https://buymeacoffee.com/yourusername)
- 💼 Purchasing a [Pro license](https://healthspeedchecker.com/pro) for advanced features

## 📧 Contact

- GitHub Issues: [Project Issues](https://github.com/yourusername/health-speed-checker/issues)
- Email: support@healthspeedchecker.com
- Discord: [Join our community](https://discord.gg/yourinvite)

---

**Remember**: This tool provides recommendations. Always backup your data before making system changes.

Made with ❤️ for the privacy-conscious community
