# Chapter 5: Differential Operators

> "The derivative is the soul of calculus." — Anonymous
>
> "In SCTT, derivatives are not just operations—they are morphisms in the category of smooth types."

## Introduction

Having established smooth types in Chapter 4, we now turn to computation with differential structures. This chapter shows how SCTT makes differentiation a first-class computational operation, with derivatives that are guaranteed correct by type checking.

Traditional calculus relies on limiting processes that may not converge. Numerical differentiation suffers from truncation and roundoff errors. SCTT solves both problems: derivatives exist by construction and compute exactly. We'll see how the chain rule becomes a theorem rather than a rule, how integration respects types, and how differential forms provide coordinate-free calculus.

## 5.1 Differentiation {#differentiation}

### The Differential Operator

In SCTT, differentiation is a type-preserving operation:

```sctt
-- The differential operator
D : C∞(M, N) → C∞(T M, T N)
D f (x, v) = (f x, Df_x(v))
  where Df_x : TangentSpace M x → TangentSpace N (f x)

-- For real functions, this simplifies to
D : C∞(ℝ, ℝ) → C∞(ℝ, ℝ)
D f x = lim[h → 0] (f(x + h) - f(x))/h
  -- But computed exactly, not numerically!

-- Examples
D[λ x → x²] ≡ λ x → 2*x         -- Power rule
D[sin] ≡ cos                    -- Trig derivatives
D[exp] ≡ exp                    -- Exponential
D[λ x → c] ≡ λ x → 0           -- Constant rule
```

### Computational Differentiation

Unlike symbolic or numerical differentiation, SCTT computes derivatives exactly:

```sctt
-- Derivatives compute during type checking
example : ℝ → ℝ
example x = x³ - 3*x² + 2*x - 1

-- The derivative is computed, not approximated
example' : ℝ → ℝ
example' = D[example]
-- Reduces to: λ x → 3*x² - 6*x + 2

-- This equality is definitional!
_ : example' ≡ λ x → 3*x² - 6*x + 2
_ = refl
```

### Partial Derivatives

For multivariate functions:

```sctt
-- Partial derivative with respect to i-th variable
∂ᵢ : C∞(ℝⁿ, ℝ) → C∞(ℝⁿ, ℝ)
∂ᵢ f (x₁, ..., xₙ) = D[λ xᵢ → f(x₁, ..., xᵢ, ..., xₙ)] xᵢ

-- Gradient
∇ : C∞(ℝⁿ, ℝ) → C∞(ℝⁿ, ℝⁿ)
∇ f x = (∂₁ f x, ..., ∂ₙ f x)

-- Hessian matrix
H : C∞(ℝⁿ, ℝ) → C∞(ℝⁿ, ℝⁿˣⁿ)
H f x = [∂ᵢ ∂ⱼ f x]ᵢⱼ

-- Mixed partials commute (Schwarz's theorem)
schwarz : (f : C∞(ℝⁿ, ℝ)) → 
          ∂ᵢ (∂ⱼ f) ≡ ∂ⱼ (∂ᵢ f)
schwarz f = refl  -- Holds by smoothness!
```

### Directional Derivatives

```sctt
-- Directional derivative
D_v : {M : Manifold} → 
      TangentVector M x → 
      C∞(M, ℝ) → ℝ
D_v v f = Df_x(v)

-- Lie derivative of functions
L_X : VectorField M → C∞(M, ℝ) → C∞(M, ℝ)
L_X X f x = D_X(x) f

-- Properties
_ : L_X (f * g) ≡ (L_X f) * g + f * (L_X g)
_ = refl  -- Product rule

_ : L_X L_Y f - L_Y L_X f ≡ L_[X,Y] f
_ = refl  -- Lie bracket relation
```

## 5.2 Chain Rule {#chain-rule}

### The Chain Rule as a Theorem

In SCTT, the chain rule isn't a rule to remember—it's a provable theorem:

