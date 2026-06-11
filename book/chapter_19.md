# Chapter 19: Connecting to the Ecosystem

> "No type theory is an island." — Adapted from John Donne

## Introduction

SCTT does not exist in a vacuum. The proof assistant ecosystem includes mature tools with large libraries, active communities, and decades of engineering. A new type theory that ignores this infrastructure condemns itself to re-deriving elementary results from scratch while the rest of the field moves on.

This chapter shows how to connect SCTT to existing tools, port results between systems, and leverage infrastructure that already works. The goal is not to replace these tools but to build bridges — importing battle-tested definitions, exporting novel constructions, and using established systems as verification oracles.

The key insight is that SCTT's three layers — dependent types, cubical structure, and smooth/sensitivity structure — decompose naturally along tool boundaries:

- **Dependent types** translate directly to any proof assistant
- **Cubical structure** has a reference implementation in Cubical Agda and can be encoded via rewrite rules in Dedukti/Lambdapi
- **Smooth/sensitivity structure** is genuinely novel and requires encoding strategies specific to each target

We proceed tool by tool, being concrete about what works and what doesn't.

## 19.1 Cubical Agda: The Reference Implementation {#cubical-agda}

### What Cubical Agda Provides

Cubical Agda (Vezzosi, Mörtberg, and Abel, ICFP 2019) is the most mature implementation of cubical type theory. It provides:

