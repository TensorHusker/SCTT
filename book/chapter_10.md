# Chapter 10: Programming in SCTT

> "The purpose of computing is insight, not numbers." — Richard Hamming
>
> "In SCTT, we compute with mathematical certainty, where every program is a theorem and every execution is a proof."

## Introduction

Programming in SCTT transforms computation from an engineering discipline to a mathematical art. Every program is simultaneously executable code and a mathematical proof. Every data structure encodes precise mathematical invariants. Every algorithm comes with verified correctness guarantees.

This chapter provides a comprehensive guide to practical programming in SCTT:

1. **Basic Programming** - Functions, data structures, and control flow
2. **Proving Theorems** - Mathematical reasoning as programming
3. **Smooth Computations** - Calculus and differential equations
4. **Library Development** - Building reusable mathematical components
5. **Best Practices** - Patterns and principles for SCTT development

The key insight is that SCTT dissolves the boundary between programming and mathematics—every program is mathematical, and every proof is computational.

### The SCTT Programming Philosophy

```sctt
-- Traditional programming: Code + separate tests
traditional_approach : ProgrammingApproach
traditional_approach = {
  implementation = write_code,
  verification = write_separate_tests,
  hope = tests_cover_all_cases
}

-- SCTT programming: Code IS the proof
sctt_approach : ProgrammingApproach  
sctt_approach = {
  implementation = write_verified_code,
  verification = type_checking,
  guarantee = mathematical_correctness
}

-- Example: Verified sorting
sort : (xs : List A) → {ys : List A | Sorted ys ∧ Permutation xs ys}
-- The type guarantees correctness!
```

### Programming Paradigms in SCTT

SCTT supports multiple programming paradigms, all unified by dependent types:

```sctt
-- Functional programming with dependent types
map : {A B : Type} → (A → B) → List A → List B

-- Logic programming via type inhabitation  
theorem : ∀ (P Q : Prop), P → (P → Q) → Q

-- Smooth programming with calculus
solve_ode : (f : C∞(ℝ × ℝ, ℝ)) → (y₀ : ℝ) → 
           {solution : C∞(ℝ, ℝ) | solution'(x) = f(x, solution(x)) ∧ solution(0) = y₀}

-- Geometric programming with paths
homotopy : {X : Type} {x y : X} → (p q : Path X x y) → 
          Path (Path X x y) p q

-- Modal programming with different modes
eventually : ◇ A → A  -- Eventually operator
always : □ A → A      -- Always operator
```

## 10.1 Basic Programming {#basics}

### Functions and Types

Functions in SCTT are much more expressive than in traditional languages:

#### Simple Functions

```sctt
-- Basic function definition
identity : (A : Type) → A → A
identity A x = x

-- Type inference often allows omitting type arguments
id : {A : Type} → A → A  
id x = x  -- Inferred: A is implicit

-- Function composition
compose : {A B C : Type} → (B → C) → (A → B) → A → C
compose f g x = f (g x)

-- Infix notation
_∘_ : {A B C : Type} → (B → C) → (A → B) → A → C
f ∘ g = compose f g

-- Example usage
add_then_square : ℕ → ℕ
add_then_square = (λ x → x * x) ∘ (λ x → x + 1)
```

#### Dependent Functions

Functions where the output type depends on the input value:

```sctt
-- Vector of specified length
Vector : (A : Type) → ℕ → Type
Vector A 0 = Unit
Vector A (n + 1) = A × Vector A n

-- Function producing vector of length n
replicate : {A : Type} → (n : ℕ) → A → Vector A n
replicate 0 x = tt
replicate (n + 1) x = (x, replicate n x)

-- Dependent function type
safe_head : {A : Type} → (n : ℕ) → Vector A (n + 1) → A
safe_head n (x, xs) = x  -- Guaranteed non-empty!

-- Matrix multiplication with dimension checking
matrix_mult : {m n p : ℕ} → Matrix m n → Matrix n p → Matrix m p
matrix_mult A B = -- Implementation guaranteed to type-check only with compatible dimensions
```

#### Higher-Order Functions

Functions that operate on other functions:

```sctt
-- Higher-order function with dependent types
fold : {A : Type} {B : A → Type} → 
       ((x : A) → B x → B x) → 
       B x₀ → 
       (xs : List A) → 
       B (last xs x₀)

-- Map with proof preservation
map_preserves : {A B : Type} {P : A → Type} {Q : B → Type} →
                (f : A → B) → 
                (∀ x, P x → Q (f x)) →
                (xs : List A) → 
                All P xs → 
                All Q (map f xs)

-- Function optimization with correctness proof
optimize : {A B : Type} → 
           (f : A → B) → 
           {f' : A → B | ∀ x, f x ≡ f' x ∧ MoreEfficient f' f}
```

### Data Structures

SCTT data structures can encode sophisticated invariants:

#### Basic Inductive Types

```sctt
-- Natural numbers with properties
data ℕ : Type where
  zero : ℕ
  succ : ℕ → ℕ

-- Even numbers as a subset
data Even : ℕ → Type where
  even_zero : Even 0
  even_succ_succ : ∀ n, Even n → Even (succ (succ n))

-- Lists with length information
data List (A : Type) : Type where
  nil : List A
  cons : A → List A → List A

-- Vectors (lists with statically known length)
data Vec (A : Type) : ℕ → Type where
  nil : Vec A 0
  cons : ∀ n, A → Vec A n → Vec A (n + 1)

-- Binary trees with size information
data Tree (A : Type) : ℕ → Type where
  leaf : Tree A 1
  node : ∀ {m n}, Tree A m → Tree A n → Tree A (m + n)
```

#### Sophisticated Data Structures

