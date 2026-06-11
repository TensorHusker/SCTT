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
- **Univalence**: the canonical map `pathToEquiv : (A ≡ B) → (A ≃ B)` is an equivalence (in particular, `ua : A ≃ B → Path Type A B`)
- Unlike axioms, univalence **computes** via Glue types

**Prerequisites**: [Chapter 2](./chapter_02.md) — Dependent type theory

**Time**: 3-4 hours for full chapter with exercises

> **🔬 Running Example**: Particle trajectories are paths through space. Path composition models sequential motion. See [running example §Chapter 3](./running_example.md#chapter-3-cubical-structure).

---

### Mathematical Foundations

Cubical type theory is based on a model in cubical sets—presheaves on the Cartesian cube category. The key insight is that:

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

#### Cartesian Interval Structure

SCTT uses the Cartesian cube category (ABCFHL) rather than the De Morgan variant (CCHM). The interval has no algebraic operations on its elements—only endpoints, face maps, and the diagonal:

```sctt
-- The Cartesian interval (ABCFHL)
-- No connections (∧, ∨) or reversal (~)
-- Only: endpoints, faces, degeneracies, and diagonals

i0 : I  -- left endpoint
i1 : I  -- right endpoint

-- Face maps: restriction to endpoints
_[i0/i] : (I → A) → A  -- evaluate at left
_[i1/i] : (I → A) → A  -- evaluate at right

-- Diagonal: the key cofibration replacing connections
(i = j) : Cofibration  -- diagonal constraint

-- Face formulas (cofibrations)
-- Built from: ⊥ | ⊤ | (i = 0) | (i = 1) | (i = j) | φ ∧ ψ | φ ∨ ψ
-- Note: NO negation (~), NO meets/joins on interval elements
```

> **Design Note: Why Cartesian over De Morgan?**
>
> SCTT uses the Cartesian cube category (ABCFHL) rather than the De Morgan variant (CCHM).
> The Cartesian interval has a simpler algebra — no connections (∧, ∨) or reversal (~) — which
> means fewer equational obligations in the normalizer. The diagonal cofibration (i = j) replaces
> connections for constructing univalent universes. This is the variant implemented by cctt
> (Kovács), which demonstrates that a high-performance cubical evaluator fits in a few thousand
> lines. The simpler cube category also interacts more cleanly with the smooth layer's
> equational theory (ε² = 0), since there are fewer cubical reduction rules to check
> confluence against.

#### Why Not an Inductive Type?

The interval cannot be an inductive type because:
1. It would only have two distinct elements (i0 and i1)
2. We need a continuum of points
3. Interval variables must be symbolic, not concrete

```sctt
-- We can have expressions involving interval variables
-- If i : I, then i represents a point in the interval

-- In Cartesian cubical, interval expressions are just variables:
-- i          (a dimension variable)
-- i0, i1     (the two endpoints)
-- There are NO operations (∧, ∨, ~) on interval elements themselves

-- Structure lives in *cofibrations* (face formulas):
-- (i = i0)   (left face)
-- (i = i1)   (right face)
-- (i = j)    (diagonal)
```

### Computing with the Interval

The interval enables us to define continuous deformations:

```sctt
-- A function out of the interval is a path in its codomain.
-- NB: I has NO algebraic operations (see above), so arithmetic such as
-- "2 * i + 1" is not even well-formed. A function I → Real is constant:
constant_segment : I → Real
constant_segment i = 1

-- We can compute at specific points
start : Real
start = constant_segment i0  -- evaluates to 1

end : Real  
end = constant_segment i1    -- evaluates to 1

-- Paths respect the interval endpoints
_ : constant_segment i0 ≡ constant_segment i0  -- trivially
_ = refl
```

> **⚠️ Identity paths vs. topological paths**: `Path ℝ a b` is the *identity type* of ℝ.
> Since ℝ is a set, `Path ℝ a b` is **empty** whenever `a ≠ b` — there is no path
> "interpolating from 1 to 3". Topological/smooth paths in a space `M` are a different
> notion entirely: smooth maps `C∞([0,1], M)` out of the *real* unit interval, treated
> in Chapter 4.

### Constraints and Faces

The interval supports constraints through face formulas:

#### Face Lattice

```sctt
-- Face formulas (cofibrations) in Cartesian cubical type theory
-- Note: ∧ and ∨ combine *cofibrations*, not interval elements
data Cof : Type where
  ⊤ : Cof               -- true (everywhere)
  ⊥ : Cof               -- false (nowhere)
  _=0 : I → Cof         -- left face: (i = i0)
  _=1 : I → Cof         -- right face: (i = i1)
  _=_ : I → I → Cof     -- diagonal: (i = j)
  _∧_ : Cof → Cof → Cof -- conjunction of cofibrations
  _∨_ : Cof → Cof → Cof -- disjunction of cofibrations
  -- No negation (¬): cofibrations are NOT a Boolean algebra
```

#### Partial Elements

A partial element is defined only where a formula holds:

```sctt
-- Partial type former
Partial : Cof → Type → Type

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
IsCompatible : {φ ψ : Cof} → 
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

-- ⚠️ There is NO non-trivial path in Real: ℝ is a set, so Path ℝ a b is
-- empty whenever a ≠ b (and "λ i → i" is not even well-typed: i : I is
-- not a real number). Non-trivial paths need higher structure, e.g.:
loop_path : Path S¹ base base
loop_path = λ i → loop_constructor i  -- the circle's loop (see §3.7)

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

-- Examples (with the constant path at 0)
zero_path : Path Real 0 0
zero_path = λ i → 0

start_point : Real
start_point = zero_path @ i0  -- evaluates to 0

-- Paths compute!
_ : zero_path @ i0 ≡ 0
_ = refl  -- Definitionally equal

_ : zero_path @ i1 ≡ 0  
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
hcomp : {A : Type} → {φ : Cof} →
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

#### Path Inverse (Symmetry)

In De Morgan cubical type theory, path inverse is trivially `λ i → p @ ~i` using interval reversal. Without reversal, Cartesian cubical constructs inverse via Kan filling:

```sctt
-- Path inverse in Cartesian cubical type theory
-- Without reversal (~), inverse is constructed via Kan filling:
sym : Path A x y → Path A y x
sym p = λ i → comp (λ _ → A) ((i = i0) ∨ (i = i1))
                    (λ j → [ (i = i0) ↦ p @ j , (i = i1) ↦ x ])
                    x   -- the cap is the constant x (= p @ i0), not p @ i
-- The absence of ~ means inverse requires more work but keeps the interval algebra simpler
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
-- Composition filler (honest type): the hfill term below relates refl to
-- refl ∙ p over the family λ i → Path A x (p i) — it is NOT itself the
-- left unit law.
compPath-filler : {A : Type} {x y : A} (p : Path A x y) →
      PathP (λ i → Path A x (p i)) refl (refl ∙ p)
compPath-filler p = λ i j → hfill (λ k → λ {
  (j = i0) → x;
  (j = i1) → p k
}) (inS x) i
-- The left unit law  Path (Path A x y) (refl ∙ p) p  is then derived from
-- this filler by one further composition (cubical library: lUnit).

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
comp : (A : I → Type) → {φ : Cof} →
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
fill : (A : I → Type) → {φ : Cof} →
       (u : (i : I) → Partial φ (A i)) →
       (u0 : A i0 [φ ↦ u i0]) →
       (i : I) → A i
-- In ABCFHL, filling is *definable* when composition is endpoint-indexed
-- (r → r'): fill is the instance hcom^{0→z} with a variable endpoint z.
-- fill must be taken as primitive only if comp is fixed to 0 → 1.
-- It satisfies:
--   fill A φ u u0 i0 = u0
--   fill A φ u u0 i1 = comp A φ u u0
--   fill A φ u u0 i  agrees with u i on φ
```

#### Formal Specification (Angiuli, Favonia, Harper 2017)

For implementors, the precise mathematical definitions from *Computational Higher Type Theory III* (Angiuli, Hou, Harper 2017) are essential reference. The two core Kan operations for Cartesian cubical type theory are:

**Coercion** (`coe`). Given a type `A` varying in dimension `x`, coercion sends an element of `A⟨r/x⟩` to an element of `A⟨r'/x⟩`:

```
coe^{r→r'}_{x.A}(M) : A⟨r'/x⟩    when M : A⟨r/x⟩
```

The key computation rule: when `r = r'`, coercion is the identity: `coe^{r→r}_{x.A}(M) ≡ M`. For compound types, coercion distributes structurally:

- `coe^{r→r'}_{x. Π(a:A).B}(f) = λa. coe^{r→r'}_{x.B[coe^{r'→x}_{x.A}(a)/a]}(f (coe^{r'→r}_{x.A}(a)))` — note the substituted argument is coerced to the *bound* dimension `x`, so it varies along the coercion
- `coe^{r→r'}_{x. Σ(a:A).B}(p) = (coe^{r→r'}_{x.A}(π₁(p)), coe^{r→r'}_{x.B[a:=fill]}(π₂(p)))`

**Homogeneous composition** (`hcom`). Given a homogeneous type `A`, a cofibration `φ`, a tube `u : (i : I) → Partial φ A`, and a cap `u₀ : A` agreeing with `u` on `φ`, `hcom` produces the composite:

```
hcom^{r→r'}_A [φ ↦ u] u₀ : A
```

Subject to: (1) `hcom^{r→r}_A [φ ↦ u] u₀ ≡ u₀`, and (2) `hcom^{r→r'}_A [⊤ ↦ u] u₀ ≡ u r'`.

The distinction between `coe` (heterogeneous, one element) and `hcom` (homogeneous, partial elements) is characteristic of the Cartesian approach. In CCHM, these are combined into a single `comp` operation, and the de Morgan algebra (connections) is used to define filling from composition. In the Cartesian setting filling is instead obtained from endpoint-indexed composition: fill is the instance `hcom^{0→z}` with a variable endpoint `z`; only if composition were fixed to `0→1` would filling need to be primitive.

#### Example: Path Lifting

```sctt
-- Filling the composition square (honest type): the fill term below
-- relates refl to refl ∙ p over the family λ i → Path A x (p i).
path_filler : {A : Type} {x y : A} →
               (p : Path A x y) →
               PathP (λ i → Path A x (p i)) refl (refl ∙ p)
path_filler p = λi j → fill (λ _ → A) {(j = i0) ∨ (j = i1)}
                             (λk → λ { (j = i0) → x;
                                      (j = i1) → p k })
                             x i
-- The unit law Path (Path A x y) (refl ∙ p) p follows by one further
-- composition (cubical library: lUnit).
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

-- This gives a degenerate square over p (honest type): the j = i0 face is
-- p definitionally, but the j = i1 face is the hcomp of the constant tube
-- over p — equal to p only *propositionally*, not definitionally.
path_lifting : {A : Type} {x y : A} →
               (p : Path A x y) →
               PathP (λ j → Path A x y) p
                     (λ i → hcomp (λ k → λ { (i = i0) → x;
                                             (i = i1) → y }) (p i))
path_lifting p = λ j i → hfill (∂ i) (λ k → λ {
  (i = i0) → x;
  (i = i1) → y
}) (inS (p i)) j
-- A genuine Path (Path A x y) p p is then derived by one further
-- composition collapsing the hcomp face.
```

## 3.4 Higher Paths {#higher-paths}

### Paths Between Paths

Paths form a hierarchy—we can have paths between paths:

```sctt
-- A 2-path (homotopy) is a path between paths
Square : {A : Type} {a b c d : A} →
         Path A a b → Path A c d →
         Path A a c → Path A b d → Type
Square p q r s = PathP (λ i → Path A (p i) (q i)) r s

-- Example: commutative square
-- (honest type: this hfill term is again the composition filler,
--  PathP (λ i → Path A x (p i)) refl (refl ∙ p), read as a square)
comm_square : {A : Type} {x y : A} →
              (p : Path A x y) →
              PathP (λ i → Path A x (p i)) refl (refl ∙ p)
comm_square p = λ i j → hfill (λ k → λ {
  (j = i0) → x;
  (j = i1) → p k
}) (inS x) i
-- In De Morgan cubical, this would be simply p (i ∧ j).
-- Without connections, we construct the square via Kan filling.
-- A square with face p on both sides is derived from this filler by one
-- further composition (cubical library: lUnit).
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
-- (proof term omitted)
```

The proof is the *interchange law* argument: on 2-loops there are two
composition operations (composing in either of the two dimensions), they
share a unit (`refl`), and each is a homomorphism for the other. Sliding `α`
and `β` past one another through a 3-dimensional cube — first composing
horizontally, then vertically — interchanges their order, yielding
`α ∙ β ≡ β ∙ α`. The fully formal construction involves several nested
`hcomp`s; see `EH` in `Cubical.Homotopy.Loopspace` of the cubical Agda
library for the complete proof term.

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

-- The univalence axiom becomes a theorem! Full univalence says the
-- canonical map pathToEquiv : (A ≡ B) → (A ≃ B) is itself an equivalence;
-- ua below is its inverse (merely having ua : A ≃ B → Path Type A B
-- would be strictly weaker).
ua : {A B : Type} → A ≃ B → Path Type A B
ua e = λ i → Glue B (λ {
  (i = i0) → (A, e);
  (i = i1) → (B, id_equiv)
})

-- The canonical map (an equivalence, with inverse ua)
pathToEquiv : {A B : Type} → Path Type A B → A ≃ B
pathToEquiv p = transport_equiv p
```

### Computing with Univalence

Unlike axiomatic univalence, cubical univalence computes:

```sctt
-- Transport along ua computes to the equivalence — up to a path, not
-- definitionally: in CCHM/ABCFHL/cubical Agda, transport (ua e) x unfolds
-- to e.fst x wrapped in an extra (trivial) transport, removed by
-- transportRefl.
transport_ua : {A B : Type} (e : A ≃ B) (x : A) →
               transport (ua e) x ≡ e.fst x
transport_ua e x = transportRefl (e.fst x)  -- propositional, NOT refl in general
-- It is definitional only for types with trivial transp, such as Bool —
-- which is why the Bool example below computes by refl.

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
-- A line of types over Bool: at i0 we glue Bool along the negation
-- equivalence, at i1 along the identity. (Note the dimension i must be
-- bound, and the glued types must actually be equivalent to Bool.)
notLine : I → Type
notLine = λ i → Glue Bool (λ {
  (i = i0) → (Bool, notEquiv);
  (i = i1) → (Bool, idEquiv)
})
-- This is exactly ua notEquiv: transporting along it negates a boolean.

-- Constructing non-trivial paths
twist : Path Type (A × B) (B × A)
twist = λ i → Glue (A × B) ((i = i0) ∨ (i = i1)) (λ {
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