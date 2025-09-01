#!/bin/bash

# Build script for compiling SCTT Rust modules to WASM

set -e

echo "Building SCTT WASM modules..."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to build a module
build_module() {
    local module=$1
    echo -e "${YELLOW}Building ${module}...${NC}"
    
    cd "$module"
    
    # Build with wasm-pack
    if command -v wasm-pack &> /dev/null; then
        wasm-pack build --target web --out-dir ../../blog/wasm --no-typescript
    else
        # Fallback to cargo with wasm32 target
        cargo build --target wasm32-unknown-unknown --release
        
        # Copy wasm files to blog directory
        mkdir -p ../../blog/wasm
        cp target/wasm32-unknown-unknown/release/*.wasm ../../blog/wasm/
    fi
    
    cd ..
    echo -e "${GREEN}✅ ${module} built successfully${NC}"
}

# Install wasm32 target if not present
if ! rustup target list --installed | grep -q wasm32-unknown-unknown; then
    echo "Installing wasm32-unknown-unknown target..."
    rustup target add wasm32-unknown-unknown
fi

# Build each module
modules=("sctt-core" "sctt-smooth" "sctt-cubical" "sctt-checker")

for module in "${modules[@]}"; do
    if [ -d "$module" ]; then
        build_module "$module"
    else
        echo -e "${RED}Module ${module} not found${NC}"
    fi
done

echo -e "${GREEN}All modules built successfully!${NC}"
echo "WASM files are in blog/wasm/"

# Optional: Install wasm-pack if not present
if ! command -v wasm-pack &> /dev/null; then
    echo -e "${YELLOW}Note: wasm-pack is not installed. Install it for better WASM builds:${NC}"
    echo "curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh"
fi