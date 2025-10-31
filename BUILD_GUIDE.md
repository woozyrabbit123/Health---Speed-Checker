# 🔨 Build Guide - Health & Speed Checker

This guide will walk you through building the Health & Speed Checker from source.

## 📋 Prerequisites

### Required Software

1. **Rust** (1.75 or higher)
   ```bash
   # Install Rust via rustup
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

   # Or on Windows, download from: https://rustup.rs/

   # Verify installation
   rustc --version
   cargo --version
   ```

2. **Node.js** (18 or higher)
   ```bash
   # Download from: https://nodejs.org/

   # Verify installation
   node --version
   npm --version
   ```

3. **Platform-Specific Dependencies**

   **Windows:**
   - Windows 10/11 SDK
   - Visual Studio Build Tools 2019 or later
   - WebView2 (usually pre-installed on Windows 11)

   **macOS:**
   ```bash
   xcode-select --install
   ```

   **Linux (Debian/Ubuntu):**
   ```bash
   sudo apt update
   sudo apt install -y \
     libwebkit2gtk-4.0-dev \
     build-essential \
     curl \
     wget \
     libssl-dev \
     libgtk-3-dev \
     libayatana-appindicator3-dev \
     librsvg2-dev
   ```

## 🚀 Building the Project

### Step 1: Clone the Repository

```bash
git clone https://github.com/yourusername/health-speed-checker.git
cd health-speed-checker
```

### Step 2: Install Dependencies

```bash
# Install UI dependencies
cd ui
npm install
cd ..
```

### Step 3: Build Options

#### Option A: Development Build (with hot-reload)

```bash
# From the root directory
npm run dev

# Or manually:
cd ui
npm run tauri:dev
```

This will:
- Start the Vite dev server
- Build the Rust backend
- Launch the app with hot-reload enabled

#### Option B: Production Build

```bash
# From the root directory
npm run build

# Or manually:
cd agent
cargo build --release

cd ../ui
npm run build
npm run tauri:build
```

The built application will be in:
- **Windows**: `ui/src-tauri/target/release/health-checker.exe`
- **macOS**: `ui/src-tauri/target/release/bundle/dmg/`
- **Linux**: `ui/src-tauri/target/release/health-checker`

#### Option C: CLI Only Build

```bash
cd agent
cargo build --release

# The CLI binary will be at:
# target/release/health-checker (or health-checker.exe on Windows)
```

### Step 4: Run Tests

```bash
# Test Rust code
cd agent
cargo test --verbose

# Test with coverage (requires cargo-tarpaulin)
cargo install cargo-tarpaulin
cargo tarpaulin --out Html

# Lint Rust code
cargo clippy -- -D warnings

# Format Rust code
cargo fmt
```

### Step 5: Run the Application

#### CLI Mode:
```bash
cd agent
cargo run --release -- scan --quick
```

#### Desktop App:
```bash
cd ui
npm run tauri:dev
```

## 🔧 Common Build Issues

### Issue: "Failed to load Tauri"

**Solution:**
```bash
cd ui
npm install @tauri-apps/cli @tauri-apps/api
```

### Issue: "Rust compiler not found"

**Solution:**
```bash
rustup update
rustup default stable
```

### Issue: WebView2 missing (Windows)

**Solution:**
Download and install from: https://developer.microsoft.com/en-us/microsoft-edge/webview2/

### Issue: Permission denied (Linux/macOS)

**Solution:**
```bash
chmod +x ./target/release/health-checker
```

## 📦 Creating Distribution Packages

### Windows (.msi installer)

```bash
cd ui
npm run tauri:build
# Output: src-tauri/target/release/bundle/msi/
```

### macOS (.dmg)

```bash
cd ui
npm run tauri:build
# Output: src-tauri/target/release/bundle/dmg/
```

### Linux (AppImage, .deb)

```bash
cd ui
npm run tauri:build
# Outputs:
# - src-tauri/target/release/bundle/appimage/
# - src-tauri/target/release/bundle/deb/
```

### Portable Version (all platforms)

Simply zip the release binary:

```bash
# Windows
cd target/release
7z a health-checker-windows-portable.zip health-checker.exe

# macOS/Linux
cd target/release
tar czf health-checker-portable.tar.gz health-checker
```

## 🔐 Code Signing (Optional)

### Windows

```bash
# Requires a code signing certificate
signtool sign /f certificate.pfx /p password /tr http://timestamp.digicert.com /td sha256 /fd sha256 health-checker.exe
```

### macOS

```bash
# Requires Apple Developer certificate
codesign --force --deep --sign "Developer ID Application: Your Name" health-checker.app
```

## 🧪 Advanced Development

### Enable Debug Logging

```bash
# Set environment variable
export RUST_LOG=debug

# Or in Windows:
set RUST_LOG=debug

# Run with debug output
cargo run
```

### Profile Performance

```bash
# Install flamegraph
cargo install flamegraph

# Generate profile
cargo flamegraph --bin health-checker

# Opens flamegraph.svg in browser
```

### Build for Different Architectures

```bash
# List available targets
rustup target list

# Add target
rustup target add x86_64-pc-windows-gnu

# Build for specific target
cargo build --release --target x86_64-pc-windows-gnu
```

## 📁 Project Structure After Build

```
health-speed-checker/
├── agent/
│   ├── target/
│   │   └── release/
│   │       └── health-checker(.exe)    # CLI binary
│   └── Cargo.lock                       # Generated
├── ui/
│   ├── node_modules/                    # Generated
│   ├── dist/                            # Generated
│   └── src-tauri/
│       └── target/
│           └── release/
│               ├── health-checker(.exe) # GUI binary
│               └── bundle/              # Platform installers
└── package-lock.json                    # Generated
```

## 🎯 Quick Commands Reference

```bash
# Full project setup
npm run install-deps

# Development mode
npm run dev

# Build everything
npm run build

# Run all tests
npm run test

# Lint code
npm run lint

# Format code
npm run format

# Clean build artifacts
npm run clean
```

## 🆘 Getting Help

If you encounter issues:

1. Check the [CONTRIBUTING.md](CONTRIBUTING.md) guide
2. Search existing [GitHub Issues](https://github.com/yourusername/health-speed-checker/issues)
3. Ask in [Discussions](https://github.com/yourusername/health-speed-checker/discussions)
4. Join our [Discord](https://discord.gg/yourinvite)

## ✅ Build Checklist

Before considering your build complete:

- [ ] Rust toolchain installed and up-to-date
- [ ] Node.js and npm installed
- [ ] Platform dependencies installed
- [ ] `cargo test` passes
- [ ] `cargo clippy` shows no warnings
- [ ] Application runs in dev mode
- [ ] Production build completes successfully
- [ ] Packaged installer/binary works on clean system

---

**Happy Building!** 🎉
