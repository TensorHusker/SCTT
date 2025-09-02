# Chapter 2: Type Theory Foundations

> "A proof is a program, a proposition is a type." — The Curry-Howard Correspondence

## Introduction

Before we can add smooth structures to type theory, we need a solid foundation. This chapter introduces dependent type theory—the bedrock upon which SCTT is built. If you're familiar with languages like Haskell or ML, you'll recognize many concepts, but with a crucial twist: types can depend on values.

## 2.1 Types and Terms {#types-and-terms}

### What is a Type?

In SCTT, a type is a collection of values that share common structure. But unlike set theory, types come with computational meaning:

```sctt
-- Basic types
Bool : Type
Nat : Type  
Real : Type

-- Values (terms) of these types
true : Bool
false : Bool
zero : Nat
pi : Real
```

### The Judgment Forms

Type theory is built on four fundamental judgments:

1. **Type formation**: `A : Type` ("A is a type")
2. **Term introduction**: `a : A` ("a is a term of type A")
3. **Definitional equality**: `a ≡ b : A` ("a equals b by definition")
4. **Type equality**: `A ≡ B : Type` ("types A and B are definitionally equal")

### Functions as First-Class Citizens

Functions aren't just mappings—they're fundamental objects:

```sctt
-- Function type
A → B : Type

-- Function definition
double : Nat → Nat
double n = n + n

-- Function application
result : Nat
result = double 3  -- evaluates to 6
```

### The Universe Hierarchy

To avoid paradoxes, types themselves have types:

```sctt
Type₀ : Type₁
Type₁ : Type₂
-- ... and so on

-- Examples
Nat : Type₀
Type₀ → Type₀ : Type₁
```

This hierarchy ensures logical consistency while maintaining expressiveness.

## 2.2 Dependent Types {#dependent-types}

### The Power of Dependency

Here's where type theory becomes revolutionary: types can depend on values.

```sctt
-- A type that depends on a natural number
Vec : Nat → Type
Vec n = -- vectors of length n

-- Now we can express precise types
empty_vector : Vec 0
three_vector : Vec 3

-- This won't type-check!
-- wrong : Vec 2
-- wrong = three_vector  -- Error: Vec 3 ≠ Vec 2
```

### Pi Types (Dependent Functions)

The dependent function type `(x : A) → B(x)` is the heart of dependent type theory:

```sctt
-- A function whose output type depends on input value
replicate : (n : Nat) → A → Vec A n
replicate zero _ = empty
replicate (succ n) x = cons x (replicate n x)

-- The return type changes with the input!
test1 : Vec Bool 3
test1 = replicate 3 true

test2 : Vec Nat 5  
test2 = replicate 5 zero
```

### Sigma Types (Dependent Pairs)

Dependent pairs package a value with a type that depends on it:

```sctt
-- Sigma type syntax
Σ (x : A), B(x)

-- Example: a number with a proof it's positive
PositiveReal : Type
PositiveReal = Σ (x : Real), (x > 0)

-- Creating a dependent pair
sqrt_two : PositiveReal
sqrt_two = (1.414..., proof_positive)
```

### Real-World Example: Matrix Multiplication

Dependent types catch dimension errors at compile time:

```sctt
Matrix : Nat → Nat → Type

multiply : (m n p : Nat) → 
           Matrix m n → Matrix n p → Matrix m p
-- Note: dimensions must match!

-- This type-checks
valid : Matrix 2 3 → Matrix 3 4 → Matrix 2 4
valid = multiply 2 3 4

-- This doesn't even compile!
-- invalid : Matrix 2 3 → Matrix 4 5 → Matrix 2 5
-- Error: Cannot unify 3 with 4
```

## 2.3 Function Types {#function-types}

### Simple Functions

Non-dependent functions are a special case:

```sctt
-- When B doesn't depend on x, we write:
A → B
-- Instead of:
(x : A) → B

-- Example
increment : Nat → Nat
increment n = n + 1
```

### Currying and Multi-Argument Functions

Functions always take one argument, but we can curry:

```sctt
add : Nat → Nat → Nat
add x y = x + y

-- This is really:
add : Nat → (Nat → Nat)

-- Partial application
add_five : Nat → Nat
add_five = add 5
```

### Higher-Order Functions

Functions can take and return functions:

```sctt
compose : (B → C) → (A → B) → (A → C)
compose g f x = g (f x)

map : (A → B) → List A → List B
map f [] = []
map f (x :: xs) = f x :: map f xs
```

### Computational Behavior

Functions in SCTT compute:

