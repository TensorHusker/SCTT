# SCTT - Smooth Cubical Type Theory (Pure Rust + Leptos)

A revolutionary type theory where calculus meets computation, built entirely in Rust with zero JavaScript dependencies.

## Architecture

This is a pure Rust implementation using:
- **Actix-Web**: High-performance web server
- **Leptos**: Reactive UI framework (compiles to WASM)
- **WASM**: All SCTT modules compile to WebAssembly
- **No npm/JavaScript**: Everything is Rust!

## Project Structure

```
rust/
├── sctt-core/        # Core type system
├── sctt-smooth/      # Smooth mathematics  
├── sctt-cubical/     # Cubical operations
├── sctt-checker/     # Type checker
├── sctt-web/         # Leptos frontend (compiles to WASM)
├── sctt-server/      # Actix-web backend
├── sctt-game/        # Terminal game for learning
└── sctt-playground/  # CLI REPL
```

## Quick Start

### Prerequisites

1. Install Rust: https://rustup.rs/
2. Install cargo-leptos:
```bash
cargo install cargo-leptos
```

### Development

1. Clone the repository:
```bash
git clone https://github.com/yourusername/SCTT.git
cd SCTT/rust
```

2. Run the development server:
```bash
cargo leptos serve
```

3. Open http://localhost:8080 in your browser

The development server includes:
- Hot reloading for Rust code changes
- Automatic WASM compilation
- Live CSS updates

### Production Build

```bash
cargo leptos build --release
```

This creates optimized WASM binaries and a production-ready server.

### Running Individual Components

**CLI REPL:**
```bash
cargo run --bin sctt-playground
```

**Terminal Game:**
```bash
cargo run --bin sctt-game
```

**Type Checker Tests:**
```bash
cargo test -p sctt-checker
```

## Features

### Interactive Playground
- Live type checking in the browser
- Smooth function visualization
- Path exploration tools
- No JavaScript required!

### Type System
- Dependent types
- Smooth modality
- Path types with computational content
- Interval type with De Morgan algebra

### Mathematics
- Symbolic differentiation
- Taylor series expansion
- Path composition
- Coherence verification

## How It Works

1. **Leptos Components** are written in Rust and compiled to WASM
2. **Actix-Web** serves the WASM files and handles API requests
3. **SCTT Modules** provide the type theory implementation
4. **Everything runs in the browser** via WebAssembly

## API Endpoints

The server provides REST APIs for type checking:

```
POST /api/typecheck
{
  "code": "λx. sin(x²)"
}

Response:
{
  "success": true,
  "type": "C∞(ℝ, ℝ)"
}
```

```
POST /api/evaluate
{
  "expression": "sin",
  "value": 3.14159
}

Response:
{
  "success": true,
  "result": 0.0
}
```

## Development Tips

### Watch Mode
```bash
cargo leptos watch
```

### Format Code
```bash
cargo fmt --all
```

### Run Clippy
```bash
cargo clippy --all-targets --all-features
```

### Build Individual WASM Modules
```bash
./build-wasm.sh
```

## Deployment

### Deploy to Production Server

1. Build for release:
```bash
cargo leptos build --release
```

2. Run the server:
```bash
./target/release/sctt-server
```

### Deploy to GitHub Pages (CSR mode)

1. Build for client-side rendering:
```bash
cargo leptos build --release --features csr
```

2. Copy the `target/site` directory to your GitHub Pages repository

## Contributing

We welcome contributions! Please see CONTRIBUTING.md for guidelines.

## License

MIT License - See LICENSE file for details.

## Resources

- [Leptos Documentation](https://leptos.dev)
- [Actix-Web Guide](https://actix.rs)
- [WebAssembly with Rust](https://rustwasm.github.io/book/)
- [SCTT Theory Paper](docs/theory.pdf)

## Troubleshooting

### WASM not loading?
Ensure your browser supports SharedArrayBuffer. Chrome/Firefox work best.

### Compilation errors?
Make sure you have the latest Rust version:
```bash
rustup update
```

### Performance issues?
Build with release mode for optimal performance:
```bash
cargo leptos build --release
```

## Contact

- GitHub: https://github.com/yourusername/SCTT
- Twitter: @sctt_lang
- Discord: https://discord.gg/sctt