# 🚀 Quick Start Guide — Health & Speed Checker

Get your development environment running in 10 minutes.

## Prerequisites Check

```bash
# Verify installations
rustc --version    # Should be 1.70+
node --version     # Should be 18+
cargo --version
npm --version
```

## Step 1: Clone & Setup (2 min)

```bash
# Clone the repository
git clone https://github.com/[your-username]/health-speed-checker.git
cd health-speed-checker

# Install Rust dependencies
cargo build

# Install Tauri CLI if not already installed
cargo install tauri-cli

# Install frontend dependencies
cd ui
npm install
cd ..
```

## Step 2: Run the 10-Hour Proof (5 min)

```bash
# Build and run the CLI prototype
cd agent
cargo run -- scan --quick

# Expected output:
# Scanning...
# Health: 72/100
# Speed: 85/100
# 
# Top Issues:
# - [CRITICAL] Firewall is OFF
# - [WARNING] Chrome using 67% CPU
# - [WARNING] 23 apps slow your boot
```

## Step 3: Start the Tauri Dev Server (3 min)

```bash
# From project root
npm run tauri dev

# This will:
# 1. Compile the Rust backend
# 2. Start the frontend dev server
# 3. Open the desktop app window
```

## Step 4: Test a Quick Scan

In the opened app:
1. Click "Scan Now" button
2. Watch the progress bar
3. See your Health & Speed scores
4. Review the Top 3 Issues

## Common Issues & Fixes

### Windows: "Cannot find module" error
```powershell
# Run as Administrator
npm install -g @tauri-apps/cli
```

### Linux: Permission denied on ports
```bash
# No sudo needed - we only scan local ports
# If issues persist, check firewall:
sudo ufw status
```

### macOS: "Developer cannot be verified"
```bash
# Allow the app in System Preferences > Security & Privacy
# Or remove quarantine:
xattr -d com.apple.quarantine /path/to/health-checker.app
```

## Development Commands

```bash
# Run tests
cargo test

# Run linter
cargo clippy

# Format code
cargo fmt

# Build release version
cargo build --release

# Create installer (Windows)
npm run tauri build -- --target x86_64-pc-windows-msvc

# Create portable version
# Just zip the contents of target/release/
```

## Project Structure Overview

```
health-speed-checker/
├── agent/src/
│   ├── checkers/        # Individual checker modules
│   ├── engine.rs        # Core scanning engine
│   ├── scoring.rs       # Score calculation
│   └── main.rs          # CLI entry point
├── ui/src/
│   ├── App.tsx          # Main UI component
│   └── api.ts           # Tauri command bindings
└── tauri.conf.json      # Build configuration
```

## Your First Checker

Create a new file `agent/src/checkers/example.rs`:

```rust
use async_trait::async_trait;
use crate::{Checker, Issue, IssueSeverity, ImpactCategory};

pub struct ExampleChecker;

#[async_trait]
impl Checker for ExampleChecker {
    fn name(&self) -> &'static str {
        "example_checker"
    }

    async fn run(&self, _ctx: &ScanContext) -> Vec<Issue> {
        vec![Issue {
            id: "example_issue".into(),
            severity: IssueSeverity::Info,
            title: "Example Issue Found".into(),
            description: "This is a test issue".into(),
            impact_category: ImpactCategory::Performance,
            fix: None,
        }]
    }
}
```

Register it in `agent/src/engine.rs`:
```rust
engine.register(Box::new(ExampleChecker));
```

## Need Help?

- 📖 Read the full [PROJECT_INSTRUCTIONS.md](./PROJECT_INSTRUCTIONS.md)
- 🐛 File issues at [GitHub Issues](https://github.com/[your-username]/health-speed-checker/issues)
- 💬 Ask questions in [Discussions](https://github.com/[your-username]/health-speed-checker/discussions)

---

**Ready to build?** Start with the 10-hour proof, then iterate daily!