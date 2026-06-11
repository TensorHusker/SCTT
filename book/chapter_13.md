# Chapter 13: Modal SCTT

> "In logic, a modality is a way of qualifying the truth of a judgment. But in mathematics, modalities reveal the deep structure of mathematical reasoning itself." — Michael Shulman
>
> "Modal SCTT extends this vision: modalities in smooth cubical type theory capture not just logical necessity, but geometric and computational structure."

## Introduction

Modal type theory adds operators that qualify how terms and types behave under different "modes" of reasoning. In SCTT, modalities become even more powerful—they can capture geometric properties (continuity, smoothness from [Chapter 4](./chapter_04.md)), computational properties (termination, complexity from [Chapter 6](./chapter_06.md)), and foundational distinctions (constructive vs. classical, discrete vs. continuous from [Chapter 2](./chapter_02.md)).

This chapter explores the rich modal structure that emerges when we combine:
1. **Geometric modalities**: Discrete, continuous, smooth, and cohesive structure
2. **Computational modalities**: Termination, complexity bounds, and resource usage  
3. **Logical modalities**: Necessity, possibility, and temporal reasoning
4. **Physical modalities**: Classical vs. quantum, deterministic vs. probabilistic

The result is a framework where mathematical properties can be precisely tracked through the type system, enabling new forms of verified computation. This extends the programming techniques from [Chapter 10](./chapter_10.md) and complements the applications in [Chapter 11](./chapter_11.md) and [Chapter 12](./chapter_12.md).

### Why Modalities in SCTT?

Traditional modal logic studies operators like □ ("necessarily") and ◇ ("possibly"). In SCTT, we need modalities for several reasons:

```sctt
-- Different kinds of mathematical objects require different treatment
discrete : Type → Type          -- Discrete/combinatorial objects
continuous : Type → Type        -- Topological objects  
smooth : Type → Type           -- Differential geometric objects
quantum : Type → Type          -- Quantum mechanical objects

-- Different computational properties need tracking
terminating : Computation → Type    -- Proofs of termination
polynomial : Computation → Type     -- Complexity bounds
real_time : Computation → Type      -- Time-critical computations

-- Different foundational assumptions
classical : Proposition → Type      -- Classical logic allowed
constructive : Proposition → Type   -- Constructive proofs only
computable : ℝ → Type              -- Computable reals vs. arbitrary reals
```

### The Modal Framework

We organize SCTT's modalities into several categories:

```sctt
-- Cohesive modalities (geometric) — see §13.2 for precise definitions
∫ : Type → Type    -- Shape: fundamental ∞-groupoid
♭ : Type → Type    -- Flat: discrete underlying type (constant sheaf)
♯ : Type → Type    -- Sharp: codiscrete
ℑ : Type → Type    -- Infinitesimal shape (de Rham stack)

-- Computational modalities  
⟐ : Type → Type    -- Terminating computations
⧫ : Type → Type    -- Polynomial-time computable
◇ : Type → Type    -- Eventually consistent
□ : Type → Type    -- Always/necessarily

-- Physical modalities
🅒 : Type → Type    -- Classical physics
🅠 : Type → Type    -- Quantum physics  
🆂 : Type → Type    -- Stochastic processes
🅓 : Type → Type    -- Deterministic evolution
```

## 13.1 Modalities {#modalities}

### Formal Definition of Modalities

A modality in SCTT is an operation on types with specific structural properties:

#### Basic Modal Structure

```sctt
-- A modality is a type operator with unit and bind
class Modality (◯ : Type → Type) where
  -- Unit: pure terms can be modalized
  unit : ∀ {A : Type} → A → ◯ A
  
  -- Bind: modal computations compose
  bind : ∀ {A B : Type} → ◯ A → (A → ◯ B) → ◯ B
  
  -- Laws
  left_unit : ∀ {A B} (x : A) (f : A → ◯ B) → 
              bind (unit x) f ≡ f x
  right_unit : ∀ {A} (mx : ◯ A) → 
               bind mx unit ≡ mx  
  associativity : ∀ {A B C} (mx : ◯ A) (f : A → ◯ B) (g : B → ◯ C) →
                  bind (bind mx f) g ≡ bind mx (λ x → bind (f x) g)
```

#### Modal Type Theory Rules