```sctt
-- Definitional equality through computation
factorial : Nat → Nat
factorial 0 = 1
factorial (n + 1) = (n + 1) * factorial n

-- This equality holds definitionally
_ : factorial 3 ≡ 6
_ = refl  -- No proof needed, it computes!
```

## 2.4 Inductive Types {#inductive-types}

### Defining New Types

Inductive types let us build complex structures:

```sctt
-- Natural numbers
data Nat : Type where
  zero : Nat
  succ : Nat → Nat

-- Lists
data List (A : Type) : Type where
  nil : List A
  cons : A → List A → List A

-- Binary trees
data Tree (A : Type) : Type where
  leaf : A → Tree A
  node : Tree A → Tree A → Tree A
```

### Pattern Matching and Recursion

We can analyze inductive types by pattern matching:

```sctt
length : List A → Nat
length nil = zero
length (cons _ tail) = succ (length tail)

sum_tree : Tree Nat → Nat
sum_tree (leaf n) = n
sum_tree (node left right) = sum_tree left + sum_tree right
```

### Induction Principles

Every inductive type comes with an induction principle:

```sctt
-- Induction for natural numbers
nat_ind : (P : Nat → Type) →
          P zero →
          ((n : Nat) → P n → P (succ n)) →
          (n : Nat) → P n

-- Example: proving properties
all_nats_have_property : (n : Nat) → IsEven n ∨ IsOdd n
all_nats_have_property = nat_ind _ base_case inductive_step
  where
    base_case : IsEven zero ∨ IsOdd zero
    base_case = left even_zero
    
    inductive_step : (n : Nat) → 
                     (IsEven n ∨ IsOdd n) → 
                     (IsEven (succ n) ∨ IsOdd (succ n))
    -- ... proof ...
```

### Recursive Types with Invariants

SCTT allows types that maintain invariants:

```sctt
-- Sorted lists
data SortedList : Type where
  empty : SortedList
  singleton : Nat → SortedList
  cons_sorted : (x : Nat) → 
                (xs : SortedList) → 
                x ≤ head xs →
                SortedList

-- Balanced trees (AVL)
data AVLTree : Nat → Type where
  empty : AVLTree 0
  node : (h : Nat) →
         AVLTree h → Nat → AVLTree h →
         AVLTree (succ h)
  node_unbalanced : (h : Nat) →
                    AVLTree h → Nat → AVLTree (succ h) →
                    AVLTree (succ (succ h))
```

## 2.5 Universe Hierarchy {#universes}

### Why Universes?

Without care, we encounter Russell's paradox:

```sctt
-- This would be problematic:
-- Type : Type  -- Russell's paradox!

-- Instead, we have:
Type₀ : Type₁
Type₁ : Type₂
-- etc.
```

### Universe Polymorphism

We can write code generic over universe levels:

```sctt
-- Identity function at any level
id : {ℓ : Level} → {A : Type ℓ} → A → A
id x = x

-- Works for values
test1 : Nat
test1 = id 5

-- Works for types  
test2 : Type₀
test2 = id Nat

-- Works for kinds
test3 : Type₁
test3 = id Type₀
```

### Lifting Between Universes

Sometimes we need to lift types to higher universes:

```sctt
Lift : {ℓ ℓ' : Level} → Type ℓ → Type (ℓ ⊔ ℓ')
lift : {ℓ ℓ' : Level} {A : Type ℓ} → A → Lift ℓ' A
lower : {ℓ ℓ' : Level} {A : Type ℓ} → Lift ℓ' A → A
```

### Predicative vs Impredicative

SCTT is predicative: when forming types, we can only quantify over types in lower universes:

```sctt
-- Predicative (allowed in SCTT)
∀ (A : Type₀) → A → A : Type₁

-- Impredicative (not allowed)
-- ∀ (A : Type₁) → A → A : Type₁  -- Would need to be in Type₂
```

## 2.6 Equality Types

### Propositional Equality

Beyond definitional equality (≡), we have propositional equality:

```sctt
-- The identity type
data _≡_ {A : Type} : A → A → Type where
  refl : {x : A} → x ≡ x

-- Example proofs
comm_plus : (n m : Nat) → n + m ≡ m + n
comm_plus zero m = -- ... proof ...
comm_plus (succ n) m = -- ... proof ...
```

### Transport and Substitution

Equality allows us to transport proofs:

```sctt
transport : {A : Type} {P : A → Type} {x y : A} →
            x ≡ y → P x → P y
transport refl px = px

-- Example: if n = m, then Vec A n = Vec A m
vec_transport : {n m : Nat} → n ≡ m → Vec A n → Vec A m
vec_transport eq vec = transport (cong (Vec A) eq) vec
```

