# GitHub Copilot Instructions for SCTT

## Project Overview
This repository contains the Smooth Cubical Type Theory (SCTT) system - a production-ready proof assistant and compiler featuring advanced type checking, WASM compilation, and collaborative proof development.

## Key Technologies
- **Languages**: Rust (primary), Python (for learning/playground), JavaScript/TypeScript (web interface)
- **Frameworks**: Yew (web UI), Trunk (build tool), wasm-pack (WASM compilation)
- **Deployment**: Vercel, Netlify, Docker support
- **Type Theory**: Homotopy type theory with smooth cubical structure

## Coding Standards

### Rust Code
- Follow Rust idioms and best practices
- Use descriptive variable names reflecting type theory concepts
- Implement proper error handling with Result types
- Add documentation comments for public APIs
- Ensure memory safety and zero-cost abstractions

### Type Theory Code
- Use standard mathematical notation in comments
- Follow bidirectional type checking patterns
- Maintain universe hierarchy consistency
- Implement path types and homotopy concepts correctly

### Python Code
- Located in `python/` directory, primarily for educational purposes
- Follow PEP 8 style guidelines
- Include type hints where appropriate
- Document mathematical concepts clearly

## Project Structure
```
src/
├── sctt_typechecker.rs   # Core type checking engine
├── sctt_to_wasm.rs       # WASM compiler with proof certificates
├── proof_assistant.rs    # Interactive theorem prover
├── web_interface.rs      # Yew-based web UI
├── collaborative.rs      # Real-time collaboration
├── visualization.rs      # Proof and type visualization
└── bin/
    └── sctt-server.rs    # API server with WebSocket support
```

## Key Concepts to Maintain
1. **Type Safety**: All code must maintain strong type guarantees
2. **Proof Correctness**: Never compromise on proof verification
3. **Performance**: Optimize for O(n log n) typical case in type checking
4. **WebAssembly**: Ensure efficient compilation with embedded proof certificates
5. **Mathematical Rigor**: Maintain formal correctness in all implementations

## Common Patterns

### Type Checking
```rust
// Bidirectional type checking pattern
fn check(ctx: &Context, term: &Term, ty: &Type) -> Result<(), Error>
fn infer(ctx: &Context, term: &Term) -> Result<Type, Error>
```

### Path Types
```rust
// Path type construction
Path : (A : Type) → A → A → Type
refl : Π(A : Type). Π(a : A). Path A a a
```

### Proof Development
- Use tactics for interactive theorem proving
- Support automation where possible
- Maintain proof certificates for verification

## Testing Guidelines
- Write unit tests for all type checking functions
- Include property-based tests for type theory invariants
- Test WASM compilation output
- Verify proof certificates are correctly embedded

## Documentation Requirements
- Document all public APIs
- Include examples for complex type theory concepts
- Explain mathematical foundations in comments
- Provide usage examples in doc comments

## Performance Considerations
- Type checking: Target O(n³) worst case, O(n log n) typical
- Compilation: Maintain > 10,000 lines/second throughput
- Proof verification: Keep under 1ms per function call
- Proof size overhead: Limit to < 20% of code size

## Security Considerations
- Never expose internal proof checking logic to untrusted input
- Validate all user-provided type definitions
- Ensure WASM output is sandboxed appropriately
- Protect collaborative editing sessions with proper authentication

## Development Workflow
1. Implement type theory features with formal correctness
2. Add comprehensive tests for new functionality
3. Document mathematical concepts and usage
4. Optimize performance without compromising correctness
5. Ensure WASM compilation preserves semantics

## Important Files
- `CLAUDE.md` - AI assistant instructions
- `README.md` - Project overview and setup
- `DEVELOPMENT_PLAN.md` - Development roadmap
- `HOW_TO_CONTRIBUTE.md` - Contribution guidelines

## Deployment Notes
- Supports Vercel, Netlify, and Docker deployments
- WebAssembly builds require `trunk` and `wasm-pack`
- Python playground requires separate dependency installation
- Server component uses WebSocket for real-time collaboration

## Common Commands
```bash
# Development
trunk serve --open

# Build
trunk build --release

# Python playground
python python/playground/smooth_playground.py

# Deploy
npm run deploy:vercel
```

## References
- Homotopy Type Theory foundations
- Cubical Type Theory papers in `docs/theory/`
- Example proofs in `examples/`
- Interactive tutorial at deployed site `/tutorial`