```sctt
-- Binary search trees with ordering invariant
data BST (A : Type) (compare : A → A → Ordering) : A → A → Type where
  empty : ∀ {min max}, min < max → BST A compare min max
  node : ∀ {min max} (value : A) (left : BST A compare min value) 
                                 (right : BST A compare value max),
         BST A compare min max

-- Red-black trees with balanced invariant  
data Color : Type where
  Red : Color
  Black : Color

data RBTree (A : Type) : Color → ℕ → Type where
  leaf : RBTree A Black 0
  red_node : ∀ {n} (x : A) (left right : RBTree A Black n), 
             RBTree A Red n
  black_node : ∀ {n c₁ c₂} (x : A) (left : RBTree A c₁ n) (right : RBTree A c₂ n),
               RBTree A Black (n + 1)

-- Verified insertion that maintains invariants
rb_insert : {A : Type} {n : ℕ} → A → RBTree A Black n → 
           {result : RBTree A Black (n ∨ (n + 1)) | ValidRBTree result}

-- Finite sets with decidable membership
FiniteSet : Type → Type
FiniteSet A = {S : A → Type | ∃ (xs : List A), ∀ x, S x ↔ x ∈ xs}

-- Verified union operation
union : {A : Type} → FiniteSet A → FiniteSet A → FiniteSet A
union S T = λ x → S x ∨ T x  -- Proof that this is still finite required
```

### Control Flow

Control flow in SCTT is expressed through pattern matching and dependent elimination:

#### Pattern Matching

```sctt
-- Basic pattern matching
is_zero : ℕ → Bool
is_zero zero = true
is_zero (succ n) = false

-- Pattern matching with proofs
safe_predecessor : (n : ℕ) → n ≠ 0 → ℕ
safe_predecessor zero p = absurd (p refl)  -- Contradiction!
safe_predecessor (succ n) p = n

-- Dependent pattern matching
vector_head : {A : Type} {n : ℕ} → Vec A (n + 1) → A
vector_head (cons n x xs) = x

-- Pattern matching on equality proofs
transport_via_equality : {A B : Type} → A ≡ B → A → B
transport_via_equality refl x = x
```

#### Dependent Elimination

```sctt
-- Elimination with motive
nat_elim : {P : ℕ → Type} → 
           P 0 → 
           (∀ n, P n → P (succ n)) → 
           ∀ n, P n
nat_elim base step zero = base
nat_elim base step (succ n) = step n (nat_elim base step n)

-- Induction principles become programming constructs
list_induction : {A : Type} {P : List A → Type} →
                 P nil →
                 (∀ x xs, P xs → P (cons x xs)) →
                 ∀ xs, P xs

-- Structural recursion with termination proof
merge_sort : {A : Type} → (A → A → Bool) → (xs : List A) → List A
merge_sort compare xs = 
  case length xs of
    0 → nil
    1 → xs  
    n → let (left, right) = split xs
            sorted_left = merge_sort compare left   -- Recursive call on smaller list
            sorted_right = merge_sort compare right
        in merge compare sorted_left sorted_right
  -- Termination guaranteed by structural recursion
```

### Smooth Programming

Programming with smooth functions and calculus:

#### Basic Smooth Functions

```sctt
-- Polynomial functions
poly : List ℝ → C∞(ℝ, ℝ)
poly [] x = 0
poly (a :: coeffs) x = a + x * poly coeffs x

-- Exponential function (defined by differential equation)
exp : C∞(ℝ, ℝ)
exp = solution_of_ode (λ x y → y) 1  -- y' = y, y(0) = 1

-- Trigonometric functions
sin cos : C∞(ℝ, ℝ)
sin = solution_of_ode_system 
        (λ x (y, y') → (y', -y)) 
        (0, 1)  -- sin(0) = 0, sin'(0) = 1
cos = derivative sin  -- cos = sin'

-- Function composition
compose_smooth : {M N P : SmoothType} → C∞(N, P) → C∞(M, N) → C∞(M, P)
compose_smooth g f x = g (f x)
-- Smoothness automatically verified by type system
```

#### Calculus Operations

```sctt
-- Automatic differentiation
diff : C∞(ℝ, ℝ) → C∞(ℝ, ℝ)
diff f x = D[f] x  -- Computed exactly, not numerically

-- Chain rule (automatic)
chain_rule_example : C∞(ℝ, ℝ)
chain_rule_example x = sin (x * x)  -- Derivative computed automatically

-- Integration (when computable)
integrate : C∞([a, b], ℝ) → ℝ
integrate f = ∫ f dx  -- Exact integration when possible

-- Fundamental theorem of calculus
fundamental_theorem : (f : C∞([a, b], ℝ)) → (F : Antiderivative f) →
                     ∫ᵃᵇ f dx ≡ F b - F a

-- Taylor series expansion
taylor : (f : C∞(ℝ, ℝ)) → (x₀ : ℝ) → (n : ℕ) → Polynomial
taylor f x₀ n = Σ_{k=0}^n (D^k[f](x₀) / k!) * (x - x₀)^k
```

#### Differential Equations

```sctt
-- First-order ODE solver
solve_first_order : (f : C∞(ℝ × ℝ, ℝ)) → (y₀ : ℝ) → 
                   C∞(ℝ, ℝ)
solve_first_order f y₀ = 
  unique_solution where
    y' = f (x, y)
    y(0) = y₀
  -- Existence and uniqueness guaranteed by type

-- System of ODEs
solve_system : {n : ℕ} → 
               (f : C∞(ℝ × ℝⁿ, ℝⁿ)) → 
               (y₀ : ℝⁿ) → 
               C∞(ℝ, ℝⁿ)

-- Partial differential equations (when solvable)
solve_heat_equation : (initial : C∞(ℝ, ℝ)) → 
                     C∞(ℝ × ℝ₊, ℝ)
solve_heat_equation u₀ (x, t) = 
  -- ∂u/∂t = ∂²u/∂x²
  -- u(x, 0) = u₀(x)
  solution_satisfying_pde u₀

-- Numerical methods with error bounds
euler_method : (f : C∞(ℝ × ℝ, ℝ)) → (y₀ : ℝ) → (h : ℝ₊) → 
              {approx : C∞(ℝ, ℝ) | |approx - exact_solution| ≤ error_bound h}
```

## 10.2 Proving Theorems {#proofs}

### Propositions as Types

In SCTT, mathematical theorems are types, and proofs are programs:

#### Basic Logical Operations

