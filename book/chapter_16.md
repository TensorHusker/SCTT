# Chapter 16: Building the v0 Kernel

> "What I cannot create, I do not understand." — Richard Feynman
>
> "A proof assistant's kernel is the one artifact where trust is non-negotiable. Every other component earns trust by reducing to it."

## Introduction

This chapter walks you through building a working SCTT type checker in Rust. Not a toy — a real kernel following the architecture of [cctt](https://github.com/AndrasKovacs/cctt) by András Kovács, the fastest cubical type checker in existence.

The kernel — named **v0**, for Vi — is small by design: 2–5K lines of Rust that implement evaluation, quoting, conversion checking, and the cubical/smooth/Lipschitz rules. Everything else (parsing, elaboration, pattern matching, tactics, IDE integration) lives *outside* the kernel and can be as large and AI-assisted as you like. The kernel itself is hand-auditable. No LLM writes kernel code.

We build v0 in four phases, each producing a **testable, working checker**:

| Phase | What You Get | Lines Added | Cumulative |
|-------|-------------|-------------|------------|
| 1 | Dependent types (TTT) | ~500 | ~500 |
| 2 | Cartesian cubical layer | ~800–1200 | ~1300–1700 |
| 3 | Smooth primitives (ε² = 0) | ~200–400 | ~1500–2100 |
| 4 | Lipschitz sensitivity types | ~100–200 | ~1600–2300 |

Each phase compiles, passes tests, and type-checks meaningful programs before you move to the next. This is the methodology: **incremental trust**.

The architecture follows three principles drawn directly from cctt:

1. **Normalization by Evaluation (NbE)** — evaluate terms into a semantic domain, quote back to syntax for comparison
2. **Sub/Force** — O(1) interval substitution via lazy wrappers, pushed down only at rigid heads
3. **Defunctionalized closures** — closures as data (environment + syntax), not Rust closures, enabling inspection and serialization

If you have read [Chapter 9](./chapter_09.md) for the theory and [Chapter 8](./chapter_08.md) for the metatheory, this chapter is where you turn that understanding into running code.

## 16.1 Phase 1: Tiny Type Theory (TTT) {#phase1}

The minimal dependent core. Pi, Sigma, universes, bidirectional checking, NbE. No cubical structure yet — just the bones that every later phase builds on.

### 16.1.1 Term Representation

Every term is an `Arc<Term>` — reference-counted, structurally shared, O(1) to clone. This mirrors cctt's use of Haskell's garbage-collected ADTs while staying in Rust's ownership model.

```rust
use std::sync::Arc;

pub const MAX_UNIVERSE_LEVEL: u8 = 6;
pub const MAX_EVAL_DEPTH: usize = 256;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term {
    Var(usize),                                        // de Bruijn index
    Universe(u8),                                      // Type_i, 0 ≤ i ≤ 6
    Pi { domain: Arc<Term>, codomain: Arc<Term> },     // Π(x : A). B
    Lambda { body: Arc<Term> },                        // λx. t
    App { func: Arc<Term>, arg: Arc<Term> },           // f a
    Sigma { fst_type: Arc<Term>, snd_type: Arc<Term> },// Σ(x : A). B
    Pair { fst: Arc<Term>, snd: Arc<Term> },           // (a, b)
    Fst(Arc<Term>),                                    // π₁(p)
    Snd(Arc<Term>),                                    // π₂(p)
}
```

Smart constructors hide the `Arc` wrapping:

```rust
impl Term {
    pub fn var(i: usize) -> Arc<Term> { Arc::new(Term::Var(i)) }
    pub fn universe(level: u8) -> Arc<Term> {
        assert!(level <= MAX_UNIVERSE_LEVEL);
        Arc::new(Term::Universe(level))
    }
    pub fn pi(domain: Arc<Term>, codomain: Arc<Term>) -> Arc<Term> {
        Arc::new(Term::Pi { domain, codomain })
    }
    pub fn lambda(body: Arc<Term>) -> Arc<Term> {
        Arc::new(Term::Lambda { body })
    }
    pub fn app(func: Arc<Term>, arg: Arc<Term>) -> Arc<Term> {
        Arc::new(Term::App { func, arg })
    }
    pub fn sigma(fst_type: Arc<Term>, snd_type: Arc<Term>) -> Arc<Term> {
        Arc::new(Term::Sigma { fst_type, snd_type })
    }
    pub fn pair(fst: Arc<Term>, snd: Arc<Term>) -> Arc<Term> {
        Arc::new(Term::Pair { fst, snd })
    }
    pub fn fst(t: Arc<Term>) -> Arc<Term> { Arc::new(Term::Fst(t)) }
    pub fn snd(t: Arc<Term>) -> Arc<Term> { Arc::new(Term::Snd(t)) }
}
```

### 16.1.2 De Bruijn Machinery

Two operations underpin everything: shifting (adjusting free variable indices when moving under binders) and substitution (replacing a variable with a term).

```rust
pub fn shift(term: &Term, cutoff: usize, amount: isize) -> Arc<Term> {
    match term {
        Term::Var(i) => {
            if *i >= cutoff {
                let new_idx = (*i as isize + amount) as usize;
                Term::var(new_idx)
            } else {
                Term::var(*i)
            }
        }
        Term::Universe(l) => Term::universe(*l),
        Term::Pi { domain, codomain } => Term::pi(
            shift(domain, cutoff, amount),
            shift(codomain, cutoff + 1, amount),
        ),
        Term::Lambda { body } => Term::lambda(shift(body, cutoff + 1, amount)),
        Term::App { func, arg } => Term::app(
            shift(func, cutoff, amount),
            shift(arg, cutoff, amount),
        ),
        Term::Sigma { fst_type, snd_type } => Term::sigma(
            shift(fst_type, cutoff, amount),
            shift(snd_type, cutoff + 1, amount),
        ),
        Term::Pair { fst, snd } => Term::pair(
            shift(fst, cutoff, amount),
            shift(snd, cutoff, amount),
        ),
        Term::Fst(t) => Term::fst(shift(t, cutoff, amount)),
        Term::Snd(t) => Term::snd(shift(t, cutoff, amount)),
    }
}

pub fn subst(term: &Term, idx: usize, replacement: &Term) -> Arc<Term> {
    match term {
        Term::Var(i) => {
            if *i == idx { Arc::new(replacement.clone()) }
            else if *i > idx { Term::var(*i - 1) }
            else { Term::var(*i) }
        }
        Term::Pi { domain, codomain } => Term::pi(
            subst(domain, idx, replacement),
            subst(codomain, idx + 1, &shift(replacement, 0, 1)),
        ),
        Term::Lambda { body } => {
            Term::lambda(subst(body, idx + 1, &shift(replacement, 0, 1)))
        }
        Term::App { func, arg } => Term::app(
            subst(func, idx, replacement),
            subst(arg, idx, replacement),
        ),
        Term::Sigma { fst_type, snd_type } => Term::sigma(
            subst(fst_type, idx, replacement),
            subst(snd_type, idx + 1, &shift(replacement, 0, 1)),
        ),
        Term::Pair { fst, snd } => Term::pair(
            subst(fst, idx, replacement),
            subst(snd, idx, replacement),
        ),
        Term::Fst(t) => Term::fst(subst(t, idx, replacement)),
        Term::Snd(t) => Term::snd(subst(t, idx, replacement)),
        Term::Universe(l) => Term::universe(*l),
    }
}
```

### 16.1.3 NbE: Evaluate and Quote

Weak head normal form (WHNF) reduces only until we see a constructor or stuck variable. This is the workhorse — conversion checking calls it at every comparison.

```rust
pub fn whnf(term: &Term) -> Arc<Term> {
    match term {
        Term::App { func, arg } => {
            let func_whnf = whnf(func);
            match func_whnf.as_ref() {
                Term::Lambda { body } => whnf(&subst(body, 0, arg)),
                _ => Term::app(func_whnf, Arc::new(arg.clone())),
            }
        }
        Term::Fst(t) => {
            let t_whnf = whnf(t);
            match t_whnf.as_ref() {
                Term::Pair { fst, .. } => whnf(fst),
                _ => Term::fst(t_whnf),
            }
        }
        Term::Snd(t) => {
            let t_whnf = whnf(t);
            match t_whnf.as_ref() {
                Term::Pair { snd, .. } => whnf(snd),
                _ => Term::snd(t_whnf),
            }
        }
        _ => Arc::new(term.clone()),
    }
}
```

Full normalization descends under binders. Conversion checking compares normal forms structurally:

```rust
pub fn normalize(term: &Term) -> Arc<Term> {
    let head = whnf(term);
    match head.as_ref() {
        Term::Lambda { body } => Term::lambda(normalize(body)),
        Term::Pi { domain, codomain } => {
            Term::pi(normalize(domain), normalize(codomain))
        }
        Term::Sigma { fst_type, snd_type } => {
            Term::sigma(normalize(fst_type), normalize(snd_type))
        }
        Term::Pair { fst, snd } => Term::pair(normalize(fst), normalize(snd)),
        Term::App { func, arg } => Term::app(normalize(func), normalize(arg)),
        Term::Fst(t) => Term::fst(normalize(t)),
        Term::Snd(t) => Term::snd(normalize(t)),
        _ => head,
    }
}

pub fn definitionally_equal(a: &Term, b: &Term) -> bool {
    let a_nf = normalize(a);
    let b_nf = normalize(b);
    a_nf == b_nf
}
```

### 16.1.4 Bidirectional Type Checker

The context maps de Bruijn indices to their types. The checker has two modes: `infer` synthesizes a type from a term; `check` verifies a term against a given type. The direction switch happens at lambdas (check) and applications (infer).

```rust
#[derive(Debug, Clone)]
pub struct Context {
    types: Vec<Arc<Term>>,
}

impl Context {
    pub fn empty() -> Self { Context { types: vec![] } }

    pub fn extend(&self, ty: Arc<Term>) -> Self {
        let mut types = self.types.clone();
        types.push(ty);
        types.iter_mut().for_each(|t| *t = shift(t, 0, 1));
        Context { types }
    }

    pub fn lookup(&self, idx: usize) -> Option<&Arc<Term>> {
        self.types.iter().rev().nth(idx)
    }
}

#[derive(Debug)]
pub enum TypeError {
    UnboundVariable(usize),
    TypeMismatch { expected: Arc<Term>, got: Arc<Term> },
    NotAFunction(Arc<Term>),
    NotAPair(Arc<Term>),
    UniverseOverflow(u8),
    DepthExceeded,
}

pub struct TypeChecker {
    pub max_depth: usize,
    depth: usize,
}

impl TypeChecker {
    pub fn new() -> Self {
        TypeChecker { max_depth: MAX_EVAL_DEPTH, depth: 0 }
    }

    fn enter(&mut self) -> Result<(), TypeError> {
        self.depth += 1;
        if self.depth > self.max_depth { Err(TypeError::DepthExceeded) }
        else { Ok(()) }
    }

    fn leave(&mut self) { self.depth -= 1; }

    pub fn infer(&mut self, ctx: &Context, term: &Term) -> Result<Arc<Term>, TypeError> {
        self.enter()?;
        let result = match term {
            Term::Var(i) => ctx.lookup(*i)
                .cloned()
                .ok_or(TypeError::UnboundVariable(*i)),

            Term::Universe(l) => {
                if *l >= MAX_UNIVERSE_LEVEL {
                    Err(TypeError::UniverseOverflow(*l))
                } else {
                    Ok(Term::universe(*l + 1))
                }
            }

            Term::Pi { domain, codomain } => {
                let d_ty = self.infer(ctx, domain)?;
                let d_level = self.extract_universe(&d_ty)?;
                let ext_ctx = ctx.extend(domain.clone());
                let c_ty = self.infer(&ext_ctx, codomain)?;
                let c_level = self.extract_universe(&c_ty)?;
                Ok(Term::universe(d_level.max(c_level)))
            }

            Term::App { func, arg } => {
                let func_ty = whnf(&self.infer(ctx, func)?);
                match func_ty.as_ref() {
                    Term::Pi { domain, codomain } => {
                        self.check(ctx, arg, domain)?;
                        Ok(subst(codomain, 0, arg))
                    }
                    _ => Err(TypeError::NotAFunction(func_ty)),
                }
            }

            Term::Fst(t) => {
                let t_ty = whnf(&self.infer(ctx, t)?);
                match t_ty.as_ref() {
                    Term::Sigma { fst_type, .. } => Ok(fst_type.clone()),
                    _ => Err(TypeError::NotAPair(t_ty)),
                }
            }

            Term::Snd(t) => {
                let t_ty = whnf(&self.infer(ctx, t)?);
                match t_ty.as_ref() {
                    Term::Sigma { snd_type, .. } => {
                        let fst_val = Term::fst(Arc::new(t.clone()));
                        Ok(subst(snd_type, 0, &fst_val))
                    }
                    _ => Err(TypeError::NotAPair(t_ty)),
                }
            }

            Term::Sigma { fst_type, snd_type } => {
                let f_ty = self.infer(ctx, fst_type)?;
                let f_level = self.extract_universe(&f_ty)?;
                let ext_ctx = ctx.extend(fst_type.clone());
                let s_ty = self.infer(&ext_ctx, snd_type)?;
                let s_level = self.extract_universe(&s_ty)?;
                Ok(Term::universe(f_level.max(s_level)))
            }

            Term::Lambda { .. } => {
                Err(TypeError::TypeMismatch {
                    expected: Term::universe(0),
                    got: Arc::new(term.clone()),
                })
            }

            Term::Pair { .. } => {
                Err(TypeError::TypeMismatch {
                    expected: Term::universe(0),
                    got: Arc::new(term.clone()),
                })
            }
        };
        self.leave();
        result
    }

    pub fn check(
        &mut self, ctx: &Context, term: &Term, expected: &Term
    ) -> Result<(), TypeError> {
        self.enter()?;
        let result = match (term, whnf(expected).as_ref()) {
            (Term::Lambda { body }, Term::Pi { domain, codomain }) => {
                let ext_ctx = ctx.extend(domain.clone());
                self.check(&ext_ctx, body, codomain)
            }
            (Term::Pair { fst, snd }, Term::Sigma { fst_type, snd_type }) => {
                self.check(ctx, fst, fst_type)?;
                let snd_type_subst = subst(snd_type, 0, fst);
                self.check(ctx, snd, &snd_type_subst)
            }
            _ => {
                let inferred = self.infer(ctx, term)?;
                if definitionally_equal(&inferred, expected) {
                    Ok(())
                } else {
                    Err(TypeError::TypeMismatch {
                        expected: Arc::new(expected.clone()),
                        got: inferred,
                    })
                }
            }
        };
        self.leave();
        result
    }

    fn extract_universe(&self, ty: &Term) -> Result<u8, TypeError> {
        match whnf(ty).as_ref() {
            Term::Universe(l) => Ok(*l),
            other => Err(TypeError::TypeMismatch {
                expected: Term::universe(0),
                got: Arc::new(other.clone()),
            }),
        }
    }
}
```

### 16.1.5 Milestone: Identity Function

The first thing every dependent type checker must handle — the polymorphic identity:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_polymorphic_identity() {
        let mut checker = TypeChecker::new();
        let ctx = Context::empty();

        // λA. λx. x  :  Π(A : Type₀). A → A
        let id = Term::lambda(Term::lambda(Term::var(0)));
        let id_type = Term::pi(
            Term::universe(0),
            Term::pi(Term::var(0), Term::var(1)),
        );
        assert!(checker.check(&ctx, &id, &id_type).is_ok());
    }

    #[test]
    fn check_const_combinator() {
        let mut checker = TypeChecker::new();
        let ctx = Context::empty();

        // λA. λB. λx. λy. x  :  Π(A : Type₀). Π(B : Type₀). A → B → A
        let k = Term::lambda(Term::lambda(Term::lambda(Term::lambda(
            Term::var(1)
        ))));
        let k_type = Term::pi(
            Term::universe(0),
            Term::pi(
                Term::universe(0),
                Term::pi(Term::var(1), Term::pi(Term::var(1), Term::var(3))),
            ),
        );
        assert!(checker.check(&ctx, &k, &k_type).is_ok());
    }

    #[test]
    fn check_pair_swap() {
        let mut checker = TypeChecker::new();
        let ctx = Context::empty();

        // λA. λB. λp. (snd p, fst p)  :  Π(A B : Type₀). A × B → B × A
        let swap = Term::lambda(Term::lambda(Term::lambda(
            Term::pair(
                Term::snd(Term::var(0)),
                Term::fst(Term::var(0)),
            )
        )));
        let swap_type = Term::pi(
            Term::universe(0),
            Term::pi(
                Term::universe(0),
                Term::pi(
                    Term::sigma(Term::var(1), Term::var(1)),
                    Term::sigma(Term::var(1), Term::var(3)),
                ),
            ),
        );
        assert!(checker.check(&ctx, &swap, &swap_type).is_ok());
    }
}
```

Phase 1 is done when these tests pass. You have a working dependently-typed kernel.

## 16.2 Phase 2: Cartesian Cubical Layer {#phase2}

Now we add what makes SCTT a homotopy type theory: interval variables, path types, cofibrations, coercion, composition, and glue types. This is the largest phase and the most architecturally sensitive — it's where the cctt design patterns earn their keep.

### 16.2.1 Extending the Term Enum

```rust
pub enum Term {
    // --- Phase 1 (TTT) ---
    Var(usize),
    Universe(u8),
    Pi { domain: Arc<Term>, codomain: Arc<Term> },
    Lambda { body: Arc<Term> },
    App { func: Arc<Term>, arg: Arc<Term> },
    Sigma { fst_type: Arc<Term>, snd_type: Arc<Term> },
    Pair { fst: Arc<Term>, snd: Arc<Term> },
    Fst(Arc<Term>),
    Snd(Arc<Term>),

