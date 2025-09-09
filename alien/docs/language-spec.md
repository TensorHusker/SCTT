# Alien Language Specification

**Alien** is a programming language built on Smooth Cubical Type Theory (SCTT), designed to make advanced type theory accessible and practical for real-world programming.

## Design Philosophy

Alien embraces:
- **Smooth by default**: All functions and paths are infinitely differentiable
- **Cubical native**: Dimensions and paths are first-class constructs
- **Proof-relevant**: Programs carry their correctness proofs
- **Visually intuitive**: Syntax inspired by mathematical notation

## Core Syntax

### Basic Types

```alien
-- Primitive types
Type                           -- Universe of types
Smooth                         -- Smooth types
ℝ                             -- Real numbers (smooth)
ℕ                             -- Natural numbers
𝔹                             -- Booleans

-- Function types
A → B                          -- Function from A to B
(x : A) → B(x)                 -- Dependent function (Pi type)
A ⇒ B                          -- Smooth function (C^∞)

-- Product types  
A × B                          -- Pair type
(x : A) × B(x)                 -- Dependent pair (Sigma type)

-- Path types
Path A x y                     -- Path from x to y in A
x ≡ y                          -- Syntactic sugar for paths
x ∼ y                          -- Smooth path (default)
```

### Smooth Operations

```alien
-- Differentiation
∂ f                            -- Derivative of f
∂ⁿ f                           -- n-th derivative
∇ f                            -- Gradient

-- Integration
∫ f dx                         -- Definite integral
∮ f                            -- Path integral

-- Taylor expansion
taylor f at x order n          -- Taylor series

-- Smooth paths
smooth-path from a to b        -- Construct smooth path
compose p q                    -- Smooth composition
```

### Cubical Constructs

```alien
-- Dimensions
dim i, j, k                    -- Dimension variables
i=0, i=1                       -- Dimension endpoints
i ∧ j                          -- Meet (min)
i ∨ j                          -- Join (max)
~i                             -- Negation (1-i)

-- Path construction
<i> expr                       -- Path lambda
path @ r                       -- Path application

-- Kan operations
comp A [φ ↦ u] base           -- Composition
coe A from r to s              -- Coercion/transport
hcom A [φ ↦ u] base           -- Homogeneous composition
```

### Pattern Matching

```alien
match expr with
| pattern₁ => result₁
| pattern₂ => result₂
| path <i> => result₃         -- Match on paths
```

## Example Programs

### Hello World with Types

```alien
module Hello where

main : IO Unit
main = print "Hello, Alien World! 👽"

-- With dependent types
greet : (name : String) → IO Unit
greet name = print ("Hello, " ++ name ++ "!")
```

### Smooth Functions

```alien
-- A smooth function
smooth-fn : ℝ ⇒ ℝ
smooth-fn x = x³ - 2x + 1

-- Its derivative (computed automatically)
smooth-fn' : ℝ ⇒ ℝ
smooth-fn' = ∂ smooth-fn       -- 3x² - 2

-- Verify smoothness
proof : IsSmooth smooth-fn
proof = auto                   -- Automatic proof
```

### Path Types and Homotopy

```alien
-- Define equality via paths
reflPath : (A : Type) → (x : A) → Path A x x
reflPath A x = <i> x

-- Function extensionality
funext : {A B : Type} → (f g : A → B) →
         ((x : A) → Path B (f x) (g x)) →
         Path (A → B) f g
funext f g h = <i> λ x → h x @ i

-- Circle type (HIT)
data S¹ : Type where
  base : S¹
  loop : Path S¹ base base

-- Map from circle to itself
double : S¹ → S¹
double = λ x → match x with
  | base => base
  | loop @ i => (loop ∙ loop) @ i  -- Loop twice
```

### Differential Geometry

```alien
-- Smooth manifold as a type
Manifold : Type → Type
Manifold M = (x : M) → SmoothStructure x

-- Tangent bundle
TangentBundle : Manifold M → Type
TangentBundle M = (x : M) × TangentSpace M x

-- Vector field
VectorField : Manifold M → Type
VectorField M = (x : M) → TangentSpace M x

-- Differential forms
DiffForm : ℕ → Manifold M → Type
DiffForm n M = smooth (∧ⁿ (T* M))

-- Integration of forms
integrate : DiffForm n M → Region M n → ℝ
integrate ω R = ∫[R] ω
```

