# Smooth Cubical Type Theory - Implementation Design

## 1. Architecture Overview

### 1.1 Language Choice

**Primary Implementation**: Rust
- Performance critical for type checking
- Memory safety without GC
- Good algebraic data type support
- Strong ecosystem for parsing and compilation

**Proof of Correctness**: Agda/Coq
- Formalize metatheory
- Verify key algorithms
- Extract certified core

### 1.2 Module Structure

```
sctt/
├── syntax/          # AST and concrete syntax
├── core/            # Core type theory
├── smooth/          # Smooth structure implementation
├── cubical/         # Cubical operations
├── typechecker/     # Type checking and inference
├── evaluator/       # Normalization and evaluation
├── compiler/        # Code generation
├── library/         # Standard library
└── tools/           # REPL, LSP, formatter
```

## 2. Core Data Structures

### 2.1 Abstract Syntax Tree

```rust
#[derive(Clone, Debug)]
pub enum Term {
    // Variables and binding
    Var(Name),
    
    // Core type theory
    Universe(Level),
    Pi(Box<Term>, Binder, Box<Term>),
    Lambda(Binder, Box<Term>),
    App(Box<Term>, Box<Term>),
    Sigma(Box<Term>, Binder, Box<Term>),
    Pair(Box<Term>, Box<Term>),
    Fst(Box<Term>),
    Snd(Box<Term>),
    
    // Cubical structure
    Interval,
    IZero,
    IOne,
    IMeet(Box<Term>, Box<Term>),
    IJoin(Box<Term>, Box<Term>),
    INeg(Box<Term>),
    Path(Box<Term>, Box<Term>, Box<Term>),
    PathLambda(Name, Box<Term>),
    PathApp(Box<Term>, Box<Term>),
    Comp(Box<Term>, Formula, System, Box<Term>),
    HComp(Box<Term>, Formula, System, Box<Term>),
    
    // Smooth structure
    Smooth(Box<Term>),
    SmoothLambda(Binder, Box<Term>, SmoothData),
    Diff(Box<Term>),
    TangentSpace(Box<Term>, Box<Term>),
    Jet(u32, Box<Term>),
}

#[derive(Clone, Debug)]
pub struct Binder {
    pub name: Name,
    pub ty: Box<Term>,
}

#[derive(Clone, Debug)]
pub struct SmoothData {
    pub derivatives: Vec<Term>,
    pub taylor_coeffs: Option<Vec<Term>>,
}
```

### 2.2 Values for NbE

```rust
#[derive(Clone, Debug)]
pub enum Value {
    // Neutral values (blocked computations)
    Neutral(Neutral),
    
    // Canonical values
    Universe(Level),
    Pi(Box<Value>, Closure),
    Lambda(Closure),
    Sigma(Box<Value>, Closure),
    Pair(Box<Value>, Box<Value>),
    
    // Interval values
    IZero,
    IOne,
    IMeet(Box<Value>, Box<Value>),
    IJoin(Box<Value>, Box<Value>),
    INeg(Box<Value>),
    
    // Path values
    Path(Box<Value>, Box<Value>, Box<Value>),
    PathLambda(PathClosure),
    
    // Smooth values
    Smooth(Box<Value>),
    SmoothLambda(SmoothClosure),
    SmoothManifold(ManifoldData),
}

#[derive(Clone, Debug)]
pub struct Closure {
    pub env: Environment,
    pub body: Term,
}

#[derive(Clone, Debug)]
pub struct PathClosure {
    pub env: Environment,
    pub var: Name,
    pub body: Term,
}

#[derive(Clone, Debug)]
pub struct SmoothClosure {
    pub env: Environment,
    pub var: Name,
    pub body: Term,
    pub smooth_data: SmoothData,
}
```

### 2.3 Typing Context

