# ASCII Diagrams for Key Concepts

> **Note**: These diagrams appear throughout the book chapters. They're collected here for reference and can be copied into chapters as needed.

---

## Chapter 2: Type Theory Foundations

### Universe Hierarchy

```
                Type₃
                  ↑
                  │
                Type₂
                  ↑
                  │
      ┌──────── Type₁ ────────┐
      │           ↑           │
      │           │           │
   Type₀→Type₀  Type₀      List Type₀
      ↑           ↑           ↑
      │           │           │
    Nat         Bool         Vec Nat 5

Legend:
  A ↑ B  means "A : B" (A has type B)
  No self-containing types!
```

### Dependent Function Type (Pi Type)

```
    Γ, x : A ⊢ B(x) : Type
  ───────────────────────────
    Γ ⊢ (x : A) → B(x) : Type


  For each input x : A,
  we get output of type B(x)

  Example: Vec : Nat → Type
           Vec 0 = Empty vector type
           Vec 3 = Three-element vector type
```

### Curry-Howard Correspondence

```
  Logic                 Type Theory
  ─────                 ───────────

  P ∧ Q        ⟷        P × Q
  P ∨ Q        ⟷        P ⊎ Q
  P → Q        ⟷        P → Q
  ∀x. P(x)     ⟷        Π (x : A), P x
  ∃x. P(x)     ⟷        Σ (x : A), P x
  ⊥ (false)    ⟷        ⊥ (empty type)
  ⊤ (true)     ⟷        ⊤ (unit type)

  Proof of P   ⟷        Term of type P
```

---

## Chapter 3: Cubical Structure

### The Interval Type I

```
   i0 ────────────────── i1
    0                     1

   The interval I with endpoints

   Operations:
   i ∧ j   (meet/minimum)
   i ∨ j   (join/maximum)
   ~i      (reversal: 1-i)
```

### Path as Function from I

```
        Path A x y

        x ────────────> y
       i0      p i     i1

   p : I → A
   p i0 ≡ x
   p i1 ≡ y

   At any i : I, we have p i : A
```

### Path Composition (Sequential)

```
        p           q
    x ───────> y ───────> z
    │                     │
    │    p ∙ q           │
    └─────────────────────┘

  Composition fills the bottom:

      x ─────p────→ y
      │             │
   refl             q
      │             │
      x ─────────→  z
          p ∙ q
```

### Square (2-Dimensional Path)

```
         p
     a ─────→ b
     │        │
   left     right
     │        │
     c ─────→ d
         q

   A square is a path between paths:
   Square : Path (Path A c d) left right
```

### Kan Composition (Open Box with Lid)

```
   Given an open box (5 faces of cube):

         ?  ← (missing top)
        ╱│╲
       ╱ │ ╲
      ╱  │  ╲
     └───┴───┘
    (5 faces given)

   Kan filling: computes the missing face

   Result: closed cube
         ■
        ╱│╲
       ╱ │ ╲
      ╱  │  ╲
     └───┴───┘
```

### Univalence via Glue

```
  A ≃ B  ───ua───>  Path Type A B
   │                      │
   │                      │
equivalence           type equality

   transport : Path Type A B → (A → B)

   Crucially:
   transport (ua e) ≡ e.fst
   (Univalence computes!)
```

---

## Chapter 4: Smooth Types

### Kock-Lawvere Axiom (Infinitesimals)

```
  Infinitesimal 𝔻 = {ε : ℝ | ε² = 0}

  For any f : ℝ → ℝ,
  f(x + ε) = f(x) + f'(x)·ε
             ─┬──   ───┬───
            value  derivative

  Graphically:

    f(x+ε)  ○
           ╱│
          ╱ │ f'(x)·ε
         ╱  │
  f(x)  ○───┘
        x  ε

  The function is LINEAR on infinitesimals!
```

### Tangent Bundle

```
  Tangent Bundle T M

  For manifold M:

    T M = {(x, v) | x ∈ M, v ∈ TₓM}
          ─┬─────   ─┬─   ─────┬────
         point    space  tangent vector

  Visualization (circle):

       ↑ (tangent vector at top)
       │
    ───●───  S¹


  T S¹ = S¹ × ℝ
  (each point has 1D tangent space)
```

### Differential of a Map

```
  Given f : M → N

  Differential df : T M → T N

       T M ────df───→ T N
        │              │
       π│              │π
        ↓              ↓
        M ─────f────→  N

  df(x, v) = (f(x), Df[x](v))
            ────┬───  ─────┬────
           image of  pushforward of
             point    tangent vector
```

### Smooth Path and Velocity

```
  Smooth path γ : I → M

  Velocity γ'(t) ∈ Tγ(t) M

    M:  γ(t₀) ──→ γ(t₁) ──→ γ(t₂)
             ↗       ↗       ↗
        γ'(t₀)  γ'(t₁)  γ'(t₂)
         (tangent vectors along path)
```

