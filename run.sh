#!/bin/bash

# SCTT Development Server Runner

set -e

echo "🚀 Starting SCTT Development Environment"

# Create necessary directories
mkdir -p static
mkdir -p content/posts
mkdir -p data/runetika
mkdir -p data/libertalia

# Build the Rust server
echo "📦 Building Rust server..."
cd rust
cargo build --bin sctt-server --release

# Start the server
echo "🌐 Starting server on http://localhost:8080"
./target/release/sctt-server