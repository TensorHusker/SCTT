#!/bin/bash

# Build script for SCTT with Leptos

set -e

echo "Building SCTT with Leptos..."

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

# Install cargo-leptos if not present
if ! command -v cargo-leptos &> /dev/null; then
    echo -e "${YELLOW}Installing cargo-leptos...${NC}"
    cargo install cargo-leptos
fi

# Install wasm32 target if not present
if ! rustup target list --installed | grep -q wasm32-unknown-unknown; then
    echo -e "${YELLOW}Installing wasm32-unknown-unknown target...${NC}"
    rustup target add wasm32-unknown-unknown
fi

# Build with cargo-leptos
echo -e "${GREEN}Building SCTT...${NC}"
cargo leptos build --release

echo -e "${GREEN}Build complete!${NC}"
echo "Run 'cargo leptos serve' to start the development server"