### Uniqueness of Identity Proofs (UIP)

In standard Martin-Löf type theory, all proofs of equality are equal:

```sctt
UIP : {A : Type} {x y : A} (p q : x ≡ y) → p ≡ q
```

But in SCTT (with cubical structure), this is no longer true! We'll explore this in Chapter 3.

## 2.7 Propositions as Types

### The Curry-Howard Correspondence

The profound insight: propositions are types, proofs are programs.

| Logic | Type Theory |
|-------|-------------|
| Proposition P | Type P |
| Proof of P | Term of type P |
| P ∧ Q | P × Q |
| P ∨ Q | P ⊎ Q |
| P → Q | P → Q |
| ∀x. P(x) | (x : A) → P(x) |
| ∃x. P(x) | Σ (x : A), P(x) |
| ⊥ (false) | ⊥ (empty type) |
| ⊤ (true) | ⊤ (unit type) |

### Constructive Logic

SCTT is constructive: to prove existence, we must construct:

```sctt
-- Classical (not allowed in SCTT)
-- ∃ n : Nat, IsPrime n ∧ n > 10^100

-- Constructive (required in SCTT)
large_prime : Σ (n : Nat), IsPrime n × (n > 10^100)
large_prime = (specific_number, prime_proof, size_proof)
  where
    specific_number = -- actual computation
    prime_proof = -- actual verification
    size_proof = -- actual comparison
```

### Negation and Empty Type

Negation is defined through the empty type:

```sctt
-- Empty type (no constructors)
data ⊥ : Type where

-- Negation
¬ : Type → Type
¬ A = A → ⊥

-- Principle of explosion
explosion : {A : Type} → ⊥ → A
explosion ()  -- No cases to consider!

-- Example: proof of inequality
not_equal : ¬ (2 ≡ 3)
not_equal eq = -- ... derive contradiction ...
```

## 2.8 Putting It Together

### A Complete Example: Safe Division

Let's combine everything to build safe division:

```sctt
-- First, define non-zero numbers
NonZero : Real → Type
NonZero x = ¬ (x ≡ 0)

-- Safe division type
safe_div : (x : Real) → (y : Real) → NonZero y → Real

-- Implementation
safe_div x y nonzero = x / y  -- Only compiles with proof!

-- Usage requires proof
example : Real
example = safe_div 10 2 proof_two_nonzero
  where
    proof_two_nonzero : NonZero 2
    proof_two_nonzero eq = -- ... contradiction ...

-- This won't compile!
-- bad : Real
-- bad = safe_div 10 0 ?  -- Can't provide NonZero 0
```

### Looking Ahead: Smooth Extensions

Everything we've learned extends naturally to smooth structures:

```sctt
-- Regular function type
f : Real → Real

-- Smooth function type (preview of Chapter 4)
g : C∞(Real, Real)

-- Dependent smooth function
h : (x : Real) → C∞(Interval x (x+1), Real)
```

## Exercises {#exercises}

### Understanding Check
1. What's the difference between `Type₀` and `Type₁`?
2. Give an example of a type that depends on a value.
3. Why can't we have `Type : Type`?
4. What's the computational content of a proof?

### Programming Practice
1. Define a type `Fin n` representing natural numbers less than n.
2. Implement vector concatenation with correct length.
3. Write a function that returns the nth element of a vector (when n < length).
4. Define binary trees that store their height in the type.

### Proofs
1. Prove that addition of natural numbers is associative.
2. Show that every list can be reversed twice to get the original.
3. Prove that append for lists is associative.
4. Demonstrate that `map id = id` for lists.

### Advanced
1. Define a sorted list type where the sorting invariant is in the type.
2. Implement red-black trees with balance invariants.
3. Create a type for prime numbers (numbers with a primality proof).
4. Design a type for invertible functions with their inverses.

### Research Questions
1. How would you extend the type system to handle sized types?
2. What would quotient types look like in this system?
3. How might we add effects (IO, exceptions) while preserving purity?
4. Can you design a type for continuous functions before seeing Chapter 4?

---

## Summary

We've built our foundation:
- **Types and terms** form the basic vocabulary
- **Dependent types** let types depend on values  
- **Function types** model computation
- **Inductive types** build complex structures
- **Universe hierarchy** avoids paradoxes
- **Propositions as types** unifies logic and computation

These concepts power everything in SCTT. Next, we'll add the cubical structure that makes types behave like spaces, setting the stage for smooth geometry.

---

*Next: [Chapter 3: Cubical Structure](./chapter_03.md) →*

*Previous: [Chapter 1: Introduction](./chapter_01.md) ←*