# Chapter 7: SCTT Formal Rules

> "In mathematics, the art of asking questions is more valuable than solving problems." — Georg Cantor
>
> "In SCTT, formal rules provide the questions that computation answers."

## Introduction

Having explored the intuitive and mathematical foundations of SCTT in [Chapters 1-6](./chapter_01.md), we now turn to its formal specification. This chapter presents the complete formal system of Smooth Cubical Type Theory: its judgment forms, inference rules, and computational behavior.

SCTT combines three sophisticated type systems:
1. **Martin-Löf Type Theory**: The foundation of dependent types and constructive reasoning
2. **Cubical Type Theory**: Providing computational univalence and higher-dimensional structure  
3. **Smooth Type Theory**: Adding differential structure and computational calculus

The challenge is integrating these systems while preserving their essential properties: constructivity from MLTT (detailed in [Chapter 2](./chapter_02.md)), computational univalence from CTT ([Chapter 3](./chapter_03.md)), and exact differentiation from smooth types ([Chapters 4-5](./chapter_04.md)). This formal system is implemented in the type checker described in [Chapter 9](./chapter_09.md).

### The Formal System Architecture

```sctt
-- The three-layer architecture of SCTT
SCTT_System : FormalSystem
SCTT_System = {
  -- Layer 1: Basic type theory (MLTT foundation)
  basic_judgments = {
    context_formation : "⊢ Γ ctx",
    type_formation : "Γ ⊢ A : Type",  
    term_formation : "Γ ⊢ a : A",
    definitional_equality : "Γ ⊢ a ≡ b : A"
  },
  
  -- Layer 2: Cubical structure (paths and composition)
  cubical_extension = {
    interval_type : "Γ ⊢ I : Type",
    path_types : "Γ ⊢ PathP A x y : Type",
    composition_operation : "comp",
    transport_operation : "transp"
  },
  
  -- Layer 3: Smooth structure (differential operations)
  smooth_extension = {
    smooth_types : "Γ ⊢ A : SmoothType",
    differential_operator : "Γ ⊢ D : C∞(A,B) → C∞(A,B)",
    integration_operator : "Γ ⊢ ∫ : C∞(A,ℝ) → ℝ"
  }
}
```

## 7.1 Judgment Forms {#judgments}

### Basic Judgments

SCTT extends the standard four judgment forms of Martin-Löf Type Theory:

#### Context Formation
```
⊢ Γ ctx
```
States that Γ is a well-formed context (sequence of variable declarations).

**Rules:**
```
—————————————  (Ctx-Empty)
⊢ · ctx

⊢ Γ ctx    Γ ⊢ A : Type
————————————————————————  (Ctx-Extend)  
⊢ Γ, x : A ctx
```

#### Type Formation  
```
Γ ⊢ A : Type
```
States that A is a well-formed type in context Γ.

#### Term Formation
```
Γ ⊢ a : A  
```
States that term a has type A in context Γ.

#### Definitional Equality
```
Γ ⊢ a ≡ b : A
```
States that terms a and b are definitionally equal at type A.

### Extended Judgments for SCTT

#### Smooth Type Formation
```
Γ ⊢ A : SmoothType
```
States that A is a smooth type (has differential structure).

**Rules:**
```
Γ ⊢ A : Type    Γ ⊢ smooth_structure : SmoothStructure A
———————————————————————————————————————————————————————  (Smooth-Type)
Γ ⊢ A : SmoothType

——————————————————————  (Smooth-Real)
Γ ⊢ ℝ : SmoothType

Γ ⊢ M : SmoothType    Γ ⊢ N : SmoothType  
————————————————————————————————————————  (Smooth-Function)
Γ ⊢ C∞(M,N) : SmoothType
```

#### Cubical Judgments
```
Γ ⊢ φ : F        -- Face formula
Γ ⊢ a : A [φ]    -- Partial term (defined when φ holds)
```