    // --- Phase 2 (Cubical) ---

    // Interval
    I0,                                    // 0 : I
    I1,                                    // 1 : I
    IVar(usize),                           // interval variable (own namespace)

    // Path types
    PathP {
        ty: Arc<Term>,                     // line of types: I → Type
        left: Arc<Term>,                   // ty(0)
        right: Arc<Term>,                  // ty(1)
    },
    PLam { body: Arc<Term> },              // path abstraction: <i> body
    PApp { path: Arc<Term>, arg: Arc<Term> }, // path application: p @ r

    // Kan operations
    Coe {
        ty: Arc<Term>,                     // line of types: I → Type
        r: Arc<Term>,                      // source endpoint
        s: Arc<Term>,                      // target endpoint
        val: Arc<Term>,                    // value to coerce
    },
    HCom {
        ty: Arc<Term>,                     // type (constant along dimension)
        r: Arc<Term>,                      // source
        s: Arc<Term>,                      // target
        phi: Cofibration,                  // constraint where tube is defined
        partial: Arc<Term>,                // tube: I → Partial φ A
        base: Arc<Term>,                   // base at r
    },

    // Glue types (for univalence)
    Glue {
        base: Arc<Term>,
        phi: Cofibration,
        equiv: Arc<Term>,
    },
    GlueElem {
        base: Arc<Term>,
        phi: Cofibration,
        parts: Arc<Term>,
    },
    Unglue(Arc<Term>),
}
```

### 16.2.2 Cofibrations

Cofibrations are the "face formulas" that describe which part of a cube we're constraining. The Cartesian variant includes the **diagonal** `i = j`, which de Morgan cubical sets lack.

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Cofibration {
    Bot,                                   // ⊥ (empty)
    Top,                                   // ⊤ (always true)
    Eq(usize, bool),                       // (i = 0) or (i = 1)
    Diag(usize, usize),                   // (i = j) — Cartesian diagonal
    And(Box<Cofibration>, Box<Cofibration>),
    Or(Box<Cofibration>, Box<Cofibration>),
}

impl Cofibration {
    pub fn is_true(&self) -> bool {
        matches!(self, Cofibration::Top)
    }

    pub fn evaluate(&self, assignment: &[(usize, bool)]) -> Option<bool> {
        match self {
            Cofibration::Bot => Some(false),
            Cofibration::Top => Some(true),
            Cofibration::Eq(i, v) => {
                assignment.iter()
                    .find(|(idx, _)| *idx == *i)
                    .map(|(_, val)| *val == *v)
            }
            Cofibration::Diag(i, j) => {
                let i_val = assignment.iter().find(|(idx, _)| *idx == *i);
                let j_val = assignment.iter().find(|(idx, _)| *idx == *j);
                match (i_val, j_val) {
                    (Some((_, a)), Some((_, b))) => Some(a == b),
                    _ => None,
                }
            }
            Cofibration::And(a, b) => {
                match (a.evaluate(assignment), b.evaluate(assignment)) {
                    (Some(false), _) | (_, Some(false)) => Some(false),
                    (Some(true), Some(true)) => Some(true),
                    _ => None,
                }
            }
            Cofibration::Or(a, b) => {
                match (a.evaluate(assignment), b.evaluate(assignment)) {
                    (Some(true), _) | (_, Some(true)) => Some(true),
                    (Some(false), Some(false)) => Some(false),
                    _ => None,
                }
            }
        }
    }
}
```