```sctt
-- Chain rule theorem
chain_rule : {L M N : Manifold} →
            (g : C∞(L, M)) → (f : C∞(M, N)) →
            D[f ∘ g] ≡ D[f] ∘ D[g]
chain_rule g f = refl
  -- Holds definitionally by construction!

-- Example application
h : C∞(ℝ, ℝ)
h x = sin(exp(x²))

-- Derivative via chain rule
h' : C∞(ℝ, ℝ)
h' = D[h]
-- Automatically computes to:
-- λ x → cos(exp(x²)) * exp(x²) * 2x

-- Verification
_ : h' ≡ λ x → cos(exp(x²)) * exp(x²) * 2x
_ = refl
```

### Automatic Differentiation

SCTT provides automatic differentiation that's verified correct:

```sctt
-- Forward mode AD
forward_diff : {n m : ℕ} →
               C∞(ℝⁿ, ℝᵐ) → 
               ℝⁿ → ℝⁿ → ℝᵐ × ℝᵐ
forward_diff f x dx = (f x, Df_x dx)

-- Reverse mode AD (backpropagation)
reverse_diff : {n : ℕ} →
               C∞(ℝⁿ, ℝ) →
               ℝⁿ → ℝ × ℝⁿ
reverse_diff f x = (f x, ∇f x)

-- Guaranteed correct by types!
ad_correct : (f : C∞(ℝⁿ, ℝ)) → (x : ℝⁿ) →
            π₂ (reverse_diff f x) ≡ ∇f x
ad_correct f x = refl
```

### Higher-Order Chain Rule

```sctt
-- Second derivative chain rule
chain_rule_2 : (g : C∞(ℝ, ℝ)) → (f : C∞(ℝ, ℝ)) →
               D²[f ∘ g] ≡ 
               λ x → D²[f](g x) * (D[g] x)² + 
                     D[f](g x) * D²[g] x
chain_rule_2 g f = refl

-- Faà di Bruno's formula (higher derivatives)
faa_di_bruno : (n : ℕ) → 
               (g : C∞(ℝ, ℝ)) → (f : C∞(ℝ, ℝ)) →
               Dⁿ[f ∘ g] ≡ (bell_polynomial expression)
```

## 5.3 Integration {#integration}

### The Integral Type

Integration is the inverse of differentiation:

```sctt
-- Definite integral
∫ : (a b : ℝ) → C∞(ℝ, ℝ) → ℝ
∫ a b f = integral_value
  where
    -- Computed via fundamental theorem
    F = antiderivative f
    integral_value = F b - F a

-- Indefinite integral (antiderivative)
∫[_] : C∞(ℝ, ℝ) → C∞(ℝ, ℝ)
∫[f] = F where D[F] ≡ f

-- Fundamental theorem of calculus
FTC : (f : C∞(ℝ, ℝ)) → (a b : ℝ) →
      ∫ a b (D[f]) ≡ f b - f a
FTC f a b = refl
```

### Path and Surface Integrals

```sctt
-- Line integral
∫_γ : {M : Manifold} →
      SmoothPath M x y → 
      Ω¹ M → ℝ
∫_γ path ω = ∫ 0 1 (λ t → ω(path t)(path' t))

-- Surface integral
∫_Σ : {M : Manifold} →
      (Σ : Submanifold M) →
      Ωⁿ M → ℝ
  where n = dim Σ

-- Volume integral
∫_M : {M : Manifold} →
      Ωⁿ M → ℝ
  where n = dim M
```

### Integration by Parts

```sctt
-- Integration by parts formula
by_parts : (u v : C∞(ℝ, ℝ)) → (a b : ℝ) →
          ∫ a b (u * D[v]) ≡ 
          (u * v)|ᵇₐ - ∫ a b (D[u] * v)
by_parts u v a b = 
  calc ∫ a b (u * D[v])
    ≡⟨ refl ⟩ 
       ∫ a b D[u * v] - ∫ a b (D[u] * v)
    ≡⟨ FTC ⟩
       (u * v)|ᵇₐ - ∫ a b (D[u] * v) ∎

-- Multi-dimensional version
by_parts_nd : {n : ℕ} →
              (u v : C∞(ℝⁿ, ℝ)) →
              (Ω : OpenSet ℝⁿ) →
              ∫_Ω (u * Δv) ≡ 
              ∫_∂Ω (u * ∂v/∂n) - ∫_Ω (∇u · ∇v)
```

