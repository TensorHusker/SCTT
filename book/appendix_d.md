# Appendix D: SCTT Type Signature Reference

> "Don't reinvent the wheel - but do understand how it turns."

This appendix is a compact reference for SCTT's core type signatures. It is not a tutorial — see the main chapters for motivation and worked examples. Signatures are written in SCTT pseudocode; Rust implementation details are in the source repository.

---

## The Core Term Type

The `Term` grammar defines every expression in SCTT. The kernel is a Cartesian cubical type theory (ABCFHL) extended with smooth structure.

```sctt
Term ::=
  -- TTT kernel
  | Var(Index)                         -- De Bruijn variable #n
  | Universe(Level)                    -- Type_i, levels 0..6
  | Pi(Term, Term)                     -- Π(x : A). B
  | Lambda(Term)                       -- λ. body  (single-arg, de Bruijn)
  | App(Term, Term)                    -- f a
  | Sigma(Term, Term)                  -- Σ(x : A). B
  | Pair(Term, Term)                   -- (a, b)
  | Fst(Term)                          -- π₁ p
  | Snd(Term)                          -- π₂ p
  | Id(Term, Term, Term)               -- a ≡_A b
  | Refl(Term)                         -- refl a
  | J(Term, Term, Term)                -- J M b p  (path eliminator)

  -- Cubical extensions (Cartesian, ABCFHL)
  | PathP(Term, Term, Term)            -- PathP A a b  (heterogeneous path)
  | PLam(Term)                         -- λi. t  (path abstraction)
  | PApp(Term, Term)                   -- p @ r  (path application, r : DimExpr)
  | Coe(Term, Term, Term, Term)        -- coe r r' A t
  | HCom(Term, Term, Term, Term, Term) -- hcom r r' A sys t
  | GlueType(Term, Term, Term)         -- Glue A φ Te   (univalence kit)
  | Glue(Term, Term, Term)             -- glue a φ te
  | Unglue(Term)                       -- unglue t

  -- Smooth extensions
  | SmoothReal                         -- ℝ  (smooth real line)
  | Derivative(Term)                   -- D[f]  (functional derivative)
  | Integral(Term, Term, Term)         -- ∫ a b f
  | Infinitesimal                      -- 𝔻  (nilsquare type, D² = 0)
  | Lipschitz(Term, Term, Term)        -- Lip(A, B, k)  (k-Lipschitz maps)

-- Dimension expressions (interval algebra)
DimExpr ::= Dim0 | Dim1 | DimVar(Name) | DimMeet(DimExpr, DimExpr)
           | DimJoin(DimExpr, DimExpr) | DimNeg(DimExpr)

-- Universe levels: bounded natural numbers 0..MAX_UNIVERSE_LEVEL (= 6)
Level ::= 0 | 1 | 2 | 3 | 4 | 5 | 6

-- De Bruijn index: counts binders outward from use site
Index ::= ℕ
```

---

## Core Modules

### Prelude

Essential definitions available without an explicit import.

```sctt
-- Polymorphic identity
id : {A : Type} → A → A
id x = x

-- Constant function
const : {A B : Type} → A → B → A
const x _ = x

-- Function composition
comp : {A B C : Type} → (B → C) → (A → B) → A → C
comp g f x = g (f x)

-- Flip argument order
flip : {A B C : Type} → (A → B → C) → B → A → C
flip f y x = f x y

-- Dependent function composition (diagrammatic)
_∘_ : {A : Type} {B : A → Type} {C : {x : A} → B x → Type}
    → ({x : A} (y : B x) → C y)
    → (f : (x : A) → B x)
    → (x : A) → C (f x)
```

### Data

Standard data structures.

