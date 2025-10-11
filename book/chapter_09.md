# Chapter 9: Type Checking Algorithm

> "Algorithms are the poetry of computation." — Donald Knuth
>
> "In SCTT, type checking becomes a conversation between mathematics and machine, where every step is both rigorous and executable."

## Introduction

Type checking is where the rubber meets the road in type theory—it's the algorithmic heart that determines whether our mathematical expressions are meaningful. For SCTT, type checking becomes especially sophisticated because we must handle dependent types, cubical paths, and smooth operations simultaneously.

This chapter presents the complete type checking algorithm for SCTT:

1. **Bidirectional Type Checking** - Efficient inference and checking modes
2. **Constraint Solving** - Handling unification and smooth equations  
3. **Normalization by Evaluation** - Computing with mathematical expressions
4. **Caching and Optimization** - Making type checking practical
5. **Implementation** - From theory to working code

The key insight is that SCTT's type checker is itself a mathematical object—we can prove its correctness, analyze its complexity, and optimize its performance while maintaining trustworthiness.

### The Challenge of SCTT Type Checking

```sctt
-- Traditional type checking challenges
dependent_types : Challenge
dependent_types = "Types depend on terms, requiring evaluation"

-- Cubical challenges  
cubical_paths : Challenge
cubical_paths = "Path types require dimensional analysis"

-- Smooth challenges
smooth_operations : Challenge  
smooth_operations = "Derivatives and integrals must type check"

-- Integration challenge
unified_checking : Challenge
unified_checking = "All three systems must work together seamlessly"
```

### The SCTT Type Checker Architecture

```sctt
-- High-level type checker structure
SCTTTypeChecker : System
SCTTTypeChecker = {
  -- Core bidirectional engine
  infer : Context → Term → Maybe Type,
  check : Context → Term → Type → Bool,
  
  -- Constraint solving
  unify : Type → Type → Maybe Substitution,
  solve_smooth : SmoothConstraint → Maybe SmoothSolution,
  
  -- Normalization engine
  normalize : Context → Term → NormalForm,
  definitional_equality : Context → Term → Term → Type → Bool,
  
  -- Optimization layers
  cache : InferenceCache,
  parallel : ParallelizationStrategy,
  approximation : ApproximationMethods
}
```

## 9.1 Bidirectional Type Checking {#bidirectional}

### The Bidirectional Framework

Bidirectional type checking separates inference (synthesizing types) from checking (verifying against expected types):

```sctt
-- Bidirectional judgments
infer_judgment : Context → Term → Type → Type₁
infer_judgment Γ t A = (Γ ⊢ t ⇒ A)  -- t synthesizes type A

check_judgment : Context → Term → Type → Type₁  
check_judgment Γ t A = (Γ ⊢ t ⇐ A)  -- t checks against type A

-- Mode switching
switch_to_check : (Γ ⊢ t ⇒ A) → (A ≡ B) → (Γ ⊢ t ⇐ B)
switch_to_infer : (Γ ⊢ t ⇐ A) → (Γ ⊢ (t : A) ⇒ A)
```

#### Core Inference Rules

```sctt
-- Variable inference
infer_var : (x : A) ∈ Γ → (Γ ⊢ x ⇒ A)

-- Application inference  
infer_app : (Γ ⊢ f ⇒ (x : A) → B) → (Γ ⊢ a ⇐ A) → (Γ ⊢ f a ⇒ B[a/x])

-- Universe inference
infer_type : Γ ⊢ Type₀ ⇒ Type₁

-- Annotation inference
infer_ann : (Γ ⊢ A ⇐ Type) → (Γ ⊢ t ⇐ A) → (Γ ⊢ (t : A) ⇒ A)
```

#### Core Checking Rules

```sctt
-- Lambda checking
check_lam : (Γ, x : A ⊢ body ⇐ B) → (Γ ⊢ λ x → body ⇐ (x : A) → B)

-- Pi checking  
check_pi : (Γ ⊢ A ⇐ Type) → (Γ, x : A ⊢ B ⇐ Type) → (Γ ⊢ (x : A) → B ⇐ Type)

-- Conversion rule
check_conv : (Γ ⊢ t ⇒ A) → (A ≡ B) → (Γ ⊢ t ⇐ B)
```

### SCTT-Specific Inference Rules

#### Path Type Inference

