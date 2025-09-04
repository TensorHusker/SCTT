# Chapter 4: Smooth Types

> "Nature does not make jumps." — Leibniz's Law of Continuity
>
> "In SCTT, neither does computation."

## Introduction

We now reach the heart of SCTT's innovation: adding smooth structure to our cubical foundation. While traditional type theory excels at discrete mathematics and cubical type theory captures topological structure, neither directly supports the differential geometry essential to physics, optimization, and continuous systems.

This chapter introduces smooth types—types equipped with differential structure that computes. We'll see how every type can be made smooth, how functions become differentiable by construction, and how the cubical structure from Chapter 3 naturally extends to support calculus.

### Synthetic vs Analytic Approach

Classical differential geometry builds smooth structures analytically through limits and epsilon-delta arguments. Synthetic Differential Geometry (SDG), pioneered by Lawvere and Kock, takes the opposite approach: smoothness is primitive, and all maps are smooth by construction.

SCTT adapts SDG to our cubical setting:
- **Classical SDG**: Uses nilpotent infinitesimals in a topos
- **SCTT Approach**: Uses cubical paths as infinitesimals
- **Key Innovation**: Computational content via cubical structure

## 4.1 Smooth Real Numbers {#smooth-reals}

### The Smooth Real Type

We begin with the most fundamental smooth type:

```sctt
-- The smooth real numbers
ℝ : SmoothType
ℝ = SmoothReal

-- With the standard embedding
r : Real → ℝ
r x = embed x

-- Key property: ℝ supports differentiation
D : C∞(ℝ, ℝ) → C∞(ℝ, ℝ)
D f = derivative f
```

### Smooth Structure vs Discrete Structure

The smooth reals differ fundamentally from discrete reals:

```sctt
-- Discrete reals (from Chapter 2)
Real : Type
2.5 : Real
sqrt : Real → Real  -- Might not be differentiable!

-- Smooth reals  
ℝ : SmoothType
2.5ₛ : ℝ
smooth_sqrt : C∞(ℝ₊, ℝ)  -- Guaranteed smooth where defined

-- The smooth structure ensures:
-- 1. All operations are infinitely differentiable
-- 2. Derivatives exist and compute
-- 3. Chain rule holds definitionally
```

### Infinitesimals and Tangent Vectors

SCTT realizes infinitesimals through a cubical adaptation of the Kock-Lawvere axiom:

#### The Infinitesimal Object

```sctt
-- The infinitesimal object (cubical version of D = {d : d² = 0})
𝔻 : Type
𝔻 = Σ (ε : Path ℝ 0 0), ε ∘ ε ≡ refl

-- Kock-Lawvere axiom (cubical form)
KL : (f : C∞(ℝ, ℝ)) → (x : ℝ) →
     ∃! (a b : ℝ), ∀ (ε : 𝔻), 
     f(x + ε) = a + b · ε

-- The coefficient b is the derivative!
derivative_via_KL : C∞(ℝ, ℝ) → C∞(ℝ, ℝ)
derivative_via_KL f x = the unique b from KL
```

#### Microlinearity Principle

```sctt
-- Every function is linear on infinitesimals
microlinear : (f : C∞(ℝ, ℝ)) → (x : ℝ) → (ε : 𝔻) →
              f(x + ε) ≡ f(x) + f'(x) · ε

-- This holds definitionally in SCTT!
```

#### Tangent Bundle via Infinitesimals

```sctt
-- Tangent space as maps from infinitesimals
TangentSpace : ℝ → Type
TangentSpace x = 𝔻 → ℝ
  -- Equivalently: ℝ^𝔻 in exponential notation

-- Tangent bundle
Tℝ : SmoothType
Tℝ = Σ (x : ℝ), TangentSpace x

-- Tangent vectors act as derivations
derivation : Tℝ → C∞(ℝ, ℝ) → ℝ
derivation (x, v) f = v(λh. f(x + h) - f(x))
```

### Computing with Smooth Reals

