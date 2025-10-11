# Appendix D: SCTT Standard Library

> "Don't reinvent the wheel - but do understand how it turns."

## Status: In Development

The SCTT Standard Library is currently under active development as part of the main SCTT implementation. Rather than duplicate the documentation here, we point you to the canonical sources.

---

## Library Structure

The SCTT Standard Library is organized into modules:

### Core Modules
- **Prelude**: Essential types and functions (automatically imported)
- **Data**: Standard data structures (Nat, List, Vec, Tree, etc.)
- **Function**: Function combinators and utilities
- **Logic**: Propositional and predicate logic

### Type Theory Modules
- **Path**: Path operations, composition, inversion
- **Equiv**: Equivalences and univalence utilities
- **HITs**: Higher inductive types (Circle, Suspension, etc.)
- **Truncation**: Propositional truncation and h-levels

### Smooth Modules
- **Smooth**: Smooth types and functions
- **Calculus**: Differentiation and integration
- **Manifold**: Smooth manifolds and tangent bundles
- **Forms**: Differential forms and exterior calculus

### Applied Modules
- **LinearAlgebra**: Vectors, matrices, linear maps
- **Numerical**: Numerical methods with verified bounds
- **Physics**: Physical models and simulations
- **Optimization**: Optimization algorithms

---

## Accessing the Library

### Online Documentation
Visit **[sctt-lang.org/docs/stdlib](https://sctt-lang.org/docs/stdlib)** for:
- Complete API documentation
- Usage examples
- Source code with annotations
- Tutorial and guides

### From the SCTT REPL
```sctt
-- Import a module
import Data.Vec

-- List available functions
:browse Data.Vec

-- View documentation for a function
:doc Vec.replicate

-- View source
:source Vec.map
```

### From Source
Clone the repository:
```bash
git clone https://github.com/tensorhusker/SCTT
cd SCTT/stdlib
```

---

## Quick Reference

### Common Imports

```sctt
-- Basic types
import Data.Nat
import Data.Bool
import Data.List

-- Dependent types
import Data.Vec        -- Length-indexed vectors
import Data.Fin        -- Bounded natural numbers

-- Path types
import Path.Base       -- Path operations
import Path.Equiv      -- Equivalences

-- Smooth types
import Smooth.Real     -- Smooth real numbers
import Smooth.Calculus -- Differentiation/integration

-- Applications
import Numerical.ODE   -- ODE solvers
import Physics.Mechanics  -- Classical mechanics
```

### Core Functions

**From Prelude**:
```sctt
id : {A : Type} → A → A
const : {A B : Type} → A → B → A
comp : {A B C : Type} → (B → C) → (A → B) → (A → C)
flip : {A B C : Type} → (A → B → C) → (B → A → C)
```

**From Data.List**:
```sctt
map : {A B : Type} → (A → B) → List A → List B
filter : {A : Type} → (A → Bool) → List A → List A
foldl : {A B : Type} → (B → A → B) → B → List A → B
foldr : {A B : Type} → (A → B → B) → B → List A → B
```

**From Path.Base**:
```sctt
refl : {A : Type} {x : A} → Path A x x
sym : {A : Type} {x y : A} → Path A x y → Path A y x
trans : {A : Type} {x y z : A} → Path A x y → Path A y z → Path A x z
ap : {A B : Type} {x y : A} → (f : A → B) → Path A x y → Path B (f x) (f y)
transport : {A B : Type} → Path Type A B → A → B
```

**From Smooth.Calculus**:
```sctt
D : C∞(ℝ, ℝ) → C∞(ℝ, ℝ)
gradient : {n : ℕ} → C∞(ℝⁿ, ℝ) → C∞(ℝⁿ, ℝⁿ)
integral : ℝ → ℝ → C∞(ℝ, ℝ) → ℝ
```

---

## Contributing to the Library

We welcome contributions! See [CONTRIBUTING.md](https://github.com/tensorhusker/SCTT/blob/main/CONTRIBUTING.md) for:
- Coding standards
- Documentation requirements
- Testing procedures
- Proof obligations

### Areas Needing Development
- Additional differential geometry primitives
- Numerical linear algebra
- Statistics and probability
- Machine learning primitives
- Physics simulation frameworks
- Cryptographic primitives (post-quantum)

---

## Design Philosophy

The SCTT Standard Library follows these principles:

1. **Correctness First**: Every function comes with specifications and proofs
2. **Performance Aware**: Efficient implementations with complexity annotations
3. **Beginner Friendly**: Clear documentation and examples
4. **Composable**: Small, reusable components
5. **Verified**: Critical algorithms have formal correctness proofs

---

## Version Information

- **Current Version**: 0.1.0 (alpha)
- **API Stability**: Experimental (expect breaking changes)
- **Documentation Coverage**: ~60% and growing
- **Test Coverage**: ~75%

---

## Getting Help

### Documentation
- API Docs: [sctt-lang.org/docs](https://sctt-lang.org/docs)
- Tutorials: [sctt-lang.org/learn](https://sctt-lang.org/learn)
- Examples: [github.com/tensorhusker/SCTT/tree/main/examples](https://github.com/tensorhusker/SCTT/tree/main/examples)

### Community
- Discussions: [github.com/tensorhusker/SCTT/discussions](https://github.com/tensorhusker/SCTT/discussions)
- Issues: [github.com/tensorhusker/SCTT/issues](https://github.com/tensorhusker/SCTT/issues)
- Chat: Matrix/Discord (links in repository README)

---

## Roadmap

### Near Term (v0.2)
- Complete coverage of basic data structures
- Full path algebra library
- Numerical methods with error bounds
- Comprehensive testing framework

### Medium Term (v0.3)
- Advanced differential geometry
- Machine learning primitives
- Physics simulations
- Optimization algorithms

### Long Term (v1.0)
- Stable API
- Complete documentation
- Extensive example library
- Production-ready implementations

---

*This appendix will be updated as the standard library develops. Check the online documentation for the most current information.*

*Return to: [Table of Contents](./SUMMARY.md) | [Bibliography](./bibliography.md)*