```sctt
-- Formation rule
Γ ⊢ A : Type
――――――――――――――  (◯-form)
Γ ⊢ ◯ A : Type

-- Introduction rule
Γ ⊢ a : A
――――――――――――――  (◯-intro)  
Γ ⊢ unit a : ◯ A

-- Elimination rule
Γ ⊢ m : ◯ A    Γ, x : A ⊢ n : ◯ B
――――――――――――――――――――――――――――――  (◯-elim)
Γ ⊢ bind m (λx. n) : ◯ B

-- Modal access (when applicable)
Γ ⊢ m : ◯ A
――――――――――――――  (◯-access, restricted)
Γ ⊢ extract m : A
```

### Examples of SCTT Modalities

#### The Smoothing Operation (NOT a modality)

```sctt
-- Smoothing: approximate a discrete type by a smooth one.
-- WARNING: this is a useful *operation*, but it is NOT a modality:
-- it is not idempotent and has no universal property. (The symbol ♯
-- is reserved for the codiscrete/sharp modality — see §13.2.)
Smoothen : Type → SmoothType
Smoothen A = SmoothApproximation A

-- Unit: embed discrete into smooth
unit_smooth : A → Smoothen A
unit_smooth x = constant_smooth_function x

-- Bind: smooth functions compose smoothly
bind_smooth : Smoothen A → (A → Smoothen B) → Smoothen B  
bind_smooth smooth_a f = 
  smooth_composition smooth_a (smooth_extension f)

-- Example: smooth approximation of integers
smooth_integers : Smoothen ℤ
smooth_integers = SmoothApprox {
  approximation = λ x → ⌊x + 0.5⌋,  -- Round to nearest integer
  smoothing = tanh_approximation,      -- Smooth rounding
  error_bound = 0.5,
  consistency_proof = rounding_is_closest_integer
}
```

#### The Terminating Modality (⟐)

```sctt
-- Terminating computations
⟐ : Type → Type
⟐ A = {computation : Computation A | terminates computation}

-- All terminating computations have a result  
extract_terminating : ⟐ A → A
extract_terminating (comp, proof) = run_to_completion comp proof

-- Composition preserves termination
compose_terminating : ⟐ A → (A → ⟐ B) → ⟐ B
compose_terminating (comp_a, term_a) f = 
  let result_a = extract_terminating (comp_a, term_a) in
  f result_a  -- f produces terminating computation

-- Example: verified quicksort
quicksort : List A → ⟐ (List A)
quicksort [] = unit []
quicksort (x::xs) = 
  bind (quicksort smaller) λ smaller_sorted →
  bind (quicksort larger) λ larger_sorted →
  unit (smaller_sorted ++ [x] ++ larger_sorted)
  where
    smaller = filter (< x) xs
    larger = filter (≥ x) xs
    -- Termination proof by structural recursion on length
```

#### The Classical Modality (🅒)

```sctt
-- Classical reasoning: allows law of excluded middle, but ONLY for
-- crisp (♭-discrete) or codiscrete (♯) propositions. Unrestricted LEM
-- is INCONSISTENT with the Kock-Lawvere axiom: classically, 𝔻 = {0},
-- which collapses the smooth structure (Kock; Shulman's real-cohesion).
🅒 : Type → Type
🅒 A = ClassicalType A  -- Allows non-constructive proofs

-- LEM is available only under ♯ / for crisp hypotheses
classical_lem : ∀ (P : Prop) → ♯ (P ∨ ¬P)
classical_lem P = sharp_classical_axiom
-- NOTE: there is NO extract : 🅒 A → A. The ◯-access rule of §13.1
-- does not apply to 🅒 — eliminating it into the smooth fragment
-- would let LEM contaminate ℝ and refute Kock-Lawvere.
-- 🅒 must also never commute with the smooth/cohesive modalities.

-- Classical real numbers with Dedekind cuts
classical_reals : 🅒 Type
classical_reals = 🅒 ℝ_classical where
  ℝ_classical = DedekindCuts ℚ
  -- Every real has a decimal expansion (non-constructively)

-- Classical analysis: intermediate value theorem (under the modality)
intermediate_value : 
  ∀ (f : 🅒 (ContinuousFunction [0,1] ℝ)),
  f(0) < 0 → f(1) > 0 → 🅒 (∃ c ∈ [0,1], f(c) = 0)
intermediate_value f neg pos = 
  classical_proof_by_bisection f neg pos
  -- Uses classical logic to guarantee existence; the witness
  -- stays inside 🅒 and cannot be extracted into smooth code
```