```rust
pub struct Context {
    entries: Vec<ContextEntry>,
    level: Level,
}

pub struct ContextEntry {
    pub name: Name,
    pub ty: Value,
    pub value: Option<Value>,
    pub is_smooth: bool,
}

impl Context {
    pub fn extend(&self, name: Name, ty: Value) -> Self {
        let mut ctx = self.clone();
        ctx.entries.push(ContextEntry {
            name,
            ty,
            value: None,
            is_smooth: false,
        });
        ctx
    }
    
    pub fn extend_smooth(&self, name: Name, ty: Value) -> Self {
        let mut ctx = self.clone();
        ctx.entries.push(ContextEntry {
            name,
            ty,
            value: None,
            is_smooth: true,
        });
        ctx
    }
}
```

## 3. Type Checking Algorithm

### 3.1 Bidirectional Type Checking

```rust
pub fn check(ctx: &Context, term: &Term, ty: &Value) -> Result<Term, TypeError> {
    match (term, ty) {
        (Term::Lambda(x, body), Value::Pi(a, b)) => {
            let ctx2 = ctx.extend(x.name.clone(), a.clone());
            let body_ty = b.apply(Value::Neutral(Neutral::Var(x.name.clone())));
            let body2 = check(&ctx2, body, &body_ty)?;
            Ok(Term::Lambda(x.clone(), Box::new(body2)))
        }
        
        (Term::PathLambda(i, body), Value::Path(a, left, right)) => {
            let ctx2 = ctx.extend_interval(i.clone());
            // Check body : A(i) with boundary conditions
            check_path_body(&ctx2, body, a, left, right)
        }
        
        (Term::SmoothLambda(x, body, data), Value::Smooth(inner)) => {
            check_smooth_function(ctx, x, body, data, inner)
        }
        
        _ => {
            let (term2, inferred) = infer(ctx, term)?;
            convert(ctx, &inferred, ty)?;
            Ok(term2)
        }
    }
}

pub fn infer(ctx: &Context, term: &Term) -> Result<(Term, Value), TypeError> {
    match term {
        Term::Var(x) => {
            let ty = ctx.lookup(x)?;
            Ok((Term::Var(x.clone()), ty))
        }
        
        Term::App(f, a) => {
            let (f2, f_ty) = infer(ctx, f)?;
            match f_ty {
                Value::Pi(arg_ty, res_ty) => {
                    let a2 = check(ctx, a, &arg_ty)?;
                    let a_val = eval(ctx.to_env(), &a2);
                    let res = res_ty.apply(a_val);
                    Ok((Term::App(Box::new(f2), Box::new(a2)), res))
                }
                _ => Err(TypeError::NotAFunction(f_ty))
            }
        }
        
        Term::Diff(f) => {
            let (f2, f_ty) = infer(ctx, f)?;
            match f_ty {
                Value::Smooth(Value::Pi(a, b)) => {
                    let tangent_ty = infer_tangent_map_type(ctx, &a, &b)?;
                    Ok((Term::Diff(Box::new(f2)), tangent_ty))
                }
                _ => Err(TypeError::NotSmoothFunction(f_ty))
            }
        }
        
        // ... more cases
    }
}
```

### 3.2 Conversion Checking

```rust
pub fn convert(ctx: &Context, v1: &Value, v2: &Value) -> Result<(), TypeError> {
    let n1 = readback(ctx.level(), v1);
    let n2 = readback(ctx.level(), v2);
    if alpha_equiv(&n1, &n2) {
        Ok(())
    } else {
        Err(TypeError::NotConvertible(v1.clone(), v2.clone()))
    }
}

pub fn alpha_equiv(t1: &Term, t2: &Term) -> bool {
    // Implement α-equivalence checking
    match (t1, t2) {
        (Term::Var(x), Term::Var(y)) => x == y,
        (Term::Lambda(x1, b1), Term::Lambda(x2, b2)) => {
            // Check with α-renaming
            let b2_renamed = rename(b2, &x2.name, &x1.name);
            alpha_equiv(b1, &b2_renamed)
        }
        // ... more cases
    }
}
```

## 4. Evaluation and Normalization

### 4.1 Normalization by Evaluation