```sctt
-- Path literal inference
infer_path_lit : 
  (Γ, i : I ⊢ body ⇒ A) →
  (body[i0/i] ≡ x) → (body[i1/i] ≡ y) →
  (Γ ⊢ λ i → body ⇒ Path A x y)

-- Path application inference
infer_path_app :
  (Γ ⊢ p ⇒ Path A x y) → (Γ ⊢ i ⇐ I) →
  (Γ ⊢ p @ i ⇒ A)

-- Composition inference
infer_comp :
  (Γ ⊢ A ⇐ I → Type) → (Γ ⊢ φ ⇐ Formula) →
  (Γ ⊢ u ⇐ PartialPath A φ) → (Γ ⊢ u0 ⇐ A i0[φ ↦ u i0]) →
  (Γ ⊢ comp A φ u u0 ⇒ A i1)
```

#### Smooth Type Inference

```sctt
-- Smooth function inference
infer_smooth_fun :
  (Γ ⊢ M ⇐ SmoothType) → (Γ ⊢ N ⇐ SmoothType) →
  (Γ ⊢ C∞(M,N) ⇒ SmoothType)

-- Derivative inference
infer_derivative :
  (Γ ⊢ f ⇒ C∞(M,N)) →
  (Γ ⊢ D[f] ⇒ C∞(TM,TN))

-- Integration inference (when definable)
infer_integral :
  (Γ ⊢ f ⇒ C∞([a,b], ℝ)) → (Γ ⊢ a ⇐ ℝ) → (Γ ⊢ b ⇐ ℝ) →
  (Γ ⊢ ∫ᵃᵇ f dx ⇒ ℝ)

-- Smooth function application
infer_smooth_app :
  (Γ ⊢ f ⇒ C∞(M,N)) → (Γ ⊢ x ⇐ M) →
  (Γ ⊢ f x ⇒ N)
```

### Algorithm Implementation

```sctt
-- Core bidirectional algorithm
mutual 
  infer : Context → Term → TypeChecker Type
  infer Γ t = case t of
    Var x → 
      lookup x Γ |> maybe_to_error "Variable not in scope"
    
    App f a → do
      fun_type ← infer Γ f
      (dom, cod) ← expect_function_type fun_type
      check Γ a dom
      return (cod a)  -- Apply substitution
    
    Ann t A → do
      check Γ A Type
      check Γ t A  
      return A
      
    -- Path-specific cases
    PathLit i body → do
      body_type ← infer (Γ, i : I) body
      x ← eval (substitute body i0)
      y ← eval (substitute body i1)
      return (Path body_type x y)
      
    PathApp p i → do
      path_type ← infer Γ p
      (A, x, y) ← expect_path_type path_type
      check Γ i I
      return (A i)  -- Dependent path type
      
    -- Smooth-specific cases  
    SmoothApp f x → do
      fun_type ← infer Γ f
      (M, N) ← expect_smooth_function_type fun_type
      check Γ x M
      return N
      
    Derivative f → do
      fun_type ← infer Γ f
      (M, N) ← expect_smooth_function_type fun_type
      return (C∞(TangentBundle M, TangentBundle N))
      
    -- Error cases
    _ → type_error "Cannot infer type"

  check : Context → Term → Type → TypeChecker Unit  
  check Γ t A = case t of
    Lam x body → do
      (dom, cod) ← expect_function_type A
      check (Γ, x : dom) body cod
      
    -- Conversion: switch to inference mode
    _ → do
      inferred ← infer Γ t
      definitional_equal Γ inferred A
```

## 9.2 Constraint Solving {#constraints}

### Unification in SCTT

Type checking often generates constraints that must be solved:

```sctt
-- Constraint types
data Constraint = 
  UnifyTypes Type Type |
  SmoothEquality SmoothTerm SmoothTerm |
  PathEquality PathTerm PathTerm |
  DimensionalConstraint Formula |
  ConvergenceConstraint IntegralTerm

-- Constraint solving
solve_constraints : List Constraint → TypeChecker Substitution
solve_constraints [] = return empty_substitution
solve_constraints (c::cs) = do
  partial_solution ← solve_constraint c
  remaining ← apply_substitution partial_solution cs
  rest_solution ← solve_constraints remaining
  return (compose partial_solution rest_solution)
```

#### Type Unification