```sctt
-- Logical connectives as types
_∧_ : Type → Type → Type
A ∧ B = A × B  -- Product type

_∨_ : Type → Type → Type  
A ∨ B = A ⊎ B  -- Sum type

_→_ : Type → Type → Type
A → B = A → B  -- Function type

¬_ : Type → Type
¬ A = A → ⊥  -- Negation as implication to false

-- Proof construction
and_introduction : A → B → A ∧ B
and_introduction a b = (a, b)

and_elimination_left : A ∧ B → A
and_elimination_left (a, b) = a

or_introduction_left : A → A ∨ B
or_introduction_left a = inl a

-- Proof by cases
or_elimination : A ∨ B → (A → C) → (B → C) → C
or_elimination (inl a) f g = f a
or_elimination (inr b) f g = g b
```

#### Quantifiers and Equality

```sctt
-- Universal quantification
∀_,_ : (A : Type) → (A → Type) → Type
∀ A, P = (x : A) → P x

-- Existential quantification  
∃_,_ : (A : Type) → (A → Type) → Type
∃ A, P = Σ (x : A), P x

-- Equality type (identity type)
_≡_ : {A : Type} → A → A → Type
x ≡ y = Path A x y  -- Uses cubical paths

-- Proof construction for equality
refl : {A : Type} {x : A} → x ≡ x
refl = λ i → x  -- Constant path

-- Substitution (transport)
subst : {A : Type} {P : A → Type} {x y : A} → x ≡ y → P x → P y
subst p px = transport P p px

-- Symmetry and transitivity
symm : {A : Type} {x y : A} → x ≡ y → y ≡ x
symm p = λ i → p (~ i)

trans : {A : Type} {x y z : A} → x ≡ y → y ≡ z → x ≡ z
trans p q = λ i → comp (λ j → A) (i ∨ ~ i) (λ j → λ {(i = i0) → x; (i = i1) → q j}) (p i)
```

#### Mathematical Proofs

```sctt
-- Theorem: Commutativity of addition
plus_comm : (m n : ℕ) → m + n ≡ n + m
plus_comm zero n = 
  m + n ≡⟨ refl ⟩
  0 + n ≡⟨ zero_plus n ⟩  
  n     ≡⟨ symm (plus_zero n) ⟩
  n + 0 ∎
plus_comm (succ m) n =
  (succ m) + n   ≡⟨ refl ⟩
  succ (m + n)   ≡⟨ cong succ (plus_comm m n) ⟩
  succ (n + m)   ≡⟨ symm (plus_succ n m) ⟩
  n + (succ m)   ∎

-- Theorem: sqrt(2) is irrational
sqrt_2_irrational : ¬∃ (p q : ℕ), q ≠ 0 ∧ gcd p q ≡ 1 ∧ p^2 ≡ 2 * q^2
sqrt_2_irrational (p, q, q_nonzero, coprime, equation) = 
  -- Proof by contradiction
  let p_even = even_square_means_even p equation
      q_even = even_quotient_means_even q p_even equation  
      both_even = (p_even, q_even)
      contradiction = gcd_even_contradiction coprime both_even
  in contradiction

-- Theorem: Fundamental theorem of algebra (statement)
fundamental_theorem_algebra : 
  ∀ (p : Polynomial ℂ), degree p > 0 → ∃ (z : ℂ), p z ≡ 0
-- Proof requires complex analysis techniques
```

### Induction and Recursion

Induction principles become computational tools:

#### Mathematical Induction

```sctt
-- Natural number induction  
nat_induction : {P : ℕ → Type} → 
                P 0 → 
                (∀ n, P n → P (succ n)) → 
                ∀ n, P n
nat_induction base step = nat_elim base step

-- Strong induction
strong_induction : {P : ℕ → Type} →
                   (∀ n, (∀ m, m < n → P m) → P n) →
                   ∀ n, P n

-- Well-founded recursion
well_founded_recursion : {A : Type} {R : A → A → Type} →
                        WellFounded R →
                        {P : A → Type} →
                        (∀ x, (∀ y, R y x → P y) → P x) →
                        ∀ x, P x

-- Example: Euclidean algorithm with termination proof
gcd : ℕ → ℕ → ℕ
gcd a 0 = a
gcd a b = gcd b (a mod b)  -- Terminates because (a mod b) < b
```

#### Structural Induction

```sctt
-- List induction
list_induction : {A : Type} {P : List A → Type} →
                 P [] →
                 (∀ x xs, P xs → P (x :: xs)) →
                 ∀ xs, P xs

-- Tree induction  
tree_induction : {A : Type} {P : Tree A → Type} →
                 (∀ x, P (leaf x)) →
                 (∀ left right, P left → P right → P (node left right)) →
                 ∀ t, P t

-- Example: Proving tree properties
tree_size_positive : ∀ {A} (t : Tree A), size t > 0
tree_size_positive = tree_induction
  (λ x → succ_positive 0)  -- Leaf case
  (λ left right ih_left ih_right → 
     add_positive (size left) (size right) ih_left ih_right)  -- Node case
```

### Constructive vs Classical Logic

SCTT supports both constructive and classical reasoning:

#### Constructive Proofs

```sctt
-- Constructive existence proof
constructive_sqrt : (x : ℝ) → x ≥ 0 → {y : ℝ | y^2 ≡ x}
constructive_sqrt x x_pos = 
  -- Newton's method converges to sqrt(x)
  let y = limit (newton_iteration x)
      proof = newton_convergence_proof x x_pos
  in (y, proof)

-- Constructive proof gives algorithm
find_element : {A : Type} (P : A → Bool) (xs : List A) →
               (∃ x ∈ xs, P x ≡ true) → 
               {x : A | x ∈ xs ∧ P x ≡ true}
find_element P [] existence_proof = absurd existence_proof
find_element P (x :: xs) existence_proof = 
  if P x 
    then (x, (here, refl))
    else find_element P xs (there_exists existence_proof)
```

#### Classical Proofs with Modal Types