### Modal Interactions

Modalities can compose and interact:

```sctt
-- Some modalities commute
commute_smooth_terminating : 
  Smoothen (⟐ A) ≃ ⟐ (Smoothen A)
commute_smooth_terminating = 
  -- Smooth approximation of terminating computation
  -- is the same as terminating smooth computation
  equivalence_proof

-- Others don't commute  
no_commute_classical_constructive :
  🅒 (ConstructiveProof P) ≠ ConstructiveProof (🅒 P)
-- Classical approach to constructive proof ≠ constructive classical proof

-- Modal necessity: some combinations are impossible
impossible_quantum_classical : ¬(🅠 A × 🅒 A) 
-- Cannot simultaneously be quantum and fully classical

-- Modal hierarchies
refinement_hierarchy : 
  ♭ A → Smoothen A → C∞ A → Cω A
-- Discrete → smooth → C∞ → analytic
```

## 13.2 Cohesive Structure {#cohesion}

### The Cohesive Topos Structure

SCTT naturally forms a cohesive topos, with modalities relating different levels of geometric structure:

#### The Cohesive Modalities

```sctt
-- The cohesive adjoint string (standard notation, cf. §4.6)
∫ : SmoothType → Type          -- Shape: fundamental ∞-groupoid
♭ : Type → SmoothType          -- Flat: discrete type as smooth (constant sheaf)
♯ : Type → SmoothType          -- Sharp: codiscrete type as smooth
Γ : SmoothType → Type          -- Underlying points (not itself a modality)

-- Adjunction relationships: shape ⊣ flat ⊣ sharp
shape_flat_adjunction : ∫ ⊣ ♭
flat_sharp_adjunction : ♭ ⊣ ♯
```

Underlying the modalities is an adjoint quadruple Π ⊣ Disc ⊣ Γ ⊣ Codisc
between smooth types and discrete types: Π takes the fundamental
∞-groupoid, Disc equips a type with the discrete smooth structure, Γ takes
underlying points, and Codisc equips a type with the codiscrete structure.
The modalities are the composites ∫ = Disc ∘ Π, ♭ = Disc ∘ Γ, ♯ = Codisc ∘ Γ.
Note that π₀ is just the 0-truncation of the shape: π₀ M = ‖∫ M‖₀.

For differential cohesion, the infinitesimal structure forms its own
adjoint triple (see §13.2 below):

```sctt
-- Differential cohesion: reduction ⊣ infinitesimal shape ⊣ infinitesimal flat
infinitesimal_triple : ℜ ⊣ ℑ ⊣ &
```

#### Discrete Objects

```sctt
-- Underlying points functor
Γ : SmoothType → Type
Γ M = UnderlyingSet M  -- Forgets smooth structure

-- Examples
Γ ℝ ≃ ℝ_discrete              -- Real numbers as discrete set
Γ S¹ ≃ S¹_discrete             -- Circle as discrete space
Γ C∞(ℝ,ℝ) ≃ Set_of_functions  -- Functions as discrete set

-- Properties of discrete objects
is_discrete : (M : SmoothType) → Type  
is_discrete M = (M ≃ ♭(Γ M))  -- M is isomorphic to its discretization

-- Discrete objects admit no non-constant smooth paths.
-- (Note: this does NOT give decidable equality — it says only that
-- every smooth path into a discrete type is constant.)
discrete_paths : ∀ (M : SmoothType) (d : is_discrete M)
                 (p : C∞(I, M)) →
                 ∀ (t : I) → p t ≡ p 0
discrete_paths M d p t = 
  smooth_path_into_discrete_is_constant d p t
```

#### Shape and Connected Components

