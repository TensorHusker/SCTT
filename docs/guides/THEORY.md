# Smooth Cubical Type Theory - Mathematical Theory

## 1. Introduction

Smooth Cubical Type Theory (SCTT) extends cubical type theory with smooth structure, enabling formal reasoning about smooth spaces, manifolds, and continuous/differentiable functions within a computational type theory.

## 2. Basic Type Theory

### 2.1 Judgments

```
Γ ⊢             Γ is a well-formed context
Γ ⊢ A type      A is a type in context Γ  
Γ ⊢ a : A       Term a has type A in context Γ
Γ ⊢ a ≡ b : A   Terms a and b are judgmentally equal at type A
```

### 2.2 Core Types

#### Universe
```
Γ ⊢
-----------
Γ ⊢ 𝒰 type
```

#### Dependent Functions (Π-types)
```
Γ ⊢ A type    Γ, x:A ⊢ B type
--------------------------------
Γ ⊢ Π(x:A).B type

Γ, x:A ⊢ b : B
----------------------
Γ ⊢ λx.b : Π(x:A).B

Γ ⊢ f : Π(x:A).B    Γ ⊢ a : A
---------------------------------
Γ ⊢ f(a) : B[a/x]
```

#### Dependent Pairs (Σ-types)
```
Γ ⊢ A type    Γ, x:A ⊢ B type
--------------------------------
Γ ⊢ Σ(x:A).B type

Γ ⊢ a : A    Γ ⊢ b : B[a/x]
------------------------------
Γ ⊢ (a,b) : Σ(x:A).B
```

## 3. Cubical Structure

### 3.1 The Interval

The interval I is a special type representing the unit interval [0,1]:

```
Γ ⊢
-----------
Γ ⊢ I type

Γ ⊢            Γ ⊢
-----------    -----------
Γ ⊢ 0 : I      Γ ⊢ 1 : I
```

#### Interval Operations
- **Meet**: `r ∧ s` (minimum)
- **Join**: `r ∨ s` (maximum)  
- **Negation**: `1-r`

#### De Morgan Algebra Laws
```
(r ∧ s) ∧ t ≡ r ∧ (s ∧ t)
r ∧ s ≡ s ∧ r
r ∧ 1 ≡ r
r ∧ 0 ≡ 0
r ∧ (s ∨ t) ≡ (r ∧ s) ∨ (r ∧ t)
1-(r ∧ s) ≡ (1-r) ∨ (1-s)
1-1-r ≡ r
```

### 3.2 Path Types

Paths represent equality proofs with computational content:

```
Γ ⊢ A : I → 𝒰    Γ ⊢ a : A(0)    Γ ⊢ b : A(1)
-------------------------------------------------
Γ ⊢ Path A a b type
```

#### Path Abstraction
```
Γ, i:I ⊢ t : A
----------------------
Γ ⊢ ⟨i⟩ t : Path A t[0/i] t[1/i]
```

#### Path Application
```
Γ ⊢ p : Path A a b    Γ ⊢ r : I
----------------------------------
Γ ⊢ p @ r : A(r)

(⟨i⟩ t) @ r ≡ t[r/i]
p @ 0 ≡ a
p @ 1 ≡ b
```

### 3.3 Composition Structure

#### Transport
```
transport : {A : I → 𝒰} → A(0) → A(1)
transport A a = comp A [] a
```

#### Composition
```
comp : (A : I → 𝒰) → (φ : 𝔽) → (u : Π(i:I). Partial φ (A i)) → A(0) → A(1)
```

Where:
- `𝔽` is the type of face formulas
- `Partial φ A` is the type of partial elements of A defined on φ

#### Kan Filling
```
fill : (A : I → 𝒰) → (φ : 𝔽) → (u : Π(i:I). Partial φ (A i)) → A(0) → Π(i:I). A(i)
fill A φ u a₀ = ⟨j⟩ comp (⟨i⟩ A(i ∧ j)) (φ ∨ (j=0)) u' a₀
```

## 4. Smooth Structure

### 4.1 Smooth Types

A type is smooth if it admits a smooth structure:

```
Γ ⊢ A type
----------------
Γ ⊢ Smooth A type
```

#### Smooth Interval
The interval I inherits smooth structure from ℝ:

```
Γ ⊢
------------------
Γ ⊢ I : SmoothType
```

### 4.2 Smooth Functions

#### Smooth Function Type
```
Γ ⊢ A : SmoothType    Γ ⊢ B : SmoothType
-------------------------------------------
Γ ⊢ C^∞(A, B) type
```

#### Smooth Lambda Abstraction
```
Γ, x:A ⊢ b : B    [smooth conditions satisfied]
-------------------------------------------------
Γ ⊢ λ^∞ x.b : C^∞(A, B)
```

### 4.3 Differentiation

#### Tangent Spaces
For each smooth type M and point x:M, there is a tangent space:

```
Γ ⊢ M : SmoothType    Γ ⊢ x : M
----------------------------------
Γ ⊢ T_x M type
```

#### Tangent Bundle
```
Γ ⊢ M : SmoothType
------------------------------
Γ ⊢ TM ≡ Σ(x:M). T_x M type
```

#### Differential
```
Γ ⊢ f : C^∞(M, N)    Γ ⊢ x : M
---------------------------------
Γ ⊢ df_x : T_x M → T_{f(x)} N
```