```sctt
-- Classical logic via double negation
classical : Type → Type  
classical A = ¬¬ A

-- Law of excluded middle (classical)
lem : (A : Type) → classical (A ∨ ¬ A)
lem A = λ not_A_or_not_A → 
  not_A_or_not_A (inr (λ a → not_A_or_not_A (inl a)))

-- Classical intermediate value theorem
classical_ivt : (f : C∞([0,1], ℝ)) → f 0 < 0 → f 1 > 0 → 
               classical (∃ c ∈ [0,1], f c ≡ 0)
classical_ivt f neg pos = 
  -- Uses classical logic to guarantee existence
  classical_proof_by_bisection f neg pos

-- Converting between constructive and classical
make_constructive : {A : Type} → classical A → ComputableEvidence A → A
make_constructive classical_proof evidence = 
  -- Evidence makes the classical proof constructive
  extract_constructive_content classical_proof evidence
```

## 10.3 Smooth Computations {#computations}

### Computational Calculus

SCTT enables exact computation with calculus:

#### Symbolic Differentiation

```sctt
-- Automatic differentiation rules
diff_rules : DifferentiationRules
diff_rules = {
  -- Basic functions
  d/dx[c] = 0,  -- Constant rule
  d/dx[x] = 1,  -- Power rule base case
  d/dx[x^n] = n * x^(n-1),  -- Power rule
  
  -- Arithmetic operations  
  d/dx[f + g] = d/dx[f] + d/dx[g],  -- Sum rule
  d/dx[f * g] = d/dx[f] * g + f * d/dx[g],  -- Product rule  
  d/dx[f / g] = (d/dx[f] * g - f * d/dx[g]) / g^2,  -- Quotient rule
  
  -- Composition
  d/dx[f(g(x))] = d/dx[f](g(x)) * d/dx[g](x),  -- Chain rule
  
  -- Elementary functions
  d/dx[sin(x)] = cos(x),
  d/dx[cos(x)] = -sin(x),
  d/dx[exp(x)] = exp(x),
  d/dx[log(x)] = 1/x
}

-- Automatic application of rules
auto_diff : C∞(ℝ, ℝ) → C∞(ℝ, ℝ)
auto_diff f = apply_diff_rules diff_rules f

-- Example computations
example_derivatives : List (C∞(ℝ, ℝ) × C∞(ℝ, ℝ))
example_derivatives = [
  (λ x → x^3 + 2*x - 1, λ x → 3*x^2 + 2),
  (λ x → sin(x^2), λ x → 2*x*cos(x^2)),
  (λ x → exp(sin(x)), λ x → cos(x) * exp(sin(x)))
]
```

#### Symbolic Integration

```sctt
-- Integration techniques
integration_methods : IntegrationMethods
integration_methods = {
  -- Basic antiderivatives
  ∫ 0 dx = C,
  ∫ 1 dx = x + C,
  ∫ x^n dx = x^(n+1)/(n+1) + C,  -- n ≠ -1
  ∫ 1/x dx = log|x| + C,
  
  -- Elementary functions
  ∫ sin(x) dx = -cos(x) + C,
  ∫ cos(x) dx = sin(x) + C,
  ∫ exp(x) dx = exp(x) + C,
  
  -- Integration techniques
  substitution : ∫ f(g(x)) * g'(x) dx = ∫ f(u) du where u = g(x),
  by_parts : ∫ u * dv = u*v - ∫ v * du,
  partial_fractions : RationalFunction → Sum SimpleRationalFunctions
}

-- Symbolic integration when possible
symbolic_integrate : C∞(ℝ, ℝ) → Maybe C∞(ℝ, ℝ)
symbolic_integrate f = 
  case match_pattern f integration_methods of
    Some antiderivative → Some antiderivative
    None → try_advanced_techniques f

-- Definite integration with verification
definite_integral : (f : C∞([a,b], ℝ)) → ℝ
definite_integral f = 
  case symbolic_integrate f of
    Some F → F b - F a  -- Fundamental theorem
    None → numerical_integration_with_bounds f
```

#### Solving Differential Equations

```sctt
-- ODE solving methods
ode_methods : ODEMethods
ode_methods = {
  separable : (y' = f(x) * g(y)) → (∫ dy/g(y) = ∫ f(x) dx),
  linear_first_order : (y' + P(x)*y = Q(x)) → (integrating_factor_method),
  exact : (M dx + N dy = 0 ∧ ∂M/∂y = ∂N/∂x) → (∃ F, dF = M dx + N dy),
  homogeneous : (y' = f(y/x)) → (substitution v = y/x),
  bernoulli : (y' + P(x)*y = Q(x)*y^n) → (substitution v = y^(1-n))
}

-- Automatic ODE solver
solve_ode : (equation : ODE) → (initial_condition : InitialCondition) → 
           Maybe (Solution equation initial_condition)
solve_ode eq ic = 
  case classify_ode eq of
    SeparableODE → solve_separable eq ic
    LinearFirstOrder → solve_linear_first_order eq ic
    ExactODE → solve_exact eq ic
    -- ... other cases
    UnknownType → Nothing

-- Example: Population growth model
population_growth : (k : ℝ) → (P₀ : ℝ) → C∞(ℝ, ℝ)
population_growth k P₀ = 
  -- Solve dP/dt = k*P, P(0) = P₀
  solution where
    equation = make_ode (λ t P → k * P)
    initial = make_initial 0 P₀
    Some solution = solve_ode equation initial

-- Verification that solution satisfies ODE
verify_solution : (eq : ODE) → (sol : Solution eq ic) → Proof (satisfies sol eq)
verify_solution eq sol = 
  substitute_and_simplify eq sol === zero_function
```

#### Numerical Methods with Bounds