```sctt
-- Natural numbers
data Nat : Type where
  zero : Nat
  suc  : Nat → Nat

-- Booleans
data Bool : Type where
  true false : Bool

-- Maybe / Option
data Maybe (A : Type) : Type where
  nothing : Maybe A
  just    : A → Maybe A

-- Sum type
data Either (A B : Type) : Type where
  left  : A → Either A B
  right : B → Either A B

-- Lists
data List (A : Type) : Type where
  []  : List A
  _∷_ : A → List A → List A

-- Length-indexed vectors
data Vec (A : Type) : Nat → Type where
  []  : Vec A zero
  _∷_ : {n : Nat} → A → Vec A n → Vec A (suc n)

-- Bounded naturals
data Fin : Nat → Type where
  fzero : {n : Nat} → Fin (suc n)
  fsuc  : {n : Nat} → Fin n → Fin (suc n)
```

### Function

```sctt
-- Fixed-point combinator (well-founded recursion only)
fix : {A : Type} → (A → A) → A

-- Uncurry / curry
uncurry : {A B C : Type} → (A → B → C) → A × B → C
curry   : {A B C : Type} → (A × B → C) → A → B → C

-- Application
_$_ : {A B : Type} → (A → B) → A → B
f $ x = f x

-- On: lift a binary function through a transform
on : {A B C : Type} → (B → B → C) → (A → B) → A → A → C
on f g x y = f (g x) (g y)
```

### Logic

```sctt
-- Propositions (h-level 1 types)
isProp : Type → Type
isProp A = (x y : A) → x ≡ y

-- Sets (h-level 2 types)
isSet : Type → Type
isSet A = (x y : A) → isProp (x ≡ y)

-- Propositional truncation
data ‖_‖ (A : Type) : Type where
  ∣_∣  : A → ‖ A ‖
  squash : isProp ‖ A ‖

-- Conjunction as Σ-type
_∧_ : Type → Type → Type
A ∧ B = A × B

-- Disjunction as propositional truncation of Either
_∨_ : Type → Type → Type
A ∨ B = ‖ Either A B ‖

-- Negation
¬_ : Type → Type
¬ A = A → ⊥

-- Implication is just function type
-- Biconditional
_↔_ : Type → Type → Type
A ↔ B = (A → B) × (B → A)
```

---

## Type Theory Modules

### Path

```sctt
-- Homogeneous path (definitional alias)
Path : (A : Type) → A → A → Type
Path A a b = PathP (λ _ → A) a b

-- Symmetry
sym : {A : Type} {x y : A} → x ≡ y → y ≡ x
sym p = λ i → p @ (~ i)

-- Transitivity (composition)
trans : {A : Type} {x y z : A} → x ≡ y → y ≡ z → x ≡ z
trans p q = hcom 0 1 (p @ 1) [ (i = 0) ↦ p , (i = 1) ↦ q ]

-- Congruence
ap : {A B : Type} {x y : A} → (f : A → B) → x ≡ y → f x ≡ f y
ap f p = λ i → f (p @ i)

-- Transport along a path of types
transport : {A B : Type} → A ≡ B → A → B
transport e a = coe 0 1 e a

-- Substitution (dependent transport)
subst : {A : Type} (P : A → Type) {x y : A} → x ≡ y → P x → P y
subst P e px = transport (ap P e) px

-- Path inversion (same as sym for homogeneous paths)
inv : {A : Type} {x y : A} → x ≡ y → y ≡ x
inv = sym

-- Function extensionality
funext : {A : Type} {B : A → Type} {f g : (x : A) → B x}
       → ((x : A) → f x ≡ g x) → f ≡ g
funext h = λ i x → h x @ i
```

### Equiv

```sctt
-- Fiber of f over b
fiber : {A B : Type} → (A → B) → B → Type
fiber f b = Σ(a : A). f a ≡ b

-- Contractibility
isContr : Type → Type
isContr A = Σ(x : A). (y : A) → x ≡ y

-- Equivalence: f is an equivalence iff all fibers are contractible
isEquiv : {A B : Type} → (A → B) → Type
isEquiv f = (b : _) → isContr (fiber f b)

-- Equivalence type
_≃_ : Type → Type → Type
A ≃ B = Σ(f : A → B). isEquiv f

-- Identity equivalence
idEquiv : (A : Type) → A ≃ A

-- Equivalence from iso (function + inverse + homotopies)
isoToEquiv : {A B : Type}
           → Σ(f : A → B). Σ(g : B → A). ((x : A) → g (f x) ≡ x) × ((y : B) → f (g y) ≡ y)
           → A ≃ B

-- Univalence: equivalences are paths between types
ua : {A B : Type} → A ≃ B → A ≡ B

-- Computation rule for ua
uaβ : {A B : Type} (e : A ≃ B) (x : A) → transport (ua e) x ≡ e .fst x
```

