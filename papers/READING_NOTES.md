# SCTT Reading Notes

## Paper-by-Paper Analysis

### 1. Cubical Type Theory (Cohen et al. 2015)

#### Key Insights
- **Path types as primitives**: Instead of defining paths as functions I → A, they're primitive
- **Composition operation**: `comp` allows composing paths in any dimension
- **Transport**: Moving along paths preserves type information
- **Kan filling**: Every open box has a lid

#### What We Can Use
```haskell
-- Their path type
Path : (A : Type) → A → A → Type

-- Our smooth extension
SmoothPath : (A : SmoothType) → A → A → Type
  with differential structure
```

#### Implementation Notes
- Start with their cubical structure
- Add smooth modality on top
- Ensure composition preserves smoothness

#### Questions Raised
- How does composition interact with differentiation?
- Can we have smooth Kan filling?

---

### 2. Synthetic Differential Geometry (Kock 2006)

#### Key Insights
- **Infinitesimals as first-class**: D = {ε : R | ε² = 0}
- **Microlinearity**: Every function is linear on infinitesimals
- **Tangent bundles**: T(M) as M^D

#### What We Can Use
```haskell
-- Their infinitesimal
D := {ε : R | ε² = 0}

-- Our cubical version
CubicalD := {ε : Path R 0 0 | ε ∘ ε = refl}
```

#### Integration Strategy
- Map SDG concepts to cubical setting
- Use paths instead of infinitesimals
- Preserve computational content

#### Challenges
- SDG uses classical logic, we need constructive
- Topos models vs. cubical models

---

### 3. Cubical Agda (Vezzosi et al. 2019)

#### Key Insights
- **Practical implementation**: Actually runnable code
- **Transp and hcomp**: Concrete operations for paths
- **Glue types**: Computational univalence
- **Performance**: Reasonable type checking speed

#### Code Patterns
```agda
-- They write:
myPath : Path A x y
myPath i = ...

-- We'll write:
mySmoothPath : SmoothPath A x y
mySmoothPath i = ...
  @smooth: differentiable
```

#### Implementation Plan
1. Fork Cubical Agda
2. Add smooth types module
3. Implement differential operators
4. Benchmark performance

---

### 4. Modal Homotopy Type Theory (Corfield 2019)

#### Key Insights
- **Modalities as monads**: ○ and ◊ operators
- **Cohesion**: ♭ (flat) and ♯ (sharp) modalities
- **Smooth modality**: ∞ for smooth maps

#### Our Modality Stack
```
Type
  ↓ (discrete)
CubicalType
  ↓ (paths)
SmoothCubicalType
  ↓ (smooth)
DifferentiableType
```

#### Modal Operators
- `♭`: Forget smooth structure
- `♯`: Free smooth structure
- `∞`: Smooth maps only

---

### 5. Differential Cohomology (Schreiber 2013)

#### Key Insights
- **Higher gauge theory**: Connections as homotopies
- **Smooth ∞-groupoids**: Higher categorical smooth structures
- **Chern-Weil theory**: In homotopy type theory

#### Advanced Features for SCTT
```haskell
-- Differential forms
Ω^n(M) : SmoothType
  
-- de Rham cohomology
H^n_dR(M) := Ker(d_n) / Im(d_{n-1})

-- Integration
∫ : Ω^n(M) → Path^n(M) → R
```

#### Research Direction
- Implement after basic SCTT works
- Use for physics applications
- Connect to gauge theory

---

## Synthesis: How Papers Fit Together

### Architecture Layers

```
Application Layer
  Physics (GR, QFT)          [Schreiber]
  Machine Learning           [New]
  Numerical Methods          [New]
    ↓
Smooth Structure Layer
  Differential Operators     [Kock]
  Smooth Modalities         [Corfield]
  Tangent Bundles           [Kock]
    ↓
Cubical Structure Layer
  Path Types                [Cohen]
  Composition               [Cohen]
  Univalence                [Cohen]
    ↓
Implementation Layer
  Type Checking             [Vezzosi]
  Normalization             [Sterling]
  Optimization              [New]
```

### Key Innovations We're Adding

1. **Computational Smooth Structures**
   - No existing system has this
   - Combine cubical + smooth constructively
   - Maintain canonicity

2. **Proof-Carrying Differential Code**
   - Derivatives with correctness proofs
   - Verified numerical methods
   - Smooth invariants

3. **Practical Implementation**
   - Fast type checking
   - Browser deployment
   - Interactive development

---

## Critical Papers Analysis

### What's Missing in Literature

1. **No Computational SDG**
   - Kock is classical
   - Schreiber is ∞-categorical
   - We need algorithmic

2. **No Smooth Cubical**
   - Cubical lacks smooth
   - SDG lacks cubical
   - We're merging them

3. **No Implementations**
   - Theory without code
   - We're building it

### Our Unique Position

```
Existing: Cubical OR Smooth OR Implemented
SCTT:     Cubical AND Smooth AND Implemented
```

---

## Implementation Strategy from Literature

### Phase 1: Core (From Cohen + Vezzosi)
```rust
enum Type {
    Universe(Level),
    Pi(Box<Type>, Box<Type>),
    Path(Box<Type>, Value, Value),
    // Our addition:
    Smooth(Box<Type>, Box<Type>), // C∞(A,B)
}
```

### Phase 2: Smooth (From Kock + Corfield)
```rust
enum SmoothType {
    Manifold(Dimension),
    TangentBundle(Box<SmoothType>),
    DifferentialForm(n, Box<SmoothType>),
}
```

### Phase 3: Advanced (From Schreiber)
```rust
enum HigherSmooth {
    SmoothGroupoid(Level),
    Connection(Bundle),
    GaugeField(Group),
}
```

---

## Key Algorithms to Implement

### From Cubical Papers
1. **Path composition** (Cohen)
2. **Transport** (Cohen)
3. **Glue** (Vezzosi)
4. **Normalization by Evaluation** (Sterling)

### From SDG Papers
1. **Differentiation** (Kock)
2. **Integration** (Kock)
3. **Tangent space** (Moerdijk-Reyes)

### Novel Algorithms
1. **Smooth path composition**
2. **Differential transport**
3. **Smooth normalization**

---

## Open Questions from Literature

### Theoretical
1. Does SCTT satisfy canonicity?
2. What's the complexity of smooth type checking?
3. How do we model smooth HITs?

### Practical
1. Can we achieve <100ms type checking?
2. How much memory for proof objects?
3. WebAssembly performance?

### Applications
1. Can we verify Navier-Stokes?
2. Neural network differentiation?
3. Quantum field theory?

---

## Reading Priority for Implementation

### Must Read Now
1. **Cohen et al.** - Cubical structure
2. **Vezzosi et al.** - Implementation
3. **Kock Ch. 1** - Smooth basics

### Read During Implementation
1. **Sterling NbE** - Optimization
2. **Gratzer Modal** - Modalities
3. **cooltt code** - Patterns

### Read Later
1. **Schreiber** - Advanced
2. **Higher gauge** - Physics
3. **∞-topoi** - Theory

---

## Action Items from Literature

### Immediate
- [ ] Implement basic path types (Cohen)
- [ ] Add smooth function types (Kock)
- [ ] Create test suite (Vezzosi)

### Next Week
- [ ] Composition for smooth paths
- [ ] Differential operator
- [ ] Performance benchmarks

### Next Month
- [ ] Full type checker
- [ ] Proof system
- [ ] Web deployment

---

*These notes are living documents - update as we read more papers and gain insights from implementation.*