**Face formulas** F are built from:
```
φ, ψ : F ::= ⊥ | ⊤ | (i = i0) | (i = i1) | φ ∧ ψ | φ ∨ ψ
```

#### Smooth Equality Judgment
```
Γ ⊢ f ≋ g : C∞(M,N)
```
States that smooth functions f and g are smoothly equal (equal as smooth maps).

## 7.2 Type Formation Rules {#formation}

### Universe Hierarchy

SCTT has a hierarchy of universes to avoid paradoxes:

```
Γ ⊢ Type₀ : Type₁
Γ ⊢ Type₁ : Type₂  
⋮
Γ ⊢ Typeᵢ : Typeᵢ₊₁

-- Universe cumulativity
Γ ⊢ A : Typeᵢ    i ≤ j
————————————————————————  (Cumulative)
Γ ⊢ A : Typeⱼ
```

### Function Types (Π-types)

```
Γ ⊢ A : Type    Γ, x : A ⊢ B : Type
————————————————————————————————————  (Π-form)
Γ ⊢ (x : A) → B : Type

-- Non-dependent function types  
Γ ⊢ A : Type    Γ ⊢ B : Type
————————————————————————————  (→-form)
Γ ⊢ A → B : Type
```

### Pair Types (Σ-types)

```
Γ ⊢ A : Type    Γ, x : A ⊢ B : Type
————————————————————————————————————  (Σ-form)
Γ ⊢ (x : A) × B : Type

-- Non-dependent pair types
Γ ⊢ A : Type    Γ ⊢ B : Type  
————————————————————————————  (×-form)
Γ ⊢ A × B : Type
```

### Path Types

The key innovation from cubical type theory:

```
Γ ⊢ A : I → Type    Γ ⊢ x : A i0    Γ ⊢ y : A i1
—————————————————————————————————————————————————  (PathP-form)
Γ ⊢ PathP A x y : Type

-- Homogeneous paths
Γ ⊢ A : Type    Γ ⊢ x : A    Γ ⊢ y : A
————————————————————————————————————————  (Path-form)
Γ ⊢ Path A x y : Type
```

### Smooth Function Types

```
Γ ⊢ M : SmoothType    Γ ⊢ N : SmoothType
————————————————————————————————————————  (C∞-form)
Γ ⊢ C∞(M,N) : SmoothType

-- Smooth paths
Γ ⊢ M : SmoothType    Γ ⊢ x : M    Γ ⊢ y : M
—————————————————————————————————————————————  (SmoothPath-form)
Γ ⊢ SmoothPath M x y : SmoothType
```

### Inductive Types

General schema for inductive type formation:

```
Γ ⊢ A₁ : Type    ⋯    Γ ⊢ Aₙ : Type
————————————————————————————————————————————————  (Inductive-form)
Γ ⊢ data D (parameters) : Type where
     constructor₁ : Type₁
     ⋮  
     constructorₖ : Typeₖ
```

**Examples:**
```sctt
-- Natural numbers
data ℕ : Type where
  zero : ℕ
  succ : ℕ → ℕ

-- Lists  
data List (A : Type) : Type where
  nil : List A
  cons : A → List A → List A

-- Smooth circle (higher inductive type)
data S¹ : SmoothType where
  base : S¹
  loop : SmoothPath S¹ base base
```

## 7.3 Introduction Rules {#introduction}

### Function Introduction (Lambda Abstraction)

```
Γ, x : A ⊢ b : B
—————————————————————————  (Π-intro)
Γ ⊢ λ x → b : (x : A) → B
```

### Pair Introduction

```
Γ ⊢ a : A    Γ ⊢ b : B[a/x]
————————————————————————————  (Σ-intro)
Γ ⊢ (a, b) : (x : A) × B
```

### Path Introduction

```
Γ, i : I ⊢ p : A    Γ ⊢ p[i0/i] ≡ x : A i0    Γ ⊢ p[i1/i] ≡ y : A i1
——————————————————————————————————————————————————————————————————————  (PathP-intro)
Γ ⊢ λ i → p : PathP A x y
```