```sctt
-- Shape: the fundamental ∞-groupoid; π₀ is its 0-truncation
∫ : SmoothType → Type
∫ M = FundamentalInfinityGroupoid M
π₀ M = ‖∫ M‖₀  -- Set of connected components

-- Shape preserves finite colimits
∫_preserves_colimits : 
  ∀ (diagram : Diagram SmoothType),
  ∫ (colimit diagram) ≃ colimit (∫ ∘ diagram)

-- Examples (at the level of π₀; ∫ retains higher homotopy too,
-- e.g. ∫ S¹ is the homotopy circle with π₁ = ℤ)
π₀ ℝ ≃ Unit                    -- ℝ is connected
π₀ (ℝ - {0}) ≃ Bool           -- Two components: ℝ₊ and ℝ₋  
π₀ S¹ ≃ Unit                   -- Circle is connected
π₀ (S¹ ⊔ S¹) ≃ Bool           -- Two circles have two components

-- Shape respects smooth homotopy equivalence
shape_homotopy_invariant :
  ∀ (M N : SmoothType) (f : C∞(M,N)),
  SmoothHomotopyEquivalence f → (∫ M ≃ ∫ N)
```

#### Flat Objects

```sctt
-- Flat: embed discrete into smooth world
♭ : Type → SmoothType
♭ A = ConstantSmoothSheaf A

-- Flat objects are "locally constant"
flat_property : ∀ (A : Type) (x : ♭ A) (neighborhood : OpenSet ♭ A),
               x ∈ neighborhood → 
               ∃ (constant_value : A), 
                 ∀ y ∈ neighborhood, y ≡ constant_value

-- Examples
♭ ℕ = ConstantSheaf ℕ         -- Natural numbers as constant sheaf
♭ Bool = {smooth functions ℝ → Bool that are locally constant}

-- Flat preserves all colimits
♭_preserves_all_colimits :
  ∀ (diagram : Diagram Type),
  ♭ (colimit diagram) ≃ colimit (♭ ∘ diagram)
```

#### Infinitesimal Objects

```sctt
-- Differential cohesion: ℜ ⊣ ℑ ⊣ &
-- ℜ M = reduced type (infinitesimal directions removed)
-- ℑ M = de Rham stack (infinitesimally-nearby points identified)
-- & M = infinitesimal flat
ℑ : SmoothType → SmoothType  
ℑ M = DeRhamStack M  -- M with infinitesimal neighborhoods collapsed

-- The infinitesimal disk 𝔻 is NOT ℑ ℝ — it is what ℑ collapses:
-- 𝔻 arises as the fiber of the unit map ℝ → ℑ ℝ over 0
𝔻 ≃ fiber (unit_ℑ : ℝ → ℑ ℝ) 0

-- Tangent vectors are maps FROM the infinitesimal disk INTO M
tangent_via_disk : ∀ (M : SmoothType) (x : M),
                   TangentSpace M x ≃ Σ (t : 𝔻 → M), t 0 ≡ x

-- The whole tangent bundle is the exponential by 𝔻
tangent_bundle_exponential : TangentBundle M ≃ (𝔻 → M)

-- Microlinearity via infinitesimals
microlinear : ∀ (f : C∞(ℝ,ℝ)) (x : ℝ) (ε : 𝔻),
             f(x + ε) = f(x) + f'(x) * ε
```

### Cohesive Types and Smooth Spaces

The interaction of these modalities gives us rich geometric structure:

```sctt
-- Cohesive type: object with the cohesive structure made explicit
CohesiveType : Type₁
CohesiveType = {
  carrier : SmoothType,
  discrete : Type,
  shape : Type,  
  flat_embedding : Type → SmoothType,
  infinitesimal_disk : SmoothType,
  
  -- Cohesive axioms
  discrete_points : discrete ≃ Γ carrier,
  shape_groupoid : shape ≃ ∫ carrier,  
  flat_adjunction : ∫ ⊣ ♭,
  tangent_exponential : TangentBundle carrier ≃ (infinitesimal_disk → carrier)
}

-- Examples of cohesive types
ℝ_cohesive : CohesiveType
ℝ_cohesive = CohesiveType {
  carrier = ℝ,
  discrete = ℝ_discrete,       -- Real numbers as discrete set
  shape = Unit,                -- ℝ is contractible: ∫ ℝ ≃ Unit
  flat_embedding = ♭,          -- Constant sheaf embedding
  infinitesimal_disk = 𝔻      -- Dual numbers ℝ[ε]/(ε²)
}

manifold_cohesive : (M : Manifold) → CohesiveType  
manifold_cohesive M = CohesiveType {
  carrier = M,
  discrete = UnderlyingSet M,
  shape = ∫ M,            -- Fundamental ∞-groupoid (homotopy type of M)
  flat_embedding = ♭,
  infinitesimal_disk = 𝔻_dim(M)
}
```

