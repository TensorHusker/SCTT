# Alien Programming Language 👽

A revolutionary programming language built on **Smooth Cubical Type Theory (SCTT)**, bringing advanced mathematics and type theory to practical programming.

## Features

### 🌊 Smooth by Default
- All functions are C^∞ smooth (infinitely differentiable)
- Automatic differentiation built into the type system
- Native support for calculus operations (∂, ∇, ∫)

### 🔷 Cubical Native
- First-class paths and higher-dimensional types
- Built-in Kan operations (composition, transport)
- Univalence as a programming principle

### 🔬 Mathematical Power
- Differential geometry (manifolds, forms, connections)
- Quantum computing with type-safe operations
- Category theory constructs

### ✅ Proof-Carrying Code
- Programs include their correctness proofs
- Automatic proof search with `auto` tactic
- Interactive theorem proving support

## Quick Start

```alien
-- Hello world with dependent types
greet : (name : String) → IO Unit
greet name = print ("Hello, " ++ name ++ " from Alien! 👽")

-- Smooth function with automatic differentiation
f : ℝ ⇒ ℝ
f x = x³ - 2x + 1

f' : ℝ ⇒ ℝ
f' = ∂ f  -- Automatically computed: 3x² - 2

-- Path types for equality
theorem : Path ℝ (2 + 2) 4
theorem = refl
```

## Installation

```bash
# Clone the repository
git clone https://github.com/TensorHusker/SCTT.git
cd SCTT/alien

# Build the compiler (requires Rust)
cargo build --release

# Run the REPL
cargo run --bin alien-repl

# Compile a program
cargo run --bin aliencc examples/basic.alien
```

## Language Overview

### Types

| Type | Description | Example |
|------|-------------|---------|
| `Type` | Universe of types | `Type` |
| `ℝ` | Real numbers (smooth) | `3.14159` |
| `ℕ` | Natural numbers | `42` |
| `𝔹` | Booleans | `true`, `false` |
| `A → B` | Function type | `ℝ → ℝ` |
| `A ⇒ B` | Smooth function | `ℝ ⇒ ℝ` |
| `Path A x y` | Path from x to y | `Path ℝ 0 1` |
| `(x : A) → B(x)` | Dependent function | `(n : ℕ) → Vec ℝ n` |

### Smooth Operations

```alien
-- Differentiation
∂ f           -- First derivative
∂ⁿ f          -- nth derivative
∇ f           -- Gradient

-- Integration
∫ f dx from a to b    -- Definite integral
∮ f                   -- Path integral

-- Taylor expansion
taylor f at x order n
```

### Cubical Features

```alien
-- Path construction
<i> expr              -- Path lambda
path @ r              -- Path application

-- Kan operations
comp A [φ ↦ u] base   -- Composition
coe A from r to s     -- Transport
```

## Examples

### Differential Geometry

```alien
-- Define a smooth manifold
S² : Manifold
S² = { (x,y,z) : ℝ³ | x² + y² + z² ≡ 1 }

-- Compute sphere area
sphereArea : ℝ
sphereArea = ∫[S²] areaForm  -- Result: 4π
```

### Quantum Computing

```alien
-- Quantum teleportation
teleport : Qubit → EntangledPair → IO Qubit
teleport ψ (epr₁, epr₂) = do
  (b₁, b₂) <- bellMeasure ψ epr₁
  let corrected = applyCorrection b₁ b₂ epr₂
  return corrected
```

### Machine Learning

```alien
-- Neural network with smooth activation
layer : Matrix ℝ n m ⇒ Vector ℝ n ⇒ Vector ℝ m
layer W x = tanh (W * x)  -- Smooth by default!

-- Automatic backpropagation
train network data = 
  let loss = smooth-loss network data in
  let grad = ∇ loss in  -- Automatic!
  update network grad
```

## Architecture

```
alien/
├── src/
│   ├── lexer.rs        # Unicode-aware tokenization
│   ├── parser.rs       # SCTT-aware parsing
│   ├── ast.rs          # Abstract syntax tree
│   ├── typecheck.rs    # Bidirectional type checking
│   ├── eval.rs         # Normalization by evaluation
│   └── codegen.rs      # LLVM code generation
├── examples/           # Example programs
├── stdlib/            # Standard library
└── tests/            # Test suite
```

## Standard Library

- `Alien.Prelude` - Basic types and operations
- `Alien.Math.Analysis` - Calculus and differential equations
- `Alien.Math.Topology` - Topological spaces and homotopy
- `Alien.Physics.Quantum` - Quantum mechanics and computing
- `Alien.ML` - Machine learning primitives

## Contributing

We welcome contributions! Areas of interest:

- [ ] Parser implementation
- [ ] Type checker completion
- [ ] LLVM backend
- [ ] IDE support (VSCode, Emacs)
- [ ] More standard library modules
- [ ] Documentation and tutorials

## Theory

Alien is based on Smooth Cubical Type Theory (SCTT), which extends:
- Cubical Type Theory with smooth structure
- Homotopy Type Theory with differentiability
- Dependent types with automatic differentiation

## License

MIT License - See LICENSE file

## Contact

- GitHub: https://github.com/TensorHusker/SCTT
- Discord: [Join our community](https://discord.gg/alien-lang)

---

*"Making advanced mathematics as easy as 👽"*