### Smooth Function Introduction

```
Γ ⊢ M : SmoothType    Γ ⊢ N : SmoothType    Γ, x : M ⊢ f : N    Γ ⊢ smooth_proof : IsSmoothFunction f
———————————————————————————————————————————————————————————————————————————————————————————————————  (C∞-intro)
Γ ⊢ smooth_function f smooth_proof : C∞(M,N)
```

### Inductive Type Introduction

For each constructor of an inductive type:

```
Γ ⊢ a₁ : A₁    ⋯    Γ ⊢ aₙ : Aₙ
————————————————————————————————————  (Constructor)
Γ ⊢ constructor a₁ ⋯ aₙ : InductiveType
```

## 7.4 Elimination Rules {#elimination}

### Function Elimination (Application)

```
Γ ⊢ f : (x : A) → B    Γ ⊢ a : A
—————————————————————————————————  (Π-elim)
Γ ⊢ f a : B[a/x]
```

### Pair Elimination (Projections)

```
Γ ⊢ p : (x : A) × B
————————————————————  (Σ-elim₁)
Γ ⊢ π₁ p : A

Γ ⊢ p : (x : A) × B
————————————————————————————  (Σ-elim₂)  
Γ ⊢ π₂ p : B[π₁ p/x]
```

### Path Elimination (Path Application)

```
Γ ⊢ p : PathP A x y    Γ ⊢ i : I
—————————————————————————————————  (PathP-elim)
Γ ⊢ p @ i : A i
```

### Smooth Function Application

```
Γ ⊢ f : C∞(M,N)    Γ ⊢ x : M
———————————————————————————————  (C∞-elim)
Γ ⊢ f x : N
```

### Inductive Type Elimination (Pattern Matching)

General elimination rule for inductive types:

```
Γ ⊢ x : InductiveType    
Γ, x : InductiveType ⊢ P : Type
Γ ⊢ case₁ : P[constructor₁(...)]
⋮
Γ ⊢ caseₖ : P[constructorₖ(...)]
—————————————————————————————————————————————  (Inductive-elim)
Γ ⊢ match x with 
     | constructor₁ ... → case₁
     | ⋮
     | constructorₖ ... → caseₖ
   : P[x]
```

## 7.5 Computation Rules {#computation}

### β-reduction (Computation)

#### Functions
```
Γ, x : A ⊢ b : B    Γ ⊢ a : A
—————————————————————————————————————  (Π-β)
Γ ⊢ (λ x → b) a ≡ b[a/x] : B[a/x]
```

#### Pairs  
```
Γ ⊢ a : A    Γ ⊢ b : B[a/x]
————————————————————————————  (Σ-β₁)
Γ ⊢ π₁ (a, b) ≡ a : A

Γ ⊢ a : A    Γ ⊢ b : B[a/x]  
————————————————————————————————  (Σ-β₂)
Γ ⊢ π₂ (a, b) ≡ b : B[a/x]
```

#### Paths
```
Γ, i : I ⊢ p : A    boundary conditions satisfied
——————————————————————————————————————————————  (PathP-β)
Γ ⊢ (λ i → p) @ j ≡ p[j/i] : A j
```

#### Smooth Functions
```  
Γ, x : M ⊢ f : N    Γ ⊢ smooth_proof : IsSmoothFunction f    Γ ⊢ a : M
—————————————————————————————————————————————————————————————————————————  (C∞-β)
Γ ⊢ (smooth_function f smooth_proof) a ≡ f[a/x] : N
```

### η-expansion (Uniqueness)

#### Functions
```
Γ ⊢ f : (x : A) → B
————————————————————————————  (Π-η)
Γ ⊢ f ≡ λ x → f x : (x : A) → B
```

#### Pairs
```
Γ ⊢ p : (x : A) × B
————————————————————————————  (Σ-η)
Γ ⊢ p ≡ (π₁ p, π₂ p) : (x : A) × B
```