```sctt
-- Standard unification algorithm
unify : Type → Type → TypeChecker Substitution
unify A B = case (A, B) of
  -- Identical types
  (Type, Type) → return empty_substitution
  (Var x, Var y) | x ≡ y → return empty_substitution
  
  -- Meta-variable cases
  (Meta m, A) → solve_meta m A
  (A, Meta m) → solve_meta m A
  
  -- Structural cases
  (Pi x A₁ B₁, Pi y A₂ B₂) → do
    sub1 ← unify A₁ A₂  
    sub2 ← unify (apply sub1 B₁) (apply sub1 B₂)
    return (compose sub1 sub2)
    
  (Path A₁ x₁ y₁, Path A₂ x₂ y₂) → do
    sub1 ← unify A₁ A₂
    sub2 ← unify (apply sub1 x₁) (apply sub1 x₂)  
    sub3 ← unify (apply sub2 y₁) (apply sub2 y₂)
    return (compose sub1 (compose sub2 sub3))
    
  -- Smooth function types
  (C∞(M₁,N₁), C∞(M₂,N₂)) → do
    sub1 ← unify_smooth_types M₁ M₂
    sub2 ← unify_smooth_types N₁ N₂  
    return (compose sub1 sub2)
    
  -- Failure cases
  _ → unification_error A B
```

#### Smooth Constraint Solving

```sctt
-- Solving smooth equality constraints
solve_smooth_equality : SmoothTerm → SmoothTerm → TypeChecker Substitution
solve_smooth_equality f g = 
  case (f, g) of
    -- Syntactic equality
    (t, u) | t ≡ u → return empty_substitution
    
    -- Polynomial equality (decidable)
    (Polynomial p, Polynomial q) → 
      solve_polynomial_equality p q
      
    -- Derivative equality
    (Derivative h₁, Derivative h₂) →
      solve_smooth_equality h₁ h₂
      
    -- Composition equality  
    (Compose f₁ g₁, Compose f₂ g₂) → do
      sub1 ← solve_smooth_equality f₁ f₂
      sub2 ← solve_smooth_equality g₁ g₂
      return (compose sub1 sub2)
      
    -- Meta-variable cases
    (SmoothMeta m, t) → solve_smooth_meta m t
    (t, SmoothMeta m) → solve_smooth_meta m t
    
    -- Undecidable cases - use approximation
    (f, g) → approximate_smooth_equality f g

-- Approximation for undecidable cases
approximate_smooth_equality : SmoothTerm → SmoothTerm → TypeChecker Substitution  
approximate_smooth_equality f g = do
  samples ← generate_test_points domain
  agreement ← all samples (\x → abs(f(x) - g(x)) < epsilon)
  if agreement 
    then return empty_substitution -- Assume equal
    else unification_error f g
```

#### Path Constraint Solving

```sctt
-- Path equality constraints
solve_path_equality : PathTerm → PathTerm → TypeChecker Substitution
solve_path_equality p q = case (p, q) of
  -- Syntactic equality
  (path, path') | path ≡ path' → return empty_substitution
  
  -- Composition paths
  (Compose p₁ p₂, path) → 
    solve_path_equality (compute_path_composition p₁ p₂) path
    
  -- Constant paths
  (ConstantPath x, ConstantPath y) →
    unify x y
    
  -- Reverse paths
  (Reverse p, Reverse q) →
    solve_path_equality p q
    
  -- Meta-variables
  (PathMeta m, p) → solve_path_meta m p
  (p, PathMeta m) → solve_path_meta m p
  
  -- Homotopy equivalence (higher-dimensional)
  (p, q) → solve_homotopy_constraint p q

-- Higher-dimensional constraint solving
solve_homotopy_constraint : PathTerm → PathTerm → TypeChecker Substitution
solve_homotopy_constraint p q = do
  -- Check if paths are homotopic
  homotopy ← find_homotopy p q
  case homotopy of
    Some h → return empty_substitution -- Paths are equal up to homotopy
    None → unification_error p q
```

### Dimensional Analysis

```sctt
-- Constraint solving for cubical dimensions
solve_dimensional_constraints : List Formula → TypeChecker Substitution
solve_dimensional_constraints formulas = do
  -- Convert to boolean algebra
  boolean_constraints ← map formula_to_boolean formulas
  
  -- Solve boolean satisfiability
  solution ← sat_solve boolean_constraints
  
  case solution of
    Satisfiable assignment → return (assignment_to_substitution assignment)
    Unsatisfiable → constraint_error "Inconsistent dimensional constraints"
    Unknown → timeout_error "Dimensional constraint solving timeout"

-- Example: solving (i = i0) ∨ (i = i1) 
dimensional_example : TypeChecker Substitution
dimensional_example = 
  solve_dimensional_constraints [(i = i0) ∨ (i = i1)]
  -- Result: {i ↦ i0} or {i ↦ i1}
```

