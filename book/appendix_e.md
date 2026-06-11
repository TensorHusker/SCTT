# Appendix E: Ordered Reading Path

This appendix provides a single, dependency-ordered reading list for the theoretical foundations of SCTT. The seven **core papers** form a strict dependency chain — read them in order. The **secondary papers** are grouped by topic and can be read independently after completing the core chain.

## Core Reading List (7 papers, in order)

Read these sequentially. Each depends on concepts from the ones above it.

```
                    ┌─────────────────────────┐
                    │  1. Martin-Löf (1984)    │  Dependent type theory
                    │     Intuitionistic TT    │  (the foundation)
                    └────────────┬────────────┘
                                 │
                    ┌────────────▼────────────┐
                    │  2. ABCFHL (2021)        │  Cartesian cubical TT
                    │     Cartesian Cubical TT │  (path types, Kan ops)
                    └────────────┬────────────┘
                                 │
                    ┌────────────▼────────────┐
                    │  3. Kovács (2024)        │  High-performance
                    │     cctt evaluator       │  cubical implementation
                    └────────────┬────────────┘
                                 │
                    ┌────────────▼────────────┐
                    │  4. Cockx, Tabareau,     │  Adding rewrite rules
                    │     Winterhalter (2021)   │  to dependent TT
                    │     RTT — POPL 2021      │
                    └────────────┬────────────┘
                                 │
                    ┌────────────▼────────────┐
                    │  5. Leray, Winterhalter  │  Locally-scoped rules
                    │     LRTT — POPL 2026     │  (the ε²=0 enabler)
                    └────────────┬────────────┘
                                 │
                    ┌────────────▼────────────┐
                    │  6. Kock (2006/2010)     │  Synthetic differential
                    │     SDG — 2nd edition    │  geometry foundations
                    └────────────┬────────────┘
                                 │
                    ┌────────────▼────────────┐
                    │  7. Reed & Pierce (2010) │  Sensitivity/distance
                    │     Fuzz — ICFP 2010     │  types for privacy
                    └─────────────────────────┘
```

### 1. Martin-Löf — *Intuitionistic Type Theory* (1984)

The foundation. Introduces dependent function types (Π), dependent pair types (Σ), identity types, universes, and the judgmental structure (formation, introduction, elimination, computation) that every subsequent paper assumes.

**What you need from it:** The four judgment forms, how Π and Σ types work, what identity types are, and why computation rules are definitional equalities.

**Citation:** Martin-Löf, P. *Intuitionistic Type Theory.* Bibliopolis, Naples, 1984.

### 2. ABCFHL — *Cartesian Cubical Type Theory* (2021)

Extends Martin-Löf's type theory with an abstract interval `I` (with only endpoints `i0`, `i1` and diagonal cofibration `i = j`), path types `PathP`, coercion `coe`, homogeneous composition `hcom`, and Glue types for univalence. This is the *Cartesian* variant — no connections, no reversal — which simplifies the normalizer and interacts cleanly with the smooth layer.

**What you need from it:** The interval algebra (Cartesian, not De Morgan), cofibrations, path types, Kan operations, Glue types.

**Why Cartesian over CCHM:** Fewer interval operations mean fewer cases in the evaluator, and the absence of connections avoids critical pairs with the smooth rewrite rules. See the Design Note in Chapter 3.

**Citation:** Angiuli, C., Brunerie, G., Coquand, T., Hou (Favonia), K.-B., Harper, R., Licata, D. *Syntax and models of Cartesian cubical type theory.* Mathematical Structures in Computer Science, 31(4), 2021.

### 3. Kovács — *cctt: High-performance cubical evaluator* (2024)

The implementation reference. Demonstrates how to build a cubical type checker that is fast enough to compute the Brunerie number (π₄(S³) = ±2). Key techniques: **defunctionalized closures** (not HOAS — closures must be data, not functions, so the evaluator can inspect and substitute into them), the **sub/force pattern** (O(1) lazy interval substitution), and **triple-context evaluation** (interval variables, cofibrations, fibrant environment as three separate contexts).

**What you need from it:** The evaluation architecture. This is what Chapter 16's Phase 2 implements.

**Citation:** Kovács, A. *cctt: a high-performance proof assistant for cubical type theory.* 2024. https://github.com/AndrasKovacs/cctt

### 4. Cockx, Tabareau, Winterhalter — *RTT: Rewriting Type Theory* (POPL 2021)

Extends CIC with user-defined rewrite rules. The central contribution is the **triangle property**: a condition on rule sets that is stronger than local confluence but *modular* (new rules can be checked independently against existing ones) and *decidable* (via critical pair computation). If a rule set satisfies the triangle property, the extended system preserves subject reduction and consistency.

**What you need from it:** How to add `mul(ε, ε) ⇒ 0` as a rewrite rule with a consistency witness (set ε = 0 in any model), and why the triangle property is the right confluence criterion.

**Citation:** Cockx, J., Tabareau, N., Winterhalter, T. *The taming of the rew: a type theory with computational assumptions.* POPL 2021.

### 5. Leray, Winterhalter — *LRTT: Locally-Scoped Rewriting Type Theory* (POPL 2026)

The key enabler for SCTT. Extends RTT with **local abstraction** over rewrite rules: a definition can declare that it uses the nilsquare rule, and the rule is active only within that definition's scope. This is compiled away via **monomorphization** — every use site is expanded with the rule inlined. Result: the cubical Kan operations never see ε² = 0, because it's scoped to smooth blocks. This eliminates the cross-layer critical pairs that would otherwise be intractable.

**What you need from it:** Local scoping, monomorphization, and the conservativity result (anything provable with local rules is provable without them using propositional equality).