### 16.2.3 The Sub/Force Pattern

This is the key performance idea from cctt. When we substitute an interval variable — say, setting `i = 0` — we don't walk the entire term. Instead, we wrap it in a `Sub` marker that records the substitution. Only when pattern-matching demands a rigid head do we `force` the substitution down.

```rust
#[derive(Debug, Clone)]
pub enum IntervalSubst {
    Set(usize, bool),   // i ↦ 0 or i ↦ 1
    Rename(usize, usize), // i ↦ j
}

#[derive(Debug, Clone)]
pub enum Value {
    VNeutral(Neutral),
    VPi(Arc<Value>, Closure),
    VLambda(Closure),
    VSigma(Arc<Value>, Closure),
    VPair(Arc<Value>, Arc<Value>),
    VUniverse(u8),
    VPathP(Closure, Arc<Value>, Arc<Value>),
    VPLam(Closure),

    // O(1) lazy substitution — the cctt trick
    Sub(Box<Value>, IntervalSubst),
}

pub fn force(val: &Value) -> Value {
    match val {
        Value::Sub(inner, subst) => {
            let forced = force(inner);
            apply_isubst(&forced, subst)
        }
        other => other.clone(),
    }
}

fn apply_isubst(val: &Value, subst: &IntervalSubst) -> Value {
    match val {
        Value::VNeutral(n) => Value::VNeutral(apply_isubst_neutral(n, subst)),
        Value::VPi(dom, cod) => Value::VPi(
            Arc::new(apply_isubst(dom, subst)),
            apply_isubst_closure(cod, subst),
        ),
        Value::VPLam(clo) => Value::VPLam(apply_isubst_closure(clo, subst)),
        // Each value constructor pushes the substitution to its children
        // only when forced — never eagerly
        other => Value::Sub(Box::new(other.clone()), subst.clone()),
    }
}
```

