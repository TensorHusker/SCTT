# SCTT Project Structure

## Directory Organization

```
SCTT/
├── docs/                     # Language-agnostic documentation
│   ├── theory/              # Mathematical theory
│   ├── guides/              # Learning materials
│   └── research/            # Research notes
│
├── python/                   # Python implementation
│   ├── src/                 # Source code
│   │   ├── core/           # Core type system
│   │   ├── smooth/         # Smooth structure
│   │   ├── cubical/        # Cubical operations
│   │   └── checker/        # Type checker
│   ├── playground/          # Interactive experiments
│   ├── tests/              # Test suite
│   └── requirements.txt    # Dependencies
│
├── rust/                     # Rust implementation
│   ├── sctt-core/          # Core library
│   ├── sctt-smooth/        # Smooth mathematics
│   ├── sctt-cubical/       # Cubical type theory
│   ├── sctt-checker/       # Type checker
│   ├── sctt-playground/    # Interactive REPL
│   └── Cargo.toml          # Workspace configuration
│
├── shared/                   # Shared resources
│   ├── examples/           # Example SCTT programs
│   ├── problems/           # Problem sets
│   └── data/               # Test data
│
└── scripts/                  # Build and utility scripts
    ├── setup.sh            # Environment setup
    ├── test_all.sh         # Run all tests
    └── benchmark.sh        # Performance comparison
```

## Language Choice Rationale

### Python for:
- Rapid prototyping
- Mathematical exploration
- Learning and visualization
- Integration with SymPy, NumPy, Matplotlib

### Rust for:
- Production implementation
- Performance-critical type checking
- Memory-safe core algorithms
- Future compiler backend

## Getting Started

### Python Path
```bash
cd python
pip install -r requirements.txt
python playground/start_here.py
```

### Rust Path
```bash
cd rust
cargo build --workspace
cargo run --bin sctt-playground
```

### Both Languages
```bash
./scripts/setup.sh          # Set up everything
./scripts/test_all.sh       # Test both implementations
```