#### Paths
Path η-expansion is more complex due to boundary conditions.

### Smooth-Specific Computation Rules

#### Differentiation
```
Γ ⊢ f : C∞(ℝ,ℝ)    Γ ⊢ x : ℝ
————————————————————————————————————————————————————————  (Derivative-β)
Γ ⊢ D[f] x ≡ lim[h→0] (f(x+h) - f(x))/h : ℝ

-- But computed exactly via Kock-Lawvere axiom:
Γ ⊢ D[f] x ≡ the_unique_b_such_that(∀ε:𝔻, f(x+ε) = f(x) + b·ε) : ℝ
```

#### Chain Rule
```
Γ ⊢ f : C∞(M,N)    Γ ⊢ g : C∞(N,P)
————————————————————————————————————————  (Chain-rule-β)
Γ ⊢ D[g ∘ f] ≡ D[g] ∘ D[f] : C∞(TM,TP)
```

#### Integration
```
Γ ⊢ f : C∞(ℝ,ℝ)    Γ ⊢ a : ℝ    Γ ⊢ b : ℝ
—————————————————————————————————————————————————————————  (Integration-FTC)
Γ ⊢ ∫ a b D[F] ≡ F b - F a : ℝ
  where F is antiderivative of f
```

## 7.6 Uniqueness Rules {#uniqueness}

### Standard Uniqueness (η-rules)

Already covered above for functions, pairs, etc.

### Cubical Uniqueness

#### Path Uniqueness
Paths with the same boundary are unique up to homotopy:

```
Γ ⊢ p : PathP A x y    Γ ⊢ q : PathP A x y
——————————————————————————————————————————  (Path-Unique)
Γ ⊢ ∃! (H : PathP (PathP A x y) p q), H i0 ≡ refl ∧ H i1 ≡ refl
```

#### Composition Uniqueness
The composition operation is unique:

```
Γ ⊢ φ : F    Γ ⊢ u : (i : I) → Partial φ A    Γ ⊢ u0 : A[φ ↦ u i0]
—————————————————————————————————————————————————————————————————————  (Comp-Unique)
Γ ⊢ ∃! (comp A φ u u0), satisfying_Kan_conditions
```

### Smooth Uniqueness  

#### Derivative Uniqueness
The derivative of a smooth function is unique:

```
Γ ⊢ f : C∞(M,N)    Γ ⊢ x : M
——————————————————————————————————————————————————————————————————————  (Derivative-Unique)
Γ ⊢ ∃! (df_x : LinearMap(TangentSpace M x, TangentSpace N (f x))),
      ∀(v : TangentVector M x), lim[t→0] (f(x + tv) - f(x))/t = df_x(v)
```

#### Antiderivative Uniqueness
Antiderivatives are unique up to constants:

```
Γ ⊢ f : C∞(ℝ,ℝ)    Γ ⊢ F : C∞(ℝ,ℝ)    Γ ⊢ G : C∞(ℝ,ℝ)
Γ ⊢ D[F] ≡ f : C∞(ℝ,ℝ)    Γ ⊢ D[G] ≡ f : C∞(ℝ,ℝ)
———————————————————————————————————————————————————————————————————————  (Antiderivative-Unique)
Γ ⊢ ∃! (c : ℝ), G ≡ F + constant_function c : C∞(ℝ,ℝ)
```

## 7.7 Advanced Rules

### Cubical Composition

The heart of cubical type theory—the composition operation:

```
Γ ⊢ A : I → Type    
Γ ⊢ φ : F
Γ ⊢ u : (i : I) → Partial φ (A i)  
Γ ⊢ u0 : A i0 [φ ↦ u i0]
————————————————————————————————————————————  (Comp)
Γ ⊢ comp A φ u u0 : A i1
```