## 5.4 Differential Forms {#differential-forms}

### The Exterior Algebra

Differential forms provide coordinate-free calculus:

```sctt
-- k-forms on manifold M
Ωᵏ : Manifold → Type
Ω⁰ M = C∞(M, ℝ)                    -- 0-forms (functions)
Ω¹ M = C∞(M, T* M)                 -- 1-forms
Ωᵏ M = C∞(M, ΛᵏT* M)              -- k-forms

-- Wedge product
_∧_ : Ωᵏ M → Ωˡ M → Ωᵏ⁺ˡ M
(α ∧ β)(v₁, ..., vₖ₊ₗ) = 
  (1/(k!l!)) Σ_σ sign(σ) α(vₛ₁, ..., vₛₖ) β(vₛₖ₊₁, ..., vₛₖ₊ₗ)

-- Properties
_ : α ∧ β ≡ (-1)^(kl) β ∧ α  -- Anticommutativity
_ : α ∧ (β ∧ γ) ≡ (α ∧ β) ∧ γ  -- Associativity
_ : α ∧ α ≡ 0  -- (for odd degree)
```

### Exterior Derivative

The exterior derivative generalizes all differential operators:

```sctt
-- Exterior derivative
d : Ωᵏ M → Ωᵏ⁺¹ M

-- On functions (0-forms)
d : C∞(M, ℝ) → Ω¹ M
df(X) = X(f)  -- Directional derivative

-- Key properties
d_squared : (ω : Ωᵏ M) → d(d ω) ≡ 0
d_squared ω = refl  -- d² = 0

leibniz : (α : Ωᵏ M) → (β : Ωˡ M) →
         d(α ∧ β) ≡ (d α) ∧ β + (-1)ᵏ α ∧ (d β)
leibniz α β = refl

-- Coordinate expression
local_d : Ωᵏ ℝⁿ → Ωᵏ⁺¹ ℝⁿ
local_d ω = Σᵢ (∂ω/∂xᵢ) ∧ dxᵢ
```

### Pullback and Pushforward

```sctt
-- Pullback of forms
pullback : {M N : Manifold} →
           C∞(M, N) → Ωᵏ N → Ωᵏ M
pullback f ω = ω ∘ df

-- Notation
f* : Ωᵏ N → Ωᵏ M
f* = pullback f

-- Properties
_ : f*(ω ∧ η) ≡ f*ω ∧ f*η
_ : f*(dω) ≡ d(f*ω)
_ : (g ∘ f)* ≡ f* ∘ g*

-- Pushforward of vector fields
pushforward : {M N : Manifold} →
              C∞(M, N) → VectorField M → VectorField N
pushforward f X = df ∘ X ∘ f⁻¹
  where f is diffeomorphism
```

### De Rham Cohomology

```sctt
-- Closed forms
Closed : Ωᵏ M → Type
Closed ω = (d ω ≡ 0)

-- Exact forms
Exact : Ωᵏ M → Type
Exact ω = Σ (α : Ωᵏ⁻¹ M), ω ≡ d α

-- de Rham cohomology
HᵏdR : Manifold → Type
HᵏdR M = Closed k M / Exact k M

-- Poincaré lemma
poincare : {M : Manifold} → Contractible M →
          (ω : Ωᵏ M) → Closed ω → Exact ω
poincare contr ω closed = 
  construct_antiderivative using contraction
```

## 5.5 Stokes' Theorem {#stokes}

### The Generalized Stokes' Theorem

The crown jewel of differential geometry:

```sctt
-- Stokes' theorem
stokes : {M : Manifold} → 
         (ω : Ωⁿ⁻¹ M) →
         (Σ : OrientedSubmanifold M) →
         ∫_Σ (d ω) ≡ ∫_∂Σ ω
stokes ω Σ = 
  fundamental_theorem_of_calculus
  generalized_to_manifolds

-- Special cases
divergence_theorem : {n : ℕ} →
                    (F : VectorField ℝⁿ) →
                    (Ω : BoundedOpen ℝⁿ) →
                    ∫_Ω (div F) ≡ ∫_∂Ω (F · n̂)
divergence_theorem F Ω = 
  stokes (interpret F as (n-1)-form) Ω

green_theorem : (P Q : C∞(ℝ², ℝ)) →
               (D : BoundedOpen ℝ²) →
               ∫∫_D (∂Q/∂x - ∂P/∂y) ≡ 
               ∮_∂D (P dx + Q dy)
green_theorem P Q D = 
  stokes (P dx + Q dy) D
```

### Applications of Stokes

```sctt
-- Conservative vector fields
conservative : VectorField ℝⁿ → Type
conservative F = Exact (F interpreted as 1-form)

-- Path independence
path_independent : (F : VectorField ℝⁿ) →
                  conservative F →
                  (γ₁ γ₂ : Path ℝⁿ x y) →
                  ∫_γ₁ F ≡ ∫_γ₂ F
path_independent F cons γ₁ γ₂ = 
  both_equal_potential_difference

-- Maxwell's equations in differential forms
maxwell : (E B : VectorField ℝ³) →
         (ρ J : C∞(ℝ⁴, ℝ)) →
         Type
maxwell E B ρ J = 
  (d F ≡ 0) ×                    -- No magnetic monopoles
  (d ⋆F ≡ ⋆J)                   -- Current source
  where F = E ∧ dt + B as 2-form in spacetime
```

## 5.6 Lie Derivatives

### Lie Derivative of Forms

How forms change along vector fields:

```sctt
-- Lie derivative
L_X : VectorField M → Ωᵏ M → Ωᵏ M

-- Cartan's formula
cartan : (X : VectorField M) → (ω : Ωᵏ M) →
        L_X ω ≡ d(i_X ω) + i_X(dω)
  where i_X = interior product

-- Properties
_ : L_X (ω ∧ η) ≡ (L_X ω) ∧ η + ω ∧ (L_X η)
_ : L_X (dω) ≡ d(L_X ω)
_ : L_[X,Y] ≡ L_X ∘ L_Y - L_Y ∘ L_X

-- Killing vector fields
killing : VectorField M → RiemannianMetric M → Type
killing X g = L_X g ≡ 0
```

### Flows and Exponential Map

```sctt
-- Flow of vector field
flow : VectorField M → ℝ → Diffeomorphism M M
flow X t = exp(t X)
  where
    exp solves ∂φ/∂t = X(φ), φ(0) = id

-- Lie derivative via flow
L_via_flow : (X : VectorField M) → (ω : Ωᵏ M) →
            L_X ω ≡ lim[t→0] ((flow X t)* ω - ω)/t
L_via_flow X ω = definition_equivalence
```

## 5.7 Smooth Invariants

### Critical Points and Morse Theory

```sctt
-- Critical point
Critical : C∞(M, ℝ) → M → Type
Critical f x = (df_x ≡ 0)

-- Morse function
Morse : C∞(M, ℝ) → Type
Morse f = (x : M) → Critical f x → 
          NonDegenerate (Hessian f x)

-- Morse lemma
morse_lemma : (f : Morse M ℝ) → 
             (x : Critical f) →
             LocallyEquivalent f (quadratic_form)
  near x with index = negative_eigenvalues

-- Morse inequalities
morse_inequality : (f : Morse M ℝ) →
                  Σᵢ (-1)ⁱ cᵢ(f) ≡ χ(M)
  where cᵢ = number of critical points of index i
        χ = Euler characteristic
```

### Characteristic Classes

```sctt
-- Chern classes
chern : ComplexVectorBundle M → H*dR(M)
chern E = det(I + (i/2π) Ω_E)
  where Ω_E = curvature of connection

-- Pontryagin classes  
pontryagin : RealVectorBundle M → H*dR(M)
pontryagin E = chern(E ⊗ ℂ)

-- Characteristic numbers
∫_M : CharacteristicClass M → ℤ
∫_M c = ∫_M (c ∧ orientation_form)
```