```sctt
-- Smooth arithmetic operations
_+ₛ_ : ℝ → ℝ → ℝ
_*ₛ_ : ℝ → ℝ → ℝ
expₛ : ℝ → ℝ
sinₛ : ℝ → ℝ

-- These satisfy smooth axioms
smooth_add : C∞(ℝ × ℝ, ℝ)
smooth_add (x, y) = x +ₛ y

-- Derivatives compute correctly
_ : D[λ x → x *ₛ x] ≡ λ x → 2 *ₛ x
_ = refl  -- Holds definitionally!

-- Smooth composition
example : ℝ → ℝ
example x = sinₛ(expₛ(x *ₛ x))

derivative_example : ℝ → ℝ  
derivative_example = D[example]
-- Automatically computes: 2x · eˣ² · cos(eˣ²)
```

## 4.2 Smooth Functions {#smooth-functions}

### The Smooth Function Type

Functions between smooth types automatically carry smooth structure:

#### The Internal Hom

```sctt
-- Smooth function type (exponential in smooth category)
C∞ : SmoothType → SmoothType → SmoothType
C∞ A B = B^A  -- Exponential object

-- Key theorem: All functions are smooth
all_smooth : (f : A → B) → IsSmooth f
all_smooth f = axiom  -- By construction in smooth topos!

-- But we can distinguish smoothness levels:
C⁰ : Type → Type → Type      -- Continuous
C¹ : Type → Type → Type      -- Once differentiable
C∞ : Type → Type → Type      -- Smooth (all derivatives)
Cω : Type → Type → Type      -- Analytic (Taylor series)
```

#### Smooth Evaluation Map

```sctt
-- Evaluation is smooth
eval : C∞(C∞(A,B) × A, B)
eval (f, a) = f a

-- Currying is smooth
curry : C∞(A × B, C) → C∞(A, C∞(B, C))
curry f = λa. λb. f(a, b)

-- These form a smooth cartesian closed category
```

### Smooth vs Non-Smooth

Not every function is smooth:

```sctt
-- Non-smooth function (cannot be in C∞)
abs : ℝ → ℝ
abs x = if x ≥ 0 then x else -x
-- Not differentiable at 0!

-- Smooth approximation
smooth_abs : ℝ → ℝ → ℝ  -- Parameterized by ε
smooth_abs ε x = sqrt(x² + ε²)

-- As ε → 0, approaches abs
limit : (ε : ℝ₊) → C∞(ℝ, ℝ)
limit ε = smooth_abs ε
```

### Higher Derivatives

Smooth functions have all derivatives, computed via iterated Kock-Lawvere:

#### Jet Bundles

```sctt
-- n-jet at a point (Taylor polynomial data)
Jet : ℕ → ℝ → C∞(ℝ, ℝ) → Type
Jet n x f = (f(x), f'(x), f''(x)/2!, ..., fⁿ(x)/n!)

-- Jet bundle (all Taylor data)
Jⁿ : C∞(ℝ, ℝ) → C∞(ℝ, ℝⁿ⁺¹)
Jⁿ f x = (f(x), Df(x), D²f(x), ..., Dⁿf(x))
```

#### Higher-Order Infinitesimals

```sctt
-- nth order infinitesimal neighborhood
𝔻ⁿ : Type
𝔻¹ = {ε : ℝ | ε² = 0}           -- First order
𝔻² = {ε : ℝ | ε³ = 0}           -- Second order
𝔻ⁿ = {ε : ℝ | εⁿ⁺¹ = 0}       -- nth order

-- Taylor expansion via higher infinitesimals
taylor_expansion : C∞(ℝ, ℝ) → ℝ → (ε : 𝔻ⁿ) → ℝ
taylor_expansion f x ε = 
  f(x) + f'(x)ε + f''(x)ε²/2! + ... + fⁿ(x)εⁿ/n!
```

#### Faà di Bruno Formula

```sctt
-- Chain rule for higher derivatives (computed!)
faa_di_bruno : (f g : C∞(ℝ, ℝ)) → (n : ℕ) →
               Dⁿ(f ∘ g) ≡ Σ[partitions of n] ...
-- The formula is computed automatically from
-- the Kock-Lawvere axiom iteration
```
taylor f a x n = Σ[k ≤ n] (Dᵏ[f](a) / k!) * (x - a)ᵏ