### Applications of Cohesive Structure

#### Differential Cohomology

```sctt
-- Differential characters (forms + topology)
DifferentialCharacter : (M : Manifold) → (n : ℕ) → Type
DifferentialCharacter M n = 
  fiber (curv_class : Ω^n_closed(M) → H^n(∫ M, ℝ/ℤ)) 
  -- Closed n-forms whose periods are integral

-- Chern-Weil theory
chern_weil : (E : VectorBundle M) → 
             DifferentialCharacter M (2k)
chern_weil E = 
  differential_form_part = chern_form (connection E)
  topological_part = chern_class E
  consistency = chern_weil_homomorphism

-- Example: Magnetic monopoles  
magnetic_monopole : S² → DifferentialCharacter S² 2
magnetic_monopole = 
  -- 2-form part: B = μ₀ g/(4π r²) r̂ * (solid angle form)
  -- Topological part: Depends on monopole charge g
  construct_monopole_field
```

#### Synthetic Differential Geometry

```sctt
-- Kock-Lawvere axiom using cohesion
kock_lawvere_cohesive : 
  ∀ (M : CohesiveType) (f : C∞(M.carrier, ℝ)),
  ∀ (x : M.carrier) (ε : ℑ M.carrier),
  ∃! (a b : ℝ), f(x + ε) = a + b * ε

-- This gives us automatic differentiation
automatic_derivative : C∞(ℝ, ℝ) → C∞(ℝ, ℝ)
automatic_derivative f x = 
  the_unique_b_from (kock_lawvere_cohesive ℝ_cohesive f x)

-- Integration via cohesive structure
cohesive_integration : 
  ∀ (M : CohesiveType) (ω : Ω^n M.carrier),
  (n = dimension M.carrier) →
  ∫_M ω ∈ ℝ / π₀(∂M)  -- Integration mod boundary components
```

## 13.3 Differential Cohomology {#cohomology}

### Differential Cohomology Theory

Differential cohomology unifies differential forms and topological cohomology:

#### The Differential Cohomology Hexagon

```sctt
-- Differential cohomology is the HOMOTOPY pullback (Hopkins–Singer)
-- of the integral and de Rham approaches to cohomology:
--
--                Ĥⁿ(M;ℤ)
--               ↙        ↘
--      Ωⁿ_cl,ℤ(M)        Hⁿ(M,ℤ)
--               ↘        ↙
--               Hⁿ(M,ℝ)
--
-- (arrows OUT of Ĥⁿ: curvature to the left, characteristic class
--  to the right; both legs agree in real cohomology)
H_diff : (M : Manifold) → (n : ℕ) → Type
H_diff M n = HomotopyPullback
  (characteristic : Hⁿ(M,ℤ) → Hⁿ(M,ℝ))
  (de_rham_class  : Ωⁿ_closed(M) → Hⁿ(M,ℝ))
-- NOTE: the naive set-level pullback is WRONG — it loses the flat part
-- Hⁿ⁻¹(M;ℝ/ℤ) (e.g. all holonomy data of flat connections).
```

The structure of Ĥⁿ is captured by two exact sequences:

```sctt
-- Curvature exact sequence
0 → Hⁿ⁻¹(M;ℝ/ℤ) → Ĥⁿ(M;ℤ) → Ωⁿ_cl,ℤ(M) → 0

-- Characteristic class exact sequence
0 → Ωⁿ⁻¹(M)/Ωⁿ⁻¹_cl,ℤ(M) → Ĥⁿ(M;ℤ) → Hⁿ(M;ℤ) → 0
```

#### Construction in SCTT

