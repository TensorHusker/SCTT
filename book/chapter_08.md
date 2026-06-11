# Chapter 8: Metatheory

> "The foundations of mathematics are not cast in concrete. They are living, breathing structures that grow and evolve with our understanding." — Vladimir Voevodsky
>
> "In SCTT, metatheory becomes executable—the proof sketches guide implementation, and the conjectures tell us what to verify."

## Introduction

Metatheory studies mathematical systems from the outside—asking whether they are consistent, whether proofs terminate, whether equality is decidable. For SCTT, metatheory becomes especially important because we're combining three sophisticated systems: dependent type theory, cubical structure, and smooth mathematics.

This chapter examines the metatheoretical properties we expect SCTT to enjoy:

1. **Consistency** - SCTT should not prove false statements
2. **Canonicity** - Every closed term should have a canonical form
3. **Decidability** - Type checking and equality should be algorithmically decidable
4. **Normalization** - All computations should terminate
5. **Computational Complexity** - Bounds on the computational cost

The metatheory of SCTT draws on several established results:
- **Cubical canonicity**: Huber (2016) for CCHM; Sterling and Angiuli (LICS 2021) for Cartesian cubical via Synthetic Tait Computability
- **Rewrite rule safety**: Cockx, Tabareau, and Winterhalter (POPL 2021) — the RTT framework
- **Normal form specification**: Huang (arXiv:2603.24923, 2026) — explicit normal forms for Cartesian cubical TT
- **Mechanized metatheory**: The Rewster (Leray et al., ITP 2024) — MetaCoq verification of rewrite rule safety
- **Sensitivity metatheory**: Azevedo de Amorim et al. (POPL 2017) — metric preservation for sensitivity types

Where these results cover only individual components, we state the combined SCTT result as a **conjecture** and describe the evidence supporting it. The proof sketches in this chapter remain pedagogically valuable: they show the shape a full metatheoretic argument would take, and they are largely complete for the individual subsystems. The gap lies in the interaction of all three structures simultaneously.

### Why Metatheory Matters for SCTT

Traditional type theory already requires careful metatheoretical analysis. SCTT adds new challenges:

```sctt
-- Traditional challenges
dependent_types_consistency : Challenge
dependent_types_consistency = "Russell's paradox must be avoided"

cubical_structure_coherence : Challenge  
cubical_structure_coherence = "Composition operations must satisfy laws"

-- New SCTT challenges
smooth_structure_consistency : Challenge
smooth_structure_consistency = "Smooth operations must preserve type structure"

computational_smoothness : Challenge
computational_smoothness = "Derivatives must compute correctly"

integration_coherence : Challenge
integration_coherence = "All three structures must work together"
```

### The SCTT Metatheoretical Picture

```sctt
-- SCTT metatheory: target properties (status indicated)
SCTT_Metatheory : MetatheoreticalFramework
SCTT_Metatheory = {
  -- Core properties (CONJECTURED for full SCTT; proven for subsystems)
  consistency : ¬∃(Γ : Context)(t : Term), Γ ⊢ t : ⊥,           -- Conjecture 8.1
  canonicity : ∀(t : ClosedTerm)(A : Type), (⊢ t : A) → HasCanonicalForm t,  -- Conjecture 8.2
  decidability : Decidable TypeChecking ∧ Decidable DefEq,       -- Conditional 8.3 (cubical fragment, assuming normalization; smooth layer: fragments only, see §8.3)
  normalization : StronglyNormalizing SCTT_reduction,              -- Conjecture 8.4
  
  -- Smooth-specific properties  
  smooth_consistency : SmoothOperations preserve TypeStructure,
  derivative_correctness : Computational_derivatives ≡ Mathematical_derivatives,
  
  -- Implementation properties
  computable : AllProperties have ComputationalContent,
  verifiable : AllProofs are MechanicallyCheckable
}
```

## 8.1 Consistency {#consistency}

### The Consistency Conjecture

The fundamental requirement: SCTT must not prove false statements.

#### Statement and Status

```sctt
-- Consistency conjecture for SCTT
conjecture sctt_consistency : Consistent SCTT
sctt_consistency = ¬∃(Γ : Context)(t : Term), Γ ⊢ t : ⊥

-- Status: OPEN for the full combined system
-- Partial results:
--   ✓ MLTT consistency: classical (Martin-Löf, 1984)
--   ✓ Cubical (CCHM) consistency: Huber's canonicity theorem (2016)
--   ✓ Cartesian cubical consistency: model in Cartesian cubical sets (Angiuli et al., 2021)
--   ✗ Combined cubical + smooth + sensitivity: OPEN
--   ✗ With rewrite rules (ε² = 0): requires RTT metatheory verification
```