-- Smooth functions equal their Taylor series
taylor_theorem : (f : C∞(ℝ, ℝ)) → (a : ℝ) →
                 f ≡ λ x → limit[n → ∞] (taylor f a x n)
```

### Smooth Paths as Functions

Paths in smooth types are smooth functions from the interval:

```sctt
-- Smooth path type
SmoothPath : (M : SmoothType) → M → M → Type
SmoothPath M x y = Σ (p : Path M x y),
                     C∞(I, M)

-- Velocity of a path
velocity : SmoothPath ℝ x y → I → ℝ
velocity (p, smooth_p) t = D[smooth_p](t)

-- Acceleration
acceleration : SmoothPath ℝ x y → I → ℝ
acceleration p t = D[velocity p](t)
```

## 4.3 Tangent Bundles {#tangent-bundles}

### The Tangent Bundle Construction

Every smooth type has an associated tangent bundle:

```sctt
-- Tangent bundle
T : SmoothType → SmoothType
T M = Σ (x : M), TangentSpace M x

-- Projection maps
π : T M → M
π (x, v) = x

-- Example: tangent bundle of ℝ
T ℝ ≃ ℝ × ℝ
-- Point and velocity

-- Example: tangent bundle of sphere
T S² = Σ (p : S²), (v : ℝ³) × (v ⊥ p)
-- Point with tangent vector perpendicular to surface
```

### Differential of Maps

Every smooth map has a differential:

```sctt
-- Differential (pushforward)
dF : (F : C∞(M, N)) → C∞(T M, T N)
dF F (x, v) = (F x, DF[x](v))

-- Chain rule holds definitionally
chain_rule : (F : C∞(M, N)) → (G : C∞(N, P)) →
             d(G ∘ F) ≡ dG ∘ dF
chain_rule F G = refl  -- Computational!

-- Example
F : C∞(ℝ², ℝ)
F (x, y) = x² + y²

dF : C∞(T ℝ², T ℝ)
dF ((x, y), (dx, dy)) = (x² + y², 2x*dx + 2y*dy)
```

### Vector Fields

Vector fields assign tangent vectors smoothly:

```sctt
-- Vector field type
VectorField : SmoothType → Type
VectorField M = C∞(M, T M) with π ∘ X ≡ id

-- Example: gradient field
gradient : C∞(ℝⁿ, ℝ) → VectorField ℝⁿ
gradient f x = (x, ∇f(x))

-- Flow of vector field
flow : VectorField M → ℝ → M → M
flow X t x = solution of ∂γ/∂t = X(γ(t)), γ(0) = x
```

### Cotangent Bundle and Differential Forms

The dual structure:

```sctt
-- Cotangent bundle
T* : SmoothType → SmoothType
T* M = Σ (x : M), CotangentSpace M x
  where CotangentSpace M x = TangentSpace M x → ℝ

-- Differential 1-form
Ω¹ : SmoothType → Type
Ω¹ M = C∞(M, T* M)

-- Exact forms (differentials of functions)
d : C∞(M, ℝ) → Ω¹ M
d f x = (x, λ v → Df[x](v))

-- The fundamental theorem
∫_path df = f(end) - f(start)
```

## 4.4 Manifolds {#manifolds}

### Smooth Manifolds as Types

Manifolds are types with smooth structure:

```sctt
-- Manifold type
record Manifold : Type₁ where
  carrier : Type
  smooth : SmoothStructure carrier
  dimension : ℕ
  atlas : Atlas carrier dimension

-- Examples
ℝⁿ : (n : ℕ) → Manifold
Sⁿ : (n : ℕ) → Manifold  -- n-sphere
Tⁿ : (n : ℕ) → Manifold  -- n-torus

-- Product manifold
_×ᴹ_ : Manifold → Manifold → Manifold
M ×ᴹ N = record {
  carrier = M.carrier × N.carrier;
  dimension = M.dimension + N.dimension;
  -- ...
}
```

### Charts and Atlases

Local coordinates via cubical structure:

```sctt
-- A chart is a smooth equivalence with ℝⁿ locally
Chart : (M : Manifold) → M → Type
Chart M x = Σ (U : OpenNeighborhood M x),
              C∞-equiv U (OpenBall ℝⁿ)