```rust
pub fn normalize(ctx: &Context, term: &Term) -> Term {
    let env = ctx.to_env();
    let val = eval(env, term);
    readback(ctx.level(), &val)
}

pub fn eval(env: &Environment, term: &Term) -> Value {
    match term {
        Term::Var(x) => env.lookup(x).unwrap_or(Value::Neutral(Neutral::Var(x.clone()))),
        
        Term::Lambda(x, body) => {
            Value::Lambda(Closure {
                env: env.clone(),
                body: *body.clone(),
            })
        }
        
        Term::App(f, a) => {
            let f_val = eval(env, f);
            let a_val = eval(env, a);
            do_app(f_val, a_val)
        }
        
        Term::PathLambda(i, body) => {
            Value::PathLambda(PathClosure {
                env: env.clone(),
                var: i.clone(),
                body: *body.clone(),
            })
        }
        
        Term::PathApp(p, r) => {
            let p_val = eval(env, p);
            let r_val = eval(env, r);
            do_path_app(p_val, r_val)
        }
        
        Term::Comp(ty, phi, sys, a0) => {
            do_comp(
                eval(env, ty),
                eval_formula(env, phi),
                eval_system(env, sys),
                eval(env, a0)
            )
        }
        
        // ... more cases
    }
}

pub fn do_app(f: Value, a: Value) -> Value {
    match f {
        Value::Lambda(clos) => clos.apply(a),
        Value::Neutral(n) => Value::Neutral(Neutral::App(Box::new(n), Box::new(a))),
        _ => panic!("Invalid application")
    }
}

pub fn readback(level: Level, val: &Value) -> Term {
    match val {
        Value::Neutral(n) => readback_neutral(level, n),
        
        Value::Lambda(clos) => {
            let var = fresh_var(level);
            let body = clos.apply(Value::Neutral(Neutral::Var(var.clone())));
            Term::Lambda(
                Binder { name: var, ty: Box::new(Term::Universe(0)) },
                Box::new(readback(level + 1, &body))
            )
        }
        
        Value::Pi(a, b) => {
            let var = fresh_var(level);
            let b_val = b.apply(Value::Neutral(Neutral::Var(var.clone())));
            Term::Pi(
                Box::new(readback(level, a)),
                Binder { name: var.clone(), ty: Box::new(readback(level, a)) },
                Box::new(readback(level + 1, &b_val))
            )
        }
        
        // ... more cases
    }
}
```

## 5. Cubical Operations

### 5.1 Composition

```rust
pub fn do_comp(
    ty: Value,
    phi: Formula,
    sys: System,
    a0: Value
) -> Value {
    // Implement Kan composition
    match ty {
        Value::Pi(a, b) => {
            // Composition for Pi types
            comp_pi(a, b, phi, sys, a0)
        }
        
        Value::Sigma(a, b) => {
            // Composition for Sigma types
            comp_sigma(a, b, phi, sys, a0)
        }
        
        Value::Path(a, left, right) => {
            // Composition for Path types
            comp_path(a, left, right, phi, sys, a0)
        }
        
        Value::Smooth(inner) => {
            // Composition for smooth types preserves smoothness
            let inner_comp = do_comp(inner, phi, sys, extract_smooth(a0));
            Value::Smooth(Box::new(inner_comp))
        }
        
        _ => {
            // For neutral types, create neutral composition
            Value::Neutral(Neutral::Comp(
                Box::new(ty),
                phi,
                sys,
                Box::new(a0)
            ))
        }
    }
}

fn comp_pi(a: Box<Value>, b: Closure, phi: Formula, sys: System, a0: Value) -> Value {
    Value::Lambda(Closure {
        env: Environment::new(),
        body: {
            // λx. comp (B x) φ (λi. sys i x) (a0 x)
            let x = fresh_var(0);
            let bx = b.apply(Value::Neutral(Neutral::Var(x.clone())));
            let sys_x = apply_system_to_arg(&sys, &Value::Neutral(Neutral::Var(x.clone())));
            let a0_x = do_app(a0, Value::Neutral(Neutral::Var(x.clone())));
            term_of_comp(bx, phi, sys_x, a0_x)
        }
    })
}
```

### 5.2 Transport