#### Proof Strategy (Sketch)

The standard approach would proceed by logical relations:

```sctt
-- Proof sketch by logical relations
proof_sketch sctt_consistency = 
  logical_relations_model models SCTT ∧
  logical_relations_model ⊨ ⊥ → Empty ∧
  soundness : (Γ ⊢ t : A) → (logical_relations_model ⊨ Γ ⊢ t : A)
  ⟹ ¬∃(t : Term), ⊢ t : ⊥
```

> **Honest Assessment**: No published proof establishes consistency of the **full** SCTT
> system combining Cartesian cubical types, nilsquare infinitesimals (ε² = 0 as a rewrite rule),
> and Lipschitz sensitivity types. The individual components have consistency results, and the
> RTT framework (Cockx et al., 2021) provides conditions under which rewrite rules preserve
> consistency, but the combined system's consistency remains a conjecture.

#### The Logical Relations Model

We construct a model where every type is interpreted consistently:

```sctt
-- Logical relations interpretation
⟦_⟧ : Type → (Context → Type₁)

-- Base types
⟦ℕ⟧ Γ = {n : ℕ | n is closed numeral}
⟦ℝ⟧ Γ = {r : ℝ | r is computable real}
⟦⊥⟧ Γ = Empty  -- Empty type has no elements

-- Function types
⟦A → B⟧ Γ = {f : ⟦A⟧ Γ → ⟦B⟧ Γ | f preserves logical relations}

-- Dependent types  
⟦(x : A) → B⟧ Γ = ∀(a : ⟦A⟧ Γ), ⟦B⟧ (Γ, x ↦ a)

-- Path types
⟦Path A x y⟧ Γ = {p : I → ⟦A⟧ Γ | p(i0) = ⟦x⟧ Γ ∧ p(i1) = ⟦y⟧ Γ}

-- Smooth types
⟦C∞(M,N)⟧ Γ = {f : ⟦M⟧ Γ → ⟦N⟧ Γ | f is smooth ∧ f preserves structure}

-- Universe hierarchy prevents paradoxes
⟦Type₀⟧ Γ = SmallTypes
⟦Type₁⟧ Γ = TypesContaining SmallTypes
-- ...continuing stratification
```

#### Soundness of the Model

```sctt
-- Fundamental lemma: syntactic derivations are preserved semantically
fundamental_lemma : 
  ∀(Γ : Context)(t : Term)(A : Type),
  (Γ ⊢ t : A) → (⟦Γ⟧ ⊨ ⟦t⟧ : ⟦A⟧)

-- Proof by induction on typing derivations
proof fundamental_lemma Γ t A derivation = 
  case derivation of
    Var x → variable_interpretation
    App f a → application_preservation  
    Lam x body → abstraction_introduction
    -- ... cases for all constructors
    
    -- Smooth-specific cases
    SmoothApp f x → smooth_application_preservation
    Derivative f → derivative_interpretation_correct
    Integral f → integration_interpretation_sound
```

### Consistency of Smooth Operations

The challenge: ensuring smooth operations don't introduce inconsistencies.

#### Derivative Consistency

```sctt
-- Derivatives must be well-defined and consistent
derivative_consistency : 
  ∀(f : C∞(ℝ,ℝ))(x : ℝ), 
  ∃!(df_dx : ℝ), IsDerivative f x df_dx

-- Implementation via dual numbers is consistent
dual_number_consistency :
  ∀(f : C∞(ℝ,ℝ))(x : ℝ)(ε : DualNumber),
  f(x + ε) = f(x) + f'(x) * ε   -- exact: ε² = 0 by nilpotency, so there is no higher-order remainder

-- No contradictions arise from smooth structure
smooth_no_contradiction :
  ∀(smooth_derivation : SmoothDerivation),
  smooth_derivation ≠ ⊢ ⊥
```

#### Integration Consistency  