## 9.3 Normalization by Evaluation {#nbe}

### The NBE Algorithm

Normalization by Evaluation evaluates terms to semantic values, then quotes back to normal forms:

```sctt
-- NBE structure
data Value = 
  VNeutral Neutral |
  VLam (Value → Value) |
  VPi Value (Value → Value) |
  VPath (Interval → Value) |
  VSmoothFunction SmoothValue |
  VReal RealValue

data Neutral = 
  NVar Variable |
  NApp Neutral Value |
  NPathApp Neutral Value |
  NSmoothApp Neutral Value

-- Main NBE functions
evaluate : Environment → Term → Value
quote : Context → Value → Term
normalize : Context → Term → Term
normalize Γ t = quote Γ (evaluate (env_from_context Γ) t)
```

#### Evaluation

```sctt
-- Evaluation function
evaluate env term = case term of
  Var x → 
    lookup x env |> maybe (NVar x |> VNeutral)
    
  Lam x body →
    VLam (\v → evaluate (extend env x v) body)
    
  App f a → 
    let vf = evaluate env f
        va = evaluate env a
    in apply_value vf va
    
  -- Path evaluation
  PathLit i body →
    VPath (\interval → evaluate (extend env i (VInterval interval)) body)
    
  PathApp p i →
    let vp = evaluate env p
        vi = evaluate env i  
    in apply_path vp vi
    
  -- Smooth evaluation
  SmoothApp f x →
    let vf = evaluate env f
        vx = evaluate env x
    in apply_smooth vf vx
    
  Derivative f →
    let vf = evaluate env f
    in compute_derivative_value vf

-- Value application
apply_value : Value → Value → Value
apply_value f a = case f of
  VLam body → body a
  VNeutral neu → VNeutral (NApp neu a)
  _ → evaluation_error "Not a function"

-- Path application  
apply_path : Value → Value → Value
apply_path p i = case p of
  VPath body → body (extract_interval i)
  VNeutral neu → VNeutral (NPathApp neu i)
  _ → evaluation_error "Not a path"

-- Smooth application
apply_smooth : Value → Value → Value  
apply_smooth f x = case f of
  VSmoothFunction body → body (extract_smooth_value x)
  VNeutral neu → VNeutral (NSmoothApp neu x)
  _ → evaluation_error "Not a smooth function"
```

#### Quotation

```sctt
-- Quotation function
quote ctx value = case value of
  VNeutral neu → quote_neutral ctx neu
  
  VLam body → 
    let x = fresh_var ctx
        var_val = VNeutral (NVar x)
        body_val = body var_val
        body_term = quote (extend ctx x) body_val
    in Lam x body_term
    
  VPi dom cod →
    let dom_term = quote ctx dom
        x = fresh_var ctx  
        var_val = VNeutral (NVar x)
        cod_val = cod var_val
        cod_term = quote (extend ctx x) cod_val
    in Pi x dom_term cod_term
    
  VPath body →
    let i = fresh_interval_var ctx
        interval_val = VInterval i
        body_val = body interval_val
        body_term = quote (extend_interval ctx i) body_val
    in PathLit i body_term
    
  VSmoothFunction body →
    quote_smooth_function ctx body
    
  VReal r →
    RealLiteral r

-- Neutral quotation
quote_neutral ctx neu = case neu of
  NVar x → Var x
  NApp neu val → App (quote_neutral ctx neu) (quote ctx val)
  NPathApp neu val → PathApp (quote_neutral ctx neu) (quote ctx val)
  NSmoothApp neu val → SmoothApp (quote_neutral ctx neu) (quote ctx val)
```

### Smooth NBE

Special handling for smooth operations:

```sctt
-- Smooth value representation
data SmoothValue = 
  SPolynomial Polynomial |
  SRational RationalFunction |
  SElementary ElementaryFunction |
  SDerivative SmoothValue |
  SComposition SmoothValue SmoothValue |
  SNeutral SmoothNeutral

-- Derivative computation in values
compute_derivative_value : Value → Value
compute_derivative_value v = case v of
  VSmoothFunction (SPolynomial p) → 
    VSmoothFunction (SPolynomial (derivative_polynomial p))
    
  VSmoothFunction (SElementary Sin) →
    VSmoothFunction (SElementary Cos)
    
  VSmoothFunction (SElementary Cos) →
    VSmoothFunction (SComposition (SElementary (Constant (-1))) (SElementary Sin))
    
  VSmoothFunction (SComposition f g) →
    -- Chain rule: (f ∘ g)' = (f' ∘ g) * g'
    let df = compute_derivative_value (VSmoothFunction f)
        dg = compute_derivative_value (VSmoothFunction g)
        df_g = compose_smooth_values df (VSmoothFunction g)  
    in multiply_smooth_values df_g dg
    
  VNeutral neu → VNeutral (NSmoothDerivative neu)

-- Smooth function quotation
quote_smooth_function : Context → SmoothValue → Term
quote_smooth_function ctx sv = case sv of
  SPolynomial p → PolynomialLit p
  SRational r → RationalLit r
  SElementary f → ElementaryLit f
  SDerivative v → Derivative (quote_smooth_function ctx v)
  SComposition f g → 
    Compose (quote_smooth_function ctx f) (quote_smooth_function ctx g)
  SNeutral neu → quote_smooth_neutral ctx neu
```

### NBE for Definitional Equality

```sctt
-- Definitional equality via NBE
definitional_equal : Context → Term → Term → Type → Bool
definitional_equal Γ t₁ t₂ A = 
  let n₁ = normalize Γ t₁
      n₂ = normalize Γ t₂
  in syntactic_equal n₁ n₂

-- Optimized equality for special cases
optimized_definitional_equal : Context → Term → Term → Type → Bool
optimized_definitional_equal Γ t₁ t₂ A = 
  -- Quick syntactic check first
  if syntactic_equal t₁ t₂ then true
  else case A of
    -- Smooth function equality
    C∞(M,N) → smooth_function_equal Γ t₁ t₂
    
    -- Path equality  
    Path B x y → path_equal Γ t₁ t₂ B
    
    -- General case
    _ → definitional_equal Γ t₁ t₂ A

-- Smooth function equality (special handling)
smooth_function_equal : Context → Term → Term → Bool
smooth_function_equal Γ f g = 
  case (normalize Γ f, normalize Γ g) of
    -- Polynomial equality is decidable
    (PolynomialLit p, PolynomialLit q) → polynomial_equal p q
    
    -- Elementary function equality
    (ElementaryLit e₁, ElementaryLit e₂) → elementary_equal e₁ e₂
    
    -- Derivative equality
    (Derivative f', Derivative g') → smooth_function_equal Γ f' g'
    
    -- Composition equality
    (Compose f₁ g₁, Compose f₂ g₂) → 
      smooth_function_equal Γ f₁ f₂ && smooth_function_equal Γ g₁ g₂
      
    -- Undecidable cases - approximate
    (f', g') → approximate_smooth_equal Γ f' g'
```

## 9.4 Caching and Optimization {#optimization}

### Intelligent Caching

```sctt
-- Cache structure
data TypeCheckCache = TypeCheckCache {
  type_cache : Map Term Type,
  normal_form_cache : Map Term Term,
  equality_cache : Map (Term, Term, Type) Bool,
  smooth_derivative_cache : Map SmoothTerm SmoothTerm,
  path_composition_cache : Map (PathTerm, PathTerm) PathTerm
}

-- Cache-aware type checking
cached_infer : TypeCheckCache → Context → Term → TypeChecker (Type, TypeCheckCache)
cached_infer cache Γ t = 
  case lookup t cache.type_cache of
    Some A → return (A, cache)
    None → do
      A ← infer Γ t  -- Compute type
      let cache' = insert t A cache.type_cache
      return (A, cache')

-- Cache invalidation strategy
invalidate_cache : TypeCheckCache → Term → TypeCheckCache
invalidate_cache cache changed_term = 
  let dependent_terms = find_dependent_terms cache changed_term
      cache' = remove_terms dependent_terms cache
  in cache'

-- Incremental type checking
incremental_type_check : TypeCheckCache → Edit → TypeChecker TypeCheckCache
incremental_type_check cache edit = case edit of
  ChangeDefinition name new_term → do
    -- Invalidate dependent cached entries
    let cache' = invalidate_dependent cache name
    -- Re-check only affected terms
    affected ← find_affected_terms cache name
    cache'' ← recheck_terms cache' affected
    return cache''
    
  AddDefinition name term type → do
    -- Check new definition
    check empty_context term type
    -- Add to cache
    let cache' = insert_definition cache name term type
    return cache'
```