```rust
pub fn transport(ty_path: Value, a: Value) -> Value {
    // transport A a = comp A [] a
    do_comp(ty_path, Formula::False, System::Empty, a)
}

pub fn transp(ty: Closure, r: Value, a: Value) -> Value {
    // More general transport with explicit interval
    match r {
        Value::IZero => a,
        Value::IOne => ty.apply(Value::IOne),
        _ => {
            let ty_path = Value::PathLambda(PathClosure {
                env: ty.env,
                var: fresh_var(0),
                body: ty.body,
            });
            transport(ty_path, a)
        }
    }
}
```

## 6. Smooth Structure Implementation

### 6.1 Smooth Functions

```rust
pub struct SmoothFunction {
    pub domain: SmoothManifold,
    pub codomain: SmoothManifold,
    pub function: Box<dyn Fn(Point) -> Point>,
    pub differential: Box<dyn Fn(Point) -> LinearMap>,
    pub higher_derivatives: Vec<Box<dyn Fn(Point) -> Tensor>>,
}

impl SmoothFunction {
    pub fn compose(&self, other: &SmoothFunction) -> Result<SmoothFunction, Error> {
        if self.domain != other.codomain {
            return Err(Error::TypeMismatch);
        }
        
        Ok(SmoothFunction {
            domain: other.domain.clone(),
            codomain: self.codomain.clone(),
            function: Box::new(move |x| {
                let y = (other.function)(x);
                (self.function)(y)
            }),
            differential: Box::new(move |x| {
                let y = (other.function)(x);
                let df = (other.differential)(x);
                let dg = (self.differential)(y);
                dg.compose(&df)
            }),
            higher_derivatives: compute_higher_derivatives_composition(self, other),
        })
    }
    
    pub fn verify_smooth(&self, order: usize) -> bool {
        // Verify smoothness up to given order
        // This would involve numerical checks in practice
        true
    }
}
```

### 6.2 Manifolds

```rust
pub struct SmoothManifold {
    pub dimension: usize,
    pub charts: Vec<Chart>,
    pub atlas: Atlas,
}

pub struct Chart {
    pub id: ChartId,
    pub domain: OpenSet,
    pub map: Box<dyn Fn(Point) -> EuclideanPoint>,
    pub inverse: Box<dyn Fn(EuclideanPoint) -> Point>,
}

pub struct Atlas {
    pub transitions: HashMap<(ChartId, ChartId), TransitionMap>,
}

pub struct TransitionMap {
    pub from_chart: ChartId,
    pub to_chart: ChartId,
    pub map: SmoothFunction,
}

impl SmoothManifold {
    pub fn tangent_bundle(&self) -> VectorBundle {
        VectorBundle {
            base: self.clone(),
            fiber_dim: self.dimension,
            local_trivializations: self.compute_tangent_trivializations(),
        }
    }
    
    pub fn cotangent_bundle(&self) -> VectorBundle {
        VectorBundle {
            base: self.clone(),
            fiber_dim: self.dimension,
            local_trivializations: self.compute_cotangent_trivializations(),
        }
    }
}
```

### 6.3 Differentiation

```rust
pub fn differentiate(ctx: &Context, f: &Value) -> Result<Value, TypeError> {
    match f {
        Value::SmoothLambda(clos) => {
            // Extract derivative from smooth data
            Ok(clos.smooth_data.derivatives[0].clone())
        }
        
        Value::Neutral(n) => {
            // Create neutral differentiation
            Ok(Value::Neutral(Neutral::Diff(Box::new(n.clone()))))
        }
        
        _ => Err(TypeError::NotDifferentiable(f.clone()))
    }
}

pub fn compute_jet(order: u32, f: &SmoothFunction, point: Point) -> Jet {
    let mut derivatives = vec![f.function(point)];
    
    for k in 1..=order {
        let deriv = if k <= f.higher_derivatives.len() {
            f.higher_derivatives[k-1](point)
        } else {
            // Compute derivative numerically if not stored
            numerical_derivative(f, point, k)
        };
        derivatives.push(deriv);
    }
    
    Jet {
        base_point: point,
        values: derivatives,
        order,
    }
}
```

## 7. Standard Library

### 7.1 Basic Smooth Types

