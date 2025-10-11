# Chapter 3: Cubical Structure

> "In mathematics, the art of proposing a question must be held of higher value than solving it." — Georg Cantor
>
> "In cubical type theory, paths are not just proofs of equality—they are the very fabric of mathematical space."

## Introduction

Traditional type theory treats equality as a mere proposition: either two things are equal or they aren't. But mathematics is richer than this. The way two things are equal matters. A circle can be equal to itself in infinitely many ways—by rotating it through any angle. Cubical type theory captures this richness by making paths first-class citizens.

This chapter introduces the revolutionary cubical structure that gives SCTT its power. We'll see how paths become computational objects, how spaces emerge from types, and how the univalence axiom becomes a theorem rather than an axiom. The foundations established in [Chapter 2](./chapter_02.md) are essential background, and this cubical structure will be enriched with smooth geometry in [Chapter 4](./chapter_04.md).

---

### ⚡ Quick Start: What You'll Learn

**If you only have 20 minutes**, read:
- [§3.1 The Interval Type](#interval) — The dimension `I`
- [§3.2 Path Types](#paths) — Paths as functions from `I`
- [§3.5 Univalence](#univalence) — Equivalent types are equal

**Core takeaways**:
- The interval type `I` has endpoints `i0` and `i1`
- `Path A x y` is a continuous function `I → A` from `x` to `y`
- Paths can be composed like paths in topology
- **Univalence**: `A ≃ B → Path Type A B` (equivalence implies equality)
- Unlike axioms, univalence **computes** via Glue types

**Prerequisites**: [Chapter 2](./chapter_02.md) — Dependent type theory

**Time**: 3-4 hours for full chapter with exercises

> **🔬 Running Example**: Particle trajectories are paths through space. Path composition models sequential motion. See [running example §Chapter 3](./running_example.md#chapter-3-cubical-structure).

---

### Mathematical Foundations

Cubical type theory is based on a model in cubical sets—presheaves on the category of cubes with connections and symmetries. The key insight is that:

1. **Cubes model higher equalities**: An n-cube represents an n-dimensional path
2. **Composition is geometric**: Kan filling operations give computational content
3. **Univalence computes**: Via Glue types, equivalence becomes equality

## 3.1 The Interval Type {#interval}

### The Computational Interval

At the heart of cubical type theory lies a deceptively simple type:

```sctt
-- The interval type I
I : PreType  -- Not a proper type!

-- With two endpoints
i0 : I  -- left endpoint (0)
i1 : I  -- right endpoint (1)

-- And the crucial property:
-- I behaves like the real interval [0,1]
```

But `I` is not an ordinary type—it's a pretype that represents "dimension" or "direction".

#### De Morgan Algebra Structure

The interval forms a de Morgan algebra:

```sctt
-- Operations on I
_∧_ : I → I → I  -- meet (minimum)
_∨_ : I → I → I  -- join (maximum)
~_ : I → I        -- involution (1 - i)

-- De Morgan laws:
-- ~(i ∧ j) = ~i ∨ ~j
-- ~(i ∨ j) = ~i ∧ ~j
-- ~~i = i
-- i ∧ (j ∨ k) = (i ∧ j) ∨ (i ∧ k)
```

#### Why Not an Inductive Type?

The interval cannot be an inductive type because:
1. It would only have two distinct elements (i0 and i1)
2. We need a continuum of points
3. Interval variables must be symbolic, not concrete

```sctt
-- We can have expressions involving interval variables
-- If i : I, then i represents a point in the interval

-- Examples of interval expressions:
-- i ∧ j     (minimum/meet)
-- i ∨ j     (maximum/join)  
-- ~i        (reversal: 1-i)
```

### Computing with the Interval

The interval enables us to define continuous deformations:

```sctt
-- A function from the interval is a path
line_segment : I → Real
line_segment i = 2 * i + 1
-- At i0: gives 1
-- At i1: gives 3
-- Continuously interpolates between

-- We can compute at specific points
start : Real
start = line_segment i0  -- evaluates to 1

end : Real  
end = line_segment i1    -- evaluates to 3

-- And at symbolic points
middle : I → Real
middle i = line_segment (i ∧ ~i)  -- stays at midpoint
```

### Constraints and Faces

The interval supports constraints through face formulas:

#### Face Lattice

```sctt
-- Face formulas form a Boolean algebra
data FaceFormula : Type where
  ⊤ : FaceFormula              -- true (everywhere)
  ⊥ : FaceFormula              -- false (nowhere)
  _=ᵢ_ : I → I → FaceFormula  -- equality constraint
  _∧_ : FaceFormula → FaceFormula → FaceFormula
  _∨_ : FaceFormula → FaceFormula → FaceFormula
  ¬_ : FaceFormula → FaceFormula
```

#### Partial Elements

A partial element is defined only where a formula holds:

```sctt
-- Partial type former
Partial : FaceFormula → Type → Type

-- Example: boundary of a square
square_boundary : (i j : I) → 
                  Partial ((i = i0) ∨ (i = i1) ∨ 
                          (j = i0) ∨ (j = i1)) A
square_boundary i j [(i = i0)] = left_edge j
square_boundary i j [(i = i1)] = right_edge j
square_boundary i j [(j = i0)] = bottom_edge i
square_boundary i j [(j = i1)] = top_edge i
```

#### Systems and Compatibility

Systems must be compatible on overlapping faces:

```sctt
-- Compatibility condition
IsCompatible : {φ ψ : FaceFormula} → 
               Partial φ A → Partial ψ A → Type
IsCompatible u v = ∀ (i : I), (φ ∧ ψ)(i) → u(i) ≡ v(i)
```

## 3.2 Path Types {#paths}

### Paths as Functions from the Interval

A path in type `A` from `x` to `y` is a continuous function from `I`:

#### Path Type Formation

```sctt
-- Dependent path type
PathP : (A : I → Type) → A i0 → A i1 → Type
PathP A x y = Π (i : I), A i 
              [i ↦ i0] ↦ x  -- boundary condition
              [i ↦ i1] ↦ y  -- boundary condition

-- Non-dependent version
Path : (A : Type) → A → A → Type
Path A x y = PathP (λ _ → A) x y
```

#### Formal Rules for Paths

```
Γ ⊢ A : I → Type   Γ ⊢ x : A i0   Γ ⊢ y : A i1
—————————————————————————————————————————————  (Path-form)
Γ ⊢ PathP A x y : Type

Γ, i : I ⊢ p : A i   Γ ⊢ p[i0/i] ≡ x   Γ ⊢ p[i1/i] ≡ y
——————————————————————————————————————————————————  (Path-intro)
Γ ⊢ λi. p : PathP A x y

Γ ⊢ p : PathP A x y   Γ ⊢ r : I
———————————————————————————————  (Path-elim)
Γ ⊢ p @ r : A r
```

### Creating Paths

We construct paths using lambda abstraction:

```sctt
-- A trivial path (reflexivity)
refl : {A : Type} {x : A} → Path A x x
refl {x = x} = λ i → x

-- A non-trivial path in Real
linear_path : Path Real 0 1
linear_path = λ i → i  -- directly use interval variable

-- A path in functions
function_path : Path (Nat → Nat) (λ n → n) (λ n → n + 0)
function_path = λ i n → n + (neutralize i)
  where neutralize : I → Nat
        neutralize i = -- ... proof that 0 is neutral ...
```

### Path Application and Computation

We can apply paths at specific points:

```sctt
-- Path application (get point along path)
app : {A : Type} {x y : A} → Path A x y → I → A

-- Examples
midpoint : Real
midpoint = linear_path (i0 ∨ i1) / 2

-- Paths compute!
_ : linear_path i0 ≡ 0
_ = refl  -- Definitionally equal

_ : linear_path i1 ≡ 1  
_ = refl  -- Definitionally equal
```

### Dependent Paths

When types vary along a path, we need dependent paths:

```sctt
-- Dependent path over a type family
PathP : (A : I → Type) → A i0 → A i1 → Type

-- Example: path between vectors of different lengths
resize_path : (n m : Nat) → Path Nat n m → 
              Vec A n → Vec A m → Type
resize_path n m p v w = PathP (λ i → Vec A (p i)) v w

-- Transporting along paths
transport : {A B : Type} → Path Type A B → A → B
transport p x = transp (λ i → p i) i0 x
```

## 3.3 Composition and Transport {#composition}

### Path Composition

We can compose paths using the sophisticated composition operations:

#### Homogeneous Composition (hcomp)

```sctt
-- Horizontal composition for homogeneous types
hcomp : {A : Type} → {φ : FaceFormula} →
        (u : (i : I) → Partial φ A) →
        (u0 : A [φ ↦ u i0]) → A

-- Path composition via hcomp
_∙_ : {A : Type} {x y z : A} → 
      Path A x y → Path A y z → Path A x z
p ∙ q = λ i → hcomp (λ j → λ {
  (i = i0) → x;         -- left boundary
  (i = i1) → q j        -- right boundary  
}) (p i)                 -- bottom
```

#### The Double Composition Square

```
        p
    x ———→ y
    |       |
  refl      q  
    |       |
    ↓       ↓
    x ———→ z
       p∙q
```

#### Properties of Composition

```sctt
-- Left identity
lid : {A : Type} {x y : A} (p : Path A x y) →
      Path (Path A x y) (refl ∙ p) p
lid p = λ i j → hfill (λ k → λ {
  (j = i0) → x;
  (j = i1) → p k
}) (inS x) i

-- Associativity (up to higher path)
assoc : {A : Type} {w x y z : A}
        (p : Path A w x) (q : Path A x y) (r : Path A y z) →
        Path (Path A w z) ((p ∙ q) ∙ r) (p ∙ (q ∙ r))
```

### The comp Operation

The fundamental composition operation for dependent types:

#### General Composition

```sctt
-- Composition in type families
comp : (A : I → Type) → {φ : FaceFormula} →
       (u : (i : I) → Partial φ (A i)) →
       (u0 : A i0 [φ ↦ u i0]) → A i1

-- Key properties:
-- 1. Extends partial element u to total element
-- 2. Agrees with u on φ
-- 3. Starts from u0 at i0
```

#### Kan Filling Operation

```sctt
-- Kan filling (composition with intermediate results)
fill : (A : I → Type) → {φ : FaceFormula} →
       (u : (i : I) → Partial φ (A i)) →
       (u0 : A i0 [φ ↦ u i0]) →
       (i : I) → A i
fill A {φ} u u0 i = comp (λj → A (i ∧ j)) {φ ∨ (i = i0)}
                          (λj → λ { (φ = 1) → u (i ∧ j);
                                   (i = i0) → u0 })
                          u0

-- Crucial: fill i0 = u0, fill i1 = comp A u u0
```

#### Example: Path Lifting

```sctt
-- Lifting a path to a path of paths
path_lifting : {A : Type} {x y : A} →
               (p : Path A x y) →
               Path (Path A x x) refl p
path_lifting p = λi j → fill A {i0 ∨ i1}
                             (λk → λ { (j = i0) → x;
                                      (j = i1) → p k })
                             x i
```

### Transport: Moving Along Paths

Transport is how we move values along paths in types:

```sctt
-- Basic transport
transport : {A B : Type} → Path Type A B → A → B
transport p a = transp (λ i → p i) i0 a

-- Example: using univalence (preview)
Bool_to_Bool : Bool → Bool
Bool_to_Bool = transport Bool_equiv_Bool
  where Bool_equiv_Bool : Path Type Bool Bool
        Bool_equiv_Bool = ua negation_equivalence

-- Transport preserves structure
transport_preserves : {P : Type → Type} →
                     (p : Path Type A B) →
                     P A → P B
transport_preserves p = transport (λ i → P (p i))
```

### Kan Filling

Every open box has a lid—this is the computational content of being Kan:

```sctt
-- Kan filling operation
hfill : {A : Type} →
        (φ : 𝔽) →
        (u : (i : I) → Partial φ A) →
        (u0 : A [ φ ↦ u i0 ]) →
        (i : I) → A

-- This gives us path lifting
path_lifting : {A : Type} {x y : A} →
               (p : Path A x y) →
               Path (Path A x y) p p
path_lifting p = λ i j → hfill (∂ i) (λ k → λ {
  (i = i0) → x;
  (i = i1) → y
}) (inS (p i)) j
```

## 3.4 Higher Paths {#higher-paths}

### Paths Between Paths

Paths form a hierarchy—we can have paths between paths:

```sctt
-- A 2-path (homotopy) is a path between paths
Square : {A : Type} {a b c d : A} →
         Path A a b → Path A c d →
         Path A a c → Path A b d → Type
Square p q r s = Path (Path A _ _) 
                      (λ i → r i) 
                      (λ i → s i)

-- Example: commutative square
comm_square : {A : Type} {x y : A} →
              (p : Path A x y) →
              Square p p refl refl
comm_square p = λ i j → p (i ∧ j)
```

### Cubes and Higher Dimensions

The cubical structure extends to arbitrary dimensions:

```sctt
-- 3-dimensional cube
Cube : {A : Type} → 
       (faces : ...) → -- 6 square faces
       Type
       
-- Example: Eckmann-Hilton
eckmann_hilton : {A : Type} {x : A} →
                 (α β : Path (Path A x x) refl refl) →
                 Path _ (α ∙ β) (β ∙ α)
eckmann_hilton α β = λ i j k → 
  hcomp (λ l → λ {
    (i = i0) → α j k;
    (i = i1) → β j k;
    (j = i0) → x;
    (j = i1) → x;
    (k = i0) → x;
    (k = i1) → x
  }) x
```

### Loop Spaces

Loops are paths from a point to itself:

```sctt
-- Loop space
Ω : (A : Type) → A → Type
Ω A x = Path A x x

-- Higher loop spaces
Ω² : (A : Type) → A → Type
Ω² A x = Ω (Ω A x) refl

-- The fundamental group (set-truncated loops)
π₁ : (A : Type) → A → Set
π₁ A x = ∥ Ω A x ∥₀

-- Example: the circle has non-trivial loops
loop : Ω S¹ base
loop = λ i → loop_constructor i
```

## 3.5 Univalence {#univalence}

### The Univalence Principle

The crown jewel of cubical type theory: equivalent types are equal.

```sctt
-- Type equivalence
_≃_ : Type → Type → Type
A ≃ B = Σ (f : A → B), isEquiv f

-- The univalence axiom becomes a theorem!
ua : {A B : Type} → A ≃ B → Path Type A B
ua e = λ i → Glue B (λ {
  (i = i0) → (A, e);
  (i = i1) → (B, id_equiv)
})

-- Its inverse
ua⁻¹ : {A B : Type} → Path Type A B → A ≃ B
ua⁻¹ p = transport_equiv p
```

### Computing with Univalence

Unlike axiomatic univalence, cubical univalence computes:

```sctt
-- Transport along ua computes to the equivalence
transport_ua : {A B : Type} (e : A ≃ B) (x : A) →
               transport (ua e) x ≡ e.fst x
transport_ua e x = refl  -- Holds definitionally!

-- Example: computing with boolean negation
not_path : Path Type Bool Bool
not_path = ua (not, not_is_equiv)

-- Transport along not_path is negation
_ : transport not_path true ≡ false
_ = refl  -- Computes directly!
```

### Structure Identity Principle

Univalence implies that isomorphic structures are equal:

```sctt
-- Groups with isomorphic structure are equal
group_ua : (G H : Group) → GroupIso G H → Path Group G H
group_ua G H iso = λ i → record {
  carrier = ua iso.carrier_equiv i;
  op = λ x y → iso.op_preservation i x y;
  identity = iso.identity_preservation i;
  -- ... other fields ...
}

-- This means we can transport group theory!
transfer_theorem : (P : Group → Type) →
                   (G H : Group) → GroupIso G H →
                   P G → P H
transfer_theorem P G H iso = transport (λ i → P (group_ua G H iso i))
```

## 3.6 Glue Types

### The Computational Engine

Glue types make univalence compute:

```sctt
-- Glue type formation
Glue : (A : Type) → (φ : 𝔽) → 
       Partial φ (Σ (T : Type), T ≃ A) → Type

-- Gluing and ungluing
glue : {A : Type} {φ : 𝔽} {Te : Partial φ (Σ Type (_≃ A))} →
       PartialP φ (λ o → Te o .fst) →
       A → Glue A φ Te

unglue : {A : Type} {φ : 𝔽} {Te : Partial φ (Σ Type (_≃ A))} →
         Glue A φ Te → A
```

### Examples with Glue

```sctt
-- The interval as glued type
I_as_glue : Type
I_as_glue = Glue Bool (i0 ∨ i1) (λ {
  (i = i0) → (Unit, unit_to_bool_equiv);
  (i = i1) → (Unit, unit_to_bool_equiv')
})

-- Constructing non-trivial paths
twist : Path Type (A × B) (B × A)
twist = λ i → Glue (A × B) (i ∨ ~i) (λ {
  (i = i0) → (A × B, id_equiv);
  (i = i1) → (B × A, swap_equiv)
})
```

## 3.7 Cubical in Practice

### Higher Inductive Types

Cubical structure enables higher inductive types:

```sctt
-- The circle as a HIT
data S¹ : Type where
  base : S¹
  loop : Path S¹ base base

-- The torus
data T² : Type where
  point : T²
  line1 : Path T² point point
  line2 : Path T² point point
  square : Square line1 line1 line2 line2

-- Suspension
data Susp (A : Type) : Type where
  north : Susp A
  south : Susp A
  merid : A → Path (Susp A) north south
```

### Computing Homotopy Groups

```sctt
-- Computing π₁(S¹)
π₁_S¹ : π₁ S¹ base ≃ ℤ
π₁_S¹ = wind_equiv
  where
    wind : Ω S¹ base → ℤ
    wind p = -- winding number computation
    
-- The hopf fibration
hopf : S³ → S²
hopf = -- sophisticated construction using cubical structure

-- Showing π₃(S²) = ℤ
π₃_S² : π₃ S² base ≃ ℤ
π₃_S² = hopf_degree_equiv
```

## 3.8 Looking Ahead: Smooth Paths

The cubical structure we've built provides the foundation for smooth paths:

```sctt
-- Preview of smooth paths (Chapter 4)
SmoothPath : (M : Manifold) → M → M → Type
SmoothPath M x y = Σ (p : Path M x y), 
                     (i : I) → is_smooth_at (p i)

-- Smooth homotopy
SmoothHomotopy : {M : Manifold} →
                 (f g : C∞(M, N)) → Type
SmoothHomotopy f g = Σ (H : I → C∞(M, N)),
                       (H i0 ≡ f) × (H i1 ≡ g)
```

## Exercises {#exercises}

### Conceptual Understanding
1. Why can't the interval type `I` be defined as an ordinary inductive type?
2. What's the computational content of a path?
3. How does transport along `ua e` compute?
4. Why do we need both `comp` and `hcomp`?

### Path Construction
1. Define a path from `n + 0` to `n` for any `n : Nat`.
2. Construct a non-trivial path in `Path Type Bool Bool`.
3. Build a square with all four sides being `refl`.
4. Create a path between two equivalent sorting algorithms.

### Proofs Using Paths
1. Prove that path composition is associative (up to a higher path).
2. Show that `transport refl = id`.
3. Demonstrate that `sym (sym p) = p` for any path `p`.
4. Prove the interchange law for 2-paths.

### Advanced Cubical
1. Define the Klein bottle as a HIT.
2. Compute π₁ of the figure-eight space.
3. Implement the join of two types.
4. Show that the torus is equivalent to S¹ × S¹.

### Research Directions
1. How would you add computational content to ∞-groupoid structure?
2. Can you design a notion of "smooth interval" for Chapter 4?
3. What would a directed version of paths look like?
4. How might we compute higher homotopy groups efficiently?

## Summary

We've explored the revolutionary cubical structure:
- The **interval type** `I` provides dimension
- **Path types** make equality proof-relevant
- **Composition and transport** give computational meaning
- **Higher paths** capture homotopical structure  
- **Univalence** becomes computational via Glue types

This cubical foundation transforms types into spaces with rich geometric structure. Next, we'll add smoothness to this structure, creating the mathematical framework for differential geometry in type theory.

---

*Next: [Chapter 4: Smooth Types](./chapter_04.md) →*

*Previous: [Chapter 2: Type Theory Foundations](./chapter_02.md) ←*