```sctt
-- Numerical integration with error bounds
numerical_integration : (f : C∞([a,b], ℝ)) → (ε : ℝ₊) → 
                       {result : ℝ | |result - ∫ᵃᵇ f dx| ≤ ε}
numerical_integration f ε = 
  adaptive_quadrature f a b ε

-- ODE numerical solving with error control
numerical_ode_solve : (f : C∞(ℝ × ℝ, ℝ)) → (y₀ : ℝ) → (h : ℝ₊) → (T : ℝ) →
                     {approx : C∞([0,T], ℝ) | 
                      |approx(t) - exact_solution(t)| ≤ error_bound(h,t) ∀t ∈ [0,T]}
numerical_ode_solve f y₀ h T = 
  runge_kutta_4 f y₀ h T with_error_analysis

-- Root finding with guaranteed convergence
find_root : (f : C∞(ℝ, ℝ)) → (a b : ℝ) → f a * f b < 0 → 
           {r : ℝ | r ∈ [a,b] ∧ f r ≡ 0}
find_root f a b sign_change = 
  bisection_method f a b sign_change with_convergence_proof

-- Optimization with certificates
minimize : (f : C∞(ℝ, ℝ)) → (domain : ClosedInterval) → 
          {x : ℝ | x ∈ domain ∧ ∀ y ∈ domain, f x ≤ f y}
minimize f domain = 
  critical_points ← find_critical_points f domain
  boundary_points ← evaluate_at_boundary f domain  
  candidates ← critical_points ++ boundary_points
  return (argmin f candidates) with_optimality_proof
```

### Advanced Smooth Programming

#### Manifold Computations

```sctt
-- Working with manifolds
manifold_programming : ManifoldOperations
manifold_programming = {
  -- Tangent space computations
  tangent_vector : (M : Manifold) → (p : M) → TangentSpace M p,
  cotangent_vector : (M : Manifold) → (p : M) → CotangentSpace M p,
  
  -- Differential forms
  exterior_derivative : Ω^k M → Ω^(k+1) M,
  wedge_product : Ω^k M → Ω^l M → Ω^(k+l) M,
  
  -- Integration on manifolds
  integrate_form : (M : OrientedManifold n) → Ω^n M → ℝ,
  
  -- Lie derivatives
  lie_derivative : VectorField M → DifferentialForm M → DifferentialForm M
}

-- Example: Computing curvature
gaussian_curvature : (S : Surface) → (p : S) → ℝ
gaussian_curvature S p = 
  let metric = riemannian_metric S
      christoffel = christoffel_symbols metric
      riemann = riemann_tensor christoffel
  in contract_riemann riemann p

-- Geodesic computation
find_geodesic : (M : RiemannianManifold) → (p q : M) → 
               Path M p q
find_geodesic M p q = 
  solve_geodesic_equation M p q with_minimizing_property
```

#### Partial Differential Equations

```sctt
-- PDE solving framework
pde_solver : PDESolver
pde_solver = {
  -- Classification
  classify_pde : PDE → PDEType,
  
  -- Solution methods
  separation_of_variables : LinearPDE → Maybe AnalyticSolution,
  green_function_method : LinearPDE → BoundaryConditions → Maybe Solution,
  finite_difference : PDE → BoundaryConditions → NumericalSolution,
  finite_element : PDE → BoundaryConditions → Mesh → NumericalSolution
}

-- Example: Heat equation
solve_heat_equation : (initial : C∞(ℝ, ℝ)) → (boundary : BoundaryConditions) →
                     C∞(ℝ × ℝ₊, ℝ)
solve_heat_equation u₀ bc = 
  -- ∂u/∂t = ∂²u/∂x²
  solution where
    pde = make_pde (λ (x,t) u ux uxx ut → ut - uxx)
    solution = separation_of_variables pde u₀ bc

-- Wave equation with verification
solve_wave_equation : (initial_pos : C∞(ℝ, ℝ)) → (initial_vel : C∞(ℝ, ℝ)) →
                     {u : C∞(ℝ × ℝ, ℝ) | ∂²u/∂t² = c² * ∂²u/∂x² ∧ 
                                          u(x,0) = initial_pos(x) ∧
                                          ∂u/∂t(x,0) = initial_vel(x)}
```

## 10.4 Library Development {#libraries}

### Designing SCTT Libraries

Building reusable mathematical components:

#### Library Architecture

```sctt
-- Module system for mathematical libraries
module Algebra.Group where
  -- Group interface
  Group : (G : Type) → Type
  Group G = {
    op : G → G → G,
    identity : G,
    inverse : G → G,
    
    -- Group laws (automatically verified)
    associativity : ∀ a b c, op a (op b c) ≡ op (op a b) c,
    left_identity : ∀ a, op identity a ≡ a,
    right_identity : ∀ a, op a identity ≡ a,
    left_inverse : ∀ a, op (inverse a) a ≡ identity,
    right_inverse : ∀ a, op a (inverse a) ≡ identity
  }
  
  -- Generic group operations
  power : {G : Type} → Group G → G → ℕ → G
  power grp g 0 = grp.identity
  power grp g (n + 1) = grp.op g (power grp g n)
  
  -- Group homomorphisms
  GroupHomomorphism : {G H : Type} → Group G → Group H → (G → H) → Type
  GroupHomomorphism grp_G grp_H f = 
    ∀ a b, f (grp_G.op a b) ≡ grp_H.op (f a) (f b)

-- Extending to rings and fields
module Algebra.Ring where
  import Algebra.Group
  
  Ring : (R : Type) → Type
  Ring R = {
    add_group : Group R,
    mult : R → R → R,
    one : R,
    
    -- Ring laws
    mult_assoc : ∀ a b c, mult a (mult b c) ≡ mult (mult a b) c,
    mult_identity : ∀ a, mult one a ≡ a ∧ mult a one ≡ a,
    distributivity : ∀ a b c, mult a (add_group.op b c) ≡ 
                             add_group.op (mult a b) (mult a c)
  }
```

#### Mathematical Structures

