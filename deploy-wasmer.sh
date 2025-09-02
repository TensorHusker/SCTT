#!/bin/bash

# SCTT Wasmer Deployment Script

set -e

echo "🚀 Building SCTT for Wasmer Edge"

# Install dependencies if needed
if ! command -v wasmer &> /dev/null; then
    echo "Installing Wasmer..."
    curl https://get.wasmer.io -sSfL | sh
fi

# Add WASI target if not present
rustup target add wasm32-wasi

# Build WASM module
echo "📦 Building WASM module..."
cd rust
cargo build --target wasm32-wasi --release --package sctt-wasm

# Create wasmer.toml
cat > wasmer.toml << EOF
[package]
name = "sctt"
version = "0.1.0"
description = "Smooth Cubical Type Theory Engine"
license = "MIT"

[[module]]
name = "sctt"
source = "target/wasm32-wasi/release/sctt_wasm.wasm"
abi = "wasi"

[command]
name = "sctt"
module = "sctt"

[[command]]
name = "typecheck"
module = "sctt"
runner = "wasi"

[[command]]
name = "evaluate"
module = "sctt"
runner = "wasi"
EOF

# Test locally
echo "🧪 Testing WASM module..."
wasmer run target/wasm32-wasi/release/sctt_wasm.wasm

# Deploy to Wasmer Edge (requires account)
echo "☁️ Deploying to Wasmer Edge..."
echo "Run 'wasmer login' first if you haven't already"
# wasmer deploy

echo "✅ SCTT is ready for Wasmer deployment!"
echo ""
echo "To deploy:"
echo "1. Run: wasmer login"
echo "2. Run: wasmer deploy"
echo "3. Your app will be available at: https://your-namespace.wasmer.app"