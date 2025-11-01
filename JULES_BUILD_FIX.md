# Build Fix Instructions for Jules

## Problem
Tauri build failing due to missing `javascriptcoregtk-4.0` dependency.

## Root Cause
- Tauri 1.5 requires `webkit2gtk-4.1` on Linux
- System doesn't have the correct WebKit packages installed
- The error mentions `javascriptcoregtk-4.0` which is the OLD version

## Solutions (Try in Order)

### Solution 1: Install Correct System Dependencies ✅ RECOMMENDED

**Step 1: Identify Your Linux Distribution**
```bash
# Run this to see what distro you're on
cat /etc/os-release
```

**Step 2: Install Dependencies Based on Distro**

**For Ubuntu 22.04+ / Debian 12+ / WSL2 Ubuntu:**
```bash
sudo apt-get update

sudo apt-get install -y \
    libwebkit2gtk-4.1-dev \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    patchelf

# Verify it worked
pkg-config --modversion webkit2gtk-4.1
# Should output: 2.42.x or similar
```

**For Ubuntu 20.04 / Debian 11 (Older Versions):**
```bash
sudo apt-get update

# These older distros use webkit2gtk-4.0
sudo apt-get install -y \
    libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    libgtk-3-dev \
    libappindicator3-dev \
    librsvg2-dev \
    patchelf
```

**For Fedora 38+:**
```bash
sudo dnf install \
    webkit2gtk4.1-devel \
    openssl-devel \
    curl \
    wget \
    file \
    libappindicator-gtk3-devel \
    librsvg2-devel \
    patchelf
```

**For Arch Linux:**
```bash
sudo pacman -S \
    webkit2gtk-4.1 \
    base-devel \
    curl \
    wget \
    file \
    openssl \
    gtk3 \
    libappindicator-gtk3 \
    librsvg \
    patchelf
```

**Step 3: Try Building Again**
```bash
cd ui/src-tauri
cargo clean
cargo build
```

---

### Solution 2: Downgrade to webkit2gtk-4.0 Support

If your distro doesn't have webkit2gtk-4.1, modify Cargo.toml:

**File**: `ui/src-tauri/Cargo.toml`

**Change line 13 from:**
```toml
tauri-build = { version = "1.5", features = [] }
```

**To:**
```toml
tauri-build = { version = "1.5", features = ["custom-protocol"] }
```

**Change line 20 from:**
```toml
tauri = { version = "1.5", features = ["shell-open", "dialog-all", "fs-all", "notification-all", "path-all", "process-exit", "process-relaunch", "clipboard-write-text"] }
```

**To:**
```toml
# Use Tauri 1.4 which supports webkit2gtk-4.0
tauri = { version = "1.4", features = ["shell-open", "dialog-all", "fs-all", "notification-all", "path-all", "process-exit", "process-relaunch", "clipboard-write-text"] }
```

**Also change line 13:**
```toml
tauri-build = { version = "1.4", features = [] }
```

Then install webkit2gtk-4.0:
```bash
sudo apt-get install libwebkit2gtk-4.0-dev
```

And try building:
```bash
cargo clean
cargo build
```

---

### Solution 3: Use Windows Native Build (Easiest) 🚀

**Instead of building on WSL/Linux, build directly on Windows:**

**Step 1: Install Dependencies on Windows**
- Download and install [Microsoft Visual Studio C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
- During install, select "Desktop development with C++"
- Install [WebView2 Runtime](https://developer.microsoft.com/en-us/microsoft-edge/webview2/) (usually pre-installed on Windows 10/11)

**Step 2: Install Rust on Windows (if not already)**
```powershell
# In PowerShell
winget install Rustlang.Rustup
```

**Step 3: Build on Windows**
```powershell
# In PowerShell, navigate to project
cd "c:\Users\woozy\Downloads\building shit with ai\Health & Speed Checker\ui\src-tauri"
cargo clean
cargo build --release
```

**Windows is MUCH easier for Tauri** - no webkit2gtk dependency hell!

---

### Solution 4: Use Docker (Advanced)

Create a build container with all dependencies:

**File**: `Dockerfile` (create in project root)
```dockerfile
FROM rust:1.75-bullseye

# Install Tauri dependencies
RUN apt-get update && apt-get install -y \
    libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    patchelf

WORKDIR /app
```

**Build with Docker:**
```bash
docker build -t tauri-builder .
docker run -v "$(pwd):/app" tauri-builder cargo build --manifest-path /app/ui/src-tauri/Cargo.toml
```

---

## Verification Commands

After installing dependencies, verify they're correct:

```bash
# Check webkit version
pkg-config --modversion webkit2gtk-4.1
# OR
pkg-config --modversion webkit2gtk-4.0

# Check if pkg-config can find it
pkg-config --cflags webkit2gtk-4.1

# List all webkit packages
dpkg -l | grep webkit
# OR
rpm -qa | grep webkit
```

---

## If Still Stuck

**Provide this info:**
1. Your Linux distribution and version: `cat /etc/os-release`
2. Output of: `pkg-config --list-all | grep webkit`
3. Output of: `dpkg -l | grep webkit` (Debian/Ubuntu) OR `rpm -qa | grep webkit` (Fedora/RHEL)
4. Output of: `rustc --version && cargo --version`

---

## Quick Decision Tree

```
Are you on Windows?
├─ YES → Use Solution 3 (Windows native build) ← EASIEST
└─ NO → Are you on Ubuntu 22.04+?
    ├─ YES → Use Solution 1 with webkit2gtk-4.1
    └─ NO → Are you on Ubuntu 20.04 or older?
        ├─ YES → Use Solution 2 (downgrade to Tauri 1.4)
        └─ NO → Use Solution 4 (Docker) or switch to Windows
```

---

## Expected Result

After successful setup, you should see:

```bash
$ cargo build
   Compiling health-speed-checker-ui v0.1.0
   ...
   Finished dev [unoptimized + debuginfo] target(s) in 2m 34s
```

No errors about `javascriptcoregtk-4.0` or missing dependencies.

---

**Last Updated**: 2025-11-01
**Created By**: Claude Code (helping Jules)