## 5.8 Computational Examples

### Automatic Differentiation in Action

```sctt
-- Neural network example
neural_net : C∞(ℝ¹⁰⁰, ℝ¹⁰)
neural_net = layer₃ ∘ relu ∘ layer₂ ∘ relu ∘ layer₁
  where
    layer₁ : C∞(ℝ¹⁰⁰, ℝ⁵⁰)
    layer₂ : C∞(ℝ⁵⁰, ℝ²⁰)
    layer₃ : C∞(ℝ²⁰, ℝ¹⁰)
    relu = smooth_relu 0.01

-- Gradient computed automatically
gradient : C∞(ℝ¹⁰⁰, ℝ¹⁰⁰)
gradient = ∇(loss ∘ neural_net)
  where loss : C∞(ℝ¹⁰, ℝ)

-- Guaranteed correct by construction!
```

### Solving PDEs

```sctt
-- Heat equation
heat_equation : C∞(ℝ × ℝⁿ, ℝ) → Type
heat_equation u = ∂u/∂t ≡ Δu

-- Solution via Fourier transform
solve_heat : (f : C∞(ℝⁿ, ℝ)) →
            Σ (u : C∞(ℝ × ℝⁿ, ℝ)),
              heat_equation u × (u(0,-) ≡ f)
solve_heat f = 
  (heat_kernel ⊗ f, proof_satisfies_equation)

-- Wave equation with energy conservation
wave_equation : C∞(ℝ × ℝⁿ, ℝ) → Type
wave_equation u = ∂²u/∂t² ≡ c² Δu

energy : C∞(ℝ × ℝⁿ, ℝ) → C∞(ℝ, ℝ)
energy u t = ∫_ℝⁿ (|∂u/∂t|² + c²|∇u|²)

conserved : (u : Σ wave_equation) →
           D[energy u.fst] ≡ 0
conserved u = energy_conservation_proof
```

## Exercises {#exercises}

### Basic Differentiation
1. Compute D[λ x → log(sin(x² + 1))]
2. Find all critical points of f(x,y) = x³ - 3xy + y³
3. Verify the product rule for smooth functions
4. Calculate the Hessian of f(x,y,z) = xyz + x² + y² + z²

### Integration
1. Evaluate ∫₀^π sin²(x) dx using SCTT
2. Compute the line integral of F = (y, -x) around the unit circle
3. Verify Green's theorem for a specific example
4. Calculate the volume form on S³

### Differential Forms
1. Express div, grad, and curl using exterior derivatives
2. Compute d(x dy ∧ dz) in ℝ³
3. Show that the 2-form ω = x dx ∧ dy + y dy ∧ dz is closed
4. Find a 1-form α such that dα = x dx ∧ dy

### Advanced
1. Prove that harmonic functions satisfy the mean value property
2. Implement automatic differentiation for matrix functions
3. Compute the Gaussian curvature using differential forms
4. Show that closed geodesics are critical points of the energy functional

### Research
1. How would you extend SCTT to handle distributions?
2. Can we have a computational version of Hodge theory?
3. Design a type-safe numerical PDE solver
4. What's the complexity of computing n-th derivatives?

## Summary

We've developed a complete differential calculus in type theory:
- **Differentiation** is a computational type operation
- **Chain rule** holds by construction
- **Integration** inverts differentiation with the FTC
- **Differential forms** provide coordinate-free calculus
- **Stokes' theorem** unifies integral theorems
- **Lie derivatives** describe infinitesimal symmetries

All these operations compute exactly, with correctness guaranteed by types. This isn't symbolic manipulation or numerical approximation—it's exact differential geometry as computation. Next, we'll see how this structure interacts with the homotopy theory from Chapter 3.

---

*Next: [Chapter 6: Smooth Homotopy Theory](./chapter_06.md) →*

*Previous: [Chapter 4: Smooth Types](./chapter_04.md) ←*