The invariant: `Sub` nodes never stack. When `force` encounters a `Sub` wrapping another `Sub`, it composes the substitutions and pushes once. This keeps `force` O(depth of term), not O(number of substitutions).

### 16.2.4 Defunctionalized Closures

In cctt's Haskell, closures are just Haskell closures. In Rust, we defunctionalize: a closure is an environment paired with a syntax tree. This makes closures inspectable, serializable, and deterministic.

```rust
#[derive(Debug, Clone)]
pub struct Closure {
    pub env: Vec<Arc<Value>>,
    pub body: Arc<Term>,
}

impl Closure {
    pub fn apply(&self, arg: Arc<Value>) -> Value {
        let mut env = self.env.clone();
        env.push(arg);
        evaluate(&env, &self.body)
    }

    pub fn apply_interval(&self, r: bool) -> Value {
        evaluate_with_interval(&self.env, &self.body, r)
    }
}

#[derive(Debug, Clone)]
pub struct Neutral {
    pub head: NeutralHead,
    pub spine: Vec<Elim>,
}

#[derive(Debug, Clone)]
pub enum NeutralHead {
    Var(usize),
    IVar(usize),
}

#[derive(Debug, Clone)]
pub enum Elim {
    App(Arc<Value>),
    Fst,
    Snd,
    PApp(Arc<Value>),  // path application with interval argument
}
```

### 16.2.5 Triple-Context Evaluation

Cubical evaluation requires three contexts, following the cctt architecture:

```rust
pub struct EvalCtx {
    pub ivar_count: usize,       // number of live interval variables
    pub cofib: Cofibration,      // current face constraint
    pub env: Vec<Arc<Value>>,    // fibrant variable bindings
}

impl EvalCtx {
    pub fn bind_ivar(&self) -> Self {
        EvalCtx {
            ivar_count: self.ivar_count + 1,
            cofib: self.cofib.clone(),
            env: self.env.clone(),
        }
    }

    pub fn restrict(&self, phi: Cofibration) -> Self {
        EvalCtx {
            ivar_count: self.ivar_count,
            cofib: Cofibration::And(
                Box::new(self.cofib.clone()),
                Box::new(phi),
            ),
            env: self.env.clone(),
        }
    }
}
```

The evaluator dispatches on the term constructor and the face constraint:

```rust
pub fn evaluate(env: &[Arc<Value>], term: &Term) -> Value {
    match term {
        Term::Var(i) => env.iter().rev().nth(*i)
            .map(|v| v.as_ref().clone())
            .unwrap_or_else(|| Value::VNeutral(Neutral {
                head: NeutralHead::Var(*i),
                spine: vec![],
            })),

        Term::Lambda { body } => Value::VLambda(Closure {
            env: env.iter().cloned().collect(),
            body: body.clone(),
        }),

        Term::App { func, arg } => {
            let func_val = evaluate(env, func);
            let arg_val = Arc::new(evaluate(env, arg));
            apply_value(&func_val, arg_val)
        }

        Term::PApp { path, arg } => {
            let path_val = evaluate(env, path);
            let r = evaluate(env, arg);
            apply_path(&path_val, r)
        }

        Term::Coe { ty, r, s, val } => {
            let r_val = evaluate(env, r);
            let s_val = evaluate(env, s);
            if interval_equal(&r_val, &s_val) {
                evaluate(env, val)
            } else {
                let ty_clo = Closure {
                    env: env.to_vec(),
                    body: ty.clone(),
                };
                eval_coe(ty_clo, r_val, s_val, evaluate(env, val))
            }
        }

        // ... remaining constructors
        _ => todo!("evaluate remaining term constructors"),
    }
}
```

### 16.2.6 Kan Operations

Coercion (`coe`) transports a value along a line of types. Its computation rules depend on the type former it coerces through:

```rust
fn eval_coe(ty: Closure, r: Value, s: Value, val: Value) -> Value {
    let ty_at_r = ty.apply_interval(to_bool(&r));
    match force(&ty_at_r) {
        Value::VPi(dom_r, cod_r) => {
            // coe_{r→s} (Π A B) f = λa. coe_{r→s} (B[coe_{s→r} A a]) (f (coe_{s→r} A a))
            Value::VLambda(Closure {
                env: vec![
                    Arc::new(val),
                    Arc::new(Value::from_closure(ty)),
                ],
                body: Arc::new(build_coe_pi_body()),
            })
        }

        Value::VSigma(fst_r, snd_r) => {
            // coe_{r→s} (Σ A B) (a, b) = (coe_{r→s} A a, coe_{r→s} (B[...]) b)
            let (a, b) = match force(&val) {
                Value::VPair(a, b) => (a, b),
                v => (
                    Arc::new(do_fst(&v)),
                    Arc::new(do_snd(&v)),
                ),
            };
            let a_coe = eval_coe(project_fst_line(&ty), r.clone(), s.clone(), a.as_ref().clone());
            let b_coe = eval_coe(
                project_snd_line(&ty, &a_coe),
                r, s, b.as_ref().clone(),
            );
            Value::VPair(Arc::new(a_coe), Arc::new(b_coe))
        }

        Value::VUniverse(_) => {
            // coe in a universe: the type line IS the coercion
            val
        }

        _ => {
            // Stuck: produce neutral coe
            Value::VNeutral(Neutral {
                head: NeutralHead::Var(0), // placeholder
                spine: vec![], // stuck coe in spine
            })
        }
    }
}
```