### Quantum Computing

```alien
-- Quantum state type
Qubit : Type
Qubit = Sphere 2              -- Bloch sphere

-- Quantum gates as smooth paths
Hadamard : Qubit ⇒ Qubit
Hadamard q = smooth-rotate q (π/4)

-- Entanglement as higher paths
Entangle : Qubit × Qubit → Path² (Qubit × Qubit)
Entangle (q₁, q₂) = <i j> 
  let phase = smooth i * j in
  (rotate q₁ phase, rotate q₂ (-phase))
```

### Machine Learning

```alien
-- Neural network layer
Layer : ℕ → ℕ → Type
Layer n m = Matrix ℝ n m × Vector ℝ m

-- Smooth activation
activation : ℝ ⇒ ℝ
activation x = tanh x          -- Smooth by default

-- Backpropagation via automatic differentiation
train : Network → Dataset → Network
train net data = 
  let loss = smooth-loss net data in
  let grad = ∇ loss in          -- Automatic gradient
  update net grad
```

## Type System Features

### Dependent Types
```alien
Vec : Type → ℕ → Type
Vec A zero = Unit
Vec A (suc n) = A × Vec A n

-- Length-indexed append
append : {A : Type} {n m : ℕ} →
         Vec A n → Vec A m → Vec A (n + m)
```

### Smooth Types
```alien
-- Smooth dependent type
SmoothDep : (x : ℝ) → Smooth (Type x)

-- Smooth family of types varying continuously
TypeFamily : ℝ ⇒ Type
TypeFamily t = if t < 0 then ℝ else ℂ  -- Smooth transition
```

### Higher Inductive Types
```alien
-- Torus as HIT
data Torus : Type where
  point : Torus
  meridian : Path Torus point point
  longitude : Path Torus point point
  surface : Path² Torus 
            (meridian ∙ longitude)
            (longitude ∙ meridian)
```

## Module System

```alien
module MyModule (param : Type) where
  -- Private definitions
  private helper : param → param
  helper x = x

  -- Public exports
  public myFunction : param → Type
  myFunction x = Path param x (helper x)

-- Import modules
import MyModule ℝ as M
open M                        -- Bring into scope
```

## Effects and IO

```alien
-- Effect system based on smooth modalities
effect IO : Type → Type
effect State S : Type → Type
effect Smooth : Type → Type   -- Computations with smoothness

-- Monadic syntax
do
  x <- readLine
  let y = process x
  print y
  return y
```

## Proof Tactics

```alien
-- Automatic proof search
theorem : (A : Type) → A → A
theorem A x = auto

-- Interactive proof mode
lemma : Path ℝ (1 + 1) 2
lemma = proof
  refl                         -- Reflexivity
  compute                      -- Reduce to normal form
  qed
```

## Compilation Targets

Alien can compile to:
- **Native code**: Via LLVM with smoothness optimizations
- **JavaScript**: For web deployment
- **WASM**: For portable execution
- **Coq/Agda**: For formal verification
- **LaTeX**: For mathematical documentation

## Standard Library

```alien
-- Prelude (auto-imported)
Alien.Prelude
  - Basic types and operations
  - Path operations
  - Smooth functions

-- Mathematics
Alien.Math.Analysis          -- Calculus, differential equations
Alien.Math.Topology          -- Topological spaces, homotopy
Alien.Math.Algebra           -- Groups, rings, fields
Alien.Math.Category          -- Category theory

-- Computing
Alien.Data.Structures        -- Lists, trees, graphs
Alien.Algorithm              -- Smooth algorithms
Alien.Quantum                -- Quantum computing
Alien.ML                     -- Machine learning

-- Physics
Alien.Physics.Classical      -- Mechanics, E&M
Alien.Physics.Quantum        -- QM, QFT
Alien.Physics.Relativity     -- GR with smooth spacetime
```

## Language Goals

1. **Accessibility**: Make SCTT practical for everyday programming
2. **Performance**: Optimize smooth operations at compile time
3. **Safety**: Type-level guarantees for numerical stability
4. **Interoperability**: FFI with existing languages
5. **Visual Tools**: IDE support with visual path editors