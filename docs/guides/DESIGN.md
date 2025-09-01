# Smooth Cubical Type Theory - Design Document

## Overview

This document provides a high-level overview of Smooth Cubical Type Theory (SCTT). For detailed information, see:
- [THEORY.md](THEORY.md) - Mathematical foundations and type theory
- [IMPLEMENTATION.md](IMPLEMENTATION.md) - Implementation architecture and code design

## 1. Mathematical Foundations

### 1.1 Core Type Theory

#### Base Types
- **Interval Type** `I`: The unit interval [0,1] with endpoints 0, 1
- **Universe** `U`: Type of small types
- **Dependent Functions** `Π(x:A).B(x)`
- **Dependent Pairs** `Σ(x:A).B(x)`

#### Cubical Structure
- **Path Types**: `Path A a b` representing paths from `a` to `b` in type `A`
- **Composition**: `comp`
- **Transport**: `transp`
- **Homogeneous composition**: `hcomp`

### 1.2 Smooth Structure

#### Smooth Interval
Extend the interval type `I` with smooth structure:
- **Smooth maps**: `I^n → I` representing C^∞ functions
- **Derivatives**: First-class derivative operations
- **Taylor series**: Representation of smooth functions via power series

#### Smooth Types
- **Manifold types**: Types with local Euclidean structure
- **Vector bundles**: Dependent types with vector space fibers
- **Differential forms**: Types representing differential k-forms

### 1.3 Key Innovations

#### Smooth Path Types
```
SmoothPath : (A : SmoothType) → (a b : A) → Type
```
Paths that are smooth with respect to the smooth structure on A.

#### Tangent Bundles
```
T : SmoothType → SmoothType
T(M) = Σ(x:M). TangentSpace(M, x)
```

#### Integration
```
∫ : {n : ℕ} → DifferentialForm(n) → Cube(n) → ℝ
```

## 2. Type System Design

### 2.1 Judgments

```
Γ ⊢ A : Type         -- A is a type in context Γ
Γ ⊢ A : SmoothType   -- A is a smooth type
Γ ⊢ a : A            -- a has type A
Γ ⊢ f : C^∞(A, B)    -- f is a smooth map from A to B
```

### 2.2 Structural Rules

#### Smooth Function Types
```
Γ ⊢ A : SmoothType    Γ, x:A ⊢ B : SmoothType
------------------------------------------------
        Γ ⊢ (x:A) →^∞ B : SmoothType
```

#### Differentiation
```
Γ ⊢ f : (x:ℝ^n) →^∞ ℝ^m
-------------------------
Γ ⊢ Df : (x:ℝ^n) → Lin(ℝ^n, ℝ^m)
```

### 2.3 Computation Rules

#### β-reduction for smooth functions
```
((λ^∞ x:A. b) a) ≡ b[a/x]
```

#### Chain rule
```
D(g ∘ f) ≡ (Dg ∘ f) · Df
```

## 3. Implementation Architecture

### 3.1 Core Language

#### Abstract Syntax
```typescript
type Term = 
  | { tag: "var", name: string }
  | { tag: "lambda", param: string, type: Term, body: Term }
  | { tag: "app", func: Term, arg: Term }
  | { tag: "pi", param: string, type: Term, body: Term }
  | { tag: "sigma", param: string, type: Term, body: Term }
  | { tag: "interval" }
  | { tag: "path", type: Term, left: Term, right: Term }
  | { tag: "smooth", inner: Term }
  | { tag: "diff", func: Term }
  // ... more constructors
```

#### Type Checking
```typescript
function typeCheck(ctx: Context, term: Term): Term {
  switch(term.tag) {
    case "var":
      return lookupType(ctx, term.name)
    case "lambda":
      const bodyType = typeCheck(
        extend(ctx, term.param, term.type),
        term.body
      )
      return { tag: "pi", param: term.param, type: term.type, body: bodyType }
    // ... more cases
  }
}
```

### 3.2 Smooth Structure Implementation

#### Smooth Functions
```typescript
interface SmoothFunction {
  domain: SmoothManifold
  codomain: SmoothManifold
  evaluate: (point: Point) => Point
  differentiate: (point: Point) => LinearMap
  taylorSeries?: (point: Point, order: number) => Polynomial
}
```

#### Manifolds
```typescript
interface SmoothManifold {
  dimension: number
  charts: Chart[]
  transitionMaps: Map<[Chart, Chart], SmoothFunction>
  tangentBundle: () => VectorBundle
}
```

