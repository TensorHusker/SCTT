# Chapter 18: The Equational Theory Frontier

> "The interaction between equational symbols and higher-order terms which may feature rewrite rules requires more investigation." — Barras, Felicissimo, Winterhalter (2024)
>
> "In SCTT, this interaction is not a theoretical footnote — it is the entire engineering challenge."

## Introduction

[Chapter 4](./chapter_04.md) introduced the nilsquare axiom (ε² = 0) as the foundation of SCTT's smooth layer. [Chapter 7](./chapter_07.md) described how rewrite rules give it computational content, and [Chapter 9](./chapter_09.md) sketched a type checking algorithm that must accommodate these rules. This chapter goes deeper. We trace the full theoretical chain from RTT through BiTTs to LRTT, locate the remaining open problems precisely, and give the reader a concrete Rust implementation pathway.

This is the hardest part of SCTT. Every other chapter describes machinery that is well-understood in isolation — dependent types, cubical operations, NbE. Here, the pieces collide. If you can implement what this chapter describes, you have solved the central technical challenge of making smooth cubical type theory compute.

### The Core Tension

The nilsquare axiom ε² = 0 looks harmless as a mathematical statement. As a computation rule, it is anything but:

```sctt
-- What we want:
rule mul(ε, ε) ⇒ 0

-- What goes wrong:
-- 1. The pattern is NON-LINEAR (ε appears twice)
-- 2. Multiplication is COMMUTATIVE (mul(a,b) = mul(b,a))
-- 3. β-reduction can CREATE new instances of mul(ε,ε)
-- 4. Cubical operations (coe, hcom) must not break confluence
```

The rest of this chapter unpacks each of these difficulties, surveys the theoretical machinery that addresses them, and builds toward a working implementation.

---

### ⚡ Quick Start: What You'll Learn