### HITs

Higher inductive types (constructors only; elimination rules omitted for brevity).

```sctt
-- Circle S¹
data S¹ : Type where
  base : S¹
  loop : base ≡ base

-- Suspension ΣA
data Susp (A : Type) : Type where
  north south : Susp A
  merid : A → north ≡ south

-- Propositional truncation (see Logic above)
-- Set truncation (h-level 2 truncation)
data ∥_∥₂ (A : Type) : Type where
  ∣_∣₂     : A → ∥ A ∥₂
  squash₂  : isSet ∥ A ∥₂

-- Pushout
data Pushout {A B C : Type} (f : C → A) (g : C → B) : Type where
  inl   : A → Pushout f g
  inr   : B → Pushout f g
  push  : (c : C) → inl (f c) ≡ inr (g c)

-- Integers (as a HIT)
data ℤ : Type where
  pos  : Nat → ℤ
  negsuc : Nat → ℤ         -- negsuc n = -(n+1)
```

### Truncation

```sctt
-- h-level predicate
isOfHLevel : Nat → Type → Type
isOfHLevel 0 A = isContr A
isOfHLevel 1 A = isProp A
isOfHLevel (suc n) A = (x y : A) → isOfHLevel n (x ≡ y)

-- Truncation to h-level n
data Trunc (n : Nat) (A : Type) : Type where
  ∣_∣ : A → Trunc n A
  hub : (f : S¹ → Trunc n A) → Trunc n A    -- (simplified; see text)

-- Propositional truncation eliminator
∥∥-rec : {A B : Type} → isProp B → (A → B) → ‖ A ‖ → B
```

---

## Smooth Modules

### Smooth

```sctt
-- The smooth real line
ℝ : Type   -- primitive; supports +, ×, -, /, algebraic ops

-- Smooth functions between types
C∞ : Type → Type → Type   -- C∞(A, B) = smooth maps A → B

-- Smoothness predicate for a function
isSmooth : {A B : Type} → (A → B) → Type

-- Smooth subtype
Smooth : (A : Type) → Type
Smooth A = Σ(f : ℝ → A). isSmooth f

-- Dual numbers: ℝ[ε]/(ε²)  (forward-mode AD carrier)
record DualNumber : Type where
  real  : ℝ
  dual  : ℝ   -- infinitesimal part; ε² = 0

-- Nilsquare element 𝔻 (the infinitesimal type)
𝔻 : Type   -- one-element type with algebraic laws ε² = 0

-- Smooth evaluation (forward pass)
smooth-eval : C∞(ℝ, ℝ) → DualNumber → DualNumber
```

### Calculus

```sctt
-- Functional derivative operator
D : C∞(ℝ, ℝ) → C∞(ℝ, ℝ)

-- Partial derivative (fixes all but the i-th argument)
∂ : {n : Nat} → C∞(ℝⁿ, ℝ) → Fin n → C∞(ℝⁿ, ℝ)

-- Gradient
gradient : {n : Nat} → C∞(ℝⁿ, ℝ) → C∞(ℝⁿ, ℝⁿ)

-- Jacobian matrix
jacobian : {m n : Nat} → C∞(ℝⁿ, ℝᵐ) → C∞(ℝⁿ, Mat m n)

-- Definite integral (with verified bounds)
integral : (a b : ℝ) → C∞(ℝ, ℝ) → ℝ

-- Taylor expansion up to degree n around point x₀
taylor : (n : Nat) → (x₀ : ℝ) → C∞(ℝ, ℝ) → Poly n

-- Differentiate a value-level term
differentiate : Value → Value   -- acts on smooth Value inhabitants

-- Chain rule (automatic derivation)
chain : {A B C : Type} → C∞(B, C) → C∞(A, B) → C∞(A, C)
-- chain g f = g ∘ f,  D(chain g f) = (D g ∘ f) · D f
```