```rust
// In library/smooth_types.sctt

def ℝ : SmoothType := 
  record {
    carrier := Real;
    smooth_structure := standard_smooth_structure;
  }

def ℝ^n (n : ℕ) : SmoothType :=
  record {
    carrier := Vec Real n;
    smooth_structure := product_smooth_structure n;
  }

def S¹ : SmoothType :=
  SmoothHIT {
    point base : S¹;
    path loop : base ≡ base;
    smooth_structure := circle_smooth_structure;
  }

def S^n (n : ℕ) : SmoothType :=
  SmoothHIT {
    point north : S^n;
    point south : S^n;
    path meridian : (x : S^(n-1)) → north ≡ south;
    smooth_structure := sphere_smooth_structure n;
  }
```

### 7.2 Lie Groups

```rust
// In library/lie_groups.sctt

record LieGroup :=
  { G : SmoothType
  ; unit : G
  ; mult : C^∞(G × G, G)
  ; inv : C^∞(G, G)
  ; assoc : ∀ x y z → mult(mult(x,y),z) ≡ mult(x,mult(y,z))
  ; unit_left : ∀ x → mult(unit, x) ≡ x
  ; unit_right : ∀ x → mult(x, unit) ≡ x
  ; inv_left : ∀ x → mult(inv(x), x) ≡ unit
  ; inv_right : ∀ x → mult(x, inv(x)) ≡ unit
  }

def GL(n : ℕ, 𝔽 : Field) : LieGroup :=
  record {
    G := { M : Matrix n n 𝔽 | det(M) ≠ 0 };
    unit := identity_matrix n;
    mult := matrix_multiplication;
    inv := matrix_inverse;
    -- proofs...
  }
```

## 8. Tooling

### 8.1 REPL

```rust
pub struct Repl {
    context: Context,
    environment: Environment,
    history: Vec<String>,
}

impl Repl {
    pub fn run(&mut self) {
        loop {
            print!("> ");
            let input = read_line();
            
            match self.process_command(&input) {
                Ok(output) => println!("{}", output),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
    }
    
    fn process_command(&mut self, input: &str) -> Result<String, Error> {
        if input.starts_with(":") {
            self.process_meta_command(input)
        } else {
            let term = parse(input)?;
            let (term2, ty) = infer(&self.context, &term)?;
            let val = eval(&self.environment, &term2);
            let norm = readback(self.context.level(), &val);
            Ok(format!("{} : {}", norm, ty))
        }
    }
}
```

### 8.2 Language Server Protocol

```rust
pub struct ScttLanguageServer {
    connection: Connection,
    documents: HashMap<Url, Document>,
    context: Context,
}

impl ScttLanguageServer {
    pub async fn run(&mut self) {
        for msg in &self.connection.receiver {
            match msg {
                Message::Request(req) => self.handle_request(req).await,
                Message::Notification(not) => self.handle_notification(not).await,
                _ => {}
            }
        }
    }
    
    async fn handle_hover(&self, params: HoverParams) -> Option<Hover> {
        let doc = self.documents.get(&params.text_document.uri)?;
        let term = doc.term_at_position(params.position)?;
        let (_, ty) = infer(&self.context, &term).ok()?;
        
        Some(Hover {
            contents: HoverContents::Scalar(
                MarkedString::String(format!("{:?} : {:?}", term, ty))
            ),
            range: None,
        })
    }
}
```

## 9. Testing Framework

### 9.1 Property-Based Testing

```rust
#[cfg(test)]
mod tests {
    use quickcheck::{quickcheck, TestResult};
    
    #[test]
    fn test_normalization_preserves_typing() {
        fn prop(term: WellTypedTerm) -> TestResult {
            let ctx = Context::new();
            let (_, ty1) = infer(&ctx, &term.0).unwrap();
            let norm = normalize(&ctx, &term.0);
            let (_, ty2) = infer(&ctx, &norm).unwrap();
            TestResult::from_bool(convert(&ctx, &ty1, &ty2).is_ok())
        }
        quickcheck(prop as fn(WellTypedTerm) -> TestResult);
    }
    
    #[test]
    fn test_smooth_composition_associative() {
        fn prop(f: SmoothFn, g: SmoothFn, h: SmoothFn) -> TestResult {
            if !compatible(f, g) || !compatible(g, h) {
                return TestResult::discard();
            }
            
            let comp1 = f.compose(&g.compose(&h));
            let comp2 = f.compose(&g).compose(&h);
            
            TestResult::from_bool(smooth_equal(&comp1, &comp2))
        }
        quickcheck(prop as fn(SmoothFn, SmoothFn, SmoothFn) -> TestResult);
    }
}
```