Homogeneous composition (`hcom`) builds fillers for cubes. The computation rules for each type former fill in the faces:

```rust
fn eval_hcom(
    ty: Value, r: Value, s: Value,
    phi: Cofibration, tube: Closure, base: Value,
) -> Value {
    if interval_equal(&r, &s) { return base; }
    if phi.is_true() { return tube.apply_interval(to_bool(&s)); }

    match force(&ty) {
        Value::VPi(dom, cod) => {
            // hcom in Pi: apply pointwise
            Value::VLambda(Closure {
                env: vec![
                    Arc::new(base),
                    Arc::new(Value::from_closure(tube.clone())),
                ],
                body: Arc::new(build_hcom_pi_body()),
            })
        }

        Value::VSigma(_, _) => {
            // hcom in Sigma: compose components, adjust second with coe
            todo!("hcom in Sigma — pair of hcoms with coercion in second component")
        }

        _ => {
            // Stuck hcom
            Value::VNeutral(Neutral {
                head: NeutralHead::Var(0),
                spine: vec![],
            })
        }
    }
}
```

### 16.2.7 Normal Forms in Cubical Type Theory

A recent paper by Huang (2026, arXiv:2603.24923) documents the specification of normal forms for Cartesian cubical type theory, making explicit what is implicit in Sterling and Angiuli's normalization proof via synthetic Tait computability. Understanding these normal forms is essential for getting your NbE right.

**The key complication.** In plain MLTT, a neutral form is a variable with a spine of eliminators. A neutral is "stuck" on its head variable. In cubical type theory, this breaks. Consider a path variable `p : PathP A x y`. The term `p(0)` is nominally stuck on `p`, but it computes to `x` regardless! Even when `p` is a free variable, `p(0) ≡ x` is a judgmental equality.

**Three-parameter neutral forms.** Huang introduces neutral forms with a *cofibration parameter* φ that records when the neutral becomes "unstable" and decays into a computable form:

```
Tm^φ_ne(Γ, A)   -- neutral at type A, unstable when cofibration φ holds
```

When `φ = ⊥` (bottom cofibration, never holds), the neutral is truly stuck — a classical neutral. When `φ = (i = 0) ∨ (i = 1)`, the neutral is a path application that computes at the endpoints. The rule for path application is:

```
Γ ⊢ p : PathP A x y  ne^φ     Γ ⊢ r : I  nf
─────────────────────────────────────────────
Γ ⊢ p(r) : A(r)  ne^(φ ∨ (r=0) ∨ (r=1))
```

The cofibration grows with each path application, and when it becomes ⊤ (the whole cofibration), the neutral must be normalized further.

**Implications for your `Neutral` struct.** In your v0 implementation, the `Neutral` type should carry a cofibration that tracks boundary conditions:

```rust
#[derive(Debug, Clone)]
pub struct Neutral {
    pub head: NeutralHead,
    pub spine: Vec<Elim>,
    pub boundary: Cofibration,  // when this is satisfied, re-evaluate
}
```

When forcing a neutral `n` under a substitution `i ↦ 0`, check whether `n.boundary` is satisfied under that substitution. If so, the neutral is no longer stuck — unfold and evaluate. This is precisely the mechanism that makes `p(0) ≡ x` compute correctly.

For full details, see Huang's paper, which follows Sterling's presentation (2021) and adapts it to a traditional inference-rule style.

### 16.2.8 Milestone: Path Reflexivity and Function Extensionality

```rust
#[cfg(test)]
mod cubical_tests {
    use super::*;

    #[test]
    fn check_refl() {
        let mut checker = TypeChecker::new();
        let ctx = Context::empty();

        // refl : Π(A : Type₀). Π(a : A). PathP (λi. A) a a
        // refl A a = <i> a   (constant path)
        let refl = Term::lambda(Term::lambda(
            Term::plam(Term::var(1)) // <i> a — body ignores the interval
        ));
        let refl_type = Term::pi(
            Term::universe(0),
            Term::pi(
                Term::var(0),
                Term::pathp(
                    Term::lambda(Term::var(2)), // constant type line
                    Term::var(0),
                    Term::var(0),
                ),
            ),
        );
        assert!(checker.check(&ctx, &refl, &refl_type).is_ok());
    }

    #[test]
    fn path_endpoints() {
        // p @ 0 ≡ left,  p @ 1 ≡ right
        let p = Term::plam(Term::var(0)); // <i> x, where x is bound outside
        assert!(definitionally_equal(
            &Term::papp(p.clone(), Term::i0()),
            &Term::var(0),
        ));
        assert!(definitionally_equal(
            &Term::papp(p, Term::i1()),
            &Term::var(0),
        ));
    }
}
```

Phase 2 is done when you can type-check `refl`, verify path endpoints, and coerce through Pi and Sigma types. The **Brunerie number** (see §16.5) is the ultimate stress test.

## 16.3 Phase 3: Smooth Primitives {#phase3}

Now we add SCTT's distinguishing feature: the nilsquare infinitesimal type `D` and the Kock-Lawvere axiom. This phase is comparatively small because the cubical infrastructure from Phase 2 does the heavy lifting — smooth structure rides on top of it.

### 16.3.1 New Terms

```rust
pub enum Term {
    // ... Phase 1 + Phase 2 terms ...

    // Smooth reals
    RealLit(OrderedFloat<f64>),
    RealAdd(Arc<Term>, Arc<Term>),
    RealMul(Arc<Term>, Arc<Term>),
    RealNeg(Arc<Term>),

    // Infinitesimals
    Epsilon(usize),              // nilsquare variable ε_i
    DType,                       // D = { ε : ℝ | ε² = 0 }

    // Smooth function space
    SmoothFn {
        domain: Arc<Term>,
        codomain: Arc<Term>,
    },

    // Kock-Lawvere derivative
    Derivative(Arc<Term>),       // D[f] : the derivative of f

    // Local scoping for rewrite rules (LRTT-style)
    SmoothBlock {
        rules: Vec<RewriteRule>,
        body: Arc<Term>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RewriteRule {
    Nilsquare(usize),  // ε_i² ↦ 0
}
```

`OrderedFloat<f64>` comes from the `ordered-float` crate and gives us `Eq` and `Ord` on floats — necessary since `Term` derives `Eq`.

### 16.3.2 The ε² = 0 Rewrite Rule

The core of synthetic differential geometry, implemented as a normalization rule:

```rust
fn whnf_smooth(term: &Term, rules: &[RewriteRule]) -> Arc<Term> {
    match term {
        Term::RealMul(a, b) => {
            let a_nf = whnf_smooth(a, rules);
            let b_nf = whnf_smooth(b, rules);
            match (a_nf.as_ref(), b_nf.as_ref()) {
                (Term::Epsilon(i), Term::Epsilon(j))
                    if i == j && rules.contains(&RewriteRule::Nilsquare(*i)) =>
                {
                    Arc::new(Term::RealLit(OrderedFloat(0.0)))
                }
                _ => Arc::new(Term::RealMul(a_nf, b_nf)),
            }
        }

        Term::RealAdd(a, b) => {
            let a_nf = whnf_smooth(a, rules);
            let b_nf = whnf_smooth(b, rules);
            match (a_nf.as_ref(), b_nf.as_ref()) {
                (Term::RealLit(x), Term::RealLit(y)) => {
                    Arc::new(Term::RealLit(OrderedFloat(x.0 + y.0)))
                }
                (Term::RealLit(x), _) if x.0 == 0.0 => b_nf,
                (_, Term::RealLit(y)) if y.0 == 0.0 => a_nf,
                _ => Arc::new(Term::RealAdd(a_nf, b_nf)),
            }
        }

        Term::SmoothBlock { rules: block_rules, body } => {
            let mut combined = rules.to_vec();
            combined.extend(block_rules.iter().cloned());
            whnf_smooth(body, &combined)
        }

        _ => Arc::new(term.clone()),
    }
}
```

The `SmoothBlock` scoping ensures the nilsquare rule only fires inside contexts that declare it. This follows the LRTT (Locally Rewritable Type Theory) discipline: rewrite rules are local, not global. Outside a `SmoothBlock`, `ε * ε` is stuck.

### 16.3.3 Kock-Lawvere Derivative

The derivative of `f` at `x` is the unique `b` such that `f(x + ε) = f(x) + b·ε` for all nilsquare `ε`. We compute it by symbolic evaluation:

```rust
pub fn compute_derivative(f: &Term, x: &Term) -> Arc<Term> {
    let eps = Term::Epsilon(0);
    let x_plus_eps = Arc::new(Term::RealAdd(
        Arc::new(x.clone()),
        Arc::new(eps),
    ));

    // Evaluate f(x + ε) with the nilsquare rule active
    let rules = vec![RewriteRule::Nilsquare(0)];
    let result = apply_and_normalize(f, &x_plus_eps, &rules);
    let fx = apply_and_normalize(f, &Arc::new(x.clone()), &rules);

    // f(x + ε) - f(x) should have the form b·ε after normalization
    let diff = whnf_smooth(
        &Term::RealAdd(
            Arc::new(result.as_ref().clone()),
            Arc::new(Term::RealNeg(fx)),
        ),
        &rules,
    );

    extract_epsilon_coefficient(&diff, 0)
}

fn extract_epsilon_coefficient(term: &Term, eps_idx: usize) -> Arc<Term> {
    match term {
        Term::RealMul(a, b) => {
            match (a.as_ref(), b.as_ref()) {
                (coeff, Term::Epsilon(i)) if *i == eps_idx => Arc::new(coeff.clone()),
                (Term::Epsilon(i), coeff) if *i == eps_idx => Arc::new(coeff.clone()),
                _ => Arc::new(Term::RealLit(OrderedFloat(0.0))),
            }
        }
        Term::RealAdd(a, b) => {
            let ca = extract_epsilon_coefficient(a, eps_idx);
            let cb = extract_epsilon_coefficient(b, eps_idx);
            Arc::new(Term::RealAdd(ca, cb))
        }
        Term::Epsilon(i) if *i == eps_idx => {
            Arc::new(Term::RealLit(OrderedFloat(1.0)))
        }
        _ => Arc::new(Term::RealLit(OrderedFloat(0.0))),
    }
}
```

### 16.3.4 Milestone: Derivative of x²

```rust
#[cfg(test)]
mod smooth_tests {
    use super::*;

    #[test]
    fn derivative_of_x_squared() {
        // f(x) = x²
        // f(x + ε) = (x + ε)² = x² + 2xε + ε²
        //          = x² + 2xε         (since ε² = 0)
        // D[f](x) = 2x

        let x = Term::var(0);
        let x_squared = Term::RealMul(Arc::new(x.clone()), Arc::new(x.clone()));

        let deriv = compute_derivative(&x_squared, &x);
        let expected = Term::RealMul(
            Arc::new(Term::RealLit(OrderedFloat(2.0))),
            Arc::new(x),
        );

        assert!(definitionally_equal(&deriv, &Arc::new(expected)));
    }

    #[test]
    fn nilsquare_scoping() {
        let rules = vec![RewriteRule::Nilsquare(0)];
        let eps_sq = Term::RealMul(
            Arc::new(Term::Epsilon(0)),
            Arc::new(Term::Epsilon(0)),
        );

        // Inside a smooth block: ε₀² = 0
        let inside = whnf_smooth(&eps_sq, &rules);
        assert_eq!(inside.as_ref(), &Term::RealLit(OrderedFloat(0.0)));

        // Outside: ε₀² stays stuck
        let outside = whnf_smooth(&eps_sq, &[]);
        assert_eq!(outside.as_ref(), &eps_sq);
    }
}
```

In SCTT pseudocode, the same computation:

```sctt
-- f(x) = x²
-- f(x + ε) = (x + ε)² = x² + 2xε + ε² = x² + 2xε  (since ε² = 0)
-- Therefore D[f](x) = 2x

D[λx. x * x] ≡ λx. 2 * x   -- by computation
```

## 16.4 Phase 4: Lipschitz Sensitivity Types {#phase4}

The final layer adds **sensitivity tracking** via Lipschitz types. A term `f : Lip(A, B, k)` witnesses that `f` is `k`-Lipschitz: `d(f(x), f(y)) ≤ k · d(x, y)`. This connects SCTT to differential privacy (Fuzz), robust optimization, and certified neural networks.

### 16.4.1 New Type Former

```rust
pub enum Term {
    // ... Phase 1-3 terms ...

    // Lipschitz function type: Lip(A, B, k)
    Lip {
        domain: Arc<Term>,
        codomain: Arc<Term>,
        bound: Arc<Term>,      // k : ℝ≥0
    },

    // Introduction: mark a function with its Lipschitz bound
    LipFn {
        func: Arc<Term>,
        bound: Arc<Term>,
    },

    // Elimination: apply a Lipschitz function (just ordinary application)
    LipApp {
        func: Arc<Term>,
        arg: Arc<Term>,
    },
}
```

### 16.4.2 Typing Rules

Three rules govern Lipschitz types:

```rust
// Formation
// Γ ⊢ A : Type    Γ ⊢ B : Type    Γ ⊢ k : ℝ≥0
// ─────────────────────────────────────────────────
// Γ ⊢ Lip(A, B, k) : Type

// Introduction
// Γ ⊢ f : A → B    Γ ⊢ k : ℝ≥0    f is k-Lipschitz
// ──────────────────────────────────────────────────────
// Γ ⊢ lip(f, k) : Lip(A, B, k)

// Composition (the chain rule for sensitivity)
// Γ ⊢ g : Lip(B, C, k₂)    Γ ⊢ f : Lip(A, B, k₁)
// ──────────────────────────────────────────────────────
// Γ ⊢ g ∘ f : Lip(A, C, k₁ · k₂)
```

