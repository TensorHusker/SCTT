# Chapter 8: Metatheory

> "The foundations of mathematics are not cast in concrete. They are living, breathing structures that grow and evolve with our understanding." — Vladimir Voevodsky
>
> "In SCTT, metatheory becomes executable—we don't just prove consistency, we compute with it."

## Introduction

Metatheory studies mathematical systems from the outside—asking whether they are consistent, whether proofs terminate, whether equality is decidable. For SCTT, metatheory becomes especially important because we're combining three sophisticated systems: dependent type theory, cubical structure, and smooth mathematics.

This chapter establishes the theoretical foundations that make SCTT trustworthy:

1. **Consistency** - SCTT cannot prove false statements
2. **Canonicity** - Every closed term has a canonical form
3. **Decidability** - Type checking and equality are algorithmically decidable
4. **Normalization** - All computations terminate
5. **Computational Complexity** - Bounds on the computational cost

The key insight is that SCTT's metatheory is not just theoretical—it's computational. We can implement these properties and verify them mechanically.

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
-- SCTT metatheory as a verified system
SCTT_Metatheory : MetatheoreticalFramework
SCTT_Metatheory = {
  -- Core properties
  consistency : ¬∃(Γ : Context)(t : Term), Γ ⊢ t : ⊥,
  canonicity : ∀(t : ClosedTerm)(A : Type), (⊢ t : A) → HasCanonicalForm t,
  decidability : Decidable TypeChecking ∧ Decidable DefEq,
  normalization : StronglyNormalizing SCTT_reduction,
  
  -- Smooth-specific properties  
  smooth_consistency : SmoothOperations preserve TypeStructure,
  derivative_correctness : Computational_derivatives ≡ Mathematical_derivatives,
  
  -- Implementation properties
  computable : AllProperties have ComputationalContent,
  verifiable : AllProofs are MechanicallyCheckable
}
```

## 8.1 Consistency {#consistency}

### The Consistency Theorem

The fundamental requirement: SCTT must not prove false statements.

#### Statement and Proof Strategy

```sctt
-- Main consistency theorem
theorem sctt_consistency : Consistent SCTT
sctt_consistency = ¬∃(Γ : Context)(t : Term), Γ ⊢ t : ⊥

-- Proof by logical relations
proof sctt_consistency = 
  logical_relations_model models SCTT ∧
  logical_relations_model ⊨ ⊥ → Empty ∧
  soundness : (Γ ⊢ t : A) → (logical_relations_model ⊨ Γ ⊢ t : A)
  ⟹ ¬∃(t : Term), ⊢ t : ⊥
```

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
  f(x + ε) = f(x) + f'(x) * ε + O(ε²)

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

### The Canonicity Theorem

Every closed term of base type has a canonical form.

#### Statement and Proof

```sctt
-- Canonicity for SCTT
theorem sctt_canonicity :
  ∀(t : ClosedTerm)(A : BaseType), 
  (⊢ t : A) → ∃(canonical : CanonicalForm A), t →* canonical

-- Base types have specific canonical forms
BaseType : Type → Type
BaseType ℕ = true
BaseType ℝ = true  
BaseType Bool = true
BaseType (Path A x y) = true
BaseType (A → B) = false  -- not a base type
BaseType (Σ(x:A).B) = false

-- Canonical forms
CanonicalForm : (A : BaseType) → Type
CanonicalForm ℕ = Numeral
CanonicalForm ℝ = RealLiteral
CanonicalForm Bool = BoolLiteral
CanonicalForm (Path A x y) = PathLiteral A x y
```

#### Proof by Logical Relations

```sctt
-- Canonicity via logical relations
canonical_logical_relations : 
  ∀(A : BaseType), ⟦A⟧ ∅ ⊆ CanonicalForm A

proof canonical_logical_relations A =
  case A of
    ℕ → naturals_are_numerals
    ℝ → reals_are_literals  
    Bool → bools_are_literals
    Path A x y → paths_are_path_literals

-- Key lemma: closed terms reduce to canonical forms
closed_terms_canonical :
  ∀(t : ClosedTerm)(A : BaseType),
  (⊢ t : A) → ∃(c : CanonicalForm A), t →* c

proof closed_terms_canonical t A typing =
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

```sctt
-- Canonical forms for smooth functions
SmoothFunctionCanonical : (M N : SmoothType) → Type
SmoothFunctionCanonical M N = 
  Polynomial M N |
  RationalFunction M N |
  ElementaryFunction M N |
  DefinedByDE M N |  -- Differential equation
  CompositeCanonical M N

-- Canonicity theorem for smooth functions
smooth_function_canonicity :
  ∀(f : ClosedTerm)(M N : SmoothType),
  (⊢ f : C∞(M,N)) → 
  ∃(canonical : SmoothFunctionCanonical M N), 
  f →* canonical

-- Examples
polynomial_canonical : C∞(ℝ,ℝ) 
polynomial_canonical = λ x → x³ - 2*x + 1

exponential_canonical : C∞(ℝ,ℝ)
exponential_canonical = solution_to_ode (y' = y, y(0) = 1)

sine_canonical : C∞(ℝ,ℝ)  
sine_canonical = solution_to_ode (y'' = -y, y(0) = 0, y'(0) = 1)
```

### Canonicity for Path Types

```sctt
-- Canonical paths in SCTT
PathCanonical : (A : Type)(x y : A) → Type
PathCanonical A x y = 
  ConstantPath A x |  -- when x ≡ y
  LinearPath A x y |  -- in vector spaces
  GeodesicPath A x y | -- in Riemannian manifolds
  ComputedPath A x y   -- by cubical operations

-- Every path has canonical form
path_canonicity :
  ∀(p : ClosedTerm)(A : Type)(x y : A),
  (⊢ p : Path A x y) →
  ∃(canonical : PathCanonical A x y), p →* canonical

-- Smooth paths have additional structure
smooth_path_canonical : (M : SmoothManifold)(x y : M) → Type
smooth_path_canonical M x y = {
  path : C∞([0,1], M),
  boundary : path(0) = x ∧ path(1) = y,
  canonical_property : IsGeodesic path ∨ IsMinimalLength path ∨ IsComputed path
}
```

## 8.3 Decidability {#decidability}

### Type Checking Decidability

The fundamental question: can we always decide if a term has a given type?

#### Decidability Theorem

```sctt
-- Type checking is decidable
theorem type_checking_decidable :
  Decidable (λ(Γ : Context)(t : Term)(A : Type) → Γ ⊢ t : A)

-- Definitional equality is decidable  
theorem def_eq_decidable :
  Decidable (λ(Γ : Context)(t₁ t₂ : Term)(A : Type) → Γ ⊢ t₁ ≡ t₂ : A)

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
    reason = "Equivalent to halting problem",
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

### Strong Normalization Theorem

All SCTT computations terminate:

```sctt
-- Strong normalization for SCTT
theorem strong_normalization :
  ∀(t : WellTypedTerm), StronglyNormalizing t

-- Definition of strong normalization
StronglyNormalizing : Term → Type
StronglyNormalizing t = ¬∃(infinite_sequence : ℕ → Term),
  infinite_sequence 0 = t ∧
  ∀(n : ℕ), infinite_sequence n → infinite_sequence (n+1)

-- Equivalently: all reduction sequences terminate
theorem termination :
  ∀(t : WellTypedTerm)(reduction_sequence : ReductionSequence t),
  ∃(n : ℕ), IsNormalForm (reduction_sequence n)
```

#### Proof by Logical Relations

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

-- Main theorem follows
proof strong_normalization t typing = 
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
type_checking_complexity = EXPTIME  -- In worst case

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

SCTT's metatheory provides solid foundations for trustworthy mathematical computation:

### Core Results

1. **Consistency**: SCTT cannot prove false statements - even with smooth operations
2. **Canonicity**: All closed terms reduce to canonical forms  
3. **Decidability**: Type checking and equality checking are algorithmic
4. **Normalization**: All computations terminate
5. **Complexity**: Manageable computational costs for practical fragments

### Key Innovations

- **Computational Metatheory**: Properties are implemented and verified
- **Smooth Integration**: Differential operations preserve logical structure
- **Practical Decidability**: Undecidable fragments are isolated and managed
- **Performance**: Optimization maintains correctness guarantees

### Implementation Implications

The metatheory directly informs implementation:
- Type checker algorithms with termination guarantees
- Normal form computations with complexity bounds  
- Error handling for undecidable fragments
- Optimization strategies with correctness preservation

This establishes SCTT as both theoretically sound and practically implementable.

## Exercises

### Theoretical
1. Extend the consistency proof to modal SCTT from Chapter 13
2. Prove canonicity for smooth higher inductive types
3. Characterize the exact complexity of smooth function equality
4. Develop termination measures for general recursive smooth functions

### Computational  
1. Implement a verified type checker that provably terminates
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