### Manifold

```sctt
-- A smooth manifold: a type with a smooth atlas
record Manifold : Type₁ where
  carrier  : Type
  dim      : Nat
  atlas    : List (Σ(U : carrier → Type). C∞(Σ carrier U, ℝᵈⁱᵐ))
  smooth   : -- compatibility conditions on atlas

-- Tangent space at a point
TangentSpace : (M : Manifold) → M .carrier → Type
TangentSpace M p = C∞(C∞(M .carrier, ℝ), ℝ)   -- derivations

-- Tangent bundle
TangentBundle : Manifold → Type
TangentBundle M = Σ(p : M .carrier). TangentSpace M p

-- Smooth map between manifolds
SmoothMap : Manifold → Manifold → Type
SmoothMap M N = Σ(f : M .carrier → N .carrier). isSmooth f

-- Differential of a smooth map
df : {M N : Manifold} → SmoothMap M N
   → (p : M .carrier) → TangentSpace M p → TangentSpace N (f p)

-- Riemannian metric
Metric : Manifold → Type
Metric M = (p : M .carrier) → TangentSpace M p → TangentSpace M p → ℝ
```

### Forms

```sctt
-- k-form on a manifold M
DifferentialForm : (k : Nat) → Manifold → Type

-- Zero-form (smooth function)
Ω⁰ : Manifold → Type
Ω⁰ M = C∞(M .carrier, ℝ)

-- Exterior derivative
d : {k : Nat} → DifferentialForm k M → DifferentialForm (suc k) M

-- Wedge product
_∧_ : {j k : Nat} → DifferentialForm j M → DifferentialForm k M
    → DifferentialForm (j + k) M

-- Interior product (contraction with a vector field)
ι : {k : Nat} → VectorField M → DifferentialForm k M
  → DifferentialForm (k - 1) M

-- Integration of a top form over M (requires orientation)
∫_over_ : DifferentialForm (M .dim) M → Manifold → ℝ

-- Stokes' theorem (as a type)
stokes : {k : Nat} (M : OrientedManifoldWithBoundary) (ω : DifferentialForm k M)
       → ∫ (d ω) over M ≡ ∫ ω over (∂ M)
```

---

## Applied Modules

### LinearAlgebra

```sctt
-- Fixed-size matrix
Mat : (m n : Nat) → Type
Mat m n = Vec (Vec ℝ n) m

-- Vector space (axioms bundled)
record VectorSpace : Type₁ where
  carrier : Type
  zero    : carrier
  _+_     : carrier → carrier → carrier
  _·_     : ℝ → carrier → carrier
  -- coherence laws ...

-- Standard operations
_+ᵥ_      : {n : Nat} → Vec ℝ n → Vec ℝ n → Vec ℝ n
_·ᵥ_      : ℝ → Vec ℝ n → Vec ℝ n
dot       : {n : Nat} → Vec ℝ n → Vec ℝ n → ℝ
norm      : {n : Nat} → Vec ℝ n → ℝ
matMul    : Mat m k → Mat k n → Mat m n
transpose : Mat m n → Mat n m
det       : Mat n n → ℝ
inv       : (A : Mat n n) → isInvertible A → Mat n n
```

### Numerical

```sctt
-- Verified bound: |result - true| ≤ ε
record WithBound (A : Type) (ε : ℝ) : Type where
  value : A
  proof : bound-proof value ε

-- Interval arithmetic
Interval : Type
Interval = Σ(lo hi : ℝ). lo ≤ hi

-- ODE initial value problem: ẋ = f(t,x), x(t₀) = x₀
solve-ode : (f : ℝ → ℝ → ℝ) (t₀ x₀ tₑ h : ℝ)
          → WithBound (ℝ → ℝ) (step-error h)

-- Numerical integration with error certificate
quad : (f : C∞(ℝ, ℝ)) (a b : ℝ) (n : Nat)
     → WithBound ℝ (O(1/n²))

-- Lipschitz constant extraction
lipschitz-const : C∞(ℝ, ℝ) → ℝ
-- returns k such that |f(x) - f(y)| ≤ k · |x - y|
```