With computation rules:
```
-- Boundary condition
Γ ⊢ comp A φ u u0 ≡ u i1 : A i1    (when φ = ⊤)

-- Base case  
Γ ⊢ comp A ⊥ u u0 ≡ transp A i0 u0 : A i1
```

### Transport

Moving along paths in type families:

```
Γ ⊢ A : I → Type    Γ ⊢ a : A i0
——————————————————————————————————  (Transp)
Γ ⊢ transp A i0 a : A i1

-- With computation rule
Γ ⊢ transp (λ _ → B) i0 b ≡ b : B    (constant family)
```

### Glue Types

The mechanism that makes univalence compute:

```
Γ ⊢ A : Type    
Γ ⊢ φ : F
Γ ⊢ Te : Partial φ (Σ (T : Type), T ≃ A)
————————————————————————————————————————————————  (Glue-form)
Γ ⊢ Glue A φ Te : Type

-- With introduction/elimination
Γ ⊢ glue : PartialP φ (λ o → Te o .fst) → A → Glue A φ Te
Γ ⊢ unglue : Glue A φ Te → A
```

### Smooth Composition

Extension of cubical composition to smooth types:

```
Γ ⊢ M : I → SmoothType
Γ ⊢ φ : F  
Γ ⊢ u : (i : I) → Partial φ (M i)
Γ ⊢ u0 : M i0 [φ ↦ u i0]
Γ ⊢ smooth_conditions : AllPartialElementsAreSmooth u
—————————————————————————————————————————————————————————  (Smooth-Comp)
Γ ⊢ smooth_comp M φ u u0 : M i1

-- Smoothness is preserved
Γ ⊢ smooth_proof : IsSmooth (smooth_comp M φ u u0)
```

## 7.8 Definitional Equality Rules

### Congruence Rules

Definitional equality is preserved by all type formers:

```
Γ ⊢ A ≡ A' : Type    Γ ⊢ B ≡ B' : Type
————————————————————————————————————————  (→-cong)
Γ ⊢ A → B ≡ A' → B' : Type

Γ ⊢ A ≡ A' : Type    Γ, x : A ⊢ B ≡ B' : Type
——————————————————————————————————————————————  (Π-cong)
Γ ⊢ (x : A) → B ≡ (x : A') → B' : Type

-- Similar rules for all type formers
```

### Smooth Congruence

```
Γ ⊢ M ≡ M' : SmoothType    Γ ⊢ N ≡ N' : SmoothType
————————————————————————————————————————————————————  (C∞-cong)
Γ ⊢ C∞(M,N) ≡ C∞(M',N') : SmoothType

Γ ⊢ f ≡ f' : C∞(M,N)
——————————————————————————————  (Derivative-cong)
Γ ⊢ D[f] ≡ D[f'] : C∞(TM,TN)
```

### Reflexivity, Symmetry, Transitivity

```
Γ ⊢ a : A
————————————————  (Eq-refl)
Γ ⊢ a ≡ a : A

Γ ⊢ a ≡ b : A
————————————————  (Eq-symm)
Γ ⊢ b ≡ a : A

Γ ⊢ a ≡ b : A    Γ ⊢ b ≡ c : A
——————————————————————————————  (Eq-trans)
Γ ⊢ a ≡ c : A
```

## 7.9 Consistency and Normalization

### Consistency Theorem

**Theorem 7.9.1 (Consistency of SCTT)**: SCTT is consistent, i.e., there is no term of the empty type:
```
¬∃(Γ : Context)(t : Term), Γ ⊢ t : ⊥
```

**Proof sketch**: By constructing a model in cubical sets with smooth structure, showing that the empty type has no elements in any model.

### Strong Normalization

**Theorem 7.9.2 (Strong Normalization)**: Every well-typed SCTT term has a normal form, and every reduction sequence terminates.

**Note**: This is more complex in SCTT due to smooth computations potentially involving infinite data, but holds for the purely type-theoretic fragment.

### Canonicity

**Theorem 7.9.3 (Canonicity)**: Every closed term of natural number type is definitionally equal to a numeral.