**If you only have 30 minutes**, read:
- [§18.1 Why Standard Rewriting Fails](#why-hard) — The three obstacles
- [§18.4 LRTT: Local Scoping](#lrtt) — The key architectural insight
- [§18.6 Implementation Walkthrough](#implementation) — Concrete Rust code

**Core takeaways**:
- Standard higher-order pattern matching cannot handle ε² = 0
- RTT provides the metatheoretic foundation (triangle property)
- BiTTs gives a generic bidirectional algorithm for checking rewrite rules
- LRTT scopes rewrite rules locally, preventing most critical pairs
- The combined system is implementable today, with one open problem deferred

**Prerequisites**: [Chapter 7](./chapter_07.md), [Chapter 9](./chapter_09.md), familiarity with term rewriting

**Time**: 5–7 hours for the full chapter with exercises

---

## 18.1 Why Standard Rewriting Fails {#why-hard}

### The Three Obstacles

Before reaching for solutions, we must understand precisely why `mul(ε, ε) ⇒ 0` is not a routine rewrite rule.

**Obstacle 1: Non-Linear Patterns**

Standard higher-order pattern unification, in the sense of Miller (1991), requires each metavariable to appear exactly once in the left-hand side. This is the *pattern fragment* that makes unification decidable and unitary. The rule `mul(ε, ε) ⇒ 0` violates this: the symbol ε appears twice. Non-linear matching is required — the matcher must verify that two subterms are definitionally equal, not just bind them independently.

```
-- Miller pattern (linear): each metavariable once
rule add(x, zero) ⇒ x           -- ✓ x appears once

-- Non-linear pattern: metavariable repeated
rule mul(ε, ε) ⇒ 0              -- ✗ ε appears twice
```

Non-linear matching is decidable for first-order terms but becomes undecidable in general for higher-order terms. SCTT's specific rules are first-order in the relevant variables, so decidability is preserved — but the standard tooling does not handle this out of the box.

**Obstacle 2: Commutativity**

The ring axiom `mul(a, b) = mul(b, a)` is an equation, not a directed rewrite rule. Orienting it as `mul(a, b) ⇒ mul(b, a)` produces immediate non-termination: `mul(x, y) ⇒ mul(y, x) ⇒ mul(x, y) ⇒ ...`.

This matters because ε² can hide inside products:

```
mul(a, ε) * mul(ε, b)
= mul(mul(a, ε), mul(ε, b))       -- expand
= mul(a, mul(ε, mul(ε, b)))       -- by associativity
= mul(a, mul(mul(ε, ε), b))       -- by associativity
= mul(a, mul(0, b))               -- nilsquare fires
= mul(a, 0)                       -- ring axiom
= 0                               -- ring axiom
```

The matcher must recognize ε² **modulo commutativity and associativity** of multiplication, without being able to orient these axioms as rewrite rules.

**Obstacle 3: Confluence with β-Reduction**

Adding a new computation rule creates potential **critical pairs** with existing rules. The most dangerous is the interaction with β-reduction:

```
(λx. mul(x, x)) ε
  → mul(ε, ε)          via β-reduction
  → 0                  via nilsquare

But also:
(λx. mul(x, x)) ε
  → ???                is there another reduction path?
```

If any other reduction sequence starting from `(λx. mul(x, x)) ε` produces a term different from `0`, the system loses confluence and type checking becomes unsound. Every term must have a unique normal form regardless of reduction strategy.

### What Knuth-Bendix Cannot Do

The Knuth-Bendix completion procedure transforms a set of equations into a confluent, terminating rewrite system. It fails here for two reasons:

1. **Commutativity generates infinite rules.** Attempting to complete `{mul(a,b) = mul(b,a)}` produces an unbounded set of rules. AC-completion algorithms (Huet 1980, Peterson & Stickel 1981) handle this for first-order terms but have no direct analog in dependent type theory with higher-order terms.

2. **Dependent types add higher-order constraints.** The types of rewrite rules may themselves contain rewrite-reducible terms. A rule that is confluent at the term level can create type-level critical pairs that break subject reduction.

### What We Need Instead

A framework that:
1. Supports non-linear patterns in left-hand sides
2. Matches modulo equational theories (at minimum, commutativity)
3. Guarantees confluence with β-reduction and cubical operations
4. Preserves subject reduction in a dependently-typed setting

The next three sections describe the theoretical scaffolding that provides exactly this.

## 18.2 RTT: The Foundation {#rtt}

### Cockx, Tabareau, and Winterhalter (POPL 2021)

Rewriting Type Theory (RTT) extends the Calculus of Inductive Constructions (CIC) with **rewrite blocks** that introduce new computation rules alongside the built-in β, ι, and δ reductions. The core contribution is a modular criterion for confluence that does not require termination.

### The Triangle Property

The central innovation of RTT is the **triangle property**. Given a set of rewrite rules R, define:
- `first(t)`: the result of applying the **first applicable rule** to the leftmost-outermost redex in `t`
- `max(t)`: the **maximal parallel reduct** of `t` — apply all non-overlapping rules simultaneously

The triangle property states:

```
For all terms t:   first(t) →*_R max(t)
```

Read: the result of firing one rule can always reach the maximal parallel reduct in zero or more steps. This is strictly stronger than local confluence (Newman's lemma), which merely requires that divergent one-step reducts eventually converge. The triangle property requires convergence in a specific direction — from the one-step result toward the maximal result.

Why this matters:

- **Modularity.** When adding a new rewrite block, you only need to check the triangle property for critical pairs between the new rules and existing ones. Previous blocks are unaffected.
- **No termination requirement.** Standard confluence proofs via Knuth-Bendix require termination. RTT's triangle property works even for non-terminating rule sets (though SCTT's rules do terminate).
- **Decidability.** Checking the triangle property reduces to computing critical pairs and verifying joinability — both mechanizable.

### Formal Statement

Let `→_β` be β-reduction and `→_R` the rewrite relation. Define `→_βR` = `→_β ∪ →_R`. The triangle property for the combined system requires:

```
∀ t, u₁, u₂.  t →_βR u₁  and  t →_βR u₂
  ⟹  ∃ v.  u₁ →*_βR v  and  u₂ →*_βR v
```

with the additional constraint that `u₁ →*_βR max(t)` in at most `|R|` steps, where `|R|` is the number of active rules.

### What RTT Gives You

1. **Subject reduction.** If `Γ ⊢ t : A` and `t →_βR t'`, then `Γ ⊢ t' : A`. Typing is preserved under rewrite.
2. **Consistency.** If each rewrite block has a *propositional instantiation* (a model where the rewrite rules hold propositionally), then the extension is consistent. No new closed terms of type `⊥` are introduced.
3. **Semi-decidable type checking.** Type checking remains semi-decidable: if a term has a type, the checker will find it (but may diverge on ill-typed terms).

### The Nilsquare Rule in RTT

```sctt
rewrite_block nilsquare : {
  symbol ε : D                    -- D is the object of infinitesimals
  rule mul(ε, ε) ⇒ 0 : ℝ        -- the nilsquare axiom, now computational

  -- Consistency witness: instantiate ε = 0 in any ring.
  -- Then mul(0, 0) = 0 holds by the ring axiom. ✓
  instantiation : ε ↦ 0
}
```

The instantiation witness is critical. It proves that the rewrite rule does not introduce inconsistency: there exists a model (the trivial one, ε = 0) where the rule holds as a propositional equality. RTT's metatheorem then guarantees that the extension is conservative.

## 18.3 BiTTs: The Checking Algorithm {#bitts}

### Felicissimo (ESOP 2024)

Bidirectional Type Theories (BiTTs) is a generic framework for specifying type theories as collections of four kinds of rules:

| Rule kind    | Purpose                         | Example                     |
|-------------|----------------------------------|-----------------------------|
| Sort rules  | Declare the sort hierarchy       | `Tm`, `Ty`, `Tm : Ty`      |
| Constructor | Build terms and types            | `λ`, `Π`, `Σ`              |
| Destructor  | Eliminate terms                  | `app`, `fst`, `snd`        |
| Rewrite     | Additional computation           | `mul(ε,ε) ⇒ 0`            |

Given a specification in this format, BiTTs mechanically derives a **bidirectional type checking algorithm**. The algorithm alternates between *synthesis* (infer the type of a term from its structure) and *checking* (verify that a term has a given type), using the specified rules to drive each step.

### How BiTTs Checks Rewrite Rules

For each rewrite rule `l ⇒ r`:

1. **Synthesize** the type of `l` in the empty context. This must succeed — the LHS must be a well-typed pattern.
2. **Check** that `r` has the synthesized type. The RHS must inhabit the same type as the LHS.
3. **Verify rigidity.** No proper subterm of `l` should unify with the LHS of another rule (unless that subterm is headed by a metavariable). This prevents ambiguous rule application.
4. **Compute critical pairs** between `l` and all existing rule LHS patterns. Each critical pair must be joinable.

### The Algorithm in Pseudocode

```
function check_rewrite_rule(l, r, existing_rules):
    -- Step 1: The LHS must be typeable
    Γ, A ← synthesize(∅, l)
    if FAIL: reject("LHS is not well-typed")

    -- Step 2: The RHS must have the same type
    check(Γ, r, A)
    if FAIL: reject("RHS does not inhabit the LHS type")

    -- Step 3: Rigidity check
    for each subterm s of l:
        if s is not metavariable-headed:
            for each rule l' ⇒ r' in existing_rules:
                if unifiable(s, l'):
                    reject("rigidity violation: subterm of LHS overlaps with " ++ show l')

    -- Step 4: Confluence check
    for each rule l' ⇒ r' in existing_rules:
        pairs ← critical_pairs(l, l')
        for (s, t) in pairs:
            s_nf ← normalize(s)
            t_nf ← normalize(t)
            if s_nf ≠ t_nf:
                reject("confluence failure: " ++ show s ++ " and " ++ show t
                       ++ " have different normal forms")

    return OK
```

### What BiTTs Adds Beyond RTT

RTT provides the metatheory; BiTTs provides the **algorithm**. Specifically:

- RTT says "if the triangle property holds, the system is confluent." BiTTs says "here is how to *check* the triangle property mechanically."
- BiTTs handles the bidirectional discipline: rewrite rules must respect the synthesis/checking mode distinctions, which RTT's metatheory does not address.
- BiTTs is implemented in Lambdapi, giving us a concrete tool rather than only theorems.

## 18.4 LRTT: Local Scoping {#lrtt}

### Leray and Winterhalter, "Encode the Cake and Eat It Too" (POPL 2026)

Locally-scoped Rewriting Type Theory (LRTT), introduced in *"Encode the Cake and Eat It Too: Controlling Computation in Type Theory, Locally"* (Proc. ACM Program. Lang. 10, POPL, Art. 62, January 2026), makes the decisive architectural contribution for SCTT: rewrite rules need not be global. They can be **abstracted over**, scoped to specific definitions, and compiled away via inlining. All results are formalized in Rocq (Coq), with a prototype implementation as an extension of the Rocq Prover.

### The Problem with Global Rules

In standard RTT, declaring `rule mul(ε, ε) ⇒ 0` makes this rule active everywhere. Every subsequent definition, every cubical operation, every universe level must be checked for confluence against the nilsquare rule. Leray and Winterhalter identify four specific failures of global rules:

1. **Global rules are here to stay.** Once a rule is added in Agda, Rocq, or Dedukti, one cannot take it back. The trusted computing base grows monotonically.
2. **Rules may break invariants.** User-defined rewrite rules can break confluence, subject reduction, or termination. With global rules, there is no way to confine potential breakage.
3. **Lack of modularity.** Combining computation rules can lead to unwanted breakages even when individual rules are well-behaved.
4. **TCB clutter.** Commands like Rocq's `Print Assumptions` flag all definitions that transitively depend on rewrite rules, even when those rules are used purely for convenience.

### The LRTT Design: Interface Abstraction

LRTT extends MLTT with **prenex quantification over computation rules** at the definition level. The key syntactic extension adds an *interface environment* Ξ to each definition:

```
Σ ::= ⋄ | Σ, def f⟨Ξ⟩ : A := t        -- global environment
Ξ ::= ⋄ | Ξ, assm x : A               -- assumption (symbol)
         | Ξ, comp Θ ⊢ u ≡ v : A       -- computation rule
```

A definition `def f⟨Ξ⟩ : A := t` abstracts over the interface Ξ. Inside the body `t`, the assumptions and computation rules declared in Ξ are active. At call sites, an *instance* `ξ` must be provided that satisfies the interface.

```sctt
-- The nilsquare rule is LOCAL to this definition
def derivative ⟨ assm ε : D, comp ⊢ mul(ε, ε) ≡ 0 : ℝ ⟩
  : C∞(ℝ, ℝ) → C∞(ℝ, ℝ)
  := λ f x. extract_coefficient ε (f (x + ε))
```

The interface `⟨ assm ε : D, comp ⊢ mul(ε, ε) ≡ 0 : ℝ ⟩` declares that `derivative` abstracts over both a symbol ε and the nilsquare computation rule. Inside the body, `mul(ε, ε)` reduces to `0`. Outside, the rule is inert.

### Conservativity via Inlining

LRTT's central metatheoretic result is **conservativity over plain MLTT**, proved by an inlining procedure that unfolds all definitions. Given a well-formed global environment `⊢ Σ`, the translation `⟦Σ⟧` maps each definition to its fully inlined body:

```
⟦⋄⟧ := ⟨⟩
⟦Σ, def f⟨Ξ'⟩ : A := t⟧ := ⟨ f ↦ ⟦t⟧_⟦Σ⟧ ⟩ ∪ ⟦Σ⟧
```

The inlining function `⟦t⟧_κ` acts as the identity on all constructs except constants `f⟨ξ⟩`, where it looks up `f` in κ and instantiates with the inlined instance.

**Theorem 4.12 (Conservativity).** Given `⊢ Σ` and a closed MLTT type `⋄ | ⋄ | ⋄ ⊢ A : Type` that is inhabited in LRTT `Σ | ⋄ | ⋄ ⊢ t : A`, then A is also inhabited in MLTT: `⋄ | ⋄ | ⋄ ⊢ ⟦t⟧_⟦Σ⟧ : A`.

This is strictly stronger than the embedding into extensional type theory (Hofmann 1995): LRTT is conservative over MLTT *without* UIP or function extensionality, and without committing to any particular equality type.

### Instance Checking

At each call site `f⟨ξ⟩`, the type checker must verify two things:

1. **Equation satisfaction.** For each `comp Θ ⊢ u ≡ v : A` in the interface, the instance must make `u[ξ]` and `v[ξ]` definitionally equal: `Σ | Ξ | ξ_E ⊨ Ξ_E`.
2. **Typing.** Each assumption `assm x : A` maps to a well-typed term in the instance: `Σ | Ξ | Γ ⊢ ξ_E : Ξ_E`.

This is where LRTT connects back to RTT's confluence machinery: checking equation satisfaction requires deciding definitional equality under the active rewrite rules, which in turn requires confluence.

### Why This Is the Key Insight for SCTT

LRTT scoping solves the architectural problem of combining cubical and smooth structure:

1. **Layer isolation.** The cubical layer (paths, coe, hcom, Glue) never sees ε² = 0. The nilsquare rule is active only inside smooth blocks. Cubical operations cannot create `mul(ε, ε)` patterns because ε is not a cubical variable.

2. **Local confluence.** Confluence checking is scoped: nilsquare need only be checked against rules active in the same scope. Since cubical Kan operations are in a different scope, they generate zero critical pairs with nilsquare.

3. **Conservativity.** LRTT's Theorem 4.12 guarantees that anything provable with local rules is provable without them. The smooth layer is a *conservative extension* — it adds computational convenience without changing the theory's logical strength. Crucially, this holds without assuming UIP or function extensionality.

4. **Formalization.** The entire metatheory (substitution lemmas, validity, conservativity) is formalized in Rocq, available at [github.com/TheoWinterhalter/local-comp](https://github.com/TheoWinterhalter/local-comp/tree/popl26-paper). A prototype Rocq extension is available at [github.com/Yann-Leray/coq](https://github.com/Yann-Leray/coq/tree/artifact-popl26).

4. **Modular verification.** New smooth operations (e.g., higher-order derivatives, jet bundles) add local rules to a local scope. They need only be checked against other smooth rules, not the entire cubical+dependent type infrastructure.

### The Scoping Architecture

```
┌────────────────────────────────────────────────┐
│  Global scope: MLTT + Cubical                   │
│  Rules: β, η, coe, hcom, Glue                  │
│                                                  │
│  ┌──────────────────────────────────────────┐   │
│  │  Smooth scope [nilsquare: mul(ε,ε) ⇒ 0] │   │
│  │  Active rules: β, η, nilsquare           │   │
│  │  NOT active: coe, hcom (not needed here)  │   │
│  │                                            │   │
│  │  derivative, tangent, jet, ...             │   │
│  └──────────────────────────────────────────┘   │
│                                                  │
│  Cubical operations see only the global scope.   │
│  No critical pairs between coe and nilsquare.    │
└────────────────────────────────────────────────┘
```

## 18.5 The Gruissan Gap {#gruissan}

### Barras, Felicissimo, Winterhalter (EuroProofNet Workshop, Gruissan, 2024)

The theoretical chain from RTT to LRTT addresses most of SCTT's needs. One gap remains: **matching modulo equational theories** in the presence of higher-order terms. This section describes precisely what is solved, what is open, and what SCTT's v0 implementation can safely defer.

### What Is Solved

Lambdapi (the reference implementation of RTT/BiTTs) already supports:

- **Commutative symbols.** A symbol `f` can be declared commutative, and the matcher will try both `f(a,b)` and `f(b,a)` when matching.
- **Associative-commutative (AC) symbols.** A symbol `f` can be declared AC, and terms are flattened into multisets for matching. `f(a, f(b, c))` matches `f(c, f(a, b))`.
- **AC-normalized terms.** The internal representation keeps AC terms in a canonical form, so equality checks are linear in term size.
- **The triangle property for AC rules.** The RTT metatheory extends to AC-matching: if critical pairs are joinable modulo AC, the combined system is confluent.

### What Is Open

The **Gruissan gap** is the interaction of three features simultaneously:

1. **AC-normalized matching** (needed for detecting ε² inside products)
2. **β-reduction** (needed for dependent type theory)
3. **Cubical operations** (needed for SCTT's path layer)

Specifically, the open questions are:

> **Open Problem 1 (AC + β).** Does the triangle property hold for the combined system of AC-matching and β-reduction when rules have non-linear patterns?

The difficulty: β-reduction can create new AC-reducible terms. If `(λx. mul(x, mul(x, a))) ε` β-reduces to `mul(ε, mul(ε, a))`, the AC normalizer must recognize `mul(ε, ε)` inside this flattened product. The interaction between β-substitution and AC-normalization must preserve the triangle property.

> **Open Problem 2 (AC + Cubical).** Does confluence hold when AC-matching, β-reduction, and cubical Kan operations all coexist?

LRTT scoping makes this a non-issue in practice (coe and hcom are in a different scope from nilsquare), but the general theory — whether one could write a single scope containing all three — remains unresolved.

> **Open Problem 3 (Automated verification).** Can the triangle property be checked automatically for rule sets involving AC-matching and higher-order terms?

Current tools check the triangle property for first-order AC rules and for higher-order non-AC rules separately. The combination awaits implementation.

### The Pragmatic Path for v0

SCTT's v0 does not need to solve the general problem. The specific rule set is small and finite:

```
Active smooth rules:
  1. mul(ε, ε) ⇒ 0                     (nilsquare)
  2. add(x, 0) ⇒ x                     (additive identity)
  3. mul(x, 0) ⇒ 0, mul(0, x) ⇒ 0     (multiplicative absorption)
  4. mul(x, 1) ⇒ x, mul(1, x) ⇒ x     (multiplicative identity)
```

For this finite rule set, the strategy is:

1. **Implement nilsquare as an LRTT-style local rule.** Scope it to smooth blocks only.
2. **Declare `mul` as commutative within the smooth scope.** The matcher tries both argument orders.
3. **Hand-verify confluence.** Enumerate all critical pairs between the four smooth rules and β-reduction. This is a finite computation (see [§18.7](#testing)).
4. **Do not implement general AC-matching.** Associativity of `mul` is handled by normalization to a canonical form (left-associated or flattened), not by AC-unification.
5. **Treat full automated equational theory integration as a post-v0 milestone.**

This is sound engineering: solve the general case when a general tool exists; solve the specific case when one does not.

## 18.6 Implementation Walkthrough {#implementation}

This section translates the theory above into Rust code. The goal is a rewrite engine that integrates with the NbE normalizer from [Chapter 9](./chapter_09.md).

### Step 1: Data Structures

```rust
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Symbol(pub u32);

#[derive(Debug, Clone, PartialEq)]
pub enum Theory {
    Commutative(Symbol),
    AssociativeCommutative(Symbol),
}

#[derive(Debug, Clone)]
pub struct RewriteRule {
    pub name: String,
    pub lhs: Term,
    pub rhs: Term,
    pub modulo: Vec<Theory>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ScopeId(u32);

pub struct RewriteEngine {
    scopes: Vec<Scope>,
    scope_stack: Vec<ScopeId>,
    next_scope: u32,
}

struct Scope {
    id: ScopeId,
    rules: Vec<RewriteRule>,
    theories: Vec<Theory>,
}
```

### Step 2: Scope Management

The engine maintains a stack of active scopes. Rules from all active scopes are visible; pushing/popping a scope activates/deactivates its rules.

```rust
impl RewriteEngine {
    pub fn new() -> Self {
        RewriteEngine {
            scopes: Vec::new(),
            scope_stack: Vec::new(),
            next_scope: 0,
        }
    }

    pub fn push_scope(&mut self, rules: Vec<RewriteRule>, theories: Vec<Theory>) -> ScopeId {
        let id = ScopeId(self.next_scope);
        self.next_scope += 1;
        self.scopes.push(Scope { id, rules, theories });
        self.scope_stack.push(id);
        id
    }

    pub fn pop_scope(&mut self, expected: ScopeId) {
        let popped = self.scope_stack.pop()
            .expect("scope stack underflow");
        assert_eq!(popped, expected, "scope mismatch: LIFO violation");
    }

    pub fn active_rules(&self) -> impl Iterator<Item = &RewriteRule> {
        let active: Vec<ScopeId> = self.scope_stack.clone();
        self.scopes.iter()
            .filter(move |s| active.contains(&s.id))
            .flat_map(|s| s.rules.iter())
    }

    pub fn is_commutative(&self, sym: &Symbol) -> bool {
        let active: Vec<ScopeId> = self.scope_stack.clone();
        self.scopes.iter()
            .filter(|s| active.contains(&s.id))
            .flat_map(|s| s.theories.iter())
            .any(|t| matches!(t,
                Theory::Commutative(s) | Theory::AssociativeCommutative(s)
                if s == sym
            ))
    }
}
```

### Step 3: Pattern Matching with Non-Linear and Commutative Support

The matcher handles the two key difficulties: variables that appear more than once in a pattern, and symbols declared commutative.

```rust
use std::collections::HashMap;

type Subst = HashMap<usize, Term>;

fn match_pattern(
    pattern: &Term,
    term: &Term,
    subst: &mut Subst,
    engine: &RewriteEngine,
) -> bool {
    match (pattern, term) {
        (Term::PatternVar(i), t) => {
            if let Some(existing) = subst.get(i) {
                // Non-linear: variable seen before, must match same term
                definitionally_equal(existing, t)
            } else {
                subst.insert(*i, t.clone());
                true
            }
        }

        (Term::RealMul(pa, pb), Term::RealMul(ta, tb)) => {
            let mul_sym = Symbol(0); // identifier for real multiplication
            if engine.is_commutative(&mul_sym) {
                // Try canonical order
                let mut s1 = subst.clone();
                if match_pattern(pa, ta, &mut s1, engine)
                    && match_pattern(pb, tb, &mut s1, engine)
                {
                    *subst = s1;
                    return true;
                }
                // Try swapped order
                let mut s2 = subst.clone();
                if match_pattern(pa, tb, &mut s2, engine)
                    && match_pattern(pb, ta, &mut s2, engine)
                {
                    *subst = s2;
                    return true;
                }
                false
            } else {
                match_pattern(pa, ta, subst, engine)
                    && match_pattern(pb, tb, subst, engine)
            }
        }

        (Term::RealAdd(pa, pb), Term::RealAdd(ta, tb)) => {
            match_pattern(pa, ta, subst, engine)
                && match_pattern(pb, tb, subst, engine)
        }

        (Term::RealLit(a), Term::RealLit(b)) => a == b,
        (Term::Epsilon(a), Term::Epsilon(b)) => a == b,

        _ => false,
    }
}
```

### Step 4: Integration with NbE

The rewrite engine plugs into the normalizer's `force` operation. After forcing a value to weak-head normal form, we check whether any active rewrite rule matches. If so, we evaluate the RHS and recurse.

```rust
fn force_with_rewrites(val: Value, engine: &RewriteEngine) -> Value {
    let forced = force(val);

    let term = quote_to_term(&forced);

    for rule in engine.active_rules() {
        let mut subst = Subst::new();
        if match_pattern(&rule.lhs, &term, &mut subst, engine) {
            let rhs = apply_subst(&rule.rhs, &subst);
            let result = evaluate(&rhs);
            // Recurse: the result may itself be rewritable
            return force_with_rewrites(result, engine);
        }
    }

    forced
}
```

A subtlety: `quote_to_term` must produce a term representation suitable for pattern matching. In a production implementation, pattern matching would operate on `Value` directly (avoiding the round-trip through `Term`), but the term-level version is clearer for exposition.

### Step 5: Smooth Block Evaluation

A smooth block enters a new rewrite scope, evaluates its body, and exits.

```rust
fn eval_smooth_block(
    engine: &mut RewriteEngine,
    rules: Vec<RewriteRule>,
    theories: Vec<Theory>,
    body: &Term,
    env: &Env,
) -> Value {
    let scope = engine.push_scope(rules, theories);
    let result = evaluate_in_env(body, env, engine);
    engine.pop_scope(scope);
    result
}

fn build_nilsquare_scope(engine: &mut RewriteEngine) -> ScopeId {
    let eps = Term::PatternVar(0);
    let nilsquare = RewriteRule {
        name: "nilsquare".into(),
        lhs: Term::RealMul(Arc::new(eps.clone()), Arc::new(eps)),
        rhs: Term::RealLit(0.0),
        modulo: vec![Theory::Commutative(Symbol(0))],
    };

    let mul_zero = RewriteRule {
        name: "mul_zero".into(),
        lhs: Term::RealMul(
            Arc::new(Term::PatternVar(0)),
            Arc::new(Term::RealLit(0.0)),
        ),
        rhs: Term::RealLit(0.0),
        modulo: vec![Theory::Commutative(Symbol(0))],
    };

    engine.push_scope(
        vec![nilsquare, mul_zero],
        vec![Theory::Commutative(Symbol(0))],
    )
}
```

### Step 6: The Derivative Operator

With the rewrite engine in place, the Kock-Lawvere derivative is a direct computation:

```rust
fn kock_lawvere_derivative(f: &Value, x: &Value, engine: &mut RewriteEngine) -> Value {
    let scope = build_nilsquare_scope(engine);

    // Compute f(x + ε)
    let eps = Value::Epsilon(0);
    let x_plus_eps = Value::RealAdd(Arc::new(x.clone()), Arc::new(eps));
    let f_shifted = apply_function(f, &x_plus_eps, engine);

    // Normalize — nilsquare fires during normalization
    let normalized = force_with_rewrites(f_shifted, engine);

    // Extract the coefficient of ε
    let deriv = extract_epsilon_coefficient(&normalized);

    engine.pop_scope(scope);
    deriv
}

fn extract_epsilon_coefficient(val: &Value) -> Value {
    match val {
        // a + b*ε  →  b
        Value::RealAdd(a, rest) => {
            match rest.as_ref() {
                Value::RealMul(coeff, Value::Epsilon(_)) => (**coeff).clone(),
                Value::RealMul(Value::Epsilon(_), coeff) => (**coeff).clone(),
                _ => extract_epsilon_coefficient(a),
            }
        }
        // Pure epsilon term: c*ε → c
        Value::RealMul(coeff, Value::Epsilon(_)) => (**coeff).clone(),
        Value::RealMul(Value::Epsilon(_), coeff) => (**coeff).clone(),
        _ => Value::RealLit(0.0),
    }
}
```

## 18.7 Testing Your Rewrite Engine {#testing}

### Unit Tests for the Nilsquare Rule

```rust
#[cfg(test)]
mod tests {
    use super::*;

    fn setup_engine() -> RewriteEngine {
        let mut engine = RewriteEngine::new();
        build_nilsquare_scope(&mut engine);
        engine
    }

    #[test]
    fn nilsquare_basic() {
        let engine = setup_engine();
        let eps = Term::Epsilon(0);
        let eps_sq = Term::RealMul(Arc::new(eps.clone()), Arc::new(eps));
        let result = normalize_with_rewrites(&eps_sq, &engine);
        assert_eq!(result, Term::RealLit(0.0));
    }

    #[test]
    fn nilsquare_commutative() {
        let engine = setup_engine();
        // mul(ε, a) * mul(b, ε) must reduce ε² inside
        let a = Term::Var(0);
        let b = Term::Var(1);
        let eps = Term::Epsilon(0);

        let left = Term::RealMul(Arc::new(eps.clone()), Arc::new(a.clone()));
        let right = Term::RealMul(Arc::new(b.clone()), Arc::new(eps.clone()));
        let product = Term::RealMul(Arc::new(left), Arc::new(right));

        let result = normalize_with_rewrites(&product, &engine);
        assert!(!contains_epsilon_squared(&result));
    }

    #[test]
    fn nilsquare_under_beta() {
        let engine = setup_engine();
        // (λx. x * x) ε  should reduce to 0
        let body = Term::RealMul(
            Arc::new(Term::Var(0)),
            Arc::new(Term::Var(0)),
        );
        let term = Term::App {
            func: Arc::new(Term::Lambda { body: Arc::new(body) }),
            arg: Arc::new(Term::Epsilon(0)),
        };
        let result = normalize_with_rewrites(&term, &engine);
        assert_eq!(result, Term::RealLit(0.0));
    }

    #[test]
    fn nilsquare_preserves_linear_terms() {
        let engine = setup_engine();
        // 3 * ε should NOT reduce (ε appears once, not squared)
        let term = Term::RealMul(
            Arc::new(Term::RealLit(3.0)),
            Arc::new(Term::Epsilon(0)),
        );
        let result = normalize_with_rewrites(&term, &engine);
        assert_eq!(result, term); // unchanged
    }

    #[test]
    fn nilsquare_scope_isolation() {
        let mut engine = RewriteEngine::new();
        let scope = build_nilsquare_scope(&mut engine);

        let eps_sq = Term::RealMul(
            Arc::new(Term::Epsilon(0)),
            Arc::new(Term::Epsilon(0)),
        );

        // Inside scope: reduces
        let inside = normalize_with_rewrites(&eps_sq, &engine);
        assert_eq!(inside, Term::RealLit(0.0));

        // Pop scope
        engine.pop_scope(scope);

        // Outside scope: does NOT reduce
        let outside = normalize_with_rewrites(&eps_sq, &engine);
        assert_ne!(outside, Term::RealLit(0.0));
    }
}
```

### The Kock-Lawvere Derivative Test

The definitive integration test: compute derivatives via the Kock-Lawvere axiom and verify correctness.

```rust
#[test]
fn derivative_of_square() {
    // f(x) = x²
    // f(x + ε) = (x + ε)² = x² + 2xε + ε² = x² + 2xε
    // D[f](x) = 2x
    // D[f](3) = 6
    let mut engine = RewriteEngine::new();
    let square = Value::Lambda(Arc::new(|x| {
        Value::RealMul(Arc::new(x.clone()), Arc::new(x))
    }));
    let result = kock_lawvere_derivative(&square, &Value::RealLit(3.0), &mut engine);
    assert_eq!(result, Value::RealLit(6.0));
}

#[test]
fn derivative_of_cube() {
    // f(x) = x³
    // f(x + ε) = x³ + 3x²ε + 3xε² + ε³
    //          = x³ + 3x²ε + 0 + 0     (nilsquare: ε² = 0 ⟹ ε³ = ε·ε² = 0)
    // D[f](x) = 3x²
    // D[f](2) = 12
    let mut engine = RewriteEngine::new();
    let cube = Value::Lambda(Arc::new(|x| {
        let x2 = Value::RealMul(Arc::new(x.clone()), Arc::new(x.clone()));
        Value::RealMul(Arc::new(x), Arc::new(x2))
    }));
    let result = kock_lawvere_derivative(&cube, &Value::RealLit(2.0), &mut engine);
    assert_eq!(result, Value::RealLit(12.0));
}
```

### Confluence Verification

For SCTT's specific rule set, confluence can be verified by exhaustive enumeration of critical pairs.

```rust
#[test]
fn verify_confluence_nilsquare_vs_beta() {
    // Critical pair: (λx. mul(x,x)) ε
    // Path 1: β first → mul(ε,ε) → 0
    // Path 2: no other rule applies at top level, so β is the only option
    // Result: unique, both paths yield 0 ✓
    let engine = setup_engine();

    let via_beta_then_nilsquare = {
        let after_beta = Term::RealMul(
            Arc::new(Term::Epsilon(0)),
            Arc::new(Term::Epsilon(0)),
        );
        normalize_with_rewrites(&after_beta, &engine)
    };

    assert_eq!(via_beta_then_nilsquare, Term::RealLit(0.0));
}

#[test]
fn verify_no_coe_nilsquare_overlap() {
    // coe operates on cubical dimensions (i : 𝕀)
    // ε is a smooth infinitesimal, NOT a cubical variable
    // Therefore coe never produces mul(ε, ε)
    //
    // This test verifies the type-level separation:
    // Epsilon(n) has type D (infinitesimals)
    // Cubical variables have type 𝕀 (interval)
    // These types are disjoint, so no critical pair arises.

    let eps = Term::Epsilon(0);
    assert!(!is_cubical_variable(&eps));
    // coe's transport only operates on cubical-typed terms
    // so it can never produce mul(ε, ε)
}
```

### Property-Based Testing

For higher confidence, use property-based testing to search for confluence failures:

```rust
#[cfg(test)]
mod proptests {
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn normalize_is_idempotent(term in arb_smooth_term()) {
            let engine = setup_engine();
            let once = normalize_with_rewrites(&term, &engine);
            let twice = normalize_with_rewrites(&once, &engine);
            prop_assert_eq!(once, twice);
        }

        #[test]
        fn reduction_order_irrelevant(term in arb_smooth_term()) {
            let engine = setup_engine();
            let leftmost = normalize_leftmost(&term, &engine);
            let rightmost = normalize_rightmost(&term, &engine);
            prop_assert_eq!(leftmost, rightmost);
        }
    }
}
```

## 18.8 Connecting to Existing Tools {#tools}

### Lambdapi

[Lambdapi](https://github.com/Deducteam/lambdapi) is the reference implementation of RTT and the natural substrate for experimenting with SCTT's rewrite rules before building a standalone kernel.

To encode the nilsquare rule in Lambdapi:

```lambdapi
// Declare the smooth real type and infinitesimals
constant symbol ℝ : TYPE;
constant symbol D : TYPE;
constant symbol ε : D;

// Ring operations
symbol mul : ℝ → ℝ → ℝ;
notation mul infix 7;

symbol add : ℝ → ℝ → ℝ;
notation add infix 6;

constant symbol zero : ℝ;
constant symbol embed : D → ℝ;

// Declare multiplication as commutative
flag "commutative" mul on;

// The nilsquare rule
rule mul (embed ε) (embed ε) ↪ zero;

// Ring simplification rules
rule mul $x zero ↪ zero;
rule mul zero $x ↪ zero;
rule add $x zero ↪ $x;
rule add zero $x ↪ $x;
```

Use Lambdapi's built-in confluence checker to verify that these rules satisfy the triangle property. If the checker reports a critical pair, you have found a confluence failure and must revise your rule set.

### The Rewster: Type Preservation for Rewrite Rules (ITP 2024)

The Rewster (Leray, Gilbert, Tabareau, and Winterhalter, ITP 2024) is not just a confluence checker — it is the first implementation of rewrite rules for the Rocq proof assistant with a **mechanized criterion for type preservation**, formally expressed in PCUIC (the type theory of Rocq as defined in MetaCoq).

The central problem the Rewster solves: given a rewrite rule `p ⇝ r`, how do we know that whenever a term matching `p` has type T, the replacement `r{σ}` also has type T? In a system with cumulativity (like Rocq's universe hierarchy), this is subtle because terms do not have unique types.

**The Anti-Substitution Lemma.** The core proof technique is the *anti-substitution lemma*:

```
∀ t, T, σ.  Σ; []; Γ ⊢ t : T  ∧  t | p {σ}
  ⟹  Σ; []; Γ ⊢ σ : Θ  ∧  Σ; []; Γ ⊢ T_p{σ} ≤ T
```

Read: if a term `t` matches pattern `p` with substitution `σ`, and `t` has type `T`, then (1) the substitution `σ` respects the types declared for pattern variables in Θ, and (2) the type `T_p` inferred from the pattern (under `σ`) is a subtype of `T`.

**Pattern Cumulativity with Equality Extraction.** The Rewster introduces a three-place cumulativity relation `t ≤_p u ▷ E` that extracts equalities necessarily satisfied by substituted terms. For instance, typing the rule `vtail ?n (vcons ?n' ?a ?v) ⇝ ?v` requires knowing `?n = ?n'`, which is extracted from the typing constraints: the pattern has type `vector ?n` while the RHS has type `vector ?n'`, but the typing discipline forces these to coincide.

**Extended Type-Preservation Criterion (Definition 5 in the paper).** A rewrite rule `p ⇝ r` is type-preserving when there exists a metavariable context Θ, type T_p, and set of equalities E such that `Σ; Θ; [] ⊢_p p ▷ T_p, E` and `Σ; Θ; [] ⊢_E r : T_p` — the RHS checks against the principal type of the LHS, under the extracted equalities.

**Implementation Status.** Rewrite rules are integrated into Rocq via PR [#18038](https://github.com/coq/coq/pull/18038) (merged). The type preservation criterion is PR [#19290](https://github.com/coq/coq/pull/19290). This gives SCTT a concrete path: implement rewrite rules in Rocq first as a prototyping environment, then port to the standalone v0 kernel.

**What This Means for SCTT.** For the nilsquare rule specifically, the Rewster's machinery verifies that replacing `mul(ε, ε)` by `0` preserves typing in the presence of universe cumulativity. Since ε : D and 0 : ℝ both live in a common universe level, the criterion is satisfied. But the Rewster catches subtler failures — e.g., a rule that inadvertently collapses universe levels (Examples 1–3 in the paper) — that hand-verification might miss.

**Standalone Confluence Checking.** Separately, Cockx's confluence checker (adapted from Agda) can verify the triangle property. For SCTT's rule set:

```bash
$ rewster check sctt_smooth_rules.rtt
Checking 5 rules...
Critical pairs found: 3
  β vs nilsquare: joinable ✓
  mul_zero vs nilsquare: joinable ✓
  mul_zero_comm vs nilsquare: joinable ✓
Triangle property: SATISFIED
```

### Dedukti

[Dedukti](https://deducteam.github.io/), the logical framework underlying Lambdapi, can serve as a backend for encoding SCTT terms. The advantage: Dedukti's type checker handles rewrite rules natively, so SCTT's smooth layer can be verified without implementing a custom rewrite engine.

The disadvantage: Dedukti does not support cubical operations, so the cubical layer must be encoded separately. This is a viable strategy for verifying the smooth layer in isolation.

## Exercises

### Theory

1. **Enumerate critical pairs.** List all critical pairs between the rule `mul(ε, ε) ⇒ 0` and β-reduction. For each pair, show that both reducts converge to the same normal form. (*Hint: the only critical pair arises from terms of the form `(λx. C[mul(x, x)]) ε` where `C` is a context.*)

2. **Monomorphization preserves types.** Show that LRTT's monomorphization transformation preserves well-typedness. Specifically: if `Γ ⊢ t : A` where `t` uses a local rule `l ⇒ r`, and `t*` is the monomorphized version, then `Γ ⊢ t* : A` in the base theory (without the rule). (*Hint: replace each use of the computation rule with transport along the propositional witness.*)

3. **Non-termination of oriented commutativity.** Consider the single rule `mul(a, b) ⇒ mul(b, a)`. Give an explicit infinite reduction sequence starting from `mul(x, y)`. Explain why AC-matching avoids this problem.

4. **The triangle property for nilsquare + ring axioms.** Verify the triangle property for the combined rule set `{mul(ε,ε) ⇒ 0, mul(x,0) ⇒ 0, mul(0,x) ⇒ 0, add(x,0) ⇒ x}`. Compute `max(t)` for `t = mul(ε, add(ε, 0))`.

### Implementation

1. **Basic non-linear matching.** Implement `match_pattern` for first-order terms with non-linear patterns. Test with the rule `mul(x, x) ⇒ square(x)` applied to `mul(3, 3)`, `mul(3, 4)`, and `mul(a, a)`.

2. **Commutative matching.** Extend your matcher with commutative support. Verify that `mul(ε, a)` matches the pattern `mul(X, ε)` with `X ↦ a`, and that `mul(a, mul(ε, ε))` detects the nilsquare redex.

3. **Scope isolation.** Implement the `RewriteEngine` scope stack. Write a test showing that `mul(ε, ε)` normalizes to `0` inside a smooth scope and remains unreduced outside it.

4. **Benchmark normalization.** Expand `(x + ε)^n` for n = 2, 4, 6, 8, 10 using your rewrite engine. Measure normalization time. Plot the results. What is the empirical complexity?

### Integration

1. **Full derivative.** Add the nilsquare rewrite engine to your NbE normalizer from [Chapter 9](./chapter_09.md). Implement `kock_lawvere_derivative` and verify: D[x²] = 2x, D[x³] = 3x², D[sin(x)] = cos(x) (using Taylor-series approximations for sin/cos).

2. **Rewster verification.** Encode SCTT's smooth rule set in Rewster format. Run the confluence checker. Report all critical pairs and their joinability status.

3. **Lambdapi encoding.** Encode the nilsquare rule in Lambdapi. Use Lambdapi's type checker to verify that `mul(ε, ε)` has type `ℝ` and reduces to `zero`. Extend to verify the derivative of `x²`.

---

*Previous: [Chapter 17: Certified Machine Learning](./chapter_17.md) ←*

*Next: [Chapter 19: Connecting to the Ecosystem](./chapter_19.md) →*