### 4.4 Smooth Paths

Smooth paths are paths that vary smoothly with the interval parameter:

```
Γ ⊢ A : SmoothType    Γ ⊢ a : A    Γ ⊢ b : A
------------------------------------------------
Γ ⊢ SmoothPath A a b ≡ C^∞(I, A) ∩ Path A a b
```

## 5. Higher Smooth Structure

### 5.1 Smooth Homotopy Types

#### Smooth Circle S¹
```
S¹ : SmoothType
S¹ = SmoothHIT
  | base : S¹
  | loop : SmoothPath S¹ base base
  with smooth structure from ℝ/ℤ
```

#### Smooth Sphere S^n
```
S^n : SmoothType
S^n = SmoothHIT
  | north : S^n
  | south : S^n  
  | meridian : Π(x:S^{n-1}). SmoothPath S^n north south
  with smooth structure from unit sphere in ℝ^{n+1}
```

### 5.2 Smooth Modalities

#### Infinitesimal Modality
```
◯ : SmoothType → SmoothType
◯A = formal infinitesimal neighborhood of A
```

With unit and multiplication:
```
η : A → ◯A
μ : ◯◯A → ◯A
```

### 5.3 Jet Bundles

The k-jet bundle captures derivatives up to order k:

```
J^k : C^∞(M, N) → Type
J^k(f) = Σ(x:M). (f(x), df_x, d²f_x, ..., d^k f_x)
```

## 6. Cohesion

### 6.1 Cohesive Structure

SCTT admits a cohesive structure with modalities:

```
⊣ ♯ ⊣ ♭ ⊣ ♮
```

Where:
- `♯` : discrete types (forget smooth structure)
- `♭` : codiscrete types  
- `♮` : shape modality

### 6.2 Differential Cohesion

Additional modalities for differential structure:

```
ℑ ⊣ ℜ ⊣ &
```

Where:
- `ℑ` : infinitesimal shape
- `ℜ` : reduction modality
- `&` : infinitesimal flat modality

## 7. Axioms and Properties

### 7.1 Axiom of Smooth Choice

For smooth types, we have a choice principle:

```
Π(x:A). ∃(y:B). P(x,y) → ∃(f:C^∞(A,B)). Π(x:A). P(x, f(x))
```

### 7.2 Smooth Univalence

For smooth types A, B:

```
(A ≃^∞ B) ≃ (A ≡ B)
```

Where `≃^∞` denotes smooth equivalence (diffeomorphism).

### 7.3 Function Extensionality for Smooth Functions

```
Π(f,g : C^∞(A,B)). (Π(x:A). f(x) ≡ g(x)) → f ≡ g
```

## 8. Models

### 8.1 Smooth ∞-Topoi

SCTT has models in smooth ∞-topoi, particularly:
- The ∞-topos of smooth ∞-groupoids
- The ∞-topos of diffeological spaces
- The ∞-topos of smooth manifolds and smooth maps

### 8.2 Cubical Sets with Smooth Structure

The category of cubical sets equipped with smooth structure:
- Objects: Presheaves on the smooth cube category
- Morphisms: Natural transformations preserving smooth structure

## 9. Constructions

### 9.1 Smooth Real Numbers

```
ℝ : SmoothType
ℝ = (Cauchy sequences of rationals) / (equivalence)
    with standard smooth structure
```

### 9.2 Lie Groups

A Lie group is:
```
LieGroup = Σ(G : SmoothType). 
           Σ(· : C^∞(G × G, G)).
           Σ(e : G).
           Σ(inv : C^∞(G, G)).
           (group axioms) × (smooth compatibility)
```

### 9.3 Vector Bundles

```
VectorBundle : (M : SmoothType) → (n : ℕ) → Type
VectorBundle M n = 
  Σ(E : SmoothType).
  Σ(π : C^∞(E, M)).
  Σ(fiber : Π(x:M). (π⁻¹(x) ≃ ℝ^n)).
  (local trivialization data)
```

## 10. Computation Rules

### 10.1 β-reduction for Smooth Functions
```
(λ^∞ x:A. b)(a) ≡ b[a/x]
```

### 10.2 η-expansion for Smooth Functions
```
f ≡ λ^∞ x. f(x)  [when f : C^∞(A,B)]
```

### 10.3 Chain Rule
```
d(g ∘ f)_x ≡ dg_{f(x)} ∘ df_x
```

### 10.4 Leibniz Rule
```
d(f · g)_x ≡ df_x · g(x) + f(x) · dg_x
```

## 11. Consistency and Decidability

### 11.1 Consistency

SCTT is consistent relative to:
- ZFC + inaccessible cardinals (via smooth ∞-topos models)
- Cubical type theory + classical smooth manifold theory

### 11.2 Decidability

Type checking is decidable for:
- Core cubical terms without smooth structure
- Smooth terms with finite-order derivatives
- First-order smooth logic

Undecidable for:
- General smooth equality (requires solving differential equations)
- Higher-order smooth properties

## 12. Future Directions

1. **Synthetic Differential Geometry**: Full integration with SDG
2. **Smooth ∞-Categories**: Higher categorical smooth structure  
3. **Quantum Field Theory**: Applications to QFT formalization
4. **Numerical Methods**: Computational interpretation of smooth operations
5. **Machine Learning**: Types for differentiable programming