### Parallelization Strategies

```sctt
-- Parallel type checking architecture
data ParallelTypeChecker = ParallelTypeChecker {
  worker_count : Nat,
  task_queue : Queue TypeCheckTask,
  result_cache : ConcurrentCache,
  dependency_graph : DependencyGraph
}

-- Type checking task
data TypeCheckTask = 
  InferType Context Term |
  CheckType Context Term Type |
  NormalizeExpression Context Term |
  SolveConstraints (List Constraint)

-- Parallel execution strategy
parallel_type_check : ParallelTypeChecker → List TypeCheckTask → TypeChecker (List TypeCheckResult)
parallel_type_check checker tasks = do
  -- Analyze dependencies
  dependency_order ← topological_sort (dependency_graph tasks)
  
  -- Execute in parallel respecting dependencies
  results ← execute_parallel dependency_order checker
  
  return results

-- Work stealing for load balancing
work_stealing_scheduler : ParallelTypeChecker → IO ()
work_stealing_scheduler checker = do
  forever $ do
    task ← try_steal_work checker.task_queue
    case task of
      Some t → do
        result ← execute_task t
        store_result checker.result_cache result
      None → sleep_briefly
```

### Memory Optimization

```sctt
-- Memory-efficient term representation
data CompactTerm = 
  CVar Int |  -- De Bruijn indices
  CApp CompactTerm CompactTerm |
  CLam CompactTerm |
  CShared Int  -- Shared subterm reference

-- Maximal sharing of subterms
sharing_optimization : Term → (CompactTerm, SharingTable)
sharing_optimization term = 
  let subterms = collect_subterms term
      sharing_table = build_sharing_table subterms
      compact = convert_with_sharing term sharing_table
  in (compact, sharing_table)

-- Garbage collection of proof objects
proof_gc : TypeCheckState → TypeCheckState
proof_gc state = 
  let live_proofs = find_reachable_proofs state
      state' = remove_unreachable_proofs state live_proofs
  in compact_memory state'

-- Lazy evaluation for large expressions
data LazyTerm = 
  Evaluated Term |
  Thunk (Context → Term) |
  PartiallyEvaluated Term (Map Variable Term)

lazy_normalize : Context → LazyTerm → LazyTerm  
lazy_normalize ctx lazy_term = case lazy_term of
  Evaluated t → Evaluated t  -- Already done
  Thunk computation → 
    if should_evaluate_now ctx 
      then Evaluated (computation ctx)
      else lazy_term  -- Keep lazy
  PartiallyEvaluated t subst →
    let t' = apply_partial_substitution t subst
    in if is_normal_form t' 
       then Evaluated t'
       else PartiallyEvaluated t' subst
```

### Approximation Methods

```sctt
-- Approximate type checking for performance
approximate_type_check : Precision → Context → Term → Type → TypeChecker ApproximateResult
approximate_type_check precision Γ t A = case A of
  -- Exact checking for simple types
  ℕ → exact_check Γ t A
  Bool → exact_check Γ t A
  
  -- Approximate checking for smooth types
  C∞(M,N) → approximate_smooth_check precision Γ t M N
  
  -- Path types with geometric approximation
  Path B x y → approximate_path_check precision Γ t B x y
  
  -- Function types with sampling
  (x : Dom) → Cod → approximate_function_check precision Γ t Dom Cod

-- Smooth function approximation
approximate_smooth_check : Precision → Context → Term → SmoothType → SmoothType → TypeChecker ApproximateResult
approximate_smooth_check precision Γ f M N = do
  -- Sample function at test points
  test_points ← generate_test_points M precision
  
  -- Check type preservation at samples
  results ← map test_points $ \x → do
    fx ← evaluate_at Γ f x
    check Γ fx N
    
  -- Aggregate results
  if all results id
    then return (ProbablyCorrect precision)
    else return (ProbablyIncorrect (find_counterexamples results))

-- Error bounds for approximations
data ApproximateResult = 
  ProbablyCorrect Confidence |
  ProbablyIncorrect CounterExamples |
  InsufficientData |
  ApproximationFailed Error

-- Confidence levels
data Confidence = 
  VeryHigh |   -- > 99.9% confidence
  High |       -- > 99% confidence  
  Medium |     -- > 95% confidence
  Low          -- > 90% confidence
```

