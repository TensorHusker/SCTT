# Appendix B: Notation Guide

> "Good notation can make all the difference." — Terry Tao

This appendix provides a comprehensive reference for all mathematical notation used in this book. Symbols are grouped by category for easy lookup.

**How to use this guide:**
- Use your browser's search (Ctrl+F / Cmd+F) to find symbols
- Check the "Pronunciation" column to know how to read aloud
- See the "Chapter" column to find where concepts are introduced

---

## Table of Contents

1. [Basic Type Theory](#basic-type-theory)
2. [Judgment Forms](#judgment-forms)
3. [Function Types](#function-types)
4. [Dependent Types](#dependent-types)
5. [Equality and Paths](#equality-and-paths)
6. [Cubical Structure](#cubical-structure)
7. [Smooth Types](#smooth-types)
8. [Differential Operators](#differential-operators)
9. [Manifolds and Geometry](#manifolds-and-geometry)
10. [Integration](#integration)
11. [Differential Forms](#differential-forms)
12. [Logic and Propositions](#logic-and-propositions)
13. [Special Symbols](#special-symbols)

---

## Basic Type Theory {#basic-type-theory}

| Symbol | Pronunciation | Meaning | Example | Chapter |
|--------|--------------|---------|---------|---------|
| `Type` | "type" | Universe of types | `Bool : Type` | 2 |
| `Type ℓ` | "type level ℓ" | Universe at level ℓ | `Type₀ : Type₁` | 2 |
| `A : Type` | "A is a type" | A is a well-formed type | `Nat : Type` | 2 |
| `x : A` | "x has type A" / "x in A" | x is a term of type A | `3 : Nat` | 2 |
| `·` | "empty context" | Empty typing context | `⊢ · ctx` | 2 |
| `Γ, x : A` | "Gamma extended with x of type A" | Context with new variable | `x : Nat ⊢ x + 1 : Nat` | 2 |

---

## Judgment Forms {#judgment-forms}

| Symbol | Pronunciation | Meaning | Chapter |
|--------|--------------|---------|---------|
| `⊢ Γ ctx` | "Gamma is a context" | Γ is well-formed | 2 |
| `Γ ⊢ A : Type` | "A is a type in Gamma" | A is a type in context Γ | 2 |
| `Γ ⊢ a : A` | "a has type A in Gamma" | a is a term of type A | 2 |
| `Γ ⊢ a ≡ b : A` | "a equals b at type A" | Definitional equality | 2 |

---

## Function Types {#function-types}

| Symbol | Pronunciation | Meaning | Example | Chapter |
|--------|--------------|---------|---------|---------|
| `A → B` | "A to B" | Function type | `Nat → Bool` | 2 |
| `λ x. e` | "lambda x dot e" | Function abstraction | `λ x. x + 1` | 2 |
| `f x` | "f applied to x" | Function application | `double 3` | 2 |
| `f ∘ g` | "f after g" / "f circle g" | Function composition | `f ∘ g = λ x. f (g x)` | 2 |
| `id` | "identity" | Identity function | `id x = x` | 2 |

---

## Dependent Types {#dependent-types}

| Symbol | Pronunciation | Meaning | Example | Chapter |
|--------|--------------|---------|---------|---------|
| `Π (x : A), B(x)` | "pi x in A, B of x" | Dependent function type | `Π (n : Nat), Vec A n` | 2 |
| `(x : A) → B(x)` | "for all x in A, B of x" | Dependent function (alternative notation) | `(n : Nat) → Vec A n` | 2 |
| `Σ (x : A), B(x)` | "sigma x in A, B of x" | Dependent pair type | `Σ (n : Nat), Vec A n` | 2 |
| `A × B` | "A times B" / "A cross B" | Product type | `Nat × Bool` | 2 |
| `A ⊎ B` | "A plus B" / "A coproduct B" | Sum type | `Bool ⊎ Nat` | 2 |
| `⊤` | "top" / "unit" | Unit type | `tt : ⊤` | 2 |
| `⊥` | "bottom" / "empty" | Empty type | No terms | 2 |

---

## Equality and Paths {#equality-and-paths}

| Symbol | Pronunciation | Meaning | Example | Chapter |
|--------|--------------|---------|---------|---------|
| `a ≡ b` | "a equals b" (definitional) | Definitional equality | `2 + 2 ≡ 4` | 2, 3 |
| `Path A x y` | "path in A from x to y" | Path type | `Path Nat 0 0` | 3 |
| `x ≡_A y` | "x equals y in A" (propositional) | Path type (alternative) | `3 ≡_Nat 3` | 3 |
| `PathP A x y` | "dependent path over A from x to y" | Dependent path | `PathP (λi. A i) x y` | 3 |
| `refl` | "reflexivity" | Reflexive path | `refl : Path A x x` | 3 |
| `sym p` | "symmetry of p" | Reverse path | `sym : Path A x y → Path A y x` | 3 |
| `trans p q` / `p ∙ q` | "transitivity" / "p then q" | Path composition | `p ∙ q : Path A x z` | 3 |
| `ap f p` | "action on paths by f of p" | Apply function to path | `ap f : Path A x y → Path B (f x) (f y)` | 3 |
| `transport p` | "transport along p" | Move along path | `transport : Path Type A B → A → B` | 3 |

---

## Cubical Structure {#cubical-structure}

| Symbol | Pronunciation | Meaning | Example | Chapter |
|--------|--------------|---------|---------|---------|
| `I` | "interval" | Interval type | `i : I` | 3 |
| `i0` | "zero" / "left endpoint" | Left endpoint of I | `i0 : I` | 3 |
| `i1` | "one" / "right endpoint" | Right endpoint of I | `i1 : I` | 3 |
| `i ∧ j` | "i meet j" / "i and j" | Meet (minimum) | `(i ∧ j) : I` | 3 |
| `i ∨ j` | "i join j" / "i or j" | Join (maximum) | `(i ∨ j) : I` | 3 |
| `~i` | "not i" / "reverse i" | Involution (1 - i) | `~i : I` | 3 |
| `p @ i` | "p at i" | Path application | `(λi. x) @ i0 ≡ x` | 3 |
| `comp` | "composition" | Kan composition | `comp A φ u u0` | 3 |
| `hcomp` | "homogeneous comp" | Homogeneous composition | `hcomp φ u u0` | 3 |
| `transp` | "transport" | Transport operation | `transp A φ u0` | 3 |
| `Glue` | "glue" | Glue type | `Glue A φ Te` | 3 |
| `glue` / `unglue` | "glue" / "unglue" | Glue constructors | `unglue : Glue A φ Te → A` | 3 |

---

## Smooth Types {#smooth-types}

| Symbol | Pronunciation | Meaning | Example | Chapter |
|--------|--------------|---------|---------|---------|
| `ℝ` | "the reals" / "R" | Smooth real numbers | `x : ℝ` | 4 |
| `ℝⁿ` | "R to the n" / "R n" | n-dimensional real space | `x : ℝ³` | 4 |
| `ℝ₊` | "R plus" / "positive reals" | Strictly positive real numbers | `ε : ℝ₊` | 4 |
| `ℝ≥₀` | "R greater-equal zero" / "non-negative reals" | Non-negative real numbers (codomain of metrics and norms) | `d(x,y) : ℝ≥₀` | 4 |
| `C∞(M, N)` | "C infinity from M to N" | Smooth functions | `f : C∞(ℝ, ℝ)` | 4 |
| `Cⁿ(M, N)` | "C n from M to N" | n-times differentiable functions | `f : C²(ℝ, ℝ)` | 4 |
| `𝔻` | "D" / "infinitesimals" | Infinitesimal object | `ε : 𝔻` | 4 |
| `SmoothType` | "smooth type" | Universe of smooth types | `ℝ : SmoothType` | 4 |
| `Manifold` | "manifold" | Smooth manifold type | `M : Manifold` | 4 |
| `TM` | "tangent M" / "T M" | Tangent bundle | `v : TM` | 4 |
| `T*M` | "cotangent M" | Cotangent bundle | `α : T*M` | 4 |
| `T_x M` | "tangent space at x" | Tangent space at point x | `v : T_x M` | 4 |

---

## Differential Operators {#differential-operators}

| Symbol | Pronunciation | Meaning | Example | Chapter |
|--------|--------------|---------|---------|---------|
| `D[f]` | "D of f" / "derivative of f" | Derivative operator | `D[λx. x²] = λx. 2x` | 5 |
| `Df` | "derivative f" | Derivative (prefix) | `Df : TM → TN` | 5 |
| `f'` | "f prime" | Derivative (prime notation) | `f' x = Df x` | 5 |
| `∂f/∂x` | "partial f partial x" | Partial derivative | `∂f/∂x : ℝⁿ → ℝ` | 5 |
| `∂ᵢ f` | "partial i of f" | i-th partial derivative | `∂₁ f = ∂f/∂x₁` | 5 |
| `∇f` | "gradient f" / "nabla f" | Gradient | `∇f : ℝⁿ → ℝⁿ` | 5 |
| `∇²f` / `Δf` | "laplacian f" / "delta f" | Laplacian | `∇²f = Σᵢ ∂²f/∂xᵢ²` | 5 |
| `div F` / `∇·F` | "divergence of F" | Divergence | `div F : ℝⁿ → ℝ` | 5 |
| `curl F` / `∇×F` | "curl of F" | Curl (3D only) | `curl F : ℝ³ → ℝ³` | 5 |
| `Dⁿ[f]` | "D to the n of f" | n-th derivative | `D²[f] = D[D[f]]` | 5 |
| `Jacobian F` | "Jacobian of F" | Jacobian matrix | `Jacobian F : ℝⁿ → ℝᵐˣⁿ` | 5 |
| `Hessian f` | "Hessian of f" | Hessian matrix | `Hessian f = [∂²f/∂xᵢ∂xⱼ]` | 5 |

---

## Integration {#integration}

| Symbol | Pronunciation | Meaning | Example | Chapter |
|--------|--------------|---------|---------|---------|
| `∫` | "integral" | Integration symbol | `∫₀¹ f(x) dx` | 5 |
| `∫ a b f` | "integral from a to b of f" | Definite integral | `∫ 0 1 (λx. x²)` | 5 |
| `∫[f]` | "antiderivative of f" | Indefinite integral | `∫[f] = F where D[F] = f` | 5 |
| `∫_M ω` | "integral over M of omega" | Manifold integration | `∫_S² ω` | 5 |
| `∫_γ` | "integral over gamma" | Line integral | `∫_γ F·dr` | 5 |
| `∮` | "contour integral" | Closed path integral | `∮_C f dz` | 5 |
| `dx` | "dx" / "dee x" | Differential element | In `∫ f(x) dx` | 5 |

---

## Differential Forms {#differential-forms}

| Symbol | Pronunciation | Meaning | Example | Chapter |
|--------|--------------|---------|---------|---------|
| `Ωᵏ M` | "omega k M" / "k-forms on M" | Space of k-forms | `α : Ω² M` | 5 |
| `Ω⁰ M` | "zero-forms on M" | Functions | `Ω⁰ M = C∞(M, ℝ)` | 5 |
| `Ω¹ M` | "one-forms on M" | Covector fields | `dx, dy : Ω¹ ℝ²` | 5 |
| `dx` | "dee x" | Coordinate differential | `dx : Ω¹ ℝⁿ` | 5 |
| `α ∧ β` | "alpha wedge beta" | Wedge product | `dx ∧ dy : Ω² ℝ²` | 5 |
| `d α` | "d of alpha" / "exterior derivative of alpha" | Exterior derivative | `d : Ωᵏ → Ωᵏ⁺¹` | 5 |
| `f*` | "f pullback" / "pullback by f" | Pullback of forms | `f* : Ωᵏ N → Ωᵏ M` | 5 |
| `L_X` | "Lie derivative along X" | Lie derivative | `L_X : Ωᵏ → Ωᵏ` | 5 |
| `i_X` | "interior product with X" | Interior product | `i_X : Ωᵏ → Ωᵏ⁻¹` | 5 |
| `⋆` | "Hodge star" | Hodge dual | `⋆ : Ωᵏ → Ωⁿ⁻ᵏ` | 5 |

---

## Manifolds and Geometry {#manifolds-and-geometry}

| Symbol | Pronunciation | Meaning | Example | Chapter |
|--------|--------------|---------|---------|---------|
| `dim M` | "dimension of M" | Dimension of manifold | `dim ℝⁿ = n` | 4 |
| `π : E → M` | "pi from E to M" | Projection map | Fiber bundle projection | 4 |
| `VectorField M` | "vector field on M" | Vector field type | `X : VectorField M` | 4, 5 |
| `SmoothPath M` | "smooth path in M" | Smooth path type | `γ : SmoothPath M` | 4 |
| `[X, Y]` | "bracket X Y" / "Lie bracket of X and Y" | Lie bracket | `[X,Y] : VectorField M` | 5 |
| `g` | "g" / "metric" | Riemannian metric | `g : TM ⊗ TM → ℝ` (a section of `T*M ⊗ T*M`) | 4 |
| `⟨v, w⟩` | "v inner w" / "inner product" | Inner product | `⟨v,w⟩_g` | 4 |
| `Γⁱⱼₖ` | "Christoffel symbol i j k" | Christoffel symbols | Connection coefficients | 12 |
| `R` | "Riemann" / "curvature" | Riemann curvature tensor | Curvature | 12 |

---

## Logic and Propositions {#logic-and-propositions}

| Symbol | Pronunciation | Meaning | Example | Chapter |
|--------|--------------|---------|---------|---------|
| `∀` | "for all" | Universal quantification | `∀ x : A, P(x)` | 2 |
| `∃` | "there exists" | Existential quantification | `∃ x : A, P(x)` | 2 |
| `∃!` | "there exists unique" | Unique existence | `∃! x, P(x)` | 5 |
| `P ∧ Q` | "P and Q" | Logical conjunction | `(x > 0) ∧ (x < 10)` | 2 |
| `P ∨ Q` | "P or Q" | Logical disjunction | `(x = 0) ∨ (x = 1)` | 2 |
| `¬P` | "not P" | Negation | `¬(x < 0)` | 2 |
| `P → Q` | "P implies Q" | Implication | `x > 0 → x² > 0` | 2 |
| `P ↔ Q` | "P if and only if Q" / "P iff Q" | Bi-implication | `x = 0 ↔ |x| = 0` | 2 |
| `∥P∥` | "propositional truncation of P" | Squash type | `∥ ∃ x, P(x) ∥` | 2, 3 |
| `isProp P` | "P is a proposition" | Proposition predicate | At most one proof | 3 |

---

## Special Symbols {#special-symbols}

### Set Theory and Relations

| Symbol | Pronunciation | Meaning | Example |
|--------|--------------|---------|---------|
| `∈` | "in" / "element of" | Set membership | `x ∈ S` |
| `∉` | "not in" | Not a member | `x ∉ S` |
| `⊆` | "subset" | Subset relation | `A ⊆ B` |
| `∪` | "union" | Set union | `A ∪ B` |
| `∩` | "intersection" | Set intersection | `A ∩ B` |
| `∅` | "empty set" | Empty set | `∅` |

### Equivalences

| Symbol | Pronunciation | Meaning | Example | Chapter |
|--------|--------------|---------|---------|---------|
| `A ≃ B` | "A equivalent to B" | Type equivalence | `Bool ≃ 𝟚` | 3 |
| `A ≅ B` | "A isomorphic to B" | Isomorphism | `ℤ/2ℤ ≅ Bool` | 2 |
| `≈` | "approximately equal" | Approximate equality | `π ≈ 3.14159` | 11 |

### Inequalities

| Symbol | Pronunciation | Meaning | Example |
|--------|--------------|---------|---------|
| `≤` | "less than or equal" | Less or equal | `x ≤ y` |
| `<` | "less than" | Strictly less | `x < y` |
| `≥` | "greater than or equal" | Greater or equal | `x ≥ y` |
| `>` | "greater than" | Strictly greater | `x > y` |

### Number Systems

| Symbol | Pronunciation | Meaning |
|--------|--------------|---------|
| `ℕ` | "naturals" / "N" | Natural numbers (0,1,2,...) |
| `ℤ` | "integers" / "Z" | Integers (...,-1,0,1,...) |
| `ℚ` | "rationals" / "Q" | Rational numbers |
| `ℝ` | "reals" / "R" | Real numbers |
| `ℂ` | "complex" / "C" | Complex numbers |

### Special Spaces

| Symbol | Pronunciation | Meaning | Chapter |
|--------|--------------|---------|---------|
| `S¹` | "S one" / "circle" | Circle (1-sphere) | 3 |
| `S²` | "S two" / "2-sphere" | 2-sphere (surface) | 3 |
| `Sⁿ` | "n-sphere" | n-dimensional sphere | 3 |
| `T²` | "torus" | 2-dimensional torus | 3 |

### Limits

| Symbol | Pronunciation | Meaning |
|--------|--------------|---------|
| `lim_{x→a}` | "limit as x approaches a" | Limit |
| `lim_{n→∞}` | "limit as n goes to infinity" | Limit at infinity |
| `sup` / `⨆` | "supremum" / "least upper bound" | Supremum |
| `inf` / `⨅` | "infimum" / "greatest lower bound" | Infimum |

### Sums and Products

| Symbol | Pronunciation | Meaning | Example |
|--------|--------------|---------|---------|
| `Σ` | "sum" | Summation | `Σᵢ₌₁ⁿ i` |
| `∏` | "product" | Product | `∏ᵢ₌₁ⁿ i` |

### Greek Alphabet Quick Reference

Common in mathematics:

| Letter | Name | Common use |
|--------|------|------------|
| `α` | alpha | Angles, coefficients |
| `β` | beta | Angles, coefficients |
| `γ` | gamma | Paths, Euler's constant |
| `δ` | delta | Small differences, Dirac delta |
| `ε` | epsilon | Small positive quantities |
| `θ` | theta | Angles |
| `λ` | lambda | Functions, eigenvalues |
| `μ` | mu | Measures, means |
| `π` | pi | Pi constant (3.14159...) |
| `σ` | sigma | Standard deviation, summation |
| `φ` | phi | Golden ratio, angles |
| `ω` | omega | Differential forms, frequencies |
| `Γ` | Gamma | Contexts, Gamma function |
| `Δ` | Delta | Difference operator, Laplacian |
| `Π` | Pi | Dependent products |
| `Σ` | Sigma | Dependent sums, summation |
| `Ω` | Omega | Loop spaces, differential forms |

---

## Typography Conventions

### Bold
`**bold**` — Emphasis, definitions

### Italic
`*italic*` — Variables, book/paper titles

### Code
`` `code` `` — Type theory syntax, code

### Math
`$math$` — Mathematical expressions

---

## How to Type These Symbols

### In LaTeX
Most symbols have standard LaTeX commands:
- `\to` → `→`
- `\Rightarrow` → `⇒`
- `\mathbb{R}` → `ℝ`
- `\partial` → `∂`
- `\nabla` → `∇`
- `\int` → `∫`
- `\sum` → `Σ`
- `\prod` → `∏`

### In Unicode (Copy-Paste)
All symbols in this guide can be copied directly.

### In SCTT Code
The SCTT implementation accepts both ASCII and Unicode:
- `->` or `→` for function types
- `forall` or `∀` for universal quantification
- `exists` or `∃` for existential quantification
- `Real` or `ℝ` for real numbers

---

## Common Confusions

### `→` vs `⇒`
- `→` : Function type constructor (type-level)
- `⇒` : Implication (proposition-level)
- In SCTT, these are often the same due to Curry-Howard!

### `≡` vs `=` vs `≃`
- `≡` : Definitional/judgmental equality (built into system)
- `=` : Often used informally for mathematical equality
- `≃` : Type equivalence (can be transported along)

### `Π` vs `Σ` vs `∫`
- `Π` : Dependent product (type theory)
- `Σ` : Dependent sum (type theory) or summation (mathematics)
- `∫` : Integration (calculus)

### `D` vs `∂` vs `d`
- `D` : Total derivative operator `D[f]`
- `∂` : Partial derivative `∂f/∂x`
- `d` : Exterior derivative `d ω` or differential `dx`

---

## Further Reading

- **Chapter 2**: Introduction to type theory notation
- **Chapter 3**: Cubical and path notation
- **Chapter 4-5**: Smooth and differential notation
- **Online**: Unicode Math Symbol reference (unicode.org)
- **LaTeX**: The Comprehensive LaTeX Symbol List (CTAN)

---

*Return to: [Table of Contents](./SUMMARY.md)*