```sctt
-- Linear algebra library
module LinearAlgebra where
  -- Vector spaces
  VectorSpace : (V : Type) → (F : Field) → Type
  VectorSpace V F = {
    add : V → V → V,
    scalar_mult : F.carrier → V → V,
    zero : V,
    
    -- Vector space axioms
    add_comm : ∀ u v, add u v ≡ add v u,
    add_assoc : ∀ u v w, add u (add v w) ≡ add (add u v) w,
    add_identity : ∀ v, add zero v ≡ v,
    scalar_distributivity : ∀ a u v, scalar_mult a (add u v) ≡ 
                                     add (scalar_mult a u) (scalar_mult a v)
    -- ... other axioms
  }
  
  -- Linear transformations
  LinearMap : {V W : Type} → {F : Field} → 
              VectorSpace V F → VectorSpace W F → (V → W) → Type
  LinearMap vs_V vs_W f = 
    (∀ u v, f (vs_V.add u v) ≡ vs_W.add (f u) (f v)) ∧
    (∀ a v, f (vs_V.scalar_mult a v) ≡ vs_W.scalar_mult a (f v))
  
  -- Matrix representation
  Matrix : (m n : ℕ) → (F : Field) → Type
  Matrix m n F = Vec (Vec F.carrier n) m
  
  -- Matrix operations with dimension verification
  matrix_mult : {m n p : ℕ} {F : Field} → 
                Matrix m n F → Matrix n p F → Matrix m p F
  matrix_mult A B = -- Implementation with automatic dimension checking
```

#### Computational Geometry

```sctt
-- Computational geometry library
module Geometry where
  -- Points and vectors
  Point : (n : ℕ) → Type
  Point n = Vec ℝ n
  
  Vector : (n : ℕ) → Type  
  Vector n = Vec ℝ n
  
  -- Geometric operations
  distance : {n : ℕ} → Point n → Point n → ℝ
  distance p q = sqrt (sum (map (λ x → x^2) (zipWith (-) p q)))
  
  dot_product : {n : ℕ} → Vector n → Vector n → ℝ
  dot_product u v = sum (zipWith (*) u v)
  
  cross_product : Vector 3 → Vector 3 → Vector 3
  cross_product [u₁, u₂, u₃] [v₁, v₂, v₃] = 
    [u₂*v₃ - u₃*v₂, u₃*v₁ - u₁*v₃, u₁*v₂ - u₂*v₁]
  
  -- Convex hull with verified correctness
  convex_hull : (points : List (Point 2)) → 
               {hull : List (Point 2) | 
                IsConvexHull hull points ∧ 
                ∀ p ∈ points, InsideOrOnBoundary p hull}
  convex_hull points = graham_scan points with_correctness_proof
```

#### Scientific Computing

```sctt
-- Scientific computing library
module Scientific where
  -- Numerical linear algebra
  solve_linear_system : {n : ℕ} → Matrix n n ℝ → Vector n ℝ → 
                       Maybe (Vector n ℝ)
  solve_linear_system A b = 
    if det A ≠ 0 
      then Some (gaussian_elimination A b)
      else None
  
  -- Eigenvalue computation
  eigenvalues : {n : ℕ} → Matrix n n ℝ → List ℝ
  eigenvalues A = roots (characteristic_polynomial A)
  
  -- Optimization
  minimize_function : (f : C∞(ℝⁿ, ℝ)) → (domain : ConvexSet ℝⁿ) →
                     {x : ℝⁿ | x ∈ domain ∧ LocalMinimum f x}
  minimize_function f domain = 
    gradient_descent f domain with_convergence_guarantees
  
  -- Statistics with verified properties
  mean : (xs : List ℝ) → xs ≠ [] → ℝ
  mean xs proof = (sum xs) / (fromNat (length xs))
  
  variance : (xs : List ℝ) → xs ≠ [] → ℝ  
  variance xs proof = 
    let μ = mean xs proof
    in mean (map (λ x → (x - μ)^2) xs) (map_preserves_nonempty xs proof)
```

### API Design Principles

#### Type-Driven Design

```sctt
-- Design APIs with precise types
api_design_principles : APIDesignPrinciples
api_design_principles = {
  -- Make illegal states unrepresentable
  divide : (x y : ℝ) → y ≠ 0 → ℝ,
  
  -- Use dependent types for preconditions
  array_access : {A : Type} {n : ℕ} → Array A n → (i : ℕ) → i < n → A,
  
  -- Return evidence of postconditions
  sort : {A : Type} → (compare : A → A → Ordering) → (xs : List A) → 
         {ys : List A | Sorted compare ys ∧ Permutation xs ys},
  
  -- Use phantom types for units
  Meters : Type
  Meters = ℝ
  
  Seconds : Type 
  Seconds = ℝ
  
  -- Prevent unit confusion
  speed : Meters → Seconds → MetersPerSecond,
  
  -- Make effects explicit in types
  IO : Type → Type,
  Random : Type → Type,
  State S : Type → Type
}

-- Example: Safe array library
module SafeArray where
  -- Array with compile-time size
  Array : Type → ℕ → Type
  
  -- Safe operations
  get : {A : Type} {n : ℕ} → Array A n → (i : Fin n) → A
  set : {A : Type} {n : ℕ} → Array A n → (i : Fin n) → A → Array A n
  
  -- Bounds-checked indexing
  safe_get : {A : Type} {n : ℕ} → Array A n → ℕ → Maybe A
  safe_get arr i = 
    if i < n 
      then Some (get arr (fromNat i))
      else None
  
  -- Array operations preserve size
  map : {A B : Type} {n : ℕ} → (A → B) → Array A n → Array B n
  zipWith : {A B C : Type} {n : ℕ} → (A → B → C) → 
            Array A n → Array B n → Array C n
```

#### Error Handling

```sctt
-- Principled error handling in SCTT
module ErrorHandling where
  -- Result type for fallible operations
  Result : Type → Type → Type
  Result E A = E ⊎ A  -- Either error or success
  
  -- Maybe for operations that might fail
  Maybe : Type → Type
  Maybe A = Unit ⊎ A
  
  -- Monadic error handling
  bind : {E A B : Type} → Result E A → (A → Result E B) → Result E B
  bind (inl error) f = inl error
  bind (inr value) f = f value
  
  -- Error propagation with context
  try : {A : Type} → IO A → IO (Result IOError A)
  
  -- Validated inputs
  Validated : Type → Type → Type
  Validated E A = List E ⊎ A  -- Accumulate all errors
  
  -- Example: Parsing with detailed errors
  ParseError : Type
  ParseError = {
    position : ℕ,
    expected : String,
    actual : String
  }
  
  parse_number : String → Result ParseError ℝ
  parse_number s = -- Implementation with specific error reporting
```

## 10.5 Best Practices {#practices}

### Code Organization

#### Project Structure