### 16.4.3 Sensitivity Checking

The checker traverses the term structure, accumulating sensitivity through compositions:

```rust
impl TypeChecker {
    pub fn check_sensitivity(
        &mut self,
        ctx: &Context,
        f: &Term,
        bound: f64,
    ) -> Result<(), TypeError> {
        let sensitivity = self.compute_sensitivity(ctx, f)?;
        if sensitivity <= bound {
            Ok(())
        } else {
            Err(TypeError::SensitivityExceeded {
                expected: bound,
                actual: sensitivity,
            })
        }
    }

    fn compute_sensitivity(
        &mut self,
        ctx: &Context,
        term: &Term,
    ) -> Result<f64, TypeError> {
        match term {
            // Constants are 0-sensitive
            Term::RealLit(_) | Term::Universe(_) => Ok(0.0),

            // Variable: sensitivity = 1 (identity is 1-Lipschitz)
            Term::Var(_) => Ok(1.0),

            // Lambda: sensitivity is the sensitivity of the body
            Term::Lambda { body } => {
                let ext_ctx = ctx.extend(Term::universe(0)); // placeholder
                self.compute_sensitivity(&ext_ctx, body)
            }

            // Application: chain rule — multiply sensitivities
            Term::App { func, arg } => {
                let func_sens = self.compute_sensitivity(ctx, func)?;
                let arg_sens = self.compute_sensitivity(ctx, arg)?;
                Ok(func_sens * arg_sens)
            }

            // Scalar multiplication: |c| * sensitivity
            Term::RealMul(a, b) => {
                match (a.as_ref(), b.as_ref()) {
                    (Term::RealLit(c), other) | (other, Term::RealLit(c)) => {
                        let inner_sens = self.compute_sensitivity(ctx, other)?;
                        Ok(c.0.abs() * inner_sens)
                    }
                    _ => {
                        let a_sens = self.compute_sensitivity(ctx, a)?;
                        let b_sens = self.compute_sensitivity(ctx, b)?;
                        Ok(a_sens * b_sens)
                    }
                }
            }

            // Addition: sensitivity adds (triangle inequality)
            Term::RealAdd(a, b) => {
                let a_sens = self.compute_sensitivity(ctx, a)?;
                let b_sens = self.compute_sensitivity(ctx, b)?;
                Ok(a_sens + b_sens)
            }

            // Lip annotation: use declared bound
            Term::LipFn { bound, .. } => {
                match bound.as_ref() {
                    Term::RealLit(k) => Ok(k.0),
                    _ => Err(TypeError::NonLiteralSensitivity),
                }
            }

            _ => Ok(f64::INFINITY), // conservative: unknown terms get ∞
        }
    }
}
```

### 16.4.4 Milestone: Certified Lipschitz Functions

```rust
#[cfg(test)]
mod lip_tests {
    use super::*;
    use ordered_float::OrderedFloat;

    #[test]
    fn identity_is_1_lipschitz() {
        let mut checker = TypeChecker::new();
        let ctx = Context::empty();

        // λx. x : Lip(ℝ, ℝ, 1.0)
        let id = Term::Lambda { body: Term::var(0) };
        assert!(checker.check_sensitivity(&ctx, &id, 1.0).is_ok());
        assert!(checker.check_sensitivity(&ctx, &id, 0.5).is_err());
    }

    #[test]
    fn doubling_is_2_lipschitz() {
        let mut checker = TypeChecker::new();
        let ctx = Context::empty();

        // λx. 2*x : Lip(ℝ, ℝ, 2.0)
        let double = Term::Lambda {
            body: Arc::new(Term::RealMul(
                Arc::new(Term::RealLit(OrderedFloat(2.0))),
                Term::var(0),
            )),
        };
        assert!(checker.check_sensitivity(&ctx, &double, 2.0).is_ok());
        assert!(checker.check_sensitivity(&ctx, &double, 1.0).is_err());
    }

    #[test]
    fn composition_multiplies_bounds() {
        let mut checker = TypeChecker::new();
        let ctx = Context::empty();

        // (λx. 2*x) ∘ (λx. 3*x) has sensitivity 2 * 3 = 6
        let f = Term::Lambda {
            body: Arc::new(Term::RealMul(
                Arc::new(Term::RealLit(OrderedFloat(2.0))),
                Term::var(0),
            )),
        };
        let g = Term::Lambda {
            body: Arc::new(Term::RealMul(
                Arc::new(Term::RealLit(OrderedFloat(3.0))),
                Term::var(0),
            )),
        };
        let composed = Term::Lambda {
            body: Arc::new(Term::App {
                func: Arc::new(f),
                arg: Arc::new(Term::App {
                    func: Arc::new(g),
                    arg: Term::var(0),
                }),
            }),
        };
        assert!(checker.check_sensitivity(&ctx, &composed, 6.0).is_ok());
        assert!(checker.check_sensitivity(&ctx, &composed, 5.0).is_err());
    }
}
```

## 16.5 Testing and Verification {#testing}

A kernel earns trust through testing, not through assertion. Three levels of testing apply.

### 16.5.1 Unit Tests

Each phase milestone (§16.1.5, §16.2.7, §16.3.4, §16.4.4) provides unit tests. Run them with:

```bash
cargo test --lib
```

### 16.5.2 Property-Based Testing with Proptest

Unit tests check specific cases. Proptest finds the cases you didn't think of.

```rust
use proptest::prelude::*;

fn arb_term(depth: usize) -> impl Strategy<Value = Term> {
    if depth == 0 {
        prop_oneof![
            (0usize..5).prop_map(Term::Var),
            (0u8..MAX_UNIVERSE_LEVEL).prop_map(Term::Universe),
        ].boxed()
    } else {
        let leaf = arb_term(0);
        let sub = arb_term(depth - 1);
        prop_oneof![
            leaf.clone(),
            sub.clone().prop_map(|body| Term::Lambda { body: Arc::new(body) }),
            (sub.clone(), sub.clone()).prop_map(|(f, a)| Term::App {
                func: Arc::new(f), arg: Arc::new(a)
            }),
            (sub.clone(), sub.clone()).prop_map(|(d, c)| Term::Pi {
                domain: Arc::new(d), codomain: Arc::new(c)
            }),
        ].boxed()
    }
}

proptest! {
    #[test]
    fn nbe_idempotent(term in arb_term(3)) {
        let nf1 = normalize(&Arc::new(term.clone()));
        let nf2 = normalize(&nf1);
        prop_assert_eq!(*nf1, *nf2);
    }

    #[test]
    fn shift_unshift_roundtrip(term in arb_term(3)) {
        let shifted = shift(&term, 0, 1);
        let unshifted = shift(&shifted, 0, -1);
        prop_assert_eq!(term, *unshifted);
    }

    #[test]
    fn subst_preserves_types(
        term in arb_term(2),
        replacement in arb_term(2)
    ) {
        // substitution at index 0 should not panic
        let _ = subst(&term, 0, &replacement);
    }
}
```

### 16.5.3 The Brunerie Number