### 9.2 Golden Tests

```rust
#[test]
fn test_standard_library() {
    let files = glob("library/**/*.sctt").unwrap();
    
    for file in files {
        let content = fs::read_to_string(&file).unwrap();
        let expected = fs::read_to_string(file.with_extension("expected")).unwrap();
        
        let ctx = Context::new();
        let result = check_file(&ctx, &content);
        
        assert_eq!(format!("{:?}", result), expected);
    }
}
```

## 10. Performance Optimizations

### 10.1 Memoization

```rust
pub struct MemoizedEvaluator {
    cache: HashMap<(Term, Environment), Value>,
    hit_rate: f64,
}

impl MemoizedEvaluator {
    pub fn eval(&mut self, env: &Environment, term: &Term) -> Value {
        let key = (term.clone(), env.clone());
        
        if let Some(val) = self.cache.get(&key) {
            self.hit_rate = update_hit_rate(self.hit_rate, true);
            return val.clone();
        }
        
        let val = eval_uncached(env, term);
        self.cache.insert(key, val.clone());
        self.hit_rate = update_hit_rate(self.hit_rate, false);
        val
    }
}
```

### 10.2 Parallel Type Checking

```rust
use rayon::prelude::*;

pub fn check_mutual_definitions(
    ctx: &Context,
    defs: Vec<Definition>
) -> Result<Vec<CheckedDefinition>, TypeError> {
    // Check non-recursive definitions in parallel
    let (recursive, non_recursive): (Vec<_>, Vec<_>) = 
        defs.into_iter().partition(|d| d.is_recursive);
    
    let checked_non_rec: Vec<_> = non_recursive
        .par_iter()
        .map(|def| check_definition(ctx, def))
        .collect::<Result<Vec<_>, _>>()?;
    
    // Check recursive definitions sequentially
    let mut ctx2 = ctx.clone();
    for def in &checked_non_rec {
        ctx2 = ctx2.extend_def(def);
    }
    
    let checked_rec = check_recursive_block(&ctx2, recursive)?;
    
    Ok([checked_non_rec, checked_rec].concat())
}
```

## 11. Compilation Targets

### 11.1 Extraction to Rust

```rust
pub fn extract_to_rust(term: &Term, ty: &Value) -> String {
    match ty {
        Value::Pi(a, b) if is_first_order(a) && is_first_order(b) => {
            format!("fn {}({}: {}) -> {} {{ {} }}",
                fresh_name(),
                fresh_var(0),
                extract_type(a),
                extract_type(b),
                extract_term(term)
            )
        }
        
        Value::Smooth(inner) => {
            format!("SmoothFunction {{ {} }}", extract_smooth(term, inner))
        }
        
        _ => extract_term(term)
    }
}
```

### 11.2 WebAssembly Target

```rust
pub fn compile_to_wasm(program: &Program) -> Vec<u8> {
    let mut module = Module::new();
    
    for def in &program.definitions {
        let func = compile_definition(def);
        module.add_function(func);
    }
    
    module.emit_wasm()
}
```

## 12. Deployment

### 12.1 Package Manager

```toml
# sctt.toml
[package]
name = "my-smooth-theory"
version = "0.1.0"
authors = ["Your Name"]

[dependencies]
sctt-stdlib = "1.0"
smooth-manifolds = "0.2"

[build]
entry = "src/Main.sctt"
target = "wasm"
```

### 12.2 CI/CD Pipeline

```yaml
# .github/workflows/ci.yml
name: CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v2
    - name: Install SCTT
      run: cargo install sctt
    - name: Type Check
      run: sctt check src/
    - name: Run Tests
      run: sctt test
    - name: Build Library
      run: sctt build --release
```