### 3.3 Cubical Operations

#### Path Abstraction and Application
```typescript
function pathAbstraction(i: Interval, body: Term): Term {
  return { tag: "pathLambda", param: i, body }
}

function pathApplication(path: Term, r: IntervalTerm): Term {
  return { tag: "pathApp", path, point: r }
}
```

#### Composition
```typescript
function composition(
  type: (i: Interval) => Term,
  phi: Formula,
  u: (i: Interval) => Partial<Term>,
  a0: Term
): Term {
  // Implementation of cubical composition
}
```

## 4. Implementation Phases

### Phase 1: Core Cubical Type Theory
- [ ] Basic type system (Pi, Sigma, Universe)
- [ ] Interval type and operations
- [ ] Path types and basic path operations
- [ ] Composition and transport
- [ ] Basic type checker and evaluator

### Phase 2: Smooth Structure
- [ ] Smooth interval type
- [ ] Smooth function types
- [ ] Differentiation operator
- [ ] Chain rule implementation
- [ ] Basic smooth type checking

### Phase 3: Advanced Features
- [ ] Manifold types
- [ ] Tangent bundles
- [ ] Differential forms
- [ ] Integration
- [ ] Vector bundles

### Phase 4: Optimization & Tooling
- [ ] Efficient normalization
- [ ] Incremental type checking
- [ ] IDE support
- [ ] Proof assistant features
- [ ] Standard library

## 5. Key Algorithms

### 5.1 Normalization by Evaluation (NbE)

```typescript
type Value = 
  | { tag: "neutral", term: Neutral }
  | { tag: "lambda", closure: Closure }
  | { tag: "pair", first: Value, second: Value }
  | { tag: "smooth", value: Value }
  // ...

function evaluate(env: Environment, term: Term): Value {
  // NbE evaluation
}

function readback(value: Value): Term {
  // Convert value back to term
}
```

### 5.2 Smooth Function Composition

```typescript
function composeSmoothFunctions(
  g: SmoothFunction,
  f: SmoothFunction
): SmoothFunction {
  return {
    domain: f.domain,
    codomain: g.codomain,
    evaluate: (x) => g.evaluate(f.evaluate(x)),
    differentiate: (x) => {
      const y = f.evaluate(x)
      const df = f.differentiate(x)
      const dg = g.differentiate(y)
      return compose(dg, df) // matrix multiplication
    }
  }
}
```

## 6. Testing Strategy

### 6.1 Mathematical Properties
- Verify path composition laws
- Check smooth function composition
- Validate differentiation rules
- Test higher inductive type eliminations

### 6.2 Type System Properties
- Subject reduction
- Progress
- Canonicity
- Decidability of type checking

### 6.3 Implementation Tests
- Unit tests for each type constructor
- Property-based testing for normalization
- Integration tests for complex proofs
- Performance benchmarks

## 7. Example Programs

### 7.1 Circle as a Smooth Manifold
```
S¹ : SmoothType
S¹ = SmoothHIT
  | base : S¹
  | loop : SmoothPath S¹ base base
  where loop is parameterized smoothly by [0,1]
```

### 7.2 Smooth Function on Circle
```
f : C^∞(S¹, ℝ)
f = λ^∞(x : S¹). 
  case x of
    | base => 0
    | loop(t) => sin(2πt)
```

### 7.3 Tangent Bundle of Sphere
```
TS² : SmoothType
TS² = T(S²)
    = Σ(x : S²). TangentSpace(S², x)
```

## 8. Open Questions

1. **Coherence**: How to ensure smooth structure is coherent with cubical structure?
2. **Computational complexity**: Can we maintain decidable type checking with smooth structure?
3. **Higher derivatives**: How to elegantly represent n-th derivatives?
4. **Infinite-dimensional spaces**: Can we handle function spaces as smooth infinite-dimensional manifolds?
5. **Constructivity**: Which smooth operations can be made constructive?

## 9. References

- Cohen, Coquand, Huber, Mörtberg. "Cubical Type Theory: A Constructive Interpretation of the Univalence Axiom"
- Synthetic Differential Geometry literature
- Differential Cohesive Type Theory work
- Smooth Infinitesimal Analysis

## 10. Next Steps

1. Formalize the type system in a proof assistant (Agda/Coq)
2. Implement prototype type checker
3. Develop standard library of smooth spaces
4. Create example applications in physics/geometry
5. Write comprehensive test suite