### Physics

```sctt
-- Classical mechanics: configuration space Q, Lagrangian L : TQ → ℝ
record LagrangianSystem : Type₁ where
  Q : Manifold
  L : TangentBundle Q → ℝ
  isSmooth-L : isSmooth L

-- Euler-Lagrange equations (as a path type)
euler-lagrange : (sys : LagrangianSystem) (γ : Path ℝ sys.Q)
               → isStationary (sys.L) γ → Type

-- Hamiltonian mechanics: phase space T*Q
record HamiltonianSystem : Type₁ where
  Q : Manifold
  H : CotangentBundle Q → ℝ

-- Symplectic form on T*Q
symplectic : HamiltonianSystem → DifferentialForm 2 (cotangent-bundle sys.Q)

-- Noether's theorem (conserved quantity from symmetry)
noether : (sym : Symmetry sys) → ConservedQuantity sys
```

### Optimization

```sctt
-- Gradient descent step
gd-step : (f : C∞(ℝⁿ, ℝ)) (x : ℝⁿ) (η : ℝ) → ℝⁿ
gd-step f x η = x - η · gradient f x

-- Convergence certificate: reaches ε-neighborhood in N steps
converges : (f : C∞(ℝⁿ, ℝ)) (x₀ : ℝⁿ) (ε η : ℝ)
          → Σ(N : Nat). converges-in f x₀ ε η N

-- Lipschitz-constrained optimisation
lip-opt : Lip(ℝⁿ, ℝ, k) → ℝⁿ → ℝ → ℝⁿ   -- guaranteed descent
```

---

## Core Evaluation Functions

These are the kernel operations that the type checker calls directly.

```sctt
-- Normalize-by-evaluation: reduce a term to value form
evaluate : Env → Term → Value

-- Quote a value back to a term (for conversion checking)
quote : Level → Value → Term

-- β-reduction rules (applied inside evaluate)
--   (λ. b) a       ↦  b[a/0]       -- function application
--   π₁ (a, b)      ↦  a            -- first projection
--   π₂ (a, b)      ↦  b            -- second projection
--   J M b refl     ↦  b            -- J computation
--   (p @ 0)        ↦  p.left       -- path endpoint (dimension 0)
--   (p @ 1)        ↦  p.right      -- path endpoint (dimension 1)
--   coe r r A t    ↦  t  (when r = r' definitionally)

-- Bidirectional type checking
check : Context → Term → Term → Result<()>    -- check term has type
infer : Context → Term → Result<Term>          -- synthesize type of term

-- Definitional equality (post-NbE syntactic comparison)
definitionally-equal : Value → Value → Bool

-- Cubical Kan operations
coe-value  : DimVal → DimVal → Value → Value → Value
hcom-value : DimVal → DimVal → Value → System → Value → Value

-- Smooth kernel operations
differentiate : Value → Value                         -- D[v]
smooth-eval   : Value → DualNumber → DualNumber       -- forward-mode AD pass
```

---

## β-Reduction Summary Table

| Redex | Normal form | Rule name |
|-|-|-|
| `(λ. b) a` | `b[a/0]` | β-fun |
| `π₁ (a, b)` | `a` | β-fst |
| `π₂ (a, b)` | `b` | β-snd |
| `J M b refl` | `b` | β-J |
| `p @ 0` | `p.left` | β-path-left |
| `p @ 1` | `p.right` | β-path-right |
| `coe r r A t` | `t` | coe-refl |
| `D[λx. c]` | `λx. 0` (constant rule) | smooth-const |
| `unglue (glue a φ te)` | `a` | β-unglue |

---

*Return to: [Table of Contents](./SUMMARY.md) | [Bibliography](./bibliography.md)*