```sctt
-- Differential cohomology classes (local data presentation)
H_diff : (M : Manifold) → (n : ℕ) → Type
H_diff M n = {
  connection : LocalForms Ω^(n-1)(M),   -- Local connection (n-1)-forms
  curvature : Ω^n_closed(M),            -- Curvature (global closed n-form)  
  characteristic_class : H^n(M, ℤ),     -- Integral cohomology class
  
  -- Consistency: locally F = dA, glued by Čech data;
  -- globally [F] = characteristic_class in Hⁿ(M;ℝ)
  consistency : LocallyExact connection curvature ∧
                [curvature] ≡ image characteristic_class
}

-- Examples (note degrees: geometric objects live one degree HIGHER
-- than their connection forms)
-- n=2: Line bundles with connection (curvature F ∈ Ω², c₁ ∈ H²)
line_bundle_diff_cohomology : Manifold → Type
line_bundle_diff_cohomology M = H_diff M 2 ≃
  {(∇ : Connection, F : Ω²_closed(M), c₁ : H²(M,ℤ)) | locally F = dA, [F] = c₁}

-- n=3: Gerbes with connective structure
gerbe_diff_cohomology : Manifold → Type
gerbe_diff_cohomology M = H_diff M 3 ≃
  {gerbes with connection | curvature 3-form condition}

-- n=4: String structures / Chern–Simons data
string_diff_cohomology : Manifold → Type  
string_diff_cohomology M = H_diff M 4
```

### Chern-Weil Theory in SCTT

```sctt
-- Vector bundle with connection
VectorBundleConnection : (M : Manifold) → (n : ℕ) → Type
VectorBundleConnection M n = {
  bundle : VectorBundle M n,
  connection : Connection bundle,
  curvature : Ω²(M, EndBundle bundle)
}

-- Chern forms
chern_forms : VectorBundleConnection M n → 
              (k : ℕ) → Ω^(2k)(M)
chern_forms (E, ∇, F) k = 
  (i/2π)^k * (1/k!) * trace(F^k)
  -- F^k = F ∧ F ∧ ... ∧ F (k times)

-- Chern character
chern_character : VectorBundleConnection M n → 
                  ⊕_k Ω^(2k)(M)
chern_character (E, ∇, F) = 
  trace(exp(iF/2π))
  -- Exponential of curvature matrix

-- Atiyah-Singer index theorem setup
index_theorem_data : {
  manifold : CompactManifold M,
  vector_bundle : VectorBundleConnection M n,
  elliptic_operator : EllipticOperator bundle,
  
  -- The index is computed by differential cohomology
  analytical_index : ℤ,  -- dim ker - dim coker
  topological_index : ∫_M todd(M) ∧ chern_character(bundle)
}
```

### Hopkins-Singer Construction

The modern foundation of differential cohomology:

```sctt
-- Hopkins-Singer differential cohomology
hopkins_singer : (M : Manifold) → (n : ℕ) → Type
hopkins_singer M n = 
  ExtensionProblem {
    base : Singular_Cohomology M n ℝ,
    extension : Singular_Cohomology M n ℤ,
    obstruction : de_Rham_Cohomology M n,
    
    extension_condition : 
      ∀ (integral_class : H^n(M,ℤ)) (de_rham_class : H^n_dR(M)),
      compatible_via_chern_weil integral_class de_rham_class
  }

-- Universal property
hopkins_singer_universal : 
  ∀ (cohomology_theory : GeneralizedCohomology),
  (has_differential_refinement cohomology_theory) →
  ∃! (map : hopkins_singer → cohomology_theory),
  natural_transformation_preserving_structure map

-- Smooth Deligne cohomology
deligne_cohomology : (M : Manifold) → (n : ℕ) → Type
deligne_cohomology M n = 
  Hypercohomology M (deligne_complex n)
  where
    deligne_complex n = 
      [ℤ → Ω⁰ → Ω¹ → ... → Ω^(n-1)]
      -- Complex of sheaves with differentials
```

### Applications to Physics

#### Gauge Theory

```sctt
-- U(1) gauge theory
u1_gauge_theory : (M : Manifold⁴) → Type
u1_gauge_theory M = {
  gauge_potential : Ω¹(M),  -- A_μ dx^μ
  field_strength : Ω²(M),   -- F = dA
  action : ℝ,               -- S = ∫ (1/4) F ∧ *F
  
  differential_character : H¹_diff(M),  -- Combines A and topology
  quantization : H²(M,ℤ),               -- Dirac quantization condition
}

-- Yang-Mills theory  
yang_mills : (M : Manifold⁴) → (G : LieGroup) → Type
yang_mills M G = {
  connection : Connection (PrincipalBundle M G),
  curvature : Ω²(M, AdBundle),
  action : ℝ,  -- YM action
  
  instanton_number : H⁴(M,ℤ),  -- Topological charge
  differential_pontryagin : H⁴_diff(M)  -- Secondary characteristic class
}
```

