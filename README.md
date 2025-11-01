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

## 📖 What You Get

### 💻 Desktop Application
Beautiful native app with:
- **Real-time scanning** with progress indicators
- **Interactive charts** showing health trends over time
- **One-click fixes** for common issues
- **Historical tracking** - see your improvements
- **Export reports** to PDF/JSON for sharing

### ⌨️ Command Line Interface
Powerful CLI for automation:
- **Script-friendly** JSON output
- **Cron job** integration
- **CI/CD** pipeline support
- **Remote monitoring** capabilities

## 🎨 Screenshots

### Main Dashboard
![Dashboard](docs/images/dashboard.png)

The main dashboard shows your current health and speed scores with at-a-glance status indicators.

### Scan Results
![Scan Results](docs/images/scan-results.png)

Detailed breakdown of all detected issues, categorized by severity and type.

### Historical Trends
![Trends Chart](docs/images/trends.png)

Track your system's health and speed over time with beautiful, interactive charts.

### Issue Details
![Issue Details](docs/images/issue-details.png)

Each issue includes:
- Clear explanation of the problem
- Impact on system health/speed
- Step-by-step fix instructions
- One-click remediation when possible

## 🔍 What Gets Checked

### Security Checks
| Check | What It Does | Platforms |
|-------|--------------|-----------|
| **Firewall Status** | Verifies Windows Defender/iptables is active | Win, Lin |
| **OS Updates** | Checks for pending security patches | All |
| **Open Ports** | Scans for unexpected open ports | All |
| **Vulnerable Apps** | Detects outdated software with known CVEs | All |
| **Antivirus Status** | Confirms real-time protection is enabled | Win |
| **BitLocker/FileVault** | Checks disk encryption status | Win, Mac |

### Performance Checks
| Check | What It Does | Platforms |
|-------|--------------|-----------|
| **Startup Programs** | Identifies unnecessary auto-start apps | All |
| **Bloatware** | Detects common resource-hogging software | All |
| **CPU Hogs** | Finds processes consuming excessive CPU | All |
| **Memory Leaks** | Identifies apps with growing memory usage | All |
| **Disk Health** | S.M.A.R.T. status and fragmentation | All |
| **Network Speed** | Tests connection latency and throughput | All |

## 🧪 Example Output

### Quick Scan (5 seconds)
```bash
$ health-checker scan --quick

🩺 Health Score: 87/100  (+5 since last scan)
⚡ Speed Score:  72/100   (-3 since last scan)

⚠️ Issues Found: 3

HIGH: Windows Update Pending
  → 12 security updates available
  → Fix: health-checker fix windows_update_pending

MEDIUM: Startup Bloat Detected
  → 8 unnecessary programs at startup
  → Fix: health-checker fix startup_bloat

LOW: Disk Fragmentation
  → C: drive is 15% fragmented
  → Fix: health-checker fix disk_defrag

Scan completed in 4.2s
```

### Full Scan with JSON Output
```bash
$ health-checker scan --output json

{
  "scan_id": "scan_2025-01-15_14-30-22",
  "timestamp": 1736952622,
  "health_score": 87,
  "speed_score": 72,
  "scores": {
    "security": 92,
    "performance": 68,
    "stability": 95
  },
  "issues": [
    {
      "id": "windows_update_pending",
      "title": "Windows Update Pending",
      "severity": "high",
      "category": "security",
      "description": "12 security updates are pending installation",
      "fixable": true,
      "affected_item": "Windows Update Service"
    }
  ],
  "deltas": {
    "health": +5,
    "speed": -3,
    "issues_fixed_since_last": 2
  },
  "scan_duration_ms": 4200
}
```

### Fixing Issues
```bash
$ health-checker fix startup_bloat

🔧 Fixing: Startup Bloat Detected

Disabling unnecessary startup programs:
  ✓ Discord auto-start
  ✓ Spotify auto-start
  ✓ Steam auto-start
  ✓ Adobe Creative Cloud
  ✓ OneDrive sync (kept in background)
  ✓ Dropbox
  ✓ Epic Games Launcher
  ✓ Skype

💾 Created system restore point: BeforeFix_startup_bloat

✅ Fixed successfully!

Expected improvements:
  • Boot time: -15 to -30 seconds
  • Available RAM: +500 to +800 MB
  • CPU idle usage: -10% to -15%

Note: Restart required for changes to take full effect.
```

## 🤝 Contributing

We love contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Development Setup

```bash
# 1. Clone and enter directory
git clone https://github.com/yourusername/health-speed-checker.git
cd health-speed-checker

# 2. Run development script (handles everything)
# Windows
scripts\dev.bat

# macOS/Linux
./scripts/dev.sh

# 3. The app will launch automatically with hot-reload enabled
```

### Adding a New Checker

```rust
// agent/src/checkers/my_checker.rs
use crate::{Checker, CheckCategory, Issue, IssueSeverity, ScanContext};

pub struct MyChecker;

impl Checker for MyChecker {
    fn name(&self) -> &'static str {
        "My Custom Checker"
    }

    fn category(&self) -> CheckCategory {
        CheckCategory::Performance
    }

    fn run(&self, context: &ScanContext) -> Vec<Issue> {
        let mut issues = Vec::new();

        // Your detection logic here

        issues
    }

    fn fix(&self, issue_id: &str, params: &serde_json::Value) -> Result<FixResult, String> {
        // Your fix logic here
        Ok(FixResult {
            success: true,
            message: "Fixed successfully".to_string(),
            requires_restart: false,
        })
    }
}
```

Then register it in `agent/src/lib.rs`:
```rust
engine.register(Box::new(checkers::my_checker::MyChecker));
```

## 🤖 Automation Examples

### Daily Health Check (Cron)
```bash
# Check health every day at 2 AM
0 2 * * * /usr/local/bin/health-checker scan --quick --output json > /var/log/health-check.json
```

### CI/CD Integration
```yaml
# .github/workflows/health-check.yml
name: System Health Check

on:
  schedule:
    - cron: '0 0 * * *'  # Daily

jobs:
  health_check:
    runs-on: ubuntu-latest
    steps:
      - name: Run Health Check
        run: |
          wget https://github.com/.../health-checker-linux
          chmod +x health-checker-linux
          ./health-checker-linux scan --output json > health-report.json

      - name: Upload Report
        uses: actions/upload-artifact@v3
        with:
          name: health-report
          path: health-report.json
```

### PowerShell Script
```powershell
# Schedule daily health checks on Windows
$action = New-ScheduledTaskAction -Execute "health-checker.exe" -Argument "scan --quick"
$trigger = New-ScheduledTaskTrigger -Daily -At 2am
Register-ScheduledTask -Action $action -Trigger $trigger -TaskName "Daily Health Check"
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
