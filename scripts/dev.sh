#!/bin/bash
# Development helper script

set -e

echo "🚀 Starting Health & Speed Checker in development mode..."

# Check prerequisites
echo "📋 Checking prerequisites..."

if ! command -v rustc &> /dev/null; then
    echo "❌ Rust is not installed. Please install from https://rustup.rs/"
    exit 1
fi

if ! command -v node &> /dev/null; then
    echo "❌ Node.js is not installed. Please install from https://nodejs.org/"
    exit 1
fi

echo "✅ Prerequisites met"

# Install dependencies if needed
if [ ! -d "ui/node_modules" ]; then
    echo "📦 Installing UI dependencies..."
    cd ui
    npm install
    cd ..
fi

# Run the app
echo "🎯 Launching application..."
cd ui
npm run tauri:dev