-- Smooth transition maps
transition : Chart M x → Chart M y → 
            C∞(overlap, overlap)
transition (U, φ) (V, ψ) = ψ ∘ φ⁻¹

-- Atlas (collection of charts covering M)
Atlas : Manifold → Type
Atlas M = (x : M) → Chart M x
```

### Submanifolds and Embeddings

```sctt
-- Smooth embedding
Embedding : Manifold → Manifold → Type
Embedding M N = Σ (f : C∞(M, N)), 
                  isInjective f × 
                  isImmersion df

-- Submanifold
Submanifold : Manifold → Type
Submanifold M = Σ (S : Type),
                  Embedding S M

-- Example: Circle as submanifold of ℝ²
S¹_in_ℝ² : Submanifold ℝ²
S¹_in_ℝ² = (S¹, λ θ → (cos θ, sin θ), proofs)
```

### Riemannian Structure

Smooth types can carry metric structure:

```sctt
-- Riemannian manifold
record RiemannianManifold : Type₁ where
  manifold : Manifold
  metric : (x : M) → InnerProduct (TangentSpace M x)
  smooth : IsSmooth metric

-- Length of curves
length : {M : RiemannianManifold} → 
         SmoothPath M x y → ℝ
length γ = ∫₀¹ √(g(γ(t))(γ'(t), γ'(t))) dt

-- Geodesics minimize length
geodesic : {M : RiemannianManifold} → 
           M → M → SmoothPath M
geodesic x y = argmin length (paths from x to y)
```

## 4.5 Smooth Paths {#smooth-paths}

### Smooth Deformations

Paths in smooth types are smooth deformations:

```sctt
-- Smooth path with derivatives
SmoothPath : (M : Manifold) → M → M → Type
SmoothPath M x y = 
  Σ (γ : Path M x y),
    (t : I) → IsSmooth_at γ t

-- Smooth homotopy
SmoothHomotopy : {M N : Manifold} →
                 C∞(M, N) → C∞(M, N) → Type
SmoothHomotopy f g = 
  Σ (H : C∞(I × M, N)),
    (H(0, -) ≡ f) × (H(1, -) ≡ g)

-- Smooth isotopy (through embeddings)
Isotopy : {M N : Manifold} →
          Embedding M N → Embedding M N → Type
Isotopy f g = 
  Σ (H : SmoothHomotopy f g),
    ((t : I) → IsEmbedding H(t, -))
```

### Parallel Transport

Moving along smooth paths preserves structure:

```sctt
-- Parallel transport in tangent bundle
parallel_transport : {M : RiemannianManifold} →
                    (γ : SmoothPath M x y) →
                    TangentSpace M x → TangentSpace M y
parallel_transport γ v = 
  solution of ∇_γ'(t) V(t) = 0, V(0) = v

-- Curvature measures path-dependence
curvature : {M : RiemannianManifold} →
           (loop : SmoothPath M x x) →
           LinearMap (TangentSpace M x)
curvature loop v = 
  parallel_transport loop v - v
```

### Integration Along Paths

```sctt
-- Line integral
∫_path : {M : Manifold} →
         SmoothPath M x y → Ω¹ M → ℝ