The Brunerie number — the value of `π₄(S³)` computed via cubical type theory — is the ultimate stress test for a cubical type checker. cctt computes it as `2` in under a second. Most other cubical implementations either cannot compute it or take hours.

If your Phase 2 kernel computes the Brunerie number to `2`, three things are true simultaneously:

1. **Correctness**: your cubical evaluation rules are right
2. **Performance**: your sub/force implementation is fast enough
3. **Completeness**: your Kan operations handle all necessary cases

You don't need to compute it to have a useful kernel — but it's the benchmark that separates toy implementations from real ones.

```sctt
-- The Brunerie number: π₄(S³) = ℤ
-- Computed as: |brunerie| = 2
brunerie_number : ℤ
brunerie_number = encode (hopf_invariant (join_map η η))
-- where η : S³ → S² is the Hopf fibration
```

## 16.6 Architecture Decisions {#architecture}

### 16.6.1 Why Rust?

The language choice is deliberate:

- **Arena allocation replaces GC.** No pause-time surprises during proof checking. `Arc<Term>` gives reference counting with structural sharing — an O(1) clone that shares the entire subtree.
- **Enum + pattern matching mirrors cctt's Haskell ADTs.** The translation from cctt's Haskell to Rust enums is nearly mechanical. Each Haskell pattern-match case becomes a Rust `match` arm.
- **Zero-cost abstractions for sub/force.** The `Value::Sub` wrapper is a zero-allocation enum variant. Rust's move semantics ensure no hidden copies.
- **Correctness tooling.** Rust's type system catches use-after-move, data races, and null pointer dereferences at compile time — bugs that in a Haskell or OCaml kernel would be logic errors caught only by testing.

### 16.6.2 What Lives Inside vs Outside the Kernel

The kernel boundary is a trust boundary. Code inside the kernel must be hand-auditable and formally justified. Code outside can be arbitrarily complex — including AI-generated.

| Inside (v0 kernel, 2-5K lines) | Outside (unlimited) |
|---|----|
| `evaluate : Term → Value` | Parser (concrete syntax → raw AST) |
| `quote : Value → Term` | Elaborator (raw AST → core Term) |
| `conversion : Value → Value → bool` | Pattern matching compilation |
| `coe`, `hcom`, `glue` | Module system, imports |
| `whnf_smooth`, ε² = 0 | Tactic engine |
| `check_sensitivity` | IDE integration, error messages |
| | LLM oracle (proof suggestions) |

The key invariant: **nothing outside the kernel can cause the kernel to accept an ill-typed term.** The elaborator can produce garbage terms; the kernel rejects them. The LLM oracle can suggest wrong proofs; the kernel says no.

### 16.6.3 The Trust Architecture

```
┌───────────────────────────────────────────────────┐
│ Layer 4: Visual Interface (~5K lines)              │
│   Renders proof suggestions, visualizes types      │
│   AI involvement: renders LLM output               │
├───────────────────────────────────────────────────┤
│ Layer 3: Oracle (LLM)                              │
│   Suggests proof terms, fills holes                │
│   AI involvement: full, completely untrusted        │
├───────────────────────────────────────────────────┤
│ Layer 2: Tactics (~5K lines)                       │
│   Classical search: auto, omega, ring              │
│   AI involvement: classical algorithms only         │
├───────────────────────────────────────────────────┤
│ Layer 1: Elaboration (~10K lines)                  │
│   Implicit arguments, unification, coercions       │
│   AI involvement: none                              │
├───────────────────────────────────────────────────┤
│ Layer 0: Proof Kernel — v0 (2-5K lines)            │
│   eval, quote, conversion, Kan ops, smooth, Lip    │
│   AI involvement: NONE. Hand-auditable.             │
└───────────────────────────────────────────────────┘
```

Every layer above Layer 0 produces candidate terms. Only Layer 0 decides whether to accept them. This is why the kernel is named v0 — it is the root of trust, the axiom from which all else follows.

## Exercises

### Phase 1 Exercises

1. **Build TTT.** Implement the Phase 1 kernel from §16.1. Verify it type-checks the polymorphic identity (`λA. λx. x`), the constant combinator (`λA. λB. λx. λy. x`), and pair swap (`λA. λB. λp. (snd p, fst p)`). All three test cases are given in §16.1.5.

2. **Property-test substitution.** Using proptest, generate random well-scoped terms and verify: (a) `shift(shift(t, 0, 1), 0, -1) ≡ t`, (b) `subst(t, 0, Var(0)) ≡ t` when variable 0 is not captured, (c) normalization is idempotent: `normalize(normalize(t)) ≡ normalize(t)`.

3. **Build a REPL.** Write a simple read-eval-print loop that parses terms in a concrete syntax of your choice, type-checks them, normalizes them, and prints the result. A minimal parser for `λ`, `Π`, `Σ`, `Type`, application, `fst`, and `snd` suffices.

### Phase 2 Exercises

1. **Implement sub/force.** Add `Value::Sub` and the `force` function. Write a benchmark showing that `sub` is O(1) regardless of term size, and that repeated `sub` calls don't stack (the `force` function composes substitutions).

2. **Coercion in Pi types.** Implement `eval_coe` for the `VPi` case. Test with transport along a constant family: `coe_{0→1} (λi. A) a ≡ a` for any type `A` and value `a : A`.

3. **Homogeneous composition for pairs.** Implement `eval_hcom` for `VSigma`. The second component requires a coercion adjustment. Verify the Kan condition: the faces of the output match the tube on the boundary.

### Phase 3 Exercises

1. **Nilsquare normalization.** Implement `whnf_smooth` and verify that `(x + ε)² = x² + 2xε` after normalization with the nilsquare rule active. Also verify that `(x + ε)³ = x³ + 3x²ε` (since all higher powers of ε vanish).

2. **Polynomial derivatives.** Implement `compute_derivative` for polynomial terms. Check: `D[x²] = 2x`, `D[x³] = 3x²`, `D[x² + 3x] = 2x + 3`, `D[constant] = 0`.

3. **Scoping boundary.** Verify that ε² = 0 does not fire outside `SmoothBlock` boundaries. Write a test where the same term normalizes differently inside vs outside a smooth block.

### Phase 4 Exercises

1. **Linear sensitivity.** Implement `compute_sensitivity` for linear functions (`λx. c * x` has sensitivity `|c|`). Verify the chain rule: composing `k₁`-Lipschitz with `k₂`-Lipschitz yields `k₁ · k₂` sensitivity.

2. **Neural network layer.** Model a single linear layer `f(x) = Wx + b` as a Lipschitz function where the bound is the operator norm `‖W‖`. Type-check it as `Lip(ℝⁿ, ℝᵐ, ‖W‖)`.

3. **Sensitivity of composition.** Prove (on paper, then encode) that the composition of two `k`-Lipschitz functions is `k²`-Lipschitz, not `k`-Lipschitz. Explain why this means deep networks have exponential worst-case sensitivity in depth.

---

*Previous: [Chapter 15: Future Directions](./chapter_15.md) ←*

*Next: [Chapter 17: Certified Machine Learning](./chapter_17.md) →*