## 9.5 Implementation {#implementation}

### Core Type Checker Implementation

```sctt
-- Main type checker state
data TypeCheckState = TypeCheckState {
  context : Context,
  cache : TypeCheckCache,
  constraints : List Constraint,
  approximation_config : ApproximationConfig,
  debug_info : DebugInfo
}

-- Type checker monad
TypeChecker A = StateT TypeCheckState (Except TypeCheckError) A

-- Error handling
data TypeCheckError = 
  VariableNotInScope Variable |
  TypeMismatch Type Type |
  UnificationError Type Type |
  SmoothConstraintError SmoothConstraint |
  PathConstraintError PathConstraint |
  TimeoutError Duration |
  ApproximationError String

-- Main entry point
type_check_term : Term → Type → Either TypeCheckError Unit
type_check_term term expected_type = 
  runExcept $ evalStateT (check empty_context term expected_type) initial_state
  where
    initial_state = TypeCheckState {
      context = empty_context,
      cache = empty_cache,
      constraints = [],
      approximation_config = default_config,
      debug_info = empty_debug
    }
```

#### Context Management

```sctt
-- Context operations
extend_context : Context → Variable → Type → Context
extend_context ctx var type = (var, type) :: ctx

lookup_variable : Context → Variable → Maybe Type
lookup_variable [] var = Nothing
lookup_variable ((v, t) :: ctx) var = 
  if v == var then Some t else lookup_variable ctx var

-- Context well-formedness
well_formed_context : Context → Bool
well_formed_context [] = true
well_formed_context ((x, A) :: Γ) = 
  well_formed_context Γ && 
  free_variables A ⊆ domain Γ &&
  type_check A Type == Ok

-- Context substitution
substitute_context : Context → Substitution → Context
substitute_context ctx subst = 
  map ctx $ \(var, type) → (var, apply_substitution subst type)
```

#### Constraint Management

```sctt
-- Constraint collection
collect_constraints : Context → Term → Type → TypeChecker (List Constraint)
collect_constraints Γ t A = case (t, A) of
  -- Meta-variable generates constraint
  (Meta m, A) → return [UnifyTypes (Meta m) A]
  
  -- Smooth operations generate smooth constraints
  (SmoothApp f x, N) → do
    f_type ← infer Γ f
    (M, N') ← expect_smooth_function_type f_type
    x_constraints ← collect_constraints Γ x M
    return $ [SmoothEquality N N'] ++ x_constraints
    
  -- Path operations generate path constraints
  (PathApp p i, A) → do
    p_type ← infer Γ p
    (B, x, y) ← expect_path_type p_type
    i_constraints ← collect_constraints Γ i I
    return $ [PathEquality A (B i)] ++ i_constraints
    
  -- Recursive cases
  (App f a, B) → do
    f_constraints ← collect_constraints Γ f (infer_function_type a B)
    a_constraints ← collect_constraints Γ a (infer_domain_type f)
    return $ f_constraints ++ a_constraints
    
  _ → return []

-- Constraint solving driver
solve_all_constraints : TypeChecker Substitution
solve_all_constraints = do
  constraints ← get_constraints
  solution ← solve_constraints constraints
  apply_solution solution
  return solution
```

#### Error Reporting

```sctt
-- Detailed error messages
format_error : TypeCheckError → String
format_error error = case error of
  VariableNotInScope var → 
    "Variable '" ++ show var ++ "' is not in scope"
    
  TypeMismatch expected actual →
    "Type mismatch:\n" ++
    "  Expected: " ++ pretty_print expected ++ "\n" ++
    "  Actual:   " ++ pretty_print actual
    
  UnificationError type1 type2 →
    "Cannot unify types:\n" ++
    "  " ++ pretty_print type1 ++ "\n" ++
    "  " ++ pretty_print type2
    
  SmoothConstraintError constraint →
    "Cannot solve smooth constraint: " ++ show constraint
    
  TimeoutError duration →
    "Type checking timed out after " ++ show duration

-- Error recovery strategies
recover_from_error : TypeCheckError → TypeChecker (Maybe Recovery)
recover_from_error error = case error of
  UnificationError t1 t2 → 
    try_approximate_unification t1 t2
    
  SmoothConstraintError constraint →
    try_approximate_smooth_solving constraint
    
  TimeoutError _ →
    try_timeout_recovery
    
  _ → return Nothing

-- Gradual typing for error recovery
gradual_type_check : Context → Term → Type → TypeChecker GradualResult
gradual_type_check Γ t A = do
  result ← try $ check Γ t A
  case result of
    Right _ → return ExactlyTyped
    Left error → do
      recovery ← recover_from_error error
      case recovery of
        Some approx → return (ApproximatelyTyped approx)
        None → return (TypingFailed error)

data GradualResult = 
  ExactlyTyped |
  ApproximatelyTyped Approximation |
  TypingFailed TypeCheckError
```

