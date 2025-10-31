@echo off
REM Development helper script for Windows

echo 🚀 Starting Health ^& Speed Checker in development mode...

REM Check prerequisites
echo 📋 Checking prerequisites...

where rustc >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo ❌ Rust is not installed. Please install from https://rustup.rs/
    exit /b 1
)

where node >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo ❌ Node.js is not installed. Please install from https://nodejs.org/
    exit /b 1
)

echo ✅ Prerequisites met

REM Install dependencies if needed
if not exist "ui\node_modules" (
    echo 📦 Installing UI dependencies...
    cd ui
    call npm install
    cd ..
)

REM Run the app
echo 🎯 Launching application...
cd ui
call npm run tauri:dev
