# Appendix C: Solutions to Selected Exercises

> "The only way to learn mathematics is to do mathematics." — Paul Halmos

This appendix provides detailed solutions to selected exercises throughout the book. Solutions are chosen to:
- Illustrate important techniques
- Clarify subtle concepts
- Provide worked examples for self-study

**Not all exercises are solved here** — many are left for you to practice independently!

## Difficulty Ratings
- ★ : Straightforward, tests basic understanding
- ★★ : Moderate, requires combining concepts
- ★★★ : Challenging, needs creative insight
- ★★★★ : Research-level, open-ended

---

## Chapter 2: Type Theory Foundations

### Understanding Check

**Exercise 2.1** (★): *What's the difference between `Type₀` and `Type₁`?*

**Solution**:
- `Type₀` is the universe of small types (like `Nat`, `Bool`, function types between small types)
- `Type₁` is a larger universe that contains `Type₀` itself
- We have `Type₀ : Type₁`, but NOT `Type₀ : Type₀` (that would cause Russell's paradox)

**Key insight**: The hierarchy prevents self-reference paradoxes while allowing us to reason about types themselves.

**Example**:
```sctt
Bool : Type₀          -- Bool is a small type
Nat → Bool : Type₀    -- Functions between small types are small
Type₀ : Type₁         -- Type₀ itself is a large type
(Type₀ → Type₀) : Type₁  -- Type-level functions are large
```

---

**Exercise 2.2** (★): *Give an example of a type that depends on a value.*

**Solution**:
```sctt
Vec : Type → Nat → Type
Vec A n = -- vector of n elements of type A

-- Example usage:
Vec Bool 3 : Type₀    -- Type of 3-element boolean vectors
-- This is ["true", "false", "true"] for instance

-- The type Vec Bool n depends on the *value* n
```

**More examples**:
```sctt
-- 1. Fin n: numbers less than n
Fin : Nat → Type
Fin 0 = ⊥ (empty)
Fin (suc n) = Unit ⊎ Fin n

-- 2. Matrices
Matrix : Nat → Nat → Type → Type
Matrix m n A = Vec (Vec A n) m  -- m×n matrix of A's

-- 3. Bounded natural numbers
BoundedNat : Nat → Type
BoundedNat bound = Σ (n : Nat), n < bound
```

**Why this matters**: Dependent types let us encode invariants (like vector length) directly in types, catching errors at compile time!

---

### Programming Practice

**Exercise 2.3** (★★): *Define a type `Fin n` representing natural numbers less than n.*

**Solution**:
```sctt
-- Inductive definition
data Fin : Nat → Type where
  fzero : {n : Nat} → Fin (suc n)
  fsuc  : {n : Nat} → Fin n → Fin (suc n)

-- Examples:
-- Fin 0 has no inhabitants (no numbers less than 0)
-- Fin 1 = {fzero} (only 0 < 1)
-- Fin 2 = {fzero, fsuc fzero} (0 and 1 are < 2)
-- Fin 3 = {fzero, fsuc fzero, fsuc (fsuc fzero)} (0,1,2 < 3)

-- Intuition: Fin n has exactly n elements
card_Fin : (n : Nat) → count (Fin n) ≡ n
card_Fin zero = refl
card_Fin (suc n) = ap suc (card_Fin n)
```

**Alternative peano-style definition**:
```sctt
Fin' : Nat → Type
Fin' zero = ⊥  -- empty type
Fin' (suc n) = Unit ⊎ Fin' n  -- either 0 or (1 + something less than n)
```

---

**Exercise 2.4** (★★): *Implement vector concatenation with correct length.*

**Solution**:
```sctt
-- Type signature encodes correctness!
_++_ : {A : Type} {m n : Nat} →
       Vec A m → Vec A n → Vec A (m + n)

-- Implementation by pattern matching
[] ++ ys = ys
(x :: xs) ++ ys = x :: (xs ++ ys)

-- Proof that this has the right type:
-- Case []:
--   [] : Vec A 0
--   ys : Vec A n
--   Result: ys : Vec A n = Vec A (0 + n) ✓
--
-- Case (x :: xs):
--   x :: xs : Vec A (suc m)
--   xs : Vec A m
--   IH: xs ++ ys : Vec A (m + n)
--   Result: x :: (xs ++ ys) : Vec A (suc (m + n))
--                            = Vec A (suc m + n) ✓
```

**Testing**:
```sctt
v1 : Vec Nat 2
v1 = [1, 2]

v2 : Vec Nat 3
v2 = [3, 4, 5]

result : Vec Nat 5
result = v1 ++ v2  -- [1, 2, 3, 4, 5]
-- Note: The length 5 is computed automatically!
```

---

### Proofs

**Exercise 2.5** (★★★): *Prove that addition of natural numbers is associative.*

**Solution**:
```sctt
-- Statement
+-assoc : (m n p : Nat) → (m + n) + p ≡ m + (n + p)

-- Proof by induction on m
+-assoc zero n p =
  -- Goal: (0 + n) + p ≡ 0 + (n + p)
  -- LHS: (0 + n) + p = n + p (by definition of +)
  -- RHS: 0 + (n + p) = n + p (by definition of +)
  refl

+-assoc (suc m) n p =
  -- Goal: (suc m + n) + p ≡ suc m + (n + p)
  -- Expand definitions:
  --   (suc m + n) + p
  -- = suc (m + n) + p
  -- = suc ((m + n) + p)
  --
  --   suc m + (n + p)
  -- = suc (m + (n + p))
  --
  -- So we need: suc ((m + n) + p) ≡ suc (m + (n + p))
  ap suc (+-assoc m n p)
  -- By IH: (m + n) + p ≡ m + (n + p)
  -- Apply suc to both sides ✓
```

**Explanation of each step**:
1. **Base case** (m = 0): Both sides reduce to `n + p` by definition
2. **Inductive step** (m = suc m'):
   - IH: `(m' + n) + p ≡ m' + (n + p)`
   - Use `ap suc` to lift equality through successor
   - Result: `suc ((m' + n) + p) ≡ suc (m' + (n + p))`

---

## Chapter 3: Cubical Structure

### Conceptual Understanding

**Exercise 3.1** (★★): *Why can't the interval type `I` be defined as an ordinary inductive type?*

**Solution**:

If we tried to define I inductively:
```sctt
data I : Type where
  i0 : I
  i1 : I
```

**Problems**:
1. **Too discrete**: This only has 2 elements, but we need a *continuum* of points
2. **No operations**: We can't define `i ∧ j`, `i ∨ j`, `~i` meaningfully
3. **No computation**: Path application `p @ i` wouldn't compute smoothly

**What we actually need**:
- I is a **pretype**, not a type
- It represents *dimension* or *direction*, not values
- Elements of I are *symbolic* (like variables i, j)
- Operations (∧, ∨, ~) form a de Morgan algebra

**Analogy**: I is like a coordinate axis in geometry — you don't enumerate its points, you use it to parametrize curves.

**In practice**:
```sctt
-- We don't write:
i : I
i = i0  -- Wrong! i is a variable, not a value

-- We write:
path : Path A x y
path i = ...  -- i is a bound variable ranging over I
```

---

**Exercise 3.2** (★★): *What's the computational content of a path?*

**Solution**:

A path `p : Path A x y` is computationally:
1. **A function** from the interval: `p : I → A`
2. **With boundary conditions**:
   - `p i0` computes to `x`
   - `p i1` computes to `y`
3. **That interpolates continuously** between x and y

**Example**:
```sctt
-- Path in natural numbers
nat_path : Path Nat 2 5
nat_path i =
  case i of
    i0 → 2
    i1 → 5
    -- For intermediate i, some interpolation
    _  → 3  -- (simplified; real version would be smooth)

-- We can apply it:
nat_path i0  ≡  2  -- by computation
nat_path i1  ≡  5  -- by computation
```

**Computational behavior**:
```sctt
-- Path composition p ∙ q computes by:
(p ∙ q) @ i =
  if i ≤ 1/2 then p @ (2*i)      -- first half: traverse p
             else q @ (2*i - 1)   -- second half: traverse q
```

**Key insight**: Paths are *programs* that witness equality by providing a continuous deformation from one value to another.

---

### Path Construction

**Exercise 3.3** (★★): *Define a path from `n + 0` to `n` for any `n : Nat`.*

**Solution**:
```sctt
+-zero : (n : Nat) → Path Nat (n + 0) n

-- By induction on n
+-zero zero =
  -- Goal: Path Nat (0 + 0) 0
  --     = Path Nat 0 0
  refl  -- Reflexivity suffices

+-zero (suc n) =
  -- Goal: Path Nat (suc n + 0) (suc n)
  --     = Path Nat (suc (n + 0)) (suc n)
  --
  -- IH: Path Nat (n + 0) n
  --
  -- We need to apply suc to both endpoints
  λ i → suc (+-zero n @ i)
  -- This gives: Path Nat (suc (n + 0)) (suc n) ✓
```

**Alternative using ap**:
```sctt
+-zero' : (n : Nat) → Path Nat (n + 0) n
+-zero' zero = refl
+-zero' (suc n) = ap suc (+-zero' n)
-- ap lifts paths through functions
```

**Visualization**:
```
For n = 3:
    (3 + 0) ——————→ 3
         3          3

Path continuously deforms left side to right
```

---

**Exercise 3.4** (★★★): *Construct a non-trivial path in `Path Type Bool Bool`.*

**Solution**:

The simplest non-trivial path is the "negation path":

```sctt
not_path : Path Type Bool Bool
not_path = ua not_equiv

where
  not_equiv : Bool ≃ Bool
  not_equiv = (not, not_is_equiv)

  not : Bool → Bool
  not true = false
  not false = true

  not_is_equiv : isEquiv not
  not_is_equiv = -- proof that not is an equivalence
    ( not           -- inverse (not is its own inverse)
    , λ b → refl    -- not (not b) ≡ b
    , λ b → refl    -- not (not b) ≡ b
    )
```

**What makes this non-trivial?**
- `refl : Path Type Bool Bool` is the trivial path (identity)
- `not_path` swaps true and false as we traverse the interval

**Computational behavior**:
```sctt
transport not_path true  ≡  false  -- by computation
transport not_path false ≡  true

-- Along the path, Bool "twists"
not_path i0 ≡ Bool (with identity)
not_path i1 ≡ Bool (with negation)
```

**Why this matters**: Shows that types can be equal in multiple ways! This is the essence of univalence.

---

### Proofs Using Paths

**Exercise 3.5** (★★★): *Prove that `sym (sym p) = p` for any path `p`.*

**Solution**:
```sctt
sym_sym : {A : Type} {x y : A} (p : Path A x y) →
          Path (Path A x y) (sym (sym p)) p

sym_sym p = λ i j → p (i ∧ j ∨ ~i ∧ ~j)
```

**Explanation**:

We need a 2-dimensional path (a square):
```
        refl
    x ————————→ x
    |            |
sym p|            | sym (sym p)
    |            |
    ↓     p      ↓
    y ————————→ y
        refl
```

The expression `i ∧ j ∨ ~i ∧ ~j` creates this square:
- When `i=i0`: `i0 ∧ j ∨ i1 ∧ ~j = ~j`, giving `p (~j) = sym p j` ✓
- When `i=i1`: `i1 ∧ j ∨ i0 ∧ ~j = j`, giving `p j` ✓
- When `j=i0`: `i ∧ i0 ∨ ~i ∧ i1 = ~i`, giving `p (~i)` (but we're at endpoint x)
- When `j=i1`: `i ∧ i1 ∨ ~i ∧ i0 = i`, giving `p i` (endpoint y)

**Alternative proof using hcomp**:
```sctt
sym_sym' : {A : Type} {x y : A} (p : Path A x y) →
           sym (sym p) ≡ p
sym_sym' p = λ i j → hcomp (∂ i ∨ ∂ j) (λ { k →
  { (i = i0) → p (~j)     -- Left: sym p
  ; (i = i1) → p j        -- Right: p
  ; (j = i0) → x          -- Top: refl
  ; (j = i1) → y          -- Bottom: refl
  }}) (p (... computation ...))
```

---

## Chapter 5: Differential Operators

### Basic Differentiation

**Exercise 5.1** (★★): *Compute D[λ x → log(sin(x² + 1))].*

**Solution**:

Using the chain rule repeatedly:

**Step 1**: Identify the composition
```
f(x) = log(sin(x² + 1))
     = log ∘ sin ∘ (λ x → x² + 1)
```

**Step 2**: Apply chain rule
```sctt
D[f] = D[log] ∘ (sin ∘ g) × D[sin ∘ g]

where g(x) = x² + 1
```

**Step 3**: Compute each derivative
```sctt
-- D[log u] = 1/u
-- D[sin v] = cos v
-- D[x² + 1] = 2x

D[f](x) = (1 / sin(x² + 1)) × cos(x² + 1) × 2x
        = (2x × cos(x² + 1)) / sin(x² + 1)
        = 2x × cot(x² + 1)
```

**In SCTT notation**:
```sctt
f : C∞(ℝ, ℝ)
f x = log (sin (x² + 1))

f' : C∞(ℝ, ℝ)
f' = D[f]
f' x = 2*x * (cos (x² + 1)) / (sin (x² + 1))

-- Or equivalently:
f' x = 2*x * cot (x² + 1)
```

**Verification** (at x = 0):
```
f(0) = log(sin(1))
f'(0) = 0 × cot(1) = 0 ✓
-- Makes sense: log(sin(x²+1)) has horizontal tangent at x=0 by symmetry
```

---

**Exercise 5.2** (★★★): *Find all critical points of f(x,y) = x³ - 3xy + y³.*

**Solution**:

**Step 1**: Compute the gradient
```sctt
∇f = (∂f/∂x, ∂f/∂y)

∂f/∂x = 3x² - 3y
∂f/∂y = -3x + 3y²
```

**Step 2**: Set gradient to zero
```
3x² - 3y = 0    →  y = x²
-3x + 3y² = 0   →  x = y²
```

**Step 3**: Solve the system
Substitute y = x² into x = y²:
```
x = (x²)² = x⁴
x⁴ - x = 0
x(x³ - 1) = 0
```

Solutions: x = 0 or x = 1

**Case 1**: x = 0 ⇒ y = 0² = 0
**Critical point**: (0, 0)

**Case 2**: x = 1 ⇒ y = 1² = 1
**Critical point**: (1, 1)

**Step 4**: Classify using Hessian
```sctt
Hessian f = [∂²f/∂x²    ∂²f/∂x∂y]
            [∂²f/∂y∂x    ∂²f/∂y²  ]

H = [6x   -3]
    [-3   6y]
```

**At (0,0)**:
```
H(0,0) = [0   -3]
         [-3   0]

det(H) = 0 × 0 - (-3)(-3) = -9 < 0
```
**Saddle point** (det < 0)

**At (1,1)**:
```
H(1,1) = [6   -3]
         [-3   6]

det(H) = 36 - 9 = 27 > 0
tr(H) = 12 > 0
```
**Local minimum** (det > 0 and trace > 0)

**Answer**:
- Critical point at (0, 0): **saddle point**
- Critical point at (1, 1): **local minimum**

---

### Integration

**Exercise 5.3** (★★): *Evaluate ∫₀^π sin²(x) dx using SCTT.*

**Solution**:

**Method 1**: Using the identity sin²(x) = (1 - cos(2x))/2

```sctt
∫₀^π sin²(x) dx = ∫₀^π (1 - cos(2x))/2 dx
                = (1/2) ∫₀^π (1 - cos(2x)) dx
                = (1/2) [x - sin(2x)/2]₀^π
                = (1/2) [(π - 0) - (0 - 0)]
                = π/2
```

**Method 2**: Integration by parts

Let u = sin(x), dv = sin(x)dx
Then du = cos(x)dx, v = -cos(x)

```sctt
∫ sin²(x) dx = -sin(x)cos(x) + ∫ cos²(x) dx
             = -sin(x)cos(x) + ∫ (1 - sin²(x)) dx
             = -sin(x)cos(x) + x - ∫ sin²(x) dx

2∫ sin²(x) dx = x - sin(x)cos(x)
∫ sin²(x) dx = (x - sin(x)cos(x))/2
```

Evaluating from 0 to π:
```
[(π - 0)/2] - [(0 - 0)/2] = π/2
```

**In SCTT**:
```sctt
sin_squared_integral : ℝ
sin_squared_integral = ∫ 0 pi (λ x → sin x * sin x)

-- Computes to:
sin_squared_integral ≡ pi / 2  -- by FTC
```

**Answer**: π/2

---

### Differential Forms

**Exercise 5.4** (★★): *Express div, grad, and curl using exterior derivatives.*

**Solution**:

These classical vector calculus operators are all special cases of the exterior derivative `d`!

**Gradient** (grad):
```sctt
-- For f : C∞(ℝⁿ, ℝ), a 0-form
grad f = df  -- This is a 1-form

-- In coordinates:
df = (∂f/∂x₁)dx₁ + (∂f/∂x₂)dx₂ + ... + (∂f/∂xₙ)dxₙ

-- Vector field version (using musical isomorphism):
∇f = ♯(df) = (∂f/∂x₁, ∂f/∂x₂, ..., ∂f/∂xₙ)
```

**Divergence** (div):
```sctt
-- For F = (F₁, F₂, F₃) : Vector field on ℝ³
-- Convert to 2-form using Hodge star:
α = ⋆(F♭) = F₁ dy∧dz + F₂ dz∧dx + F₃ dx∧dy

-- Then divergence is:
div F = ⋆(d α)

-- Explicitly:
d α = (∂F₁/∂x + ∂F₂/∂y + ∂F₃/∂z) dx∧dy∧dz
div F = ∂F₁/∂x + ∂F₂/∂y + ∂F₃/∂z
```

**Curl** (curl):
```sctt
-- For F = (F₁, F₂, F₃) : Vector field on ℝ³
-- Convert to 1-form:
α = F♭ = F₁dx + F₂dy + F₃dz

-- Then curl is:
curl F = ⋆(d α)

-- Explicitly:
d α = (∂F₃/∂y - ∂F₂/∂z) dy∧dz +
      (∂F₁/∂z - ∂F₃/∂x) dz∧dx +
      (∂F₂/∂x - ∂F₁/∂y) dx∧dy

curl F = (∂F₃/∂y - ∂F₂/∂z, ∂F₁/∂z - ∂F₃/∂x, ∂F₂/∂x - ∂F₁/∂y)
```

**Summary**:
```
grad f  =  ♯(df)           (d : Ω⁰ → Ω¹)
curl F  =  ♯(⋆(d(F♭)))    (d : Ω¹ → Ω²)
div F   =  ⋆(d(⋆(F♭)))    (d : Ω² → Ω³)
```

**Key insight**: All are applications of d, the *universal* differential operator!

---

## Chapter 4: Smooth Types

### Conceptual

**Exercise 4.1** (★★): *Why do we need the infinitesimal object 𝔻?*

**Solution**:

The infinitesimal object 𝔻 is the key to **synthetic differential geometry** in SCTT.

**Classical approach** (via limits):
```
f'(x) = lim_{h→0} [f(x+h) - f(x)]/h
```
Problems:
- Limits are non-computational
- ε-δ proofs are complex
- Doesn't compose well

**Synthetic approach** (via 𝔻):
```sctt
𝔻 = {ε : I → ℝ | ε i0 ≡ 0 ∧ ε² ≡ 0}
-- Infinitesimal paths starting at 0 that vanish quadratically

-- Kock-Lawvere axiom:
∀ (f : C∞(ℝ, ℝ)) (x : ℝ) (ε : 𝔻),
  f(x + ε) = f(x) + f'(x)·ε
```

**Why this is better**:
1. **Computational**: f'(x) is defined directly, not as a limit
2. **Composable**: Chain rule becomes function composition
3. **Type-safe**: Smoothness is enforced by types
4. **Automatic**: Derivatives compute without symbolic manipulation

**Example**:
```sctt
-- Square function
square : C∞(ℝ, ℝ)
square x = x²

-- Its derivative via KL:
square (x + ε) = (x + ε)²
               = x² + 2xε + ε²
               = x² + 2xε        -- since ε² = 0 in 𝔻

∴ square'(x) = 2x  -- Read off the coefficient of ε
```

**Key insight**: 𝔻 gives us "infinitely small but not zero" quantities that we can compute with, avoiding limits entirely!

---

## Tips for Solving Exercises

### General Strategy
1. **Read carefully**: Understand what's being asked
2. **Identify the type**: What type should your answer have?
3. **Look for patterns**: Similar to examples in the text?
4. **Start simple**: Try the base case or a simple example first
5. **Use the tools**: What operations/lemmas are available?
6. **Check your work**: Does it type-check? Make sense?

### Common Techniques
- **Induction**: For recursive types (Nat, List, etc.)
- **Pattern matching**: Break down structured data
- **Path algebra**: Compose, invert, apply paths
- **Chain rule**: For composite derivatives
- **Substitution**: In integrals, change of variables

### When Stuck
1. **Simplify**: Try a concrete example
2. **Draw pictures**: Especially for paths and cubes
3. **Ask**: What structure does this preserve?
4. **Look ahead**: The concept might be explained later
5. **Take a break**: Fresh eyes often help!

---

## Further Practice

**Excellent resources for more exercises**:

1. **Type Theory**:
   - Benjamin Pierce, *Types and Programming Languages*
   - Exercises throughout the HoTT Book

2. **Cubical Theory**:
   - Cubical Agda tutorials
   - 1Lab formalization exercises

3. **Differential Geometry**:
   - Lee, *Introduction to Smooth Manifolds* (see Bibliography)
   - Tu, *An Introduction to Manifolds*

4. **Integration**:
   - Try implementing solutions in the SCTT proof assistant
   - Formalize proofs from the textbook
   - Create your own examples

---

*Return to: [Table of Contents](./SUMMARY.md)*

**Note**: If you find errors in these solutions or have alternative approaches, please contribute to the book repository!