∫_path γ ω = ∫₀¹ ω(γ(t))(γ'(t)) dt

-- Path independence for exact forms
exact_path_independent : 
  (f : C∞(M, ℝ)) →
  (γ₁ γ₂ : SmoothPath M x y) →
  ∫_path γ₁ (df) ≡ ∫_path γ₂ (df)
exact_path_independent f γ₁ γ₂ = 
  fundamental_theorem f
```

## 4.6 The Smooth Modality

### Discrete to Smooth

We can embed discrete types into smooth ones:

```sctt
-- Smooth modality
♭ : Type → SmoothType
♭ A = ConstantSheaf A

-- Discrete reals to smooth reals
discrete_to_smooth : Real → ℝ
discrete_to_smooth = embed

-- But not every smooth type is discrete
no_inverse : ¬(ℝ → Real preserving smooth structure)
```

### Cohesive Structure

The relationship between discrete and smooth:

```sctt
-- Cohesive adjunction
♭ ⊣ ♯ ⊣ ♮

-- Shape (underlying discrete type)
♯ : SmoothType → Type
♯ M = π₀(M)  -- Connected components

-- Flat (discrete points)
♮ : SmoothType → Type  
♮ M = Points(M)

-- Examples
♯ ℝ ≃ Unit      -- ℝ is connected
♮ ℝ ≃ Real      -- Points of ℝ
♯ (ℝ - {0}) ≃ Bool  -- Two components
```

## 4.7 Examples and Applications

### Optimization with Guaranteed Convergence

```sctt
-- Smooth optimization problem
minimize : C∞(ℝⁿ, ℝ) → ℝⁿ → ℝⁿ
minimize f x₀ = limit of gradient_descent
  where
    gradient_descent : ℕ → ℝⁿ
    gradient_descent 0 = x₀
    gradient_descent (n+1) = 
      xₙ - α * ∇f(xₙ)
    
    -- Convergence proof included!
    converges : IsLimit gradient_descent
    converges = smooth_descent_theorem f
```

### Differential Equations

```sctt
-- Smooth ODE
SmoothODE : Type
SmoothODE = Σ (F : VectorField ℝⁿ),
              IsLipschitz F

-- Existence and uniqueness
solve_ode : SmoothODE → ℝⁿ → C∞(ℝ, ℝⁿ)
solve_ode (F, lipschitz) x₀ = 
  unique_solution by Picard_iteration
  where
    existence : ∃! solution
    existence = picard_lindelof F lipschitz x₀
```

### Machine Learning with Smooth Types

```sctt
-- Neural network layer
Layer : (n m : ℕ) → Type
Layer n m = C∞(ℝⁿ, ℝᵐ)

-- Backpropagation is just chain rule
backprop : Layer n m → Layer m p → 
           ℝᵖ → ℝⁿ
backprop f g error = 
  D[f]ᵀ (D[g]ᵀ error)
  -- Automatic and verified!

-- Smooth activation functions
relu_smooth : ℝ → ℝ → ℝ
relu_smooth ε x = log(1 + exp(x/ε)) * ε
-- Smooth approximation of ReLU
```

## Exercises {#exercises}

### Conceptual
1. Why do we need a separate smooth real type ℝ?
2. What's the difference between Path and SmoothPath?
3. How does the tangent bundle relate to derivatives?
4. Why must smooth functions have all derivatives?

### Computational
1. Compute D[λ x → sin(x²)] explicitly.
2. Find the tangent space to S² at the north pole.
3. Calculate parallel transport around a circle.
4. Derive the geodesic equation for ℝ² with standard metric.

### Programming
1. Implement smooth spline interpolation.
2. Write automatic differentiation for polynomials.
3. Create a smooth bump function with compact support.
4. Build a Bezier curve as a smooth path.

### Advanced
1. Prove that every smooth function ℝ → ℝ has a Taylor series.
2. Show that smooth homotopy is an equivalence relation.
3. Construct the Möbius band as a smooth manifold.
4. Implement Lie derivatives of vector fields.

### Research
1. How would you define smooth higher inductive types?
2. Can we have "smooth univalence" for smooth types?
3. Design a notion of smooth ∞-groupoids.
4. What's the computational complexity of smooth type checking?

## Summary

We've introduced smooth structure to type theory:
- **Smooth real numbers** ℝ support differentiation
- **Smooth functions** C∞(M,N) have all derivatives
- **Tangent bundles** T M capture infinitesimal structure
- **Manifolds** are types with smooth atlases
- **Smooth paths** enable continuous deformation
- **The smooth modality** relates discrete and smooth

This smooth structure integrates seamlessly with the cubical foundation, giving us a type theory where differentiation is as natural as function application. Next, we'll explore how to compute with these differential operators.

---

*Next: [Chapter 5: Differential Operators](./chapter_05.md) →*

*Previous: [Chapter 3: Cubical Structure](./chapter_03.md) ←*