```sctt
-- Integration operations preserve consistency
integration_consistency :
  ∀(f : C∞([a,b], ℝ)),
  ∃!(I : ℝ), ∫ᵃᵇ f(x) dx = I

-- Fundamental theorem consistency
ftc_consistency :
  ∀(f : C∞([a,b], ℝ))(F : Antiderivative f),
  ∫ᵃᵇ f(x) dx = F(b) - F(a)

-- No paradoxes from smooth infinities
smooth_infinity_safety :
  ∀(computation : SmoothComputation),
  (computation diverges) → (computation : ¬ClosedTerm)
```

### Cubical Consistency

Ensuring cubical operations don't break consistency:

```sctt
-- Cubical composition is consistent
composition_consistency :
  ∀(A : I → Type)(φ : Formula)(u : PartialElement A φ)(u0 : A i0),
  ∃!(comp_result : A i1), comp A φ u u0 = comp_result

-- Univalence doesn't create paradoxes
univalence_consistency :
  (A ≃ B) → (A = B) doesn't imply Set-theoretic paradoxes

-- Transport preserves logical structure
transport_consistency :
  ∀(P : Type → Type)(e : A ≃ B)(a : P A),
  transport P e a : P B ∧ WellDefined
```

## 8.2 Canonicity {#canonicity}

### The Canonicity Conjecture

Every closed term of base type should have a canonical form.

#### Statement and Status

```sctt
-- Canonicity for SCTT (CONJECTURE)
conjecture sctt_canonicity :
  ∀(t : ClosedTerm)(A : BaseType), 
  (⊢ t : A) → ∃(canonical : CanonicalForm A), t →* canonical

-- Known results:
--   ✓ MLTT canonicity: classical
--   ✓ CCHM canonicity: Huber (2016), via Tait computability
--   ✓ Cartesian cubical canonicity: Sterling & Angiuli (LICS 2021),
--     via Synthetic Tait Computability (STC) — a type-theoretic
--     abstraction over algebraic gluing
--   ✗ Canonicity for cubical + ε² = 0: OPEN
--   ✗ Canonicity for full SCTT: OPEN
--
-- The Sterling-Angiuli proof provides the most promising path forward:
-- STC reduces canonicity to "trivial theorems of topos theory," and
-- Huang (arXiv:2603.24923, 2026) extracts explicit normal form specifications
-- from this proof that could be extended with nilpotent-reduced forms.

-- Base types have specific canonical forms
BaseType : Type → Type
BaseType ℕ = true
BaseType Bool = true
BaseType ℝ = false  -- a real is an infinite object: no finite literal normal form
BaseType (Path A x y) = false  -- paths are not a base type
BaseType (A → B) = false  -- not a base type
BaseType (Σ(x:A).B) = false

-- Canonical forms (only for genuine base types)
CanonicalForm : (A : BaseType) → Type
CanonicalForm ℕ = Numeral
CanonicalForm Bool = BoolLiteral

-- For ℝ the right notion is NOT a literal: a closed real canonically
-- denotes an effective Cauchy approximation — a computable sequence of
-- rationals with an explicit modulus of convergence. A real is an
-- infinite object, so it has no finite literal normal form.
```

> **Honest Assessment**: Canonicity for **Cartesian cubical type theory** alone is a theorem
> (Sterling & Angiuli, LICS 2021). What remains open is whether canonicity survives the addition
> of nilsquare infinitesimals (ε² = 0 as a rewrite rule) and Lipschitz sensitivity annotations.
> The RTT framework (Cockx et al., 2021) gives conditions for rewrite rules to preserve canonicity,
> but verifying these conditions for SCTT's specific rules is ongoing work.

#### Proof Sketch by Logical Relations

The following sketch shows how a canonicity proof would proceed, following the structure of known proofs for cubical type theory:

```sctt
-- Canonicity via logical relations (proof sketch)
canonical_logical_relations : 
  ∀(A : BaseType), ⟦A⟧ ∅ ⊆ CanonicalForm A

proof_sketch canonical_logical_relations A =
  case A of
    ℕ → naturals_are_numerals
    Bool → bools_are_literals

-- Key lemma: closed terms reduce to canonical forms
closed_terms_canonical :
  ∀(t : ClosedTerm)(A : BaseType),
  (⊢ t : A) → ∃(c : CanonicalForm A), t →* c

proof_sketch closed_terms_canonical t A typing =
  fundamental_lemma ∅ t A typing >>=
  canonical_logical_relations A
```

### Canonicity for Smooth Types

Smooth types require special treatment for canonicity:

#### Computable Reals