---

## Chapter 5: Differential Operators

### Gradient, Divergence, Curl (Unified)

```
  Exterior Derivative d unifies all:

  0-forms (functions)
      ↓ d
  1-forms (gradient)
      ↓ d
  2-forms (curl)
      ↓ d
  3-forms (divergence)


  In ℝ³:

  f : ℝ³ → ℝ
    ↓ d
  df : Ω¹(ℝ³)     (gradient)
    ↓ d
  d(df) : Ω²(ℝ³)  (curl of gradient = 0)
    ↓ d
  d(d(df)) = 0    (div curl = 0)
```

### Chain Rule (Compositional)

```
  f : A → B    g : B → C

  D[g ∘ f] = Dg ∘ Df

  Diagram:

    A ──f──→ B ──g──→ C
    │        │        │
   Df       Dg    D(g∘f)
    │        │        │
    ↓        ↓        ↓
   TA ─Df─→ TB ─Dg─→ TC

   Bottom path = D(g ∘ f)
   Commutes definitionally!
```

### Stokes' Theorem

```
  ∫∂Ω ω = ∫Ω dω

  Boundary integral = Interior integral

  Example (2D):

      ∂Ω (boundary curve)
      ┌─────────┐
      │    Ω    │  (region)
      │  (interior) │
      └─────────┘

  ∫∂Ω ω = ∮ (line integral)

  ∫Ω dω = ∬ (area integral)

  Green's theorem, divergence theorem, etc.
  are all special cases!
```

---

## Chapter 11: Scientific Computing

### Automatic Differentiation (Forward Mode)

```
  Dual numbers: a + b·ε  where ε² = 0

  Computation graph:

    x ──→ sin ──→ (·) ──→ output
    ↓      ↓       ↑
   x+ε   sin(x+ε)
         = sin(x) + cos(x)·ε

  Forward pass: compute value
  Simultaneous: compute derivative

  Complexity: O(n) for n inputs
```

### Runge-Kutta Integration

```
  RK4 time step from yₙ to yₙ₊₁:

  yₙ ──k₁──> ──k₂──> ──k₃──> ──k₄──> yₙ₊₁
   │   ↓      ↓       ↓       ↓      │
   │  slope  slope   slope   slope   │
   │   at     at      at      at     │
   │   tₙ   tₙ+h/2  tₙ+h/2   tₙ+h    │
   │                                  │
   └──────── weighted average ────────┘

  yₙ₊₁ = yₙ + (h/6)(k₁ + 2k₂ + 2k₃ + k₄)

  Error: O(h⁵) per step
        O(h⁴) globally
```

### Parallel Speedup

```
  Sequential:  ████████████████  (16 units)

  2 cores:     ████████
               ████████          (8 units each)

  4 cores:     ████
               ████
               ████
               ████              (4 units each)

  Speedup = T_sequential / T_parallel

  Ideal: S(p) = p  (linear)
  Actual: S(p) = p / (1 + overhead)

  Amdahl's Law limits speedup
```

---

## General Diagrams

### Category with Functors

```
      Category C          Category D
    ┌──────────┐        ┌──────────┐
    │ A  ──f─→ B │       │ F(A) ──F(f)─→ F(B) │
    │          │   ──F──→  │                    │
    │ C  ──g─→ D │       │ F(C) ──F(g)─→ F(D) │
    └──────────┘        └──────────┘

    Functor F preserves:
    - Objects: A ↦ F(A)
    - Morphisms: f ↦ F(f)
    - Composition: F(g ∘ f) = F(g) ∘ F(f)
    - Identities: F(id) = id
```

### Adjunction

```
     C ←───F─── D
       ─→─G─→

    F ⊣ G  (F is left adjoint to G)

    Natural bijection:
    Hom_D(F(A), B) ≅ Hom_C(A, G(B))

    Example:
    Free ⊣ Forgetful
    (between Set and Group)
```

### Commutative Diagram

```
       A ────f───→ B
       │           │
       │g          │h
       │           │
       ↓           ↓
       C ────k───→ D

    Commutes when:
    h ∘ f = k ∘ g

    All paths with same
    start and end are equal
```

---

## Usage Guidelines

Each diagram can be copied into relevant chapter sections. Formatting:

    ```
    (diagram content here)
    ```

Tips:
- Use monospaced font (code blocks)
- Keep width ≤ 80 characters
- Use box-drawing characters: ─ │ ┌ ┐ └ ┘ ├ ┤ ┬ ┴ ┼
- Use arrows: → ← ↑ ↓ ⟶ ⟵ ⇒ ⇐
- Use math symbols: ∀ ∃ ∈ ⊢ ≡ ≃ ⊎ ×

---

*These diagrams are referenced throughout the book. See individual chapters for context.*