- A full implementation of CCHM cubical type theory (De Morgan variant)
- The interval type `I`, path types, `hcomp`, `transp`, and `Glue`
- Pattern matching on interval variables
- The [1Lab](https://1lab.dev/): a comprehensive formalization library with ~100K+ LOC covering category theory, algebra, and HoTT foundations
- An active community of researchers formalizing univalent mathematics

### Using Cubical Agda as a Test Oracle

Before implementing a cubical construction in your SCTT kernel, prototype it in Cubical Agda. This catches specification errors before they become implementation bugs.

```agda
-- Prototype in Cubical Agda first
module SCTT.Prototype where

open import Cubical.Foundations.Everything

-- Test: does your path composition behave correctly?
pathComp : {A : Type} {x y z : A} → x ≡ y → y ≡ z → x ≡ z
pathComp p q i = hcomp (λ j → λ where
  (i = i0) → p i0
  (i = i1) → q j)
  (p i)

-- Test: does transport preserve structure?
transportRefl : {A : Type} (x : A) → transport refl x ≡ x
transportRefl {A} x i = transp (λ _ → A) i x

-- Test: funext works as expected?
funExt-test : {A B : Type} {f g : A → B}
  → ((x : A) → f x ≡ g x)
  → f ≡ g
funExt-test h i x = h x i
```

Once you verify a construction in Cubical Agda, translate it to SCTT. The mathematical content ports; the proof terms require re-working because the interval algebras differ.

### CCHM vs ABCFHL: The Translation

Cubical Agda uses CCHM (Cohen, Coquand, Huber, and Mörtberg, TYPES 2015) with a De Morgan algebra on the interval. SCTT targets ABCFHL (Angiuli, Brunerie, Coquand, Favonia, Harper, and Licata, MSCS 2021) with a Cartesian interval. The key differences:

| CCHM (Cubical Agda) | ABCFHL (SCTT) | Translation Strategy |
|---|---|---|
| `~ i` (interval reversal) | No reversal | Reverse the boundary: swap `i0`/`i1` in the system |
| `i ∧ j` (min connection) | No connections | Reformulate using `hcom` with boundary `(i = i0) ∨ (j = i0)` |
| `i ∨ j` (max connection) | No connections | Reformulate using `hcom` with boundary `(i = i1) ∨ (j = i1)` |
| `hcomp` | `hcom` | Similar structure; ABCFHL `hcom` uses cofibrations directly |
| `transp` | `coe` | Direct correspondence: `transp (λ i → A i) φ a` becomes `coe^{r→r'}(i.A(i), a)` |
| `Glue` types | `Glue` types | Same concept; different interval algebra means different boundary conditions |
| `hfill` (derived) | `hfill` (derived) | Reconstruct from `hcom` in each system |

A concrete example — path reversal:

```agda
-- In CCHM: reversal is a primitive
symCCHM : {A : Type} {x y : A} → x ≡ y → y ≡ x
symCCHM p i = p (~ i)
```

```
-- In ABCFHL: reversal is derived via hcom
sym_ABCFHL : PathP(i. A, x, y) → PathP(i. A, y, x)
sym_ABCFHL(p) = λ i. hcom^{0→1}(
  j. A,
  [i = 0 ↦ p(j),
   i = 1 ↦ x],
  x)
```

The ABCFHL proof uses a composition box where the CCHM proof uses a simple primitive. This is the characteristic pattern: Cartesian proofs tend to be longer but require less interval-algebraic infrastructure.

### What to Port and What Not To

**Port freely:**
- Mathematical definitions (group, ring, manifold, etc.)
- Type signatures and specifications
- Proof strategies at the informal level
- Library organization and naming conventions from the 1Lab

**Do not port directly:**
- Cubical proof terms — they depend on De Morgan connections/reversal
- `hcomp` invocations — the boundary formats differ
- `transp` calls using `~ i` or `i ∧ j` in the type line
- Any proof that case-splits on the interval using `~ i`

**Port with care:**
- `Glue` type constructions — the concept is the same but the coherences differ
- Higher inductive types — the constructors port but the eliminators need adaptation
- Proofs using `hfill` — the derived operation differs between CCHM and ABCFHL

## 19.2 Lean 4: The Practical Bridge {#lean}

### Why Lean Matters for SCTT

Lean 4 (de Moura et al., CADE 2015, with the Lean 4 rewrite) offers:

- **Mathlib**: ~100K+ lemmas covering algebra, analysis, topology, and number theory
- **Strong automation**: `simp`, `omega`, `aesop`, `norm_num`, `ring`
- **Metaprogramming**: a full-featured tactic framework in Lean itself
- **Active community**: the fastest-growing proof assistant user base

Lean does not have cubical types or smooth types natively. Its identity type is propositional (axiom K holds). This means you cannot express path composition or univalence as computational content in Lean the way you can in SCTT. But Lean is excellent for **formalizing SCTT's metatheory**.

### Formalizing SCTT Syntax and Typing in Lean 4

Encode SCTT as a deep embedding — an inductive type representing terms, with a typing judgment defined as an inductive family:

```lean
-- SCTT term syntax as a Lean 4 inductive type
inductive SCTTTerm where
  | var : Nat → SCTTTerm
  | universe : Nat → SCTTTerm
  | pi : SCTTTerm → SCTTTerm → SCTTTerm
  | sigma : SCTTTerm → SCTTTerm → SCTTTerm
  | lam : SCTTTerm → SCTTTerm
  | app : SCTTTerm → SCTTTerm → SCTTTerm
  | pair : SCTTTerm → SCTTTerm → SCTTTerm
  | fst : SCTTTerm → SCTTTerm
  | snd : SCTTTerm → SCTTTerm
  -- Cubical
  | ivar : Nat → SCTTTerm
  | pathP : SCTTTerm → SCTTTerm → SCTTTerm → SCTTTerm
  | pabs : SCTTTerm → SCTTTerm
  | papp : SCTTTerm → SCTTTerm → SCTTTerm
  | coe : SCTTTerm → SCTTTerm → SCTTTerm → SCTTTerm → SCTTTerm
  | hcom : SCTTTerm → SCTTTerm → SCTTTerm → List (SCTTTerm × SCTTTerm) → SCTTTerm → SCTTTerm
  -- Smooth
  | smooth : SCTTTerm → SCTTTerm
  | tangent : SCTTTerm → SCTTTerm
  | deriv : SCTTTerm → SCTTTerm
  | lip : SCTTTerm → SCTTTerm → SCTTTerm
  deriving Repr, BEq

-- Typing context
abbrev Context := List SCTTTerm

-- Typing judgment as an inductive family
inductive HasType : Context → SCTTTerm → SCTTTerm → Prop where
  | var_rule : (h : Γ.get? n = some A) → HasType Γ (.var n) A
  | pi_form  : HasType Γ A (.universe k) →
               HasType (A :: Γ) B (.universe k) →
               HasType Γ (.pi A B) (.universe k)
  | lam_intro : HasType (A :: Γ) t B →
                HasType Γ (.lam t) (.pi A B)
  -- ... remaining rules

-- The key metatheorem to prove
theorem type_preservation :
    ∀ (Γ : Context) (t t' A : SCTTTerm),
    HasType Γ t A → Reduces t t' → HasType Γ t' A := by
  sorry -- The actual proof goes here
```

### Encoding Lipschitz Structure via Type Classes

Lean's type class system can express sensitivity annotations:

```lean
-- Lipschitz structure as a type class
class LipschitzFun (α β : Type*) [MetricSpace α] [MetricSpace β] where
  fn : α → β
  bound : ℝ≥0
  lip_condition : ∀ x y, dist (fn x) (fn y) ≤ bound * dist x y

-- Composition preserves Lipschitz bounds (multiplicatively)
instance lipComp [MetricSpace α] [MetricSpace β] [MetricSpace γ]
    (f : LipschitzFun β γ) (g : LipschitzFun α β) :
    LipschitzFun α γ where
  fn := f.fn ∘ g.fn
  bound := f.bound * g.bound
  lip_condition := by
    intro x y
    calc dist (f.fn (g.fn x)) (f.fn (g.fn y))
        ≤ f.bound * dist (g.fn x) (g.fn y) := f.lip_condition _ _
      _ ≤ f.bound * (g.bound * dist x y)    := by ring_nf; exact mul_le_mul_left' (g.lip_condition x y) _
      _ = (f.bound * g.bound) * dist x y     := by ring
```

This lets you use Lean/Mathlib's existing metric space library to validate SCTT's Lipschitz typing rules against the standard mathematical definition.

### What Lean Can and Cannot Do for SCTT

**Can do:**
- Formalize the SCTT metatheory (syntax, typing rules, reduction)
- Prove subject reduction, progress, decidability of type checking
- Encode Lipschitz structure via type classes and Mathlib's metric spaces
- Implement and verify the SCTT type-checking algorithm as a Lean program
- Use Mathlib's automation (`simp`, `omega`, `ring`) to discharge proof obligations

**Cannot do:**
- Express cubical path types with computational content
- Compute with `hcom` and `coe` — they'd be axioms, not computable operations
- Express univalence as a provable theorem (only as an axiom)
- Natively represent smooth types with ε² = 0

## 19.3 Rocq (Coq) and the Rewster {#rocq}

### The Rewster: Mechanized Rewrite Rule Verification

The Rewster (Leray, Gilbert, Tabareau, and Winterhalter, ITP 2024) is the tool most directly relevant to SCTT's equational theory. Built on MetaRocq (formerly MetaCoq), it provides **mechanized verification** of two critical properties for rewrite rules:

1. **Type preservation**: if `Γ ⊢ t : A` and `t →_R t'`, then `Γ ⊢ t' : A`
2. **The triangle property**: a sufficient condition for confluence of the rewrite system

This matters because SCTT's smooth layer introduces rewrite rules — most notably the nilsquare axiom ε² = 0 — and these rules must not break the type system.

### Using the Rewster for SCTT Rewrite Rules

The workflow:

```
Step 1:  Encode SCTT's rewrite rules as Rocq rewrite declarations
         using the locally-scoped rewrite rule syntax
         (Leray and Winterhalter, POPL 2026).

Step 2:  Run the Rewster's type-preservation checker.
         This verifies that each rule sends well-typed terms
         to well-typed terms.

Step 3:  Run the confluence checker (triangle property).
         This verifies that rewriting is confluent,
         so normalization order doesn't affect the result.

Step 4:  If both pass, the rewrite rules are metatheoretically safe.
         The Rewster produces a Rocq proof term as a certificate.

Step 5:  (Optional) Extract the verified checker to OCaml or Rust
         via Rocq's extraction mechanism.
```

The rules SCTT would submit to the Rewster include:

- **Nilsquare**: `ε * ε → 0` (and its extensions to higher nilpotence)
- **Kock-Lawvere**: `∀ f : D → R, ∃! (a, b), f(ε) = a + b · ε`
- **Smooth function rules**: β/η for the smooth function type
- **Lipschitz composition**: `Lip[k₁] ∘ Lip[k₂] → Lip[k₁ · k₂]`
- **Cubical computation rules**: β for `coe`, `hcom` boundary conditions

### MetaRocq as a Verification Backend

MetaRocq (Sozeau, Boulier, Forster, Tabareau, and Winterhalter, JACM 2025) formalizes the Polymorphic Cumulative Calculus of Inductive Constructions (PCUIC) — Rocq's actual kernel calculus. Within MetaRocq, you can:

- Define SCTT's typing rules as an inductive relation
- State and prove subject reduction for SCTT
- State and prove decidability of SCTT type checking
- Extract a **verified type checker** — a Rocq program that is proved correct and can be compiled to OCaml

This is the path to a formally verified SCTT implementation. The verified checker would be slower than a hand-written Rust kernel, but it would be **trusted by construction** — any term it accepts is provably well-typed.

### The Locally-Scoped Rewrite Rules Approach

The POPL 2026 distinguished paper by Leray and Winterhalter introduces locally-scoped rewrite rules that can be enabled/disabled in different parts of a development. For SCTT, this is significant: the smooth rewrite rules (ε² = 0) should only fire in smooth contexts, not globally. Locally-scoped rules give exactly this control:

```
-- Pseudocode for locally-scoped nilsquare
section smooth_context
  rewrite rule nilsquare : ε * ε ↦ 0
  
  -- In this section, ε² = 0 holds
  -- Derivatives compute by the Kock-Lawvere axiom
  
end smooth_context

-- Outside the section, ε is an ordinary variable
```

## 19.4 Dedukti and Lambdapi: The Logical Framework {#dedukti}

### What Dedukti/Lambdapi Provides

Dedukti is a logical framework based on the λΠ-calculus modulo rewriting. Lambdapi is its modern interactive front-end. Together they provide:

- A **universal encoding target**: any type theory expressible via typing rules and rewrite rules can be encoded
- Built-in support for rewrite rules with pattern matching
- Matching modulo theories (AC-normalization) in active development by the Nantes/Saclay group (Barras, Felicissimo, Winterhalter)
- A role as the **Rosetta Stone** of proof assistants: proofs can be translated between systems via Dedukti encodings

### Encoding SCTT in Lambdapi

The encoding strategy for SCTT in Lambdapi follows the standard logical framework approach. Sorts become constants, typing judgments become functions, and computation rules become rewrite rules:

```lambdapi
// Sorts
constant symbol Sort : TYPE;
constant symbol U : Sort;   // Universe of types
constant symbol I : Sort;   // The interval

// Terms
symbol Term : Sort → TYPE;

// Dependent function type
symbol pi : Term U → (Term U → Term U) → Term U;
symbol lam : Π (A : Term U) (B : Term U → Term U),
  (Term A → Term (B _)) → Term (pi A B);
symbol app : Π (A : Term U) (B : Term U → Term U),
  Term (pi A B) → Π (a : Term A), Term (B a);

// β-reduction as a rewrite rule
rule app _ _ (lam _ _ &f) &a ↪ &f &a;

// Interval endpoints
symbol i0 : Term I;
symbol i1 : Term I;

// Path type
symbol pathP : (Term I → Term U) → Term U → Term U → Term U;
symbol pabs : Π (A : Term I → Term U),
  (Π (i : Term I), Term (A i)) → Term (pathP A _ _);
symbol papp : Π (A : Term I → Term U) (a0 : Term U) (a1 : Term U),
  Term (pathP A a0 a1) → Π (i : Term I), Term (A i);

// Path β-reduction
rule papp _ _ _ (pabs _ &f) &i ↪ &f &i;

// Boundary rules
rule papp &A &a0 _ _ i0 ↪ &a0;
rule papp &A _ &a1 _ i1 ↪ &a1;

// Smooth real line
symbol R : Term U;

// Nilsquare infinitesimal
symbol eps : Term R;
symbol mul : Term R → Term R → Term R;
symbol zero : Term R;

// The nilsquare rule: ε * ε ↦ 0
rule mul eps eps ↪ zero;
```

### Encoding Smooth Functions via Rewrite Rules

The Kock-Lawvere axiom — that every function `D → R` is affine — becomes a rewrite rule on application:

```lambdapi
// The microneighborhood of zero
// D = { ε ∈ R | ε² = 0 }
symbol D : Term U;
symbol inc_D : Term D → Term R;  // inclusion D ↪ R

// Smooth function space
symbol smooth : Term U → Term U → Term U;

// Derivative via Kock-Lawvere: f(ε) = f(0) + f'(0) · ε
// The derivative is the coefficient of ε in the expansion
symbol deriv : Term (smooth R R) → Term (smooth R R);

// Key rewrite: applying a smooth function to (x + ε)
// reduces to f(x) + deriv(f)(x) · ε
// This encodes D[f](x) = (f(x + ε) - f(x)) / ε
symbol add : Term R → Term R → Term R;
rule app_smooth &f (add &x eps)
   ↪ add (app_smooth &f &x) (mul (app_smooth (deriv &f) &x) eps);
```

### The 2LTT Encoding Strategy (Barras and Maestracci, LFMTP 2020)

A direct encoding of cubical type theory in Dedukti faces a fundamental obstacle: CTT's definitional equality involves a decision procedure in a de Morgan algebra (for CCHM) or cofibration lattice (for Cartesian) that cannot be straightforwardly expressed as rewrite rules. Barras and Maestracci (2020) propose a solution via **two-layer type theory** (2LTT) as an intermediate representation.

The idea: split the encoding into two parts.

**Layer 1: 2LTT in Dedukti.** Two-layer type theories (Annenkov, Capriotti, Kraus, and Sattler 2023) are variants of MLTT where definitional equality is split into:
- *Internal equality*: the usual Martin-Löf identity type
- *External equality*: a separate judgment `a ≡_ext b` that can encode equations that are definitional in CTT but would break Dedukti's rewrite system if added as rules

The external equality is represented in Dedukti as a type `Eq_ext : A → A → TYPE` with a reflexivity witness and a cast operation. Rewrite rules can then compute along external equalities without destabilizing the rewrite system.

```lambdapi
// External equality (not the Martin-Löf identity)
symbol Eq_ext [A : Type] : Term A → Term A → TYPE;
symbol refl_ext [A : Type] [a : Term A] : Eq_ext a a;
symbol cast_ext [A B : Type] : Eq_ext A B → Term A → Term B;
rule cast_ext refl_ext &x ↪ &x;
```

**Layer 2: CTT in 2LTT.** The interval's de Morgan algebra operations (meet, join, reversal) and the cofibration lattice are encoded using external equalities. For instance, the endpoint rules `p(0) ≡ x` and `p(1) ≡ y` become:

```lambdapi
// Path application endpoint rules via external equality
symbol papp_0 [A a0 a1 p] : Eq_ext (papp A a0 a1 p i0) a0;
symbol papp_1 [A a0 a1 p] : Eq_ext (papp A a0 a1 p i1) a1;
```

The Dedukti rewrite system handles β-reduction and external equality transport; the cubical-specific equations are computed via the external equality layer.

**Implementation.** The encoding is available at [github.com/valent20000/CTTDedukti](https://github.com/valent20000/CTTDedukti). For SCTT, this strategy extends naturally: the nilsquare rule `ε² = 0` becomes an external equality within the smooth layer, while the cubical equations occupy a separate 2LTT layer. The two layers interact only through lifting/lowering operators.

### Why Dedukti Is the Right Long-Term Target

If SCTT can be fully and faithfully encoded in Dedukti:

1. **The Dedukti type checker verifies SCTT terms** — an independent trust anchor
2. **Existing confluence checkers** (like those being developed for Lambdapi) apply to SCTT's rewrite system
3. **SCTT proofs can be translated to/from other systems** via Dedukti's role as a universal interchange format
4. **The encoding itself constitutes a proof** that SCTT's rules are consistent relative to the λΠ-calculus modulo rewriting
5. **The 2LTT intermediate representation** cleanly separates equations that can be rewrite rules from those that require a decision procedure

The main open challenge is encoding `hcom` (homogeneous composition). The `hcom` operation involves a system of partial elements — a cofibration with a tube and a cap — and encoding partial elements faithfully in Dedukti's rewrite system requires care with variable binding and freshness.

## 19.5 Two-Level Type Theory and Staging {#2ltt}

### Kovács's CFTT (ICFP 2024)

Two-level type theory (2LTT) separates **compile-time** and **runtime** computation. Kovács (ICFP 2022, 2024) has shown how to use 2LTT for:

- **Closure-free functional programming**: eliminating closure allocation via staging
- **Performant type theory evaluation**: the `cctt` evaluator (Kovács, 2023) achieves state-of-the-art performance for Cartesian cubical type theory by exploiting evaluation strategies informed by 2LTT ideas

The two levels:
- **Outer (meta) level**: types and computations that are evaluated during elaboration/compilation — they disappear from the final program
- **Inner (object) level**: types and computations that remain at runtime
- **Lifting** (`⇑ A`): promotes an object-level type to the meta level
- **Splicing** (`~ t`): embeds a meta-level computation into the object level

### Staging Strategy for SCTT

The key insight is that SCTT's smooth layer — infinitesimals, the Kock-Lawvere axiom, derivative computation — can live entirely at the **meta level** and be staged away before runtime:

```
Meta level (compile time):
  - Smooth types, tangent bundles, differential forms
  - The nilsquare rule ε² = 0
  - Derivative computation via Kock-Lawvere
  - Lipschitz bound tracking

Object level (runtime):
  - Concrete numerical types (Float64, etc.)
  - Pure cubical types (paths, hcom, coe)
  - Computed derivative functions (no infinitesimals)
  - Verified Lipschitz constants (plain numbers)
```

A worked example:

```
-- Meta level: define a smooth function and take its derivative
meta let f : Smooth(ℝ, ℝ) = λ x. x * x + 3 * x + 1

meta let f' : Smooth(ℝ, ℝ) = D[f]
-- During elaboration, this computes:
-- D[λ x. x * x + 3 * x + 1]
-- = λ x. D[x * x](x) + D[3 * x](x) + D[1](x)
-- = λ x. 2 * x + 3 + 0
-- = λ x. 2 * x + 3

-- Object level: the staged residual is plain arithmetic
object let f'_compiled : ℝ → ℝ = ~ f'
-- At runtime, f'_compiled is just: λ x. 2 * x + 3
-- No infinitesimals, no smooth types, no overhead
```

### Practical Benefit: Zero-Cost Smooth Abstractions

After staging:

- The Kock-Lawvere derivative is computed at **compile time** by the type checker
- The result is a concrete numerical function at **runtime**
- No smooth infrastructure is needed at runtime — no infinitesimal arithmetic, no nilsquare rule
- The runtime kernel only needs to handle cubical operations (`hcom`, `coe`) if paths remain
- Lipschitz bounds are computed at compile time and become static assertions

The compiled output of an SCTT program could be plain Rust or C with no smooth types. All the differential computation happened during type checking. This is not speculative — Kovács's `cctt` evaluator already demonstrates that 2LTT-informed staging produces competitive performance for cubical operations.

### Integration with the `cctt` Evaluator

Kovács's `cctt` (2023) is a Cartesian cubical type theory evaluator written in Haskell that achieves evaluation speeds far exceeding earlier implementations. It uses:

- Normalization by evaluation (NbE) with closures
- Efficient representation of the interval and cofibrations
- Glued evaluation for performance (lazy and strict semantics simultaneously)

SCTT's kernel can adopt the same evaluation strategy for its cubical layer. The smooth layer would be an additional pass that stages smooth computations before feeding the result to the cubical evaluator. This decomposition keeps the performance-critical cubical evaluator simple.

## 19.6 The Translation Map {#translations}

### What Can Be Translated Where

| SCTT Feature | Cubical Agda | Lean 4 | Rocq (Coq) | Dedukti/Lambdapi |
|---|---|---|---|---|
| Dependent types (Π, Σ) | Direct | Direct | Direct | Direct |
| Inductive types | Direct | Direct | Direct | Encoding via rewrite rules |
| Path types | Translate CCHM↔ABCFHL | Axiom only (no computation) | Axiom only | Encode via rewrite rules |
| `hcom` / composition | Different primitive, same concept | Not available | Not available | Encode via rewrite rules |
| `coe` / transport | `transp` (direct correspondence) | `Eq.subst` (no computation) | `transport` (no computation) | Encode via rewrite rules |
| Univalence | Direct (different proof) | Axiom only | Axiom only | Via rewrite rules |
| HITs | Direct (different eliminator) | Quotient types only | SProp / quotients | Via rewrite rules |
| Smooth types | Not available | Encode via type classes | Not available | Via rewrite rules |
| ε² = 0 (nilsquare) | Not available | Not available | Via Rewster rewrite rules | Via rewrite rules (native) |
| Kock-Lawvere derivative | Not available | Encode via type class instances | Not available | Via rewrite rules |
| Lipschitz types | Not available | Encode via structure + Mathlib | Not available | Via rewrite rules |
| Sensitivity tracking | Not available | Encode via type classes | Not available | Via rewrite rules |

### Reading the Table

The table reveals a clear pattern:

- **Cubical Agda** is the best target for cubical content but cannot handle the smooth layer at all
- **Lean 4** can encode everything via type classes and axioms, but loses computational content for cubical and smooth features — proofs type-check but don't compute
- **Rocq** has a special role via the Rewster: it can verify the metatheory of SCTT's rewrite rules mechanically
- **Dedukti/Lambdapi** is the universal target: everything can be encoded via rewrite rules, but the encoding must be verified for correctness

### The Ideal Workflow

```
1. Prototype mathematical content in Cubical Agda
   - Fastest iteration cycle for cubical constructions
   - Test against the 1Lab's existing formalizations
   - Validate hcom/coe behavior before SCTT implementation

2. Formalize metatheory in Lean 4
   - Best automation for inductive proofs about syntax
   - Mathlib provides metric space library for Lipschitz verification
   - Strong community for getting review on formalizations

3. Verify rewrite rules via Rocq and the Rewster
   - Mechanized proof of type preservation for each rewrite rule
   - Mechanized proof of confluence (triangle property)
   - Produces proof certificates (not just yes/no)

4. Encode the full theory in Dedukti
   - Universal interchange format
   - Independent verification of SCTT terms
   - Enables translation to/from other systems

5. Implement the production kernel in Rust
   - Performance: NbE with efficient closures (à la cctt)
   - Safety: Rust's type system prevents memory bugs in the kernel
   - Interop: FFI for calling from Python/Julia for ML applications

6. Stage smooth computation via 2LTT
   - Smooth types resolved at elaboration time
   - Runtime has no infinitesimal overhead
   - Cubical operations remain for path computations
```

No single tool covers all of SCTT. The ecosystem strategy is to use each tool where it is strongest.

## 19.7 The Fuzz/DFuzz Connection {#fuzz}

### Sensitivity Typing in the Wild

SCTT's Lipschitz types are closely related to the Fuzz family of sensitivity type systems:

- **Fuzz** (Reed and Pierce, ICFP 2010): linear types indexed by sensitivity, for differential privacy
- **DFuzz** (Gaboardi, Haeberlen, Hsu, Narayan, and Pierce, POPL 2013): dependent sensitivity types
- **GSoul** (Bañados Schwerter, Garcia, and Tanter, CSF 2025): gradual sensitivity typing

The core idea is the same: if `f : A →_k B`, then `f` is `k`-sensitive (or `k`-Lipschitz), meaning it amplifies distances by at most `k`.

### What SCTT Adds to the Fuzz Picture

Fuzz tracks sensitivity for **differential privacy**. SCTT tracks Lipschitz constants for **mathematical correctness and robustness**. The differences:

| Feature | Fuzz/DFuzz | SCTT Lip Types |
|---|---|---|
| Primary application | Differential privacy | Certified robustness, AD correctness |
| Metric structure | Discrete or standard metrics | Riemannian metrics, arbitrary metric spaces |
| Composition | Multiplication of sensitivities | Same: `Lip[k₁] ∘ Lip[k₂] : Lip[k₁ · k₂]` |
| Integration with paths | None | Lipschitz functions act on paths |
| Interaction with smooth types | None | `D[f]` has Lipschitz type when `f` is `C²` |
| Higher-dimensional | Scalars only | Could track Jacobian-level sensitivity |

The Fuzz line of work has developed sophisticated techniques for **type inference** of sensitivity annotations. SCTT should import these — there is no reason to reinvent the inference algorithms when Fuzz's approach is well-understood and published.

## 19.8 Contributing to the Ecosystem {#contributing}

### For SCTT Developers

The SCTT project can contribute back to the ecosystem:

- **Cartesian cubical algorithms**: share efficient `hcom`/`coe` implementations with the Cubical Agda team — the Cartesian case sometimes admits simpler algorithms than the De Morgan case
- **Lipschitz typing ideas**: collaborate with the Fuzz/DFuzz community on shared sensitivity-typing infrastructure
- **Rewrite rule specifications**: work with the Rewster team on SCTT-specific rewrite rules and provide test cases for their confluence checker
- **Smooth-type encodings for Dedukti**: help the Dedukti team develop standard encodings for smooth and sensitivity structure, which could benefit any future type theory with these features
- **Automated boundary filling**: the techniques of Doré, Cavallo, and Mörtberg (FSCD 2024) for automating `hcom` boundary construction apply to SCTT; implementations should be shared

### For Proof Assistant Users

If you already work in Cubical Agda, Lean, Rocq, or another system:

- Test SCTT concepts in your preferred tool using the encoding strategies from this chapter
- Report where existing tools fall short for smooth or sensitivity types — this drives tool development
- Propose extensions to your tool's core that would support SCTT features natively (e.g., rewrite rules in Lean, smooth types in Agda)

### For ML Researchers

SCTT's Lipschitz types have direct implications for machine learning:

- Implement Lipschitz-typed neural network layers (spectral normalization, Parseval networks)
- Test certified robustness claims empirically — does SCTT's `Lip[k]` bound match observed perturbation sensitivity?
- Compare SCTT-style automatic differentiation (via Kock-Lawvere) with existing AD frameworks (JAX, PyTorch autograd)
- Provide benchmark datasets for certified ML, where the ground truth Lipschitz constants are known

## Exercises

### Tool Integration

1. **Cubical Agda warm-up.** Install Cubical Agda and formalize `refl : Path ℕ 0 0`. Then formalize `ap : (f : A → B) → Path A x y → Path B (f x) (f y)`. Compare the proof terms with your SCTT kernel's representation of the same constructions.

2. **Lean 4 syntax encoding.** Encode SCTT's term syntax as a Lean 4 inductive type (see §19.2). Add constructors for `hcom` and `coe`. Implement a function `whnf : SCTTTerm → SCTTTerm` in Lean that performs weak head normalization for the β-rules (ignore cubical computation for now). Test it on `app (lam (var 0)) (universe 0)`.

3. **Dedukti encoding.** Write a Lambdapi file that encodes Pi types, lambda, and application with the β rewrite rule. Verify that Lambdapi's type checker accepts the encoding and that `(λ x. x) 42` reduces to `42`.

4. **Rewster integration.** Encode the nilsquare rule `ε * ε ↦ 0` as a Rocq rewrite rule. State what the Rewster would need to verify for type preservation. (You don't need a working Rewster installation — describe the verification obligation.)

### Translation Exercises

5. **CCHM to ABCFHL.** In Cubical Agda, the proof of `sym : x ≡ y → y ≡ x` uses `~ i`. Write the ABCFHL-style proof using only `hcom` (no reversal). Verify both proofs compute the same result on a concrete path.

6. **Nilsquare in Lambdapi.** Encode the smooth real line with `eps`, `mul`, and the rewrite rule `mul eps eps ↪ zero` in Lambdapi. Define `square : R → R` as `λ x. mul x x`. Verify that `square eps` reduces to `zero`.

7. **Lipschitz in Lean 4.** Using Lean 4's `Mathlib.Topology.MetricSpace.Lipschitz`, formalize the statement that the composition of a `K₁`-Lipschitz function and a `K₂`-Lipschitz function is `(K₁ * K₂)`-Lipschitz. (Hint: `LipschitzWith` is already in Mathlib.)

### Staging Exercises

8. **Manual staging.** Take the smooth function `f(x) = x³ - 2x + 1`. Compute `D[f]` by hand using the Kock-Lawvere axiom (expand `f(x + ε)`, collect the coefficient of `ε`). What is the "staged residual" — the plain arithmetic function that remains after smooth types are erased?

9. **Two-level design.** Design a 2LTT encoding where `D[f]` computes at the meta level and the object-level residual is a plain function `ℝ → ℝ`. Apply it to `D[λ x. sin(x)]` — what meta-level computation must the elaborator perform, and what is the object-level result?

10. **Staging benchmark.** Implement a simple evaluator (in any language) that takes a polynomial given as a list of coefficients, computes its symbolic derivative (staged), and evaluates the result on a concrete input. Compare the performance with computing the derivative numerically via finite differences at each evaluation point.

---

*Previous: [Chapter 18: The Equational Theory Frontier](./chapter_18.md) ←*

*Next: [Appendices](./appendix_a.md) →*