**Citation:** Leray, G., Winterhalter, T. *Taming rewrite rules with local scoping in rewriting type theory.* POPL 2026. (Distinguished Paper)

### 6. Kock — *Synthetic Differential Geometry* (2006, 2nd edition 2010)

The mathematical foundation for SCTT's smooth layer. Introduces the **Kock-Lawvere axiom**: for the type D = {ε ∈ R | ε² = 0} of nilsquare infinitesimals, every function f : D → R uniquely decomposes as f(ε) = a + bε. This makes differentiation algebraic: D[f](x) is the coefficient of ε in f(x + ε).

**What you need from it:** The axiom schema, the definition of D, how tangent bundles and differential forms arise from it, and why ε² = 0 must be a definitional (not propositional) equality for the theory to work.

**Citation:** Kock, A. *Synthetic Differential Geometry.* London Mathematical Society Lecture Note Series, Cambridge University Press, 2nd edition, 2010.

### 7. Reed & Pierce — *Fuzz: Distance Types for Differential Privacy* (ICFP 2010)

Introduces **linear sensitivity types**: if f : !_k A ⊸ B, then f is k-sensitive (k-Lipschitz) — changing the input by δ changes the output by at most kδ. Originally designed for differential privacy, these types generalize to the Lipschitz bounds that SCTT uses for certified ML (Chapter 17) and convergence analysis.

**What you need from it:** The sensitivity monad `!_k`, how sensitivities compose (multiplication for sequential, addition for parallel), and the connection to metric semantics.

**Citation:** Reed, J., Pierce, B. C. *Distance makes the types grow stronger: a calculus for differential privacy.* ICFP 2010.

---

## Secondary Reading (by topic)

### Rewriting and Verification

| Paper | Venue | Contribution |
|-------|-------|-------------|
| Felicissimo — *BiTTs* | ESOP 2024 | Generic bidirectional type checking for theories with rewrite rules |
| Leray et al. — *Rewster* | ITP 2024 | Mechanized verification of RTT metatheory in MetaRocq |
| Barras, Felicissimo, Winterhalter — *Gruissan* | EuroProofNet 2024 | Open problems for matching modulo equational theories in HO rewriting |

### Cubical Implementations

| Paper | Venue | Contribution |
|-------|-------|-------------|
| Sterling, Angiuli — *Normalization for cubical TT* | LICS 2021 | Canonicity proof for Cartesian cubical TT |
| Huang — *Normal forms for Cartesian cubical TT* | arXiv 2026 | Explicit normal form specification with three-parameter neutrals |
| Angiuli, Hou, Harper — *Computational Higher TT III* | arXiv 2017 | Precise `coe`/`hcom` definitions for Cartesian cubical TT |
| Vezzosi, Mörtberg, Abel — *Cubical Agda* | ICFP 2019 | Production cubical proof assistant (CCHM variant) |
| Huber — *Canonicity for cubical TT* | JAR 2018 | First canonicity result for cubical type theory |

### Sensitivity and Metrics

| Paper | Venue | Contribution |
|-------|-------|-------------|
| Gaboardi et al. — *DFuzz* | POPL 2013 | Dependent sensitivity types |
| Azevedo de Amorim et al. — *Metric* | POPL 2017 | Higher-order metric reasoning |
| Aberlé — *Parametricity and SDG* | MFPS 2024 | Connects CHH parametricity with cohesion and Kock-Lawvere |

### Certified Machine Learning

| Paper | Venue | Contribution |
|-------|-------|-------------|
| Elliott — *AD as functor* | ICFP 2018 | Automatic differentiation is a correct-by-construction functor |
| Fong, Spivak, Tuyéras — *Backprop as functor* | LICS 2019 | Categorical foundation for gradient-based learning |
| Chen et al. — *Neural ODEs* | NeurIPS 2018 | Continuous-depth networks as ODE solutions |
| Cruttwell et al. — *Categorical foundations of AD* | ESOP 2022 | Cartesian differential categories for AD |

### Parametricity and Cohesion

| Paper | Venue | Contribution |
|-------|-------|-------------|
| Cavallo, Harper — *Internal parametricity* | CSL 2020 | Parametricity internal to cubical TT |
| Shulman — *Cohesive HoTT* | 2018 | Modal operators for cohesion in HoTT |
| Schreiber — *Differential cohomology in a cohesive ∞-topos* | 2013 | The categorical semantics SCTT targets |

### Two-Level Type Theory

| Paper | Venue | Contribution |
|-------|-------|-------------|
| Kovács — *CFTT* | ICFP 2024 | Two-level type theory with staging for cubical evaluation |
| Barras, Maestracci — *2LTT in Dedukti* | LFMTP 2020 | Encoding CTT in Dedukti via two-layer external equality |
| Voevodsky — *HTS* | 2013 | Original two-level type theory proposal |

---

## How to Use This List

**If you are building the v0 kernel** (Chapter 16): Read papers 1-5, then 3 for architecture. Papers 6-7 come when you reach Phases 3-4.

**If you are working on certified ML** (Chapter 17): Read paper 7 first, then Elliott and Fong/Spivak/Tuyéras from the secondary list. Papers 1-2 provide the type-theoretic foundation.

**If you are tackling the rewrite rule problem** (Chapter 18): Read papers 4 and 5 carefully, then Felicissimo (BiTTs) and Leray et al. (Rewster) from the secondary list. The Gruissan note states the open problems.

**If you are connecting to existing tools** (Chapter 19): Read paper 2 (for cubical Agda comparison), then Vezzosi/Mörtberg/Abel, Sterling/Angiuli, and Barras/Maestracci (for the Dedukti 2LTT encoding strategy) from the secondary list.

---

*Previous: [Appendix D: SCTT Standard Library](./appendix_d.md) ←*

*Next: [Bibliography](./bibliography.md) →*