### Performance Monitoring

```sctt
-- Performance metrics
data TypeCheckMetrics = TypeCheckMetrics {
  total_time : Duration,
  cache_hits : Int,
  cache_misses : Int,
  constraint_solving_time : Duration,
  normalization_time : Duration,
  smooth_computation_time : Duration,
  memory_usage : ByteSize
}

-- Performance monitoring
monitor_performance : TypeChecker A → TypeChecker (A, TypeCheckMetrics)
monitor_performance computation = do
  start_time ← get_current_time
  start_memory ← get_memory_usage
  
  result ← computation
  
  end_time ← get_current_time
  end_memory ← get_memory_usage
  
  metrics ← gather_metrics start_time end_time start_memory end_memory
  return (result, metrics)

-- Adaptive optimization based on metrics
adaptive_optimization : TypeCheckMetrics → TypeChecker OptimizationStrategy
adaptive_optimization metrics = 
  if metrics.cache_misses > threshold_cache_misses
    then return IncreaseCache
  else if metrics.constraint_solving_time > threshold_constraint_time
    then return UseApproximation
  else if metrics.memory_usage > threshold_memory
    then return CompactMemory
  else return NoChange
```

## Summary

The SCTT type checking algorithm combines sophisticated theoretical foundations with practical implementation strategies:

### Core Innovations

1. **Bidirectional Framework**: Efficient separation of inference and checking
2. **Unified Constraint Solving**: Handles dependent, cubical, and smooth constraints  
3. **NBE with Smooth Operations**: Normalization that preserves smooth structure
4. **Intelligent Optimization**: Caching, parallelization, and approximation
5. **Robust Implementation**: Error handling, performance monitoring, and recovery

### Key Technical Achievements

- **Decidable Type Checking**: Algorithm terminates on all well-typed inputs
- **Smooth Integration**: Derivatives and integrals type check correctly
- **Path Coherence**: Cubical operations maintain type structure  
- **Performance**: Practical type checking for realistic programs
- **Extensibility**: Framework supports future SCTT extensions

### Implementation Architecture

The type checker is structured as:
- **Core Engine**: Bidirectional inference with constraint generation
- **Solver**: Unification and constraint solving for all SCTT features
- **Evaluator**: NBE with support for smooth and cubical operations
- **Optimizer**: Caching, parallelization, and approximation layers
- **Infrastructure**: Error handling, debugging, and performance monitoring

This creates a type checker that is both theoretically sound and practically efficient—capable of handling the mathematical complexity of SCTT while providing useful feedback to programmers.

## Exercises

### Algorithm Design
1. Implement bidirectional type checking for smooth higher inductive types
2. Design constraint solving for modal SCTT from Chapter 13
3. Create NBE algorithms for higher categorical structures
4. Develop approximation methods for undecidable smooth constraints

### Implementation
1. Build a prototype SCTT type checker in your favorite language
2. Implement caching strategies for smooth function derivatives
3. Create parallel constraint solving for independent smooth equations
4. Design error recovery for failed smooth computations

### Optimization
1. Analyze complexity bounds for SCTT constraint solving
2. Implement lazy evaluation for large smooth expressions
3. Design memory-efficient representations for cubical paths
4. Create adaptive optimization based on usage patterns

### Applications
1. Type check a scientific computing library in SCTT
2. Verify correctness of a control system using the type checker
3. Build educational tools that explain type checking steps
4. Create IDE integration for SCTT with intelligent error messages

---

*Next: [Chapter 10: Programming in SCTT](./chapter_10.md) →*

*Previous: [Chapter 8: Metatheory](./chapter_08.md) ←*