```sctt
-- Recommended SCTT project structure
project_structure : ProjectStructure
project_structure = {
  src/ = {
    -- Core mathematical definitions
    Core/ = {
      Types.sctt,      -- Basic type definitions
      Algebra.sctt,    -- Algebraic structures  
      Analysis.sctt,   -- Analytical structures
      Topology.sctt    -- Topological structures
    },
    
    -- Domain-specific modules
    Domains/ = {
      Physics.sctt,    -- Physics applications
      Engineering.sctt, -- Engineering applications
      Economics.sctt   -- Economic models
    },
    
    -- Algorithms and implementations
    Algorithms/ = {
      Numerical.sctt,  -- Numerical methods
      Optimization.sctt, -- Optimization algorithms
      Solvers.sctt     -- Equation solvers
    },
    
    -- Examples and applications
    Examples/ = {
      BasicExamples.sctt,
      AdvancedExamples.sctt,
      Benchmarks.sctt
    }
  },
  
  tests/ = {
    -- Property-based tests
    Properties.sctt,
    -- Unit tests  
    UnitTests.sctt,
    -- Integration tests
    Integration.sctt
  },
  
  docs/ = {
    -- API documentation
    API.md,
    -- Tutorials
    Tutorial.md,
    -- Mathematical background
    Mathematics.md
  }
}
```

#### Module Design

```sctt
-- Well-designed SCTT module
module WellDesigned where
  -- Clear documentation
  {-|
  This module provides verified implementations of basic calculus operations.
  
  All functions come with mathematical correctness guarantees:
  - Derivatives are computed exactly using symbolic differentiation
  - Integrals are computed symbolically when possible, numerically with bounds otherwise
  - All operations preserve smooth structure
  -}
  
  -- Export list with documentation
  exports {
    -- Differentiation
    diff : C∞(ℝ, ℝ) → C∞(ℝ, ℝ),
    partial : {n : ℕ} → C∞(ℝⁿ, ℝ) → Fin n → C∞(ℝⁿ, ℝ),
    
    -- Integration
    integrate : C∞([a,b], ℝ) → ℝ,
    numerical_integrate : C∞([a,b], ℝ) → (ε : ℝ₊) → 
                         {result : ℝ | |result - exact| ≤ ε},
    
    -- Differential equations
    solve_ode : FirstOrderODE → InitialCondition → Solution
  }
  
  -- Internal helper functions (not exported)
  private {
    symbolic_diff_rules : DifferentiationRules,
    integration_table : IntegrationTable,
    ode_classification : ODE → ODEType
  }
```

### Performance Guidelines

#### Efficient Smooth Computing

```sctt
-- Performance best practices
performance_guidelines : PerformanceGuidelines
performance_guidelines = {
  -- Use specialized types for efficiency
  efficient_types = {
    -- Prefer polynomial representations when possible
    use_polynomial : C∞(ℝ, ℝ) → Polynomial + GeneralFunction,
    
    -- Use rational functions for quotients
    use_rational : (p q : Polynomial) → RationalFunction p q,
    
    -- Cache expensive computations
    memoize : (f : A → B) → MemoizedFunction A B
  },
  
  -- Lazy evaluation for large expressions
  lazy_evaluation = {
    -- Delay normalization until needed
    lazy_normalize : Expression → LazyExpression,
    
    -- Stream processing for infinite series
    infinite_series : (a : ℕ → ℝ) → Stream ℝ,
    
    -- Partial evaluation
    partial_apply : (f : A → B → C) → A → (B → C)
  },
  
  -- Parallel computation where appropriate
  parallelization = {
    -- Independent computations
    parallel_map : (A → B) → List A → Par (List B),
    
    -- Parallel matrix operations
    parallel_matrix_mult : Matrix m n → Matrix n p → Par (Matrix m p),
    
    -- Distributed integration
    distributed_integrate : C∞([a,b], ℝ) → Par ℝ
  }
}

-- Example: Efficient polynomial operations
efficient_polynomial : EfficientPolynomial
efficient_polynomial = {
  -- Use dense representation for small degree
  dense_poly : (degree < 10) → DensePolynomial,
  
  -- Use sparse representation for large degree
  sparse_poly : (degree ≥ 10) → SparsePolynomial,
  
  -- Specialized multiplication algorithms
  mult_algorithm : (p q : Polynomial) → 
    if (degree p * degree q < 100)
      then schoolbook_multiplication p q
      else karatsuba_multiplication p q,
  
  -- Fast evaluation using Horner's method
  eval_horner : Polynomial → ℝ → ℝ
}
```

#### Memory Management

```sctt
-- Memory-efficient SCTT programming
memory_management : MemoryManagement
memory_management = {
  -- Avoid memory leaks in recursive functions
  tail_recursion = {
    -- Use accumulator pattern
    factorial_acc : ℕ → ℕ → ℕ,
    factorial_acc n acc = 
      case n of
        0 → acc
        succ n' → factorial_acc n' (acc * succ n'),
    
    -- Convert to tail recursive form when possible
    optimize_recursion : RecursiveFunction → TailRecursiveFunction
  },
  
  -- Reuse memory where possible
  in_place_operations = {
    -- Destructive updates when safe
    array_update : Array A n → Fin n → A → Array A n,
    
    -- Reference counting for shared data
    shared_data : A → Ref A,
    
    -- Copy-on-write for large structures
    cow_array : Array A n → COWArray A n
  },
  
  -- Garbage collection hints
  gc_management = {
    -- Force collection at appropriate points
    collect_garbage : IO Unit,
    
    -- Weak references for cyclic data
    weak_ref : A → WeakRef A,
    
    -- Resource management with linear types
    with_resource : Resource → (Resource → A) → A
  }
}
```

### Testing and Verification

#### Property-Based Testing