```
∀(t : Term), (⊢ t : ℕ) → ∃(n : Numeral), ⊢ t ≡ n : ℕ
```

This extends to smooth types in a more sophisticated way involving smooth canonical forms.

## 7.10 Implementation Notes

### Bidirectional Type Checking

SCTT uses bidirectional type checking:

```
-- Inference mode: synthesize type
Γ ⊢ e ⇒ A

-- Checking mode: check against expected type  
Γ ⊢ e ⇐ A
```

**Key rules:**
```
Γ ⊢ x ⇒ Γ(x)                    -- Variable lookup

Γ ⊢ e ⇒ A    A ≡ B
————————————————————————————      -- Type conversion
Γ ⊢ e ⇐ B

Γ, x : A ⊢ b ⇐ B
————————————————————————————      -- Lambda checking
Γ ⊢ λx. b ⇐ A → B

Γ ⊢ f ⇒ A → B    Γ ⊢ a ⇐ A
———————————————————————————      -- Application synthesis
Γ ⊢ f a ⇒ B
```

### Normalization by Evaluation

SCTT implementations typically use normalization by evaluation (NbE):

1. **Evaluate** terms to semantic values
2. **Quote** values back to normal forms
3. **Compare** normal forms for equality

This is especially important for smooth functions where syntactic equality is undecidable.

### Smooth Function Representation

Smooth functions require special representation:

```sctt
-- Internal representation of smooth functions
SmoothFunctionRep = {
  syntactic_form : SyntacticTerm,
  derivative_info : DerivativeData,  
  taylor_expansion : TaylorSeries,
  numerical_implementation : ℝ → ℝ,
  error_bounds : ErrorAnalysis
}
```

## Summary

This chapter has presented the complete formal specification of SCTT:

### Core Components
- **Judgment forms**: Extended with smooth types and cubical structure
- **Formation rules**: Types, including smooth and path types
- **Introduction rules**: Constructors for all type formers
- **Elimination rules**: Destructors and computation principles
- **Computation rules**: β-reduction and smooth-specific computations
- **Uniqueness rules**: η-expansion and uniqueness principles

### Key Innovations
1. **Smooth-cubical integration**: Seamless combination of differential and homotopical structure
2. **Computational derivatives**: Exact differentiation via Kock-Lawvere axiom
3. **Verified smooth functions**: Functions that carry their own smoothness proofs
4. **Higher-dimensional composition**: Cubical operations extended to smooth types

### Implementation Insights
- **Bidirectional type checking**: Efficient inference and checking
- **Normalization by evaluation**: Handling complex equalities
- **Special smooth representations**: Hybrid symbolic-numeric computation

The formal rules of SCTT provide the foundation for implementations that can verify smooth computations while preserving the elegant theoretical properties of type theory.

## Exercises

### Rule Application
1. Derive the type of the smooth composition operator using SCTT's formal rules
2. Show that the chain rule follows from the computation rules for smooth functions
3. Verify that path composition satisfies associativity using cubical composition
4. Prove that smooth functions form a category using the formal rules

### Metatheory  
1. Prove that SCTT's judgment forms are decidable for the syntactic fragment
2. Show that definitional equality is transitive for smooth function types
3. Verify that the elimination rules for paths are admissible
4. Prove subject reduction for the smooth function β-rule

### Implementation
1. Design a bidirectional type checker for a fragment of SCTT
2. Implement normalization by evaluation for path types
3. Create a representation for smooth functions that supports differentiation
4. Build a simple interpreter for SCTT terms

### Extensions
1. Add inductive-recursive types to SCTT's formal system
2. Extend the rules to support smooth higher inductive types  
3. Design formal rules for modal SCTT (from Chapter 13)
4. Create a dependent pattern matching extension

---

*Next: [Chapter 8: Metatheory](./chapter_08.md) →*

*Previous: [Chapter 6: Addressing Limitations and Challenges](./chapter_06.md) ←*