```sctt
-- Canonical forms for smooth reals
SmoothRealCanonical : Type
SmoothRealCanonical = 
  Rational |
  AlgebraicNumber |
  ComputableTranscendental |
  DefinedByComputingProcess

-- Every closed smooth real has computational content
smooth_real_canonicity :
  ∀(r : ClosedTerm), (⊢ r : SmoothReal) → 
  ∃(computation : ℝ-computation), r evaluates_to computation

-- Examples of canonical smooth reals
canonical_examples : List SmoothRealCanonical
canonical_examples = [
  π,  -- defined by computation
  e,  -- defined by series
  √2, -- algebraic number
  3.14159..., -- rational approximation
  ∫₀¹ eˣ dx  -- defined by integration
]
```

#### Canonical Smooth Functions

There is **no** canonicity theorem for smooth function types: a closed term of
`C∞(M,N)` does not normalize to a member of some finite taxonomy of expressions
(polynomial, rational, elementary, …). Smooth functions are higher-type, infinite
objects; the only honest canonicity statements are at genuine base types (ℕ, Bool),
and for ℝ via effective Cauchy approximations as above. What we *can* say is that
closed smooth functions carry computational content: applied to a (canonical
approximation of a) point, they evaluate.

```sctt
-- Examples
polynomial_canonical : C∞(ℝ,ℝ) 
polynomial_canonical = λ x → x³ - 2*x + 1

exponential_canonical : C∞(ℝ,ℝ)
exponential_canonical = solution_to_ode (y' = y, y(0) = 1)

sine_canonical : C∞(ℝ,ℝ)  
sine_canonical = solution_to_ode (y'' = -y, y(0) = 0, y'(0) = 1)
```

### Canonicity for Path Types

Path types are not base types, so they admit no taxonomy of canonical forms.
The correct canonicity statement for paths is the cubical one: a closed term of
`Path A x y` is a function out of the interval, and canonicity applies only after
instantiating the dimension — for each closed `r : I`, the endpoint `p r` is a
closed term of `A` and (when `A` is a base type) reduces to a canonical form of `A`.

## 8.3 Decidability {#decidability}

### Type Checking Decidability

The fundamental question: can we always decide if a term has a given type?

#### Conditional Decidability

Decidability of type checking and definitional equality is **conditional**, not a
theorem for full SCTT. For the cubical fragment, decidability follows *assuming the
normalization conjecture* (Conjecture 8.4): normalize both sides and compare normal
forms. For the smooth layer, definitional equality is decidable **only for the
fragments listed below** (polynomial and rational-function identities); equality of
general smooth functions is undecidable by Richardson's theorem.

```sctt
-- Conditional: decidable for the cubical fragment, assuming normalization (Conjecture 8.4)
conditional type_checking_decidable :
  Normalization → Decidable (λ(Γ : Context)(t : Term)(A : Type) → Γ ⊢ t : A)

-- Conditional: cubical fragment as above; smooth layer only for §8.3 fragments
conditional def_eq_decidable :
  Normalization → Decidable (λ(Γ : Context)(t₁ t₂ : Term)(A : Type) → Γ ⊢ t₁ ≡ t₂ : A)

-- Implementation as algorithms
type_check : Context → Term → Type → Bool + Error
def_eq_check : Context → Term → Term → Type → Bool + Error
```

#### Algorithm Structure

```sctt
-- Bidirectional type checking algorithm
type_check Γ t A = 
  case (t, A) of
    -- Inference cases
    (Var x, A) → lookup x Γ ==? A
    (App f a, B) → 
      type_check Γ f (A → B) &&
      type_check Γ a A
      
    -- Checking cases  
    (Lam x body, A → B) →
      type_check (Γ, x : A) body B
      
    -- Conversion
    (t, A) → 
      infer_type Γ t >>= λ B →
      def_eq_check Γ A B Type

-- Type inference
infer_type : Context → Term → Maybe Type
infer_type Γ t = 
  case t of
    Var x → lookup x Γ
    App f a → 
      infer_type Γ f >>= λ(A → B) →
      type_check Γ a A >>
      return B
    -- ... other cases
```

### Smooth Type Checking

Additional challenges for smooth operations:

```sctt
-- Smooth function type checking
smooth_type_check : Context → SmoothTerm → SmoothType → Bool
smooth_type_check Γ t A = 
  syntactic_check Γ t A &&
  smoothness_check Γ t &&
  continuity_order_check Γ t A

-- Derivative type checking
derivative_type_check : Context → DerivativeTerm → Type → Bool  
derivative_type_check Γ (D[f]) (C∞(M,N)) =
  type_check Γ f (C∞(M,N)) &&
  smoothness_degree f >= 1

-- Integration type checking
integral_type_check : Context → IntegralTerm → Type → Bool
integral_type_check Γ (∫ f) ℝ =
  type_check Γ f (C∞(Domain, ℝ)) &&
  integrability_check f
```

### Decidability Limitations

Some properties remain undecidable:

```sctt
-- Undecidable problems in SCTT
undecidable_problems : List UndecidableProblem
undecidable_problems = [
  {
    problem = "Smooth function equality",
    reason = "Richardson's theorem: impossible in general",
    workaround = "Syntactic equality + approximation"
  },
  
  {
    problem = "Convergence of infinite series", 
    reason = "Π⁰₂-complete — strictly harder than the halting problem (which is Σ⁰₁)",
    workaround = "Require explicit convergence proofs"
  },
  
  {
    problem = "Zero testing for expressions",
    reason = "Schanuel's conjecture independence", 
    workaround = "Symbolic simplification + numerical bounds"
  },
  
  {
    problem = "Definite integral exact values",
    reason = "Liouville's theorem limitations",
    workaround = "Symbolic integration + verified numerics"
  }
]

-- Decidable fragments
decidable_fragments : List DecidableFragment
decidable_fragments = [
  "Polynomial arithmetic",
  "Rational function operations", 
  "Linear differential equations",
  "Finite-dimensional linear algebra",
  "Algebraic number computations"
]
```

### Practical Decidability

```sctt
-- Decidability with timeouts and approximation
practical_type_checking : Config → Context → Term → Type → Result
practical_type_checking config Γ t A = 
  timeout config.max_time $
  approximate config.precision $
  type_check Γ t A

-- Result types
data Result = 
  DefinitelyYes |
  DefinitelyNo |  
  ProbablyYes Confidence |
  ProbablyNo Confidence |
  Timeout |
  UnknownError

-- Configuration options
data Config = Config {
  max_time : Duration,
  precision : Precision,
  approximation_method : ApproxMethod,
  symbolic_simplification : Bool,
  numerical_verification : Bool
}
```

## 8.4 Normalization {#normalization}

### The Strong Normalization Conjecture

We conjecture that all well-typed SCTT computations terminate:

```sctt
-- Strong normalization for SCTT (CONJECTURE)
conjecture strong_normalization :
  ∀(t : WellTypedTerm), StronglyNormalizing t

-- Known: MLTT is strongly normalizing
-- Known: Cubical type theories terminate for well-typed terms (cctt demonstrates this empirically)
-- OPEN: Termination with locally-scoped rewrite rules (ε² = 0)
-- OPEN: Interaction of smooth computations with cubical Kan operations
--
-- Note: Earlier chapters stated this as a theorem. We correct this here:
-- it is a well-supported conjecture with strong partial evidence, not a theorem.

-- Definition of strong normalization
StronglyNormalizing : Term → Type
StronglyNormalizing t = ¬∃(infinite_sequence : ℕ → Term),
  infinite_sequence 0 = t ∧
  ∀(n : ℕ), infinite_sequence n → infinite_sequence (n+1)

-- Equivalently: all reduction sequences terminate
conjecture termination :
  ∀(t : WellTypedTerm)(reduction_sequence : ReductionSequence t),
  ∃(n : ℕ), IsNormalForm (reduction_sequence n)
```

> **Honest Assessment**: Strong normalization is proven for MLTT and holds empirically for
> cubical implementations (e.g., `cctt`, `cubicaltt`). The RTT framework (Cockx et al., 2021)
> shows that rewrite rules satisfying certain conditions preserve normalization, and the Rewster
> tool (Leray et al., ITP 2024) can mechanically check these conditions. However, no one has
> yet verified SCTT's specific rewrite rules (particularly ε² = 0) through this pipeline.
> The conjecture is well-motivated but unproven for the full system.

#### Proof Sketch by Logical Relations

The following sketch shows the standard structure a strong normalization proof would follow:

```sctt
-- Strongly normalizing logical relations
SN_logical_relations : Type → Type₁
SN_logical_relations A = {t : Term | ⊢ t : A ∧ StronglyNormalizing t}

-- Closure properties
SN_closure_app : 
  ∀(f a : Term), SN_logical_relations (A → B) f → 
                SN_logical_relations A a →
                SN_logical_relations B (f a)

SN_closure_lam :
  ∀(body : Term), (∀(a : SN_logical_relations A), SN_logical_relations B (body[a/x])) →
                  SN_logical_relations (A → B) (λ x → body)

-- Main conjecture would follow from a fundamental lemma for SN
proof_sketch strong_normalization t typing = 
  fundamental_lemma_SN t typing
```

### Smooth Termination

Smooth operations require special termination analysis:

```sctt
-- Derivative computation always terminates
derivative_termination :
  ∀(f : C∞(ℝ,ℝ)), StronglyNormalizing (compute_derivative f)

-- Integration may not terminate, but we can detect this
integration_termination :
  ∀(f : C∞([a,b], ℝ)), 
  Terminates (compute_integral f) ∨ 
  DetectableNonTermination (compute_integral f)

-- Smooth function composition terminates
smooth_composition_termination :
  ∀(f : C∞(M,N))(g : C∞(N,P)),
  StronglyNormalizing (compute_composition g f)
```

#### Termination Measures

```sctt
-- Termination measure for smooth computations
SmoothTerminationMeasure : SmoothComputation → Ordinal
SmoothTerminationMeasure comp = 
  case comp of
    polynomial_computation → FiniteOrdinal (degree polynomial)
    derivative_computation f → SmoothTerminationMeasure f + 1  
    composition_computation f g → 
      SmoothTerminationMeasure f + SmoothTerminationMeasure g
    integration_computation f domain →
      if IsElementary f then FiniteOrdinal 1
      else RequiresInfiniteAnalysis

-- Termination proof obligation
termination_proof : (comp : SmoothComputation) → 
  Accessible (SmoothTerminationMeasure comp)
```

### Cubical Termination

Cubical operations also terminate:

```sctt
-- Composition operations terminate
composition_termination :
  ∀(A : I → Type)(φ : Formula)(u : PartialPath A φ)(u0 : A i0),
  StronglyNormalizing (comp A φ u u0)

-- Transport operations terminate  
transport_termination :
  ∀(A : I → Type)(a : A i0),
  StronglyNormalizing (transport A a)

-- Glue/unglue operations terminate
glue_termination :
  ∀(glue_data : GlueData), 
  StronglyNormalizing (glue glue_data) ∧
  StronglyNormalizing (unglue glue_data)
```

## 8.5 Computational Complexity {#complexity}

### Complexity of Type Checking

```sctt
-- Type checking complexity bounds
type_checking_complexity : ComplexityBound
type_checking_complexity = NonElementary  -- worst case: no elementary bound exists for MLTT-family conversion (Statman 1979); EXPTIME-hard

-- But most practical fragments are much better
practical_fragments : List (Fragment, ComplexityBound)
practical_fragments = [
  (SimpleTypes, POLYNOMIAL),
  (LinearArithmetic, POLYNOMIAL),  
  (PolynomialFunctions, POLYNOMIAL),
  (LinearDifferentialEquations, POLYNOMIAL),
  (FiniteDimensionalLinearAlgebra, POLYNOMIAL)
]

-- Worst-case complexity comes from deep nesting
worst_case_example : Term
worst_case_example = 
  deeply_nested_dependent_types_with_complex_computation
```

#### Space Complexity

```sctt
-- Space usage analysis
space_complexity : TypeCheckingProblem → SpaceBound
space_complexity prob = 
  if simple_types prob then POLYNOMIAL_SPACE
  else if dependent_types prob then EXPONENTIAL_SPACE  
  else if smooth_operations prob then requires_real_arithmetic_space
  else UNKNOWN

-- Memory management strategies
memory_strategies : List MemoryStrategy
memory_strategies = [
  "Lazy evaluation of normal forms",
  "Garbage collection of proof objects",
  "Sharing of common subterms", 
  "Approximation for large smooth computations"
]
```

### Smooth Computation Complexity

```sctt
-- Complexity of smooth operations
smooth_operation_complexity : SmoothOperation → ComplexityClass
smooth_operation_complexity op = 
  case op of
    polynomial_evaluation → POLYNOMIAL
    derivative_computation → LINEAR_in_expression_size
    elementary_integration → EXPONENTIAL_in_expression_size
    general_integration → UNDECIDABLE
    smooth_composition → POLYNOMIAL_in_composed_functions

-- Numerical precision affects complexity
precision_complexity_tradeoff : Precision → ComplexityBound
precision_complexity_tradeoff p = 
  POLYNOMIAL (bits_of_precision p)
```

### Optimization Strategies

```sctt
-- Performance optimization techniques
optimization_techniques : List OptimizationTechnique
optimization_techniques = [
  {
    name = "Normalization by Evaluation",
    benefit = "Avoids explicit normal form construction",
    applicable_to = "Pure type theory fragment"
  },
  
  {
    name = "Computational Reflection",
    benefit = "Verified computation outside the type system",
    applicable_to = "Numerical computations"
  },
  
  {
    name = "Proof Irrelevance",
    benefit = "Erases proof terms during computation",
    applicable_to = "Propositions and equality proofs"  
  },
  
  {
    name = "Symbolic Preprocessing",
    benefit = "Simplifies expressions before type checking",
    applicable_to = "Smooth function expressions"
  }
]

-- Implementation architecture for performance
performant_architecture : ArchitectureSpec
performant_architecture = {
  kernel = MinimalTrustedCore,
  tactics = UnverifiedButFastComputations,
  reflection = ReflectionMechanism,
  caching = IntelligentCaching,
  parallelization = ParallelProofChecking
}
```

## Summary

SCTT's metatheory combines established results for individual subsystems with open conjectures for the full combined system:

### Status of Core Properties

| Property | Subsystem Status | Full SCTT Status |
|---|---|---|
| **Consistency** (§8.1) | Proven for MLTT, CCHM, Cartesian cubical | **Conjecture** |
| **Canonicity** (§8.2) | Proven for Cartesian cubical (Sterling & Angiuli, 2021) | **Conjecture** |
| **Decidability** (§8.3) | Standard for dependent TT; fragments characterized | Conditional (fragments only) |
| **Normalization** (§8.4) | Proven for MLTT; empirically holds for cubical | **Conjecture** |
| **Complexity** (§8.5) | Bounds known for fragments | Partially characterized |

The key open problem is the **interaction** of Cartesian cubical structure, nilsquare infinitesimals (ε² = 0 as rewrite rules), and Lipschitz sensitivity types. Each component has strong metatheoretic backing; the gap is in the combined system.

### What Is Proven vs. Conjectured

**Proven** (by cited work):
- Canonicity for Cartesian cubical type theory via Synthetic Tait Computability (Sterling & Angiuli, LICS 2021)
- Conditions under which rewrite rules preserve type-theoretic properties (Cockx et al., POPL 2021)
- Metric preservation for sensitivity types (Azevedo de Amorim et al., POPL 2017)
- Explicit normal form specifications for Cartesian cubical TT (Huang, arXiv:2603.24923, 2026)

**Conjectured** (well-motivated but open):
- Consistency, canonicity, and normalization for SCTT with all three extensions simultaneously
- That SCTT's specific rewrite rules satisfy the RTT framework's safety conditions

### Implementation Implications

The metatheory directly informs implementation even at the conjecture stage:
- Type checker algorithms follow the decidability results (§8.3)
- Normal form computations follow the canonical form specifications
- Error handling for undecidable fragments (§8.3) is well-characterized
- Optimization strategies (§8.5) are independent of the open conjectures

The proof sketches in this chapter show the structure that a full metatheoretic development would take, and remain valuable as implementation guides.

## Exercises

### Theoretical
1. Identify the specific conditions from the RTT framework (Cockx et al., 2021) that SCTT's ε² = 0 rule must satisfy, and attempt to verify them
2. Extend the Sterling-Angiuli STC proof technique to handle one additional SCTT feature (e.g., nilsquare infinitesimals)
3. Characterize the exact complexity of smooth function equality
4. Develop termination measures for general recursive smooth functions
5. *(Open problem)* Prove or disprove canonicity for cubical type theory extended with the rewrite rule ε² = 0

### Computational  
1. Implement a verified type checker that provably terminates for the decidable fragments (§8.3)
2. Build a canonical form computation with complexity bounds
3. Create decidability procedures for key SCTT fragments
4. Develop optimization strategies for smooth computations

### Applications
1. Apply metatheory to verify a scientific computing library
2. Use termination analysis to prove safety of control systems
3. Implement educational tools that explain why proofs terminate
4. Build performance analysis tools for SCTT programs

---

*Next: [Chapter 9: Type Checking Algorithm](./chapter_09.md) →*

*Previous: [Chapter 7: SCTT Formal Rules](./chapter_07.md) ←*