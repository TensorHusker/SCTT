#!/bin/bash

# SCTT Development Environment Setup Script
# Sets up Cubical Agda, Lean 4 with mathlib4, and related tools

set -e

echo "=================================="
echo "SCTT Proof Tools Installation"
echo "=================================="

# Detect OS
OS="$(uname -s)"
ARCH="$(uname -m)"

# Color output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

success() { echo -e "${GREEN}✓${NC} $1"; }
error() { echo -e "${RED}✗${NC} $1"; exit 1; }
info() { echo -e "${YELLOW}→${NC} $1"; }

# Check for package managers
check_homebrew() {
    if ! command -v brew &> /dev/null; then
        info "Installing Homebrew..."
        /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
    else
        success "Homebrew is installed"
    fi
}

# Install Agda and Cubical library
install_agda() {
    info "Installing Agda..."
    
    if [[ "$OS" == "Darwin" ]]; then
        check_homebrew
        brew install agda
    elif [[ "$OS" == "Linux" ]]; then
        # Try apt first, then other package managers
        if command -v apt &> /dev/null; then
            sudo apt update
            sudo apt install -y agda agda-stdlib
        elif command -v dnf &> /dev/null; then
            sudo dnf install -y Agda
        else
            error "Unsupported Linux distribution. Please install Agda manually."
        fi
    else
        error "Unsupported OS: $OS"
    fi
    
    success "Agda installed"
    
    # Install Cubical library
    info "Installing Cubical Agda library..."
    mkdir -p ~/.agda
    cd ~/.agda
    
    if [ ! -d "cubical" ]; then
        git clone https://github.com/agda/cubical.git
        cd cubical
        git checkout master
    else
        cd cubical
        git pull
    fi
    
    # Create libraries file
    echo "$HOME/.agda/cubical/cubical.agda-lib" > ~/.agda/libraries
    echo "$HOME/.agda/agda-stdlib/standard-library.agda-lib" >> ~/.agda/libraries 2>/dev/null || true
    
    success "Cubical Agda library installed"
}

# Install Lean 4
install_lean() {
    info "Installing Lean 4..."
    
    if ! command -v elan &> /dev/null; then
        info "Installing elan (Lean version manager)..."
        curl https://raw.githubusercontent.com/leanprover/elan/master/elan-init.sh -sSf | sh -s -- -y
        source $HOME/.elan/env
    else
        success "elan is already installed"
    fi
    
    # Install latest stable Lean 4
    elan default leanprover/lean4:stable
    success "Lean 4 installed"
    
    # Create a mathlib4 project template
    info "Setting up mathlib4 project template..."
    cd "$OLDPWD"
    mkdir -p lean-projects
    cd lean-projects
    
    if [ ! -d "sctt-lean" ]; then
        lake new sctt-lean math
        cd sctt-lean
        lake update
        lake exe cache get
        success "mathlib4 project created"
    else
        info "Lean project already exists"
    fi
}

# Install Python dependencies for ML/data analysis
install_python_deps() {
    info "Installing Python dependencies..."
    
    cd "$OLDPWD"
    if [ -f "python/requirements.txt" ]; then
        pip3 install -r python/requirements.txt
    fi
    
    # Additional ML/proof-related packages
    pip3 install --upgrade \
        numpy \
        pandas \
        matplotlib \
        torch \
        jupyter \
        networkx \
        sympy \
        z3-solver
    
    success "Python dependencies installed"
}

# Install Rust dependencies
install_rust_deps() {
    info "Checking Rust installation..."
    
    if ! command -v cargo &> /dev/null; then
        info "Installing Rust..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source $HOME/.cargo/env
    else
        success "Rust is installed"
    fi
    
    # Install wasm target for web deployment
    rustup target add wasm32-unknown-unknown
    
    # Install trunk for web development
    cargo install trunk
    
    success "Rust dependencies installed"
}

# Setup project structure
setup_project_structure() {
    info "Setting up project structure..."
    
    cd "$OLDPWD"
    
    # Create directories for different proof systems
    mkdir -p proofs/agda
    mkdir -p proofs/lean
    mkdir -p proofs/coq
    
    # Create data directories
    mkdir -p data/runetika
    mkdir -p data/libertalia
    
    # Create ML pipeline directories
    mkdir -p ml/models
    mkdir -p ml/datasets
    mkdir -p ml/experiments
    
    success "Project structure created"
}