```sctt
-- Property-based testing in SCTT
module PropertyTesting where
  -- Generators for test data
  Generator : Type → Type
  
  -- Basic generators
  gen_nat : Generator ℕ
  gen_real : Generator ℝ  
  gen_list : Generator A → Generator (List A)
  gen_function : Generator A → Generator B → Generator (A → B)
  
  -- Properties as predicates
  Property : Type
  Property = ∀ (input : TestInput), TestResult
  
  -- Testing differential properties
  derivative_property : Property
  derivative_property f x = 
    -- Test that derivative is computed correctly
    let h = 0.0001
        numerical_derivative = (f (x + h) - f x) / h
        symbolic_derivative = D[f] x
    in abs (numerical_derivative - symbolic_derivative) < 0.001
  
  -- Testing algebraic properties
  commutativity_property : (A : Type) → (op : A → A → A) → Property
  commutativity_property A op x y = 
    op x y ≡ op y x
  
  -- Monadic property testing
  property_test : Property → Generator TestInput → TestResult
  property_test prop gen = 
    samples ← replicate 1000 gen
    results ← map prop samples
    return (all results)
```

#### Correctness Verification

```sctt
-- Verification strategies in SCTT
verification_strategies : VerificationStrategies
verification_strategies = {
  -- Static verification via types
  static_verification = {
    -- Preconditions in function types
    safe_div : (x y : ℝ) → y ≠ 0 → ℝ,
    
    -- Postconditions in return types
    sort : List A → {result : List A | Sorted result ∧ Permutation input result},
    
    -- Invariants in data types
    BST : {tree : Tree A | SearchTreeInvariant tree}
  },
  
  -- Dynamic verification via runtime checks
  dynamic_verification = {
    -- Assertions with proof obligations
    assert : (P : Prop) → Proof P → Unit,
    
    -- Contract checking
    contract : Precondition → Postcondition → (A → B) → (A → B),
    
    -- Monitoring invariants
    monitor_invariant : Invariant → Computation → VerifiedComputation
  },
  
  -- Formal verification via proof assistants
  formal_verification = {
    -- Interactive proof development
    prove_interactively : Theorem → InteractiveProof,
    
    -- Automated proof search
    auto_prove : Theorem → Maybe Proof,
    
    -- Proof by reflection
    prove_by_computation : Theorem → ComputationalEvidence → Proof
  }
}
```

### Documentation and Literate Programming

#### Literate SCTT

```sctt
-- Literate programming in SCTT
literate_programming : LiterateProgramming
literate_programming = {
  {-| 
  # Computing the Square Root Function
  
  We implement Newton's method for computing square roots with verified convergence.
  
  ## Mathematical Background
  
  Newton's method for finding roots of f(x) = x² - a uses the iteration:
  x_{n+1} = (x_n + a/x_n) / 2
  
  ## Convergence Analysis
  
  For x₀ > 0 and a > 0, the sequence converges quadratically to √a.
  -}
  
  sqrt_newton : (a : ℝ) → a > 0 → (x₀ : ℝ) → x₀ > 0 → 
               {result : ℝ | result² ≡ a}
  sqrt_newton a a_pos x₀ x₀_pos = 
    {-| 
    The iteration function improves the approximation:
    -}
    iterate : ℝ → ℝ
    iterate x = (x + a / x) / 2
    
    {-|
    We prove convergence using the fact that the sequence is monotonically
    decreasing and bounded below by √a.
    -}
    convergence_proof : Proof (converges_to iterate √a)
    convergence_proof = -- Detailed mathematical proof
    
    {-|
    The limit is computed as the fixed point of the iteration:
    -}
    limit iterate with_convergence_proof convergence_proof
  
  {-|
  ## Example Usage
  
  Computing √2 with initial guess 1:
  -}
  sqrt_2 : ℝ
  sqrt_2 = sqrt_newton 2 (by_computation 2 > 0) 1 (by_computation 1 > 0)
  
  {-|
  ## Verification
  
  We can verify that our result is correct:
  -}
  sqrt_2_correct : sqrt_2² ≡ 2
  sqrt_2_correct = by_computation
}
```

## Summary

Programming in SCTT represents a paradigm shift toward mathematical computation with guarantees:

### Key Programming Principles

1. **Types as Specifications**: Every function type precisely specifies what the function does
2. **Proofs as Programs**: Mathematical reasoning becomes executable code
3. **Dependent Types for Safety**: Impossible states become unrepresentable
4. **Smooth Computing**: Calculus operations are first-class programming constructs
5. **Verification by Construction**: Correctness is built into the program structure

### Programming Paradigms Unified

- **Functional Programming**: Pure functions with mathematical semantics
- **Logic Programming**: Proof search via type inhabitation
- **Numerical Programming**: Exact symbolic computation with verified numerics
- **Geometric Programming**: Direct manipulation of mathematical spaces
- **Modal Programming**: Different computational contexts and modes

### Best Practices Established

- **API Design**: Make illegal states unrepresentable, express invariants in types
- **Performance**: Use appropriate representations, lazy evaluation, parallelization  
- **Testing**: Property-based testing with mathematical properties
- **Documentation**: Literate programming with mathematical explanations
- **Library Structure**: Modular design with clear mathematical abstractions

SCTT programming transforms software development into mathematical construction—where every program is a theorem, every execution is a proof, and every computation comes with certainty.

## Exercises

### Basic Programming
1. Implement a verified sorting algorithm with proof that it produces sorted output
2. Create a safe array library that prevents bounds errors at compile time
3. Design a calculator that tracks units and prevents unit conversion errors
4. Build a rational number arithmetic library with automatic simplification

### Mathematical Programming  
1. Implement symbolic differentiation for a small calculus language
2. Create a verified matrix multiplication that checks dimension compatibility
3. Build an ODE solver with convergence guarantees
4. Design a geometry library with verified geometric algorithms

### Advanced Applications
1. Create a physics simulation with verified conservation laws
2. Build a financial modeling library with risk analysis
3. Implement a cryptographic library with security proofs
4. Design a machine learning library with mathematical guarantees

### Library Development
1. Design and implement a comprehensive linear algebra library
2. Create a scientific computing library with verified numerical methods
3. Build a computer graphics library with geometric correctness proofs
4. Develop a domain-specific language embedded in SCTT

---

*Next: [Chapter 11: Scientific Computing](./chapter_11.md) →*

*Previous: [Chapter 9: Type Checking Algorithm](./chapter_09.md) ←*