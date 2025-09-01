#!/bin/bash
# Setup script for SCTT development environment

set -e

echo "🚀 Setting up SCTT development environment..."

# Check for Python
if command -v python3 &> /dev/null; then
    echo "✓ Python3 found"
    cd python
    echo "Installing Python dependencies..."
    pip3 install -r requirements.txt
    cd ..
else
    echo "⚠ Python3 not found. Please install Python 3.8+"
fi

# Check for Rust
if command -v cargo &> /dev/null; then
    echo "✓ Rust found"
    cd rust
    echo "Building Rust workspace..."
    cargo build --workspace
    cd ..
else
    echo "⚠ Rust not found. Install from https://rustup.rs"
fi

echo ""
echo "✨ Setup complete!"
echo ""
echo "Next steps:"
echo "  Python: cd python && python playground/starter_math_and_code.py"
echo "  Rust:   cd rust && cargo run --bin sctt-playground"
echo ""
echo "Happy exploring! 🎉"