# Create initial Agda files
create_agda_starter() {
    info "Creating Agda starter files..."
    
    cat > proofs/agda/SCTT.agda << 'EOF'
{-# OPTIONS --cubical --safe #-}
module SCTT where

open import Cubical.Foundations.Prelude
open import Cubical.Foundations.Univalence
open import Cubical.Foundations.HLevels
open import Cubical.Data.Nat

-- Smooth Cubical Type Theory foundations
-- This module defines the core structures for SCTT

-- Smooth structure on types
record SmoothStructure {ℓ} (A : Type ℓ) : Type (ℓ-suc ℓ) where
  field
    -- Tangent bundle
    T : A → Type ℓ
    -- Differential structure
    d : {x y : A} → (x ≡ y) → T x → T y
    -- Chain rule
    chain : {x y z : A} (p : x ≡ y) (q : y ≡ z) (v : T x)
          → d (p ∙ q) v ≡ d q (d p v)

-- Example: Smooth structure on the circle
module CircleSmooth where
  open import Cubical.HITs.S1
  
  -- Define smooth structure for S¹
  S¹-smooth : SmoothStructure S¹
  S¹-smooth = record
    { T = λ _ → ℝ  -- Tangent space is ℝ at each point
    ; d = λ p v → v  -- Simplified for now
    ; chain = λ p q v → refl
    }
    where
      ℝ : Type₀
      ℝ = Unit  -- Placeholder for real numbers
EOF
    
    success "Agda starter files created"
}

# Create initial Lean files
create_lean_starter() {
    info "Creating Lean starter files..."
    
    cat > lean-projects/sctt-lean/SCTT.lean << 'EOF'
import Mathlib.Topology.Basic
import Mathlib.Topology.Homotopy.Basic
import Mathlib.CategoryTheory.Category.Basic

namespace SCTT

-- Smooth Cubical Type Theory in Lean 4
-- Core definitions and structures

/-- A smooth structure on a type -/
structure SmoothStructure (α : Type*) where
  tangentBundle : α → Type*
  differential : ∀ {x y : α}, (x = y) → tangentBundle x → tangentBundle y
  chainRule : ∀ {x y z : α} (p : x = y) (q : y = z) (v : tangentBundle x),
    differential (p.trans q) v = differential q (differential p v)

/-- Example: Smooth structure on the unit interval -/
def unitIntervalSmooth : SmoothStructure (Set.Icc (0:ℝ) 1) where
  tangentBundle := fun _ => ℝ
  differential := fun _ v => v
  chainRule := fun _ _ _ => rfl

/-- Integration with mathlib's extensive proof library -/
example : (2 : ℕ) + 2 = 4 := by norm_num

end SCTT
EOF
    
    success "Lean starter files created"
}

# Main installation flow
main() {
    echo ""
    info "Starting installation process..."
    echo ""
    
    # Create backup of existing setup if it exists
    if [ -d ~/.agda ]; then
        info "Backing up existing Agda configuration..."
        cp -r ~/.agda ~/.agda.backup.$(date +%Y%m%d)
    fi
    
    # Run installations
    install_agda
    echo ""
    install_lean
    echo ""
    install_python_deps
    echo ""
    install_rust_deps
    echo ""
    setup_project_structure
    echo ""
    create_agda_starter
    echo ""
    create_lean_starter
    
    echo ""
    echo "=================================="
    success "Installation complete!"
    echo "=================================="
    echo ""
    echo "Next steps:"
    echo "1. Source your shell configuration:"
    echo "   source ~/.bashrc  # or ~/.zshrc"
    echo ""
    echo "2. Test Agda installation:"
    echo "   agda proofs/agda/SCTT.agda"
    echo ""
    echo "3. Test Lean installation:"
    echo "   cd lean-projects/sctt-lean && lake build"
    echo ""
    echo "4. Start Jupyter for interactive development:"
    echo "   jupyter notebook"
    echo ""
}

# Run main function
main