#### Anomaly Cancellation

```sctt
-- Quantum field theory anomalies
quantum_anomaly : (theory : QuantumFieldTheory) → Type  
quantum_anomaly theory = {
  classical_symmetry : LieGroupAction theory.fields,
  quantum_breaking : DifferentialForm theory.spacetime (dim gauge_group),
  anomaly_polynomial : CharacteristicClass theory.bundle,
  
  -- Anomaly is exact in one higher dimension
  anomaly_descent : ∃ (P : Ω^(n+1)_closed), dP = anomaly_polynomial
}

-- Green-Schwarz mechanism
green_schwarz_cancellation : 
  (supergravity : SupergravityTheory) →
  (string_theory : StringTheory) →
  anomaly supergravity + anomaly string_theory ≡ 0

-- Example: Type II string theory
type_ii_anomaly_cancellation : 
  anomaly_polynomial_gravity + anomaly_polynomial_gauge = 0
  -- Via differential cohomology computation
```

### Computational Aspects

#### Computing Differential Cohomology

```sctt
-- Algorithm for computing H^n_diff(M)
compute_differential_cohomology : 
  (M : Manifold) → (n : ℕ) → 
  ComputationWith H_diff M n
compute_differential_cohomology M n = do
  -- Step 1: Compute ordinary cohomology
  ordinary_cohom ← compute_cohomology M n ℤ
  
  -- Step 2: Compute de Rham cohomology  
  de_rham_cohom ← compute_de_rham_cohomology M n
  
  -- Step 3: Solve extension problem
  extensions ← solve_extension_problem ordinary_cohom de_rham_cohom
  
  -- Step 4: Package as differential cochains
  return (package_as_differential_cohomology extensions)

-- Spectral sequence computation
serre_spectral_sequence : 
  (fibration : Fibration E B F) → 
  (n : ℕ) → 
  SpectralSequence converging_to H_diff E n
serre_spectral_sequence fib n = 
  E²_page = H*(B, H_diff F *)  -- Cohomology with local coefficients
  differential_structure = induced_by fib.connection
```

### Examples and Calculations

#### Differential Cohomology of Common Spaces

```sctt
-- Computed from the two exact sequences (§13.3 above).
-- General facts: Ĥ⁰(M) ≅ H⁰(M;ℤ); Ĥ¹(M) ≅ C∞(M, ℝ/ℤ);
-- in top degree+1, Ĥ^{d+1}(M) ≅ H^d(M;ℝ/ℤ); zero above that.

-- Circle S¹
s1_differential_cohomology : (n : ℕ) → Type
s1_differential_cohomology 0 = ℤ                    -- Ĥ⁰(S¹) ≅ ℤ
s1_differential_cohomology 1 = C∞(S¹, ℝ/ℤ)         -- smooth ℝ/ℤ-valued functions
s1_differential_cohomology 2 = ℝ/ℤ                  -- flat U(1) connections (holonomy)
s1_differential_cohomology (n+3) = 0                 -- Zero for n ≥ 3

-- Sphere S²  
s2_differential_cohomology : (n : ℕ) → Type
s2_differential_cohomology 0 = ℤ                    -- Ĥ⁰(S²) ≅ ℤ  
s2_differential_cohomology 1 = C∞(S², ℝ/ℤ)         -- smooth ℝ/ℤ-valued functions
s2_differential_cohomology 2 = Extension ℤ (Ω¹(S²)/Ω¹_cl,ℤ(S²))
  -- U(1) bundles WITH connection: infinite-dimensional extension
  -- 0 → Ω¹/Ω¹_cl,ℤ → Ĥ²(S²) → H²(S²;ℤ) = ℤ → 0
s2_differential_cohomology 3 = ℝ/ℤ                  -- Ĥ³(S²) ≅ H²(S²;ℝ/ℤ) ≅ ℝ/ℤ
s2_differential_cohomology (n+4) = 0                 -- Zero for n ≥ 4

-- Torus T² = S¹ × S¹
torus_differential_cohomology : (n : ℕ) → Type  
torus_differential_cohomology 0 = ℤ
torus_differential_cohomology 1 = C∞(T², ℝ/ℤ)
torus_differential_cohomology 2 = Extension ℤ_with_flat_part
  -- 0 → H¹(T²;ℝ/ℤ) ≅ (ℝ/ℤ)² ⊕ … → part of Ĥ²; full group is the
  -- extension 0 → Ω¹/Ω¹_cl,ℤ → Ĥ²(T²) → H²(T²;ℤ) = ℤ → 0
torus_differential_cohomology 3 = ℝ/ℤ               -- Ĥ³(T²) ≅ H²(T²;ℝ/ℤ)
torus_differential_cohomology (n+4) = 0
```

#### Physical Examples

```sctt
-- Magnetic monopole on S²
magnetic_monopole_s2 : H²_diff S²  
magnetic_monopole_s2 = DifferentialCharacter {
  connection = vector_potential,  -- A (with Dirac string)
  curvature = magnetic_field,     -- B = dA (smooth away from string)
  characteristic_class = monopole_charge ∈ H²(S², ℤ),
  
  -- Consistency: ∫_S² B = 2π × (monopole charge)
  consistency_check = dirac_quantization_condition
}

-- Chern-Simons theory on 3-manifold
chern_simons_3d : (M : Manifold³) → H³_diff M
chern_simons_3d M = {
  gauge_field : Connection (PrincipalBundle M G),
  cs_3_form : Ω³(M),  -- CS(A) = tr(A ∧ dA + 2/3 A ∧ A ∧ A)
  level : ℤ,          -- Quantized coupling constant
  
  action = (k/4π) ∫_M cs_3_form,
  partition_function = ∑_flat_connections exp(i × action)
}
```

## Summary

Modal SCTT provides a rich framework for organizing and computing with different kinds of mathematical and physical structure:

### Key Achievements

1. **Unified Framework**: Geometric, logical, and computational modalities in one system
2. **Cohesive Structure**: Systematic relationship between discrete, continuous, and smooth
3. **Differential Cohomology**: Computational approach to modern differential geometry
4. **Physical Applications**: Natural language for gauge theory and anomaly cancellation

### Modal Hierarchy

```sctt
-- The modal structure of mathematics
Mathematics = {
  discrete : ♭,          -- Combinatorics, number theory
  continuous : ∫,        -- Topology, analysis (shape)
  smooth : smooth,       -- Differential geometry  
  codiscrete : ♯,       -- Classical/non-constructive layer
  infinitesimal : ℑ,    -- Synthetic differential geometry
  
  logical : □/◇,        -- Necessity and possibility
  computational : ⟐,     -- Termination and complexity
  physical : 🅒/🅠,      -- Classical and quantum
}
```

### Applications Unlocked

- **Verified Differential Geometry**: Automatic differentiation with rigorous error bounds
- **Computational Topology**: Algorithms for computing cohomology and characteristic classes  
- **Quantum Field Theory**: Rigorous treatment of anomalies and topological effects
- **Control Theory**: Modal logic for system properties and guarantees

Modal SCTT demonstrates how adding the right abstractions can dramatically expand the expressive power and computational capabilities of a mathematical foundation.

## Exercises

### Basic Modalities
1. Define a modality for "eventually consistent" distributed computations
2. Implement the "almost everywhere" modality for measure theory
3. Create a "finite support" modality for functions
4. Design a "real-time" modality with temporal constraints

### Cohesive Structure  
1. Compute the cohesive modalities for the Klein bottle
2. Show that the cohesive structure on manifolds gives synthetic differential geometry
3. Implement the Kock-Lawvere axiom using infinitesimal modality
4. Prove that ∫ (shape) preserves finite colimits

### Differential Cohomology
1. Compute H²_diff(ℂP²) for the complex projective plane
2. Implement the Chern-Weil homomorphism computationally
3. Verify anomaly cancellation in a specific gauge theory
4. Compute the differential cohomology of a Lie group

### Advanced Projects
1. Implement spectral sequence computations for differential cohomology
2. Create a library for computing characteristic classes of vector bundles
3. Build a verified implementation of Chern-Simons theory
4. Develop modal logic for quantum error correction

---

*Next: [Chapter 14: Higher Categories](./chapter_14.md) →*

*Previous: [Chapter 12: Physics and Engineering](./chapter_12.md) ←*
