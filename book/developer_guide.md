# The SCTT Developer's Guide

## A Complete Tutorial for Implementing Smooth Cubical Type Theory

> "There are many paths up the mountain, but the view from the top is the same." — Japanese proverb
>
> "In type theory, the path *is* the data." — SCTT, literally

This guide is for anyone who wants to **build** something with Smooth Cubical Type Theory — not just read about it. Whether you want to implement a kernel from scratch, extend an existing proof assistant, encode SCTT in a logical framework, or embed it as a DSL in your favorite language, this guide provides a concrete, step-by-step pathway for each approach.

We assume you have read Parts I–III of this book (or at least Chapters 1–3, 7, and 9) and have working knowledge of at least one typed programming language. No prior experience building type checkers is required — we build that skill here.

---

## Table of Contents

1. [Orientation: The Landscape of Approaches](#orientation)
2. [Prerequisites and Setup](#prerequisites)
3. [Approach 1: Ground-Up Rust Implementation](#approach-1)
4. [Approach 2: Forking an Existing Cubical Checker](#approach-2)
5. [Approach 3: Rewrite Rules in an Existing Proof Assistant](#approach-3)
6. [Approach 4: Encoding in Dedukti/Lambdapi](#approach-4)
7. [Approach 5: Two-Level Type Theory Staging](#approach-5)
8. [Approach 6: Embedded DSL in a Host Language](#approach-6)
9. [Approach 7: The Shallow Embedding (Quickest to Demo)](#approach-7)
10. [Cross-Cutting Concerns](#cross-cutting)
11. [Decision Matrix: Choosing Your Approach](#decision-matrix)
12. [Worked Examples](#worked-examples)
13. [Troubleshooting and Common Pitfalls](#troubleshooting)
14. [What to Build First: The Minimal Viable SCTT](#minimal-viable)

---

## 1. Orientation: The Landscape of Approaches {#orientation}

SCTT combines three independently complex type-theoretic layers:

```
┌──────────────────────────────────────────────────────────────────┐
│                     SCTT = Three Layers                          │
│                                                                  │
│   ┌──────────────┐   ┌──────────────────┐   ┌───────────────┐  │
│   │   Cubical     │   │     Smooth       │   │  Sensitivity  │  │
│   │   (paths,     │   │   (nilsquare     │   │  (Lipschitz   │  │
│   │    Kan ops,   │ + │    infinitesimals,│ + │   bounds,     │  │
│   │    univalence)│   │    SDG, tangent   │   │   distance    │  │
│   │              │   │    bundles)       │   │   types)      │  │
│   └──────┬───────┘   └────────┬─────────┘   └──────┬────────┘  │
│          │                    │                      │           │
│          └────────────────────┼──────────────────────┘           │
│                               │                                  │
│                    ┌──────────▼──────────┐                       │
│                    │  Dependent Types     │                       │
│                    │  (MLTT foundation)   │                       │
│                    └─────────────────────┘                       │
└──────────────────────────────────────────────────────────────────┘
```

Each approach handles these layers differently:

| Approach | Effort | Control | Community | Best For |
|----------|--------|---------|-----------|----------|
| 1. Ground-up Rust | Very High | Total | Build your own | Research, publication, long-term project |
| 2. Fork cctt/cooltt | High | High | Small | Fast cubical prototype |
| 3. Rewrite rules in Rocq/Agda | Medium | Medium | Large | Leveraging existing libraries |
| 4. Dedukti/Lambdapi encoding | Medium | Medium | Specialized | Cross-system interop |
| 5. Two-level staging | Medium-High | High | Growing | Performance + correctness |
| 6. Embedded DSL | Low-Medium | Low | Varies | Rapid prototyping, teaching |
| 7. Shallow embedding | Low | Low | Host lang | Quick demos, experimentation |

**Key insight:** You do not have to pick one approach. Many successful projects start with Approach 7 (shallow embedding) to prototype, graduate to Approach 3 (rewrite rules) to verify, and eventually move to Approach 1 (ground-up) for the production system. The approaches are a ladder, not a menu.

---

## 2. Prerequisites and Setup {#prerequisites}

### 2.1 Mathematical Background

You need working comfort with these concepts. If any are unfamiliar, the indicated chapters and references fill the gap.

| Concept | Where It Appears | Minimum Needed | Deep Reference |
|---------|-----------------|----------------|----------------|
| Dependent types (Π, Σ, Id) | Everywhere | Chapter 2, Martin-Löf 1984 | HoTT Book Ch. 1 |
| de Bruijn indices | Term representation | Know what they are and why | Barendregt's λ-calculus book |
| Normalization by Evaluation | Every evaluator | Chapter 9 | Abel 2013 habilitation |
| Path types, interval | Cubical layer | Chapter 3 | ABCFHL 2021 |
| Kan operations (coe, hcom) | Cubical layer | Chapter 3 §3.3 | Angiuli, Hou, Harper 2017 |
| Nilsquare infinitesimals | Smooth layer | Chapter 4 | Kock 2006 |
| Kock-Lawvere axiom | Smooth layer | The statement and what it means | Kock 2006 Ch. I |
| Rewrite rules, confluence | Equational engine | Chapter 18 | Cockx, Tabareau, Winterhalter 2021 |
| Lipschitz continuity | Sensitivity layer | The definition | Reed & Pierce 2010 |

**If you are unsure whether your background is sufficient:** Implement a simply-typed lambda calculus with NbE in your target language. If you can do that (it takes 50–100 lines), you have the programming maturity to proceed. The type theory you will learn along the way.

### 2.2 Programming Language Choices

Each approach works in different languages. Here is our recommendation per approach:

| Approach | Recommended | Also Works | Not Recommended |
|----------|-------------|------------|-----------------|
| Ground-up | **Rust**, OCaml, Haskell | Lean 4, Zig | Python, JavaScript |
| Fork cctt | **Haskell** (cctt is Haskell) | — | — |
| Fork cooltt | **OCaml** (cooltt is OCaml) | — | — |
| Rewrite rules | **Rocq** (Rewster), **Agda** | Lean 4 | — |
| Dedukti | **Lambdapi** (standard frontend) | Dedukti directly | — |
| Two-level staging | **Haskell**, OCaml, Rust | — | — |
| Embedded DSL | **Haskell** (best type system), Rust, Scala 3 | Kotlin, TypeScript | C, C++ |
| Shallow embed | **Python** (quickest), Haskell, Julia | Any language | — |

**Why Rust for the ground-up approach:** `Arc<Term>` gives you structural sharing with deterministic resource management. No GC pauses during type checking. Pattern matching on enums maps naturally to type-theoretic case analysis. The type system catches use-after-move bugs that would be silent in OCaml or Haskell. And the performance ceiling is higher than any GC'd language.

**Why Haskell is also excellent:** cctt is Haskell and it is the fastest cubical type checker. GHC's garbage collector handles the enormous term graphs that cubical type checking produces. ADTs are a perfect fit. If you prefer Haskell, use Haskell.

### 2.3 Development Environment Setup

Regardless of approach, you need:

```bash
# Rust toolchain (for Approach 1)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default stable

# Haskell toolchain (for Approaches 2, 5, 6)
curl --proto '=https' --tlsv1.2 -sSf https://get-ghcup.haskell.org | sh

# OCaml toolchain (for cooltt fork)
bash -c "sh <(curl -fsSL https://raw.githubusercontent.com/ocaml/opam/master/shell/install.sh)"
opam init && opam switch create 5.1.0

# Agda (for Approach 3)
cabal install Agda

# Rocq/Coq (for Approach 3 with Rewster)
opam install coq

# Lambdapi (for Approach 4)
opam install lambdapi

# Dedukti (for Approach 4)
opam install dedukti
```

### 2.4 Project Skeleton

For every approach, start with this directory structure:

```
your-sctt/
├── README.md              # What this is, how to build, how to test
├── src/                   # Implementation
├── test/                  # Test suite
│   ├── unit/              # Small targeted tests
│   ├── integration/       # Multi-feature interaction tests
│   └── golden/            # Expected-output tests for type checking
├── examples/              # SCTT programs that should type-check
│   ├── basic/             # Dependent types only
│   ├── cubical/           # Path types, Kan ops
│   ├── smooth/            # Nilsquare, tangent bundles
│   ├── sensitivity/       # Lipschitz bounds
│   └── combined/          # All layers interacting
├── bench/                 # Performance benchmarks
└── docs/                  # Design notes and decisions
    └── DECISIONS.md       # Record every design choice and why
```

**The DECISIONS.md file is critical.** Type theory implementation involves hundreds of small choices (de Bruijn vs. locally nameless? Implicit arguments or not? Unification or elaboration-time inference?). Recording these choices prevents you from revisiting them endlessly and helps collaborators understand the codebase.

---

## 3. Approach 1: Ground-Up Rust Implementation {#approach-1}

This is the most thorough approach and the one this book's v0 kernel follows. You build everything from scratch, gaining complete understanding and control.

### 3.1 Overview and Milestones

```
Week 1-2:  Phase 1 — MLTT kernel (Pi, Sigma, Universe, NbE)
Week 3-5:  Phase 2 — Cartesian cubical layer (I, cof, PathP, coe, hcom, Glue)
Week 6-7:  Phase 3 — Smooth primitives (D, ε²=0 rewrite, tangent)
Week 8:    Phase 4 — Lipschitz sensitivity types
Week 9-10: Integration testing, benchmarking, documentation
```

These estimates assume ~20 hours/week of focused work. Experienced type theory implementors can compress significantly; first-timers should double.

### 3.2 Phase 1: The MLTT Core (Detailed Walkthrough)

This phase produces a complete dependent type checker. Everything later builds on it. Do not cut corners here.

#### Step 1: Define the Syntax

Your term type is the central data structure. Every design decision ripples through the entire codebase.

```rust
use std::sync::Arc;

pub const MAX_UNIVERSE_LEVEL: u8 = 6;
pub const MAX_REDUCTION_DEPTH: usize = 256;

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

**Design choices to record in DECISIONS.md:**

1. **de Bruijn indices, not names.** Names require alpha-equivalence checking everywhere. de Bruijn indices make syntactic equality = alpha-equivalence. The cost is that shifting/lifting is error-prone — but the NbE approach avoids most explicit shifting.

2. **`Arc<Term>` for structural sharing.** Terms in type theory have massive sharing (the same subterm appears in many contexts). `Arc` gives O(1) cloning and pointer equality as a fast path for conversion checking.

3. **Bounded universes (MAX_LEVEL = 6).** Universe polymorphism is a research topic in itself. Bounded levels keep the system simple and are sufficient for all practical SCTT programs. Record this as a revisitable decision.

4. **No annotations on Lambda.** In a bidirectional system, lambdas are checked against a known Pi type — the domain annotation is inherited, not stored. This saves memory and avoids a class of bugs where the stored annotation disagrees with the checked type.

#### Step 2: Define the Value Domain (Semantic Values)

NbE requires a *semantic domain* — the "meaning" of terms, separate from their syntax. The interpret function sends `Term → Value`. Quoting sends `Value → Term`.

```rust
#[derive(Debug, Clone)]
pub enum Value {
    Neutral {
        ty: Arc<Value>,
        neutral: Neutral,
    },
    Universe(u8),
    Pi {
        domain: Arc<Value>,
        codomain: Closure,
    },
    Lambda {
        body: Closure,
    },
    Sigma {
        fst_type: Arc<Value>,
        snd_type: Closure,
    },
    Pair {
        fst: Arc<Value>,
        snd: Arc<Value>,
    },
}

#[derive(Debug, Clone)]
pub struct Neutral {
    pub head: NeutralHead,
    pub spine: Vec<Elim>,
}

#[derive(Debug, Clone)]
pub enum NeutralHead {
    Var(Level),
}

#[derive(Debug, Clone)]
pub enum Elim {
    App(Arc<Value>),
    Fst,
    Snd,
}

/// de Bruijn level: counts from the *bottom* of the context.
/// Levels are stable under context extension (unlike indices).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Level(pub usize);

/// A closure: a term body + its captured environment.
/// Defunctionalized — this is data, not a Rust closure.
#[derive(Debug, Clone)]
pub struct Closure {
    pub env: Env,
    pub body: Arc<Term>,
}

#[derive(Debug, Clone)]
pub struct Env {
    pub values: Vec<Arc<Value>>,
}
```

**Why de Bruijn levels for values, indices for terms?** Terms are read top-down (from the outermost binder inward), so indices counting from the innermost binder are natural. But during interpretation, we extend the environment at the *bottom* — so levels (counting from the bottom) are stable. A variable at level `l` stays at level `l` regardless of how many binders we go under. This eliminates the shifting that plagues index-based evaluators.

The conversion between them is: `index = current_depth - level - 1`.

**Why defunctionalized closures?** Rust closures (`Box<dyn Fn(Value) -> Value>`) are opaque — you cannot inspect them, serialize them, or compare them. Defunctionalized closures (env + syntax) are plain data. You can print them for debugging, serialize them for caching, and structurally compare them as an optimization.

#### Step 3: Implement the Interpreter (Term → Value)

The interpreter is a recursive function `interpret(env, term) -> Value`:

```rust
impl Env {
    pub fn new() -> Self {
        Env { values: Vec::new() }
    }

    pub fn lookup(&self, idx: usize) -> Arc<Value> {
        let len = self.values.len();
        self.values[len - 1 - idx].clone()
    }

    pub fn extend(&self, val: Arc<Value>) -> Self {
        let mut values = self.values.clone();
        values.push(val);
        Env { values }
    }
}

impl Closure {
    pub fn apply(&self, arg: Arc<Value>) -> Value {
        let env = self.env.extend(arg);
        interpret(&env, &self.body)
    }
}

pub fn interpret(env: &Env, term: &Term) -> Value {
    match term {
        Term::Var(idx) => (*env.lookup(*idx)).clone(),

        Term::Universe(level) => Value::Universe(*level),

        Term::Pi { domain, codomain } => Value::Pi {
            domain: Arc::new(interpret(env, domain)),
            codomain: Closure {
                env: env.clone(),
                body: codomain.clone(),
            },
        },

        Term::Lambda { body } => Value::Lambda {
            body: Closure {
                env: env.clone(),
                body: body.clone(),
            },
        },

        Term::App { func, arg } => {
            let func_val = interpret(env, func);
            let arg_val = Arc::new(interpret(env, arg));
            do_apply(func_val, arg_val)
        }

        Term::Sigma { fst_type, snd_type } => Value::Sigma {
            fst_type: Arc::new(interpret(env, fst_type)),
            snd_type: Closure {
                env: env.clone(),
                body: snd_type.clone(),
            },
        },

        Term::Pair { fst, snd } => Value::Pair {
            fst: Arc::new(interpret(env, fst)),
            snd: Arc::new(interpret(env, snd)),
        },

        Term::Fst(pair) => do_fst(interpret(env, pair)),

        Term::Snd(pair) => do_snd(interpret(env, pair)),
    }
}

fn do_apply(func: Value, arg: Arc<Value>) -> Value {
    match func {
        Value::Lambda { body } => body.apply(arg),
        Value::Neutral { ty, neutral } => {
            match &*ty {
                Value::Pi { codomain, .. } => {
                    let result_ty = Arc::new(codomain.apply(arg.clone()));
                    let mut spine = neutral.spine;
                    spine.push(Elim::App(arg));
                    Value::Neutral {
                        ty: result_ty,
                        neutral: Neutral {
                            head: neutral.head,
                            spine,
                        },
                    }
                }
                _ => panic!("applying non-function"),
            }
        }
        _ => panic!("applying non-function value"),
    }
}

fn do_fst(pair: Value) -> Value {
    match pair {
        Value::Pair { fst, .. } => (*fst).clone(),
        Value::Neutral { ty, neutral } => {
            match &*ty {
                Value::Sigma { fst_type, .. } => {
                    let mut spine = neutral.spine;
                    spine.push(Elim::Fst);
                    Value::Neutral {
                        ty: fst_type.clone(),
                        neutral: Neutral {
                            head: neutral.head,
                            spine,
                        },
                    }
                }
                _ => panic!("projecting non-sigma"),
            }
        }
        _ => panic!("fst of non-pair"),
    }
}

fn do_snd(pair: Value) -> Value {
    match pair {
        Value::Pair { snd, .. } => (*snd).clone(),
        Value::Neutral { ty, neutral } => {
            match &*ty {
                Value::Sigma { snd_type, .. } => {
                    let fst_val = Arc::new(do_fst(Value::Neutral {
                        ty: ty.clone(),
                        neutral: neutral.clone(),
                    }));
                    let result_ty = Arc::new(snd_type.apply(fst_val));
                    let mut spine = neutral.spine;
                    spine.push(Elim::Snd);
                    Value::Neutral {
                        ty: result_ty,
                        neutral: Neutral {
                            head: neutral.head,
                            spine,
                        },
                    }
                }
                _ => panic!("projecting non-sigma"),
            }
        }
        _ => panic!("snd of non-pair"),
    }
}
```

**Test this immediately.** Write a test that interprets `(λx. x) (λy. y)` and checks the result is `λy. y`. Write another that interprets `fst (pair a b)` and gets `a`. Do not proceed until the interpreter works for all constructors.

#### Step 4: Implement Quoting (Readback)

Quoting converts a `Value` back to a `Term`, using the current context depth to convert levels to indices:

```rust
pub fn quote(depth: usize, val: &Value) -> Term {
    match val {
        Value::Universe(level) => Term::Universe(*level),

        Value::Pi { domain, codomain } => {
            let var = Arc::new(Value::Neutral {
                ty: domain.clone(),
                neutral: Neutral {
                    head: NeutralHead::Var(Level(depth)),
                    spine: Vec::new(),
                },
            });
            Term::Pi {
                domain: Arc::new(quote(depth, domain)),
                codomain: Arc::new(quote(depth + 1, &codomain.apply(var))),
            }
        }

        Value::Lambda { body } => {
            let var = Arc::new(Value::Neutral {
                ty: Arc::new(Value::Universe(0)),
                neutral: Neutral {
                    head: NeutralHead::Var(Level(depth)),
                    spine: Vec::new(),
                },
            });
            Term::Lambda {
                body: Arc::new(quote(depth + 1, &body.apply(var))),
            }
        }

        Value::Sigma { fst_type, snd_type } => {
            let var = Arc::new(Value::Neutral {
                ty: fst_type.clone(),
                neutral: Neutral {
                    head: NeutralHead::Var(Level(depth)),
                    spine: Vec::new(),
                },
            });
            Term::Sigma {
                fst_type: Arc::new(quote(depth, fst_type)),
                snd_type: Arc::new(quote(depth + 1, &snd_type.apply(var))),
            }
        }

        Value::Pair { fst, snd } => Term::Pair {
            fst: Arc::new(quote(depth, fst)),
            snd: Arc::new(quote(depth, snd)),
        },

        Value::Neutral { neutral, .. } => quote_neutral(depth, neutral),
    }
}

fn quote_neutral(depth: usize, neutral: &Neutral) -> Term {
    match &neutral.head {
        NeutralHead::Var(Level(level)) => {
            let mut result = Term::Var(depth - level - 1);
            for elim in &neutral.spine {
                match elim {
                    Elim::App(arg) => {
                        result = Term::App {
                            func: Arc::new(result),
                            arg: Arc::new(quote(depth, arg)),
                        };
                    }
                    Elim::Fst => {
                        result = Term::Fst(Arc::new(result));
                    }
                    Elim::Snd => {
                        result = Term::Snd(Arc::new(result));
                    }
                }
            }
            result
        }
    }
}
```

**Conversion checking is now trivial:** Two values are convertible if and only if `quote(depth, v1) == quote(depth, v2)`. In practice you write a direct `Value`-to-`Value` comparison that short-circuits without allocating quoted terms, but the semantics is the same.

```rust
pub fn conversion(depth: usize, v1: &Value, v2: &Value) -> bool {
    match (v1, v2) {
        (Value::Universe(a), Value::Universe(b)) => a == b,

        (Value::Pi { domain: d1, codomain: c1 },
         Value::Pi { domain: d2, codomain: c2 }) => {
            if !conversion(depth, d1, d2) { return false; }
            let var = Arc::new(Value::Neutral {
                ty: d1.clone(),
                neutral: Neutral {
                    head: NeutralHead::Var(Level(depth)),
                    spine: Vec::new(),
                },
            });
            conversion(depth + 1, &c1.apply(var.clone()), &c2.apply(var))
        }

        (Value::Lambda { body: b1 }, Value::Lambda { body: b2 }) => {
            let var = Arc::new(Value::Neutral {
                ty: Arc::new(Value::Universe(0)),
                neutral: Neutral {
                    head: NeutralHead::Var(Level(depth)),
                    spine: Vec::new(),
                },
            });
            conversion(depth + 1, &b1.apply(var.clone()), &b2.apply(var))
        }

        (Value::Lambda { body }, other) | (other, Value::Lambda { body }) => {
            let var = Arc::new(Value::Neutral {
                ty: Arc::new(Value::Universe(0)),
                neutral: Neutral {
                    head: NeutralHead::Var(Level(depth)),
                    spine: Vec::new(),
                },
            });
            let lam_result = body.apply(var.clone());
            let other_result = do_apply(other.clone(), var);
            conversion(depth + 1, &lam_result, &other_result)
        }

        (Value::Sigma { fst_type: f1, snd_type: s1 },
         Value::Sigma { fst_type: f2, snd_type: s2 }) => {
            if !conversion(depth, f1, f2) { return false; }
            let var = Arc::new(Value::Neutral {
                ty: f1.clone(),
                neutral: Neutral {
                    head: NeutralHead::Var(Level(depth)),
                    spine: Vec::new(),
                },
            });
            conversion(depth + 1, &s1.apply(var.clone()), &s2.apply(var))
        }

        (Value::Pair { fst: a1, snd: b1 },
         Value::Pair { fst: a2, snd: b2 }) => {
            conversion(depth, a1, a2) && conversion(depth, b1, b2)
        }

        (Value::Neutral { neutral: n1, .. },
         Value::Neutral { neutral: n2, .. }) => {
            quote_neutral(depth, n1) == quote_neutral(depth, n2)
        }

        _ => false,
    }
}
```

#### Step 5: Implement Bidirectional Type Checking

The type checker has two modes:

- **Check:** Given a term and a type, verify that the term has that type.
- **Infer:** Given a term, compute its type.

```rust
pub struct Context {
    pub types: Vec<Arc<Value>>,
    pub depth: usize,
}

impl Context {
    pub fn new() -> Self {
        Context { types: Vec::new(), depth: 0 }
    }

    pub fn extend(&self, ty: Arc<Value>) -> Self {
        let mut types = self.types.clone();
        types.push(ty);
        Context { types, depth: self.depth + 1 }
    }

    pub fn lookup(&self, idx: usize) -> &Arc<Value> {
        &self.types[self.types.len() - 1 - idx]
    }

    pub fn fresh_var(&self, ty: Arc<Value>) -> Value {
        Value::Neutral {
            ty,
            neutral: Neutral {
                head: NeutralHead::Var(Level(self.depth)),
                spine: Vec::new(),
            },
        }
    }
}

pub fn check(
    ctx: &Context, env: &Env, term: &Term, expected: &Value
) -> Result<(), String> {
    match (term, expected) {
        (Term::Lambda { body }, Value::Pi { domain, codomain }) => {
            let var = Arc::new(ctx.fresh_var(domain.clone()));
            let body_type = codomain.apply(var.clone());
            let new_ctx = ctx.extend(domain.clone());
            let new_env = env.extend(var);
            check(&new_ctx, &new_env, body, &body_type)
        }

        (Term::Pair { fst, snd }, Value::Sigma { fst_type, snd_type }) => {
            check(ctx, env, fst, fst_type)?;
            let fst_val = Arc::new(interpret(env, fst));
            let snd_type_val = snd_type.apply(fst_val);
            check(ctx, env, snd, &snd_type_val)
        }

        _ => {
            let inferred = infer(ctx, env, term)?;
            if conversion(ctx.depth, &inferred, expected) {
                Ok(())
            } else {
                Err(format!(
                    "Type mismatch: expected {:?}, got {:?}",
                    quote(ctx.depth, expected),
                    quote(ctx.depth, &inferred)
                ))
            }
        }
    }
}

pub fn infer(
    ctx: &Context, env: &Env, term: &Term
) -> Result<Value, String> {
    match term {
        Term::Var(idx) => Ok((**ctx.lookup(*idx)).clone()),

        Term::Universe(level) => {
            if *level < 6 {
                Ok(Value::Universe(level + 1))
            } else {
                Err("Universe level overflow".to_string())
            }
        }

        Term::Pi { domain, codomain } => {
            let dom_level = check_is_type(ctx, env, domain)?;
            let dom_val = Arc::new(interpret(env, domain));
            let var = Arc::new(ctx.fresh_var(dom_val.clone()));
            let new_ctx = ctx.extend(dom_val);
            let new_env = env.extend(var);
            let cod_level = check_is_type(&new_ctx, &new_env, codomain)?;
            Ok(Value::Universe(dom_level.max(cod_level)))
        }

        Term::App { func, arg } => {
            let func_ty = infer(ctx, env, func)?;
            match func_ty {
                Value::Pi { domain, codomain } => {
                    check(ctx, env, arg, &domain)?;
                    let arg_val = Arc::new(interpret(env, arg));
                    Ok(codomain.apply(arg_val))
                }
                _ => Err("Applying non-function".to_string()),
            }
        }

        Term::Sigma { fst_type, snd_type } => {
            let fst_level = check_is_type(ctx, env, fst_type)?;
            let fst_val = Arc::new(interpret(env, fst_type));
            let var = Arc::new(ctx.fresh_var(fst_val.clone()));
            let new_ctx = ctx.extend(fst_val);
            let new_env = env.extend(var);
            let snd_level = check_is_type(&new_ctx, &new_env, snd_type)?;
            Ok(Value::Universe(fst_level.max(snd_level)))
        }

        Term::Fst(pair) => {
            let pair_ty = infer(ctx, env, pair)?;
            match pair_ty {
                Value::Sigma { fst_type, .. } => Ok((*fst_type).clone()),
                _ => Err("fst of non-sigma".to_string()),
            }
        }

        Term::Snd(pair) => {
            let pair_ty = infer(ctx, env, pair)?;
            match pair_ty {
                Value::Sigma { snd_type, .. } => {
                    let fst_val = Arc::new(do_fst(interpret(env, pair)));
                    Ok(snd_type.apply(fst_val))
                }
                _ => Err("snd of non-sigma".to_string()),
            }
        }

        Term::Lambda { .. } => {
            Err("Cannot infer type of lambda — use a type annotation".into())
        }

        Term::Pair { .. } => {
            Err("Cannot infer type of pair — use a type annotation".into())
        }
    }
}

fn check_is_type(ctx: &Context, env: &Env, term: &Term) -> Result<u8, String> {
    let ty = infer(ctx, env, term)?;
    match ty {
        Value::Universe(level) => Ok(level),
        _ => Err(format!("{:?} is not a type", quote(ctx.depth, &ty))),
    }
}
```

**This is your Phase 1 checkpoint.** At this point you should be able to type-check:

```
-- Identity function
λA. λx. x : Π(A : Type₀). Π(x : A). A

-- Dependent pair
(Type₀, λx. x) : Σ(A : Type₁). (A → A)

-- Church booleans
λA. λt. λf. t : Π(A : Type₀). A → A → A
```

Write tests for all of these. If they pass, Phase 1 is complete.

### 3.3 Phase 2: Cartesian Cubical Layer

This is the largest phase. You add the interval type, cofibrations, path types, and Kan operations.

#### Step 6: Extend the Syntax

Add these constructors to your `Term` and `Value` types:

```rust
pub enum Term {
    // ... all of Phase 1 ...

    // Interval
    I0,                        // left endpoint
    I1,                        // right endpoint
    IVar(usize),               // interval variable (separate namespace)

    // Cofibrations
    CofEq(Arc<Term>, Arc<Term>),
    CofAnd(Arc<Term>, Arc<Term>),
    CofOr(Arc<Term>, Arc<Term>),
    CofTop,
    CofBot,

    // Path types
    PathP {
        type_family: Arc<Term>,     // A : I → Type
        left: Arc<Term>,            // A(0)
        right: Arc<Term>,           // A(1)
    },
    PathLam { body: Arc<Term> },    // binds one interval variable
    PathApp { path: Arc<Term>, arg: Arc<Term> },

    // Kan operations
    Coe {
        type_family: Arc<Term>,     // A : I → Type (binds interval var)
        from: Arc<Term>,
        to: Arc<Term>,
        body: Arc<Term>,
    },
    HCom {
        ty: Arc<Term>,              // A : Type (homogeneous)
        from: Arc<Term>,
        to: Arc<Term>,
        cof: Arc<Term>,
        tube: Arc<Term>,
        cap: Arc<Term>,
    },

    // Glue types (for univalence)
    Glue {
        base: Arc<Term>,
        cof: Arc<Term>,
        types: Arc<Term>,
        equivs: Arc<Term>,
    },
    GlueElem {
        base_elem: Arc<Term>,
        cof: Arc<Term>,
        fiber_elems: Arc<Term>,
    },
    Unglue { elem: Arc<Term> },
}
```

#### Step 7: Implement the Sub/Force Pattern

This is the key performance innovation from cctt. Instead of eagerly substituting interval variables throughout a term, wrap the substitution lazily:

```rust
#[derive(Debug, Clone)]
pub struct Sub<T> {
    pub inner: T,
    pub subst: IntervalSubst,
}

#[derive(Debug, Clone)]
pub struct IntervalSubst {
    pub mappings: Vec<(usize, IntervalExpr)>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum IntervalExpr {
    Var(usize),
    Zero,
    One,
}
```

The `force` function pushes the substitution down exactly one level — it is called only when you need to inspect the head of a value:

```rust
pub fn force(sub_val: Sub<Value>) -> Value {
    if sub_val.subst.mappings.is_empty() {
        return sub_val.inner;
    }

    match sub_val.inner {
        Value::Pi { domain, codomain } => Value::Pi {
            domain: Arc::new(force(Sub {
                inner: (*domain).clone(),
                subst: sub_val.subst.clone(),
            })),
            codomain: apply_subst_closure(codomain, &sub_val.subst),
        },

        Value::Neutral { ty, neutral } => {
            force_neutral(neutral, &sub_val.subst, ty)
        }

        other => other,
    }
}
```

This pattern means that interval substitution is O(1) to create and only costs work proportional to what you actually inspect. In cubical type theory, most terms pass through many substitutions without being inspected — the Sub/Force pattern makes this free.

#### Step 8: Implement Cofibration Solving

Cofibrations are the "boundary conditions" that control when partial elements are defined. You need a solver that decides whether one cofibration implies another:

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Cof {
    Eq(IntervalExpr, IntervalExpr),
    And(Box<Cof>, Box<Cof>),
    Or(Box<Cof>, Box<Cof>),
    Top,
    Bot,
}

impl Cof {
    pub fn implies(&self, other: &Cof) -> bool {
        match (self, other) {
            (_, Cof::Top) => true,
            (Cof::Bot, _) => true,
            (Cof::Top, Cof::Bot) => false,

            (Cof::Eq(a, b), Cof::Eq(c, d)) => {
                (a == c && b == d) || (a == d && b == c)
            }

            (Cof::And(a, b), other) => {
                a.implies(other) || b.implies(other)
            }
            (self_cof, Cof::And(a, b)) => {
                self_cof.implies(a) && self_cof.implies(b)
            }
            (self_cof, Cof::Or(a, b)) => {
                self_cof.implies(a) || self_cof.implies(b)
            }
            (Cof::Or(a, b), other) => {
                a.implies(other) && b.implies(other)
            }
            _ => false,
        }
    }

    pub fn simplify(&self) -> Cof {
        match self {
            Cof::Eq(IntervalExpr::Zero, IntervalExpr::Zero) => Cof::Top,
            Cof::Eq(IntervalExpr::One, IntervalExpr::One) => Cof::Top,
            Cof::Eq(IntervalExpr::Zero, IntervalExpr::One) => Cof::Bot,
            Cof::Eq(IntervalExpr::One, IntervalExpr::Zero) => Cof::Bot,
            Cof::And(a, b) => {
                let a = a.simplify();
                let b = b.simplify();
                match (&a, &b) {
                    (Cof::Bot, _) | (_, Cof::Bot) => Cof::Bot,
                    (Cof::Top, x) | (x, Cof::Top) => x.clone(),
                    _ => Cof::And(Box::new(a), Box::new(b)),
                }
            }
            Cof::Or(a, b) => {
                let a = a.simplify();
                let b = b.simplify();
                match (&a, &b) {
                    (Cof::Top, _) | (_, Cof::Top) => Cof::Top,
                    (Cof::Bot, x) | (x, Cof::Bot) => x.clone(),
                    _ => Cof::Or(Box::new(a), Box::new(b)),
                }
            }
            other => other.clone(),
        }
    }
}
```

#### Step 9: Implement Path Types and Kan Operations

The Kan operations are where cubical type theory gets its computational power. Here is the structure for `coe`:

```rust
fn compute_coe(
    type_family: &Closure,
    from: IntervalExpr,
    to: IntervalExpr,
    body: Value,
) -> Value {
    // Key computation rule: coe^{r→r} is the identity
    if from == to {
        return body;
    }

    let from_type = type_family.apply_interval(from.clone());
    match from_type {
        // coe in Pi: contravariant in domain, covariant in codomain
        Value::Pi { domain, codomain } => {
            Value::Lambda {
                body: Closure::new_coe_pi(
                    type_family, from, to, body, domain, codomain
                ),
            }
        }

        // coe in Sigma: covariant in both, with filling for the second
        Value::Sigma { fst_type, snd_type } => {
            let fst_coe = compute_coe(
                &extract_fst_family(type_family),
                from.clone(), to.clone(),
                do_fst(body.clone()),
            );
            let fst_fill = compute_fill(
                &extract_fst_family(type_family),
                from.clone(), to.clone(),
                do_fst(body.clone()),
            );
            let snd_coe = compute_coe(
                &extract_snd_family(type_family, fst_fill),
                from, to,
                do_snd(body),
            );
            Value::Pair {
                fst: Arc::new(fst_coe),
                snd: Arc::new(snd_coe),
            }
        }

        // coe in a neutral type: stuck
        Value::Neutral { .. } => {
            Value::Neutral {
                ty: Arc::new(type_family.apply_interval(to)),
                neutral: Neutral {
                    head: NeutralHead::Coe {
                        type_family: type_family.clone(),
                        from, to,
                    },
                    spine: vec![Elim::CoeBody(Arc::new(body))],
                },
            }
        }

        _ => todo!("coe for other type formers"),
    }
}
```

This is the most intricate part of the implementation. Each type former has its own `coe` and `hcom` rules, and they must all be mutually consistent.

**Phase 2 checkpoint tests:**

```
-- Path reflexivity
λA. λx. λi. x : Π(A : Type). Π(x : A). Path A x x

-- Path application at endpoints
(λi. x) @ i0 ≡ x
(λi. x) @ i1 ≡ x

-- Transport: coe in a constant family is the identity
coe (λ_. A) r r' x ≡ x    (when r = r')

-- Function extensionality (derivable from coe in Pi)
```

### 3.4 Phase 3: The Smooth Layer

This is where SCTT becomes SCTT. You add nilsquare infinitesimals and the rewrite rule engine.

#### Step 10: Add Smooth Primitives

```rust
pub enum Term {
    // ... Phases 1 and 2 ...
    DType,                     // D = { ε ∈ R | ε² = 0 }
    SmoothReal,                // The smooth real line ℝ
    SmoothFn { domain: Arc<Term>, codomain: Arc<Term> },
    Tangent(Arc<Term>),        // T(M) — tangent bundle
    Deriv { func: Arc<Term> },
}
```

#### Step 11: Implement the Rewrite Rule Engine

This is the critical piece. The `ε² = 0` rule must be applied during reduction, but only in the right scope (following LRTT's local scoping principle):

```rust
#[derive(Debug, Clone)]
pub struct RewriteRule {
    pub pattern: Pattern,
    pub replacement: Arc<Term>,
    pub scope: RuleScope,
}

#[derive(Debug, Clone)]
pub enum RuleScope {
    Global,
    Local(DefinitionId),
}

#[derive(Debug, Clone)]
pub enum Pattern {
    Var(usize),
    App { head: Symbol, args: Vec<Pattern> },
    NonLinear { var: usize, positions: Vec<PatternPosition> },
}

pub fn nilsquare_rule() -> RewriteRule {
    RewriteRule {
        pattern: Pattern::NonLinear {
            var: 0,
            positions: vec![
                PatternPosition::Arg(Symbol::Mul, 0),
                PatternPosition::Arg(Symbol::Mul, 1),
            ],
        },
        replacement: Arc::new(Term::SmoothRealLit(0.0)),
        scope: RuleScope::Local(DefinitionId::SMOOTH_BLOCK),
    }
}

pub fn try_rewrite(
    rules: &[RewriteRule],
    current_scope: &[DefinitionId],
    value: &Value,
) -> Option<Value> {
    for rule in rules {
        match &rule.scope {
            RuleScope::Global => {},
            RuleScope::Local(def_id) => {
                if !current_scope.contains(def_id) {
                    continue;
                }
            }
        }
        if let Some(substitution) = match_pattern(&rule.pattern, value) {
            let result = apply_substitution(&rule.replacement, &substitution);
            return Some(interpret_with_rules(rules, current_scope, &result));
        }
    }
    None
}
```

**The LRTT integration:** Following Chapter 18, the key is that `ε² = 0` is not a global rule. It is locally scoped to definitions that declare they use it:

```rust
pub struct LocalDef {
    pub name: String,
    pub interface: Vec<RewriteRule>,
    pub body: Arc<Term>,
}

fn run_local_def(def: &LocalDef, env: &Env, scope: &[DefinitionId]) -> Value {
    let mut extended_scope = scope.to_vec();
    extended_scope.push(def.name.clone().into());
    interpret_with_rules(&def.interface, &extended_scope, env, &def.body)
}
```

#### Step 12: Implement Differentiation

With `ε² = 0`, differentiation is algebraic extraction of the coefficient:

```rust
/// Compute the derivative of a smooth function.
/// D[f](x) = the 'b' in f(x + ε) = f(x) + b·ε
///
/// Strategy:
/// 1. Run f(x + ε) in a scope where ε² = 0
/// 2. The result has the form a + b·ε (by Kock-Lawvere)
/// 3. Extract b
fn compute_derivative(func: &Value, point: &Value) -> Value {
    let epsilon = fresh_nilsquare_var();
    let shifted_input = smooth_add(point, &epsilon);
    let result = run_in_smooth_scope(func, &shifted_input);
    extract_epsilon_coefficient(&result, &epsilon)
}
```

**Phase 3 checkpoint tests:**

```
-- ε² = 0
let ε : D in mul(ε, ε) = 0

-- Derivative of x²
deriv (λ x. mul(x, x)) = λ x. add(x, x)

-- Kock-Lawvere uniqueness
∀ (f : D → ℝ). ∃! (a b : ℝ). ∀ (ε : D). f(ε) = a + mul(b, ε)
```

### 3.5 Phase 4: Sensitivity Types

The final layer adds Lipschitz bounds.

#### Step 13: Add Sensitivity Type Constructors

```rust
pub enum Term {
    // ... Phases 1-3 ...
    Lip { domain: Arc<Term>, codomain: Arc<Term>, bound: Arc<Term> },
    Bang { sensitivity: Arc<Term>, inner: Arc<Term> },
    SensLit(f64),
    SensInf,
    SensAdd(Arc<Term>, Arc<Term>),
    SensMul(Arc<Term>, Arc<Term>),
    SensMax(Arc<Term>, Arc<Term>),
}
```

#### Step 14: Implement Sensitivity Checking

Sensitivity checking is a lightweight linear-logic-style analysis layered on top of the existing type checker:

```rust
pub struct SensCtx {
    pub sensitivities: Vec<Sensitivity>,
}

#[derive(Debug, Clone)]
pub enum Sensitivity {
    Zero,
    Finite(f64),
    Infinite,
}

impl Sensitivity {
    pub fn add(&self, other: &Sensitivity) -> Sensitivity {
        match (self, other) {
            (Sensitivity::Zero, x) | (x, Sensitivity::Zero) => x.clone(),
            (Sensitivity::Infinite, _) | (_, Sensitivity::Infinite) =>
                Sensitivity::Infinite,
            (Sensitivity::Finite(a), Sensitivity::Finite(b)) =>
                Sensitivity::Finite(a + b),
        }
    }

    pub fn mul(&self, other: &Sensitivity) -> Sensitivity {
        match (self, other) {
            (Sensitivity::Zero, _) | (_, Sensitivity::Zero) => Sensitivity::Zero,
            (Sensitivity::Infinite, _) | (_, Sensitivity::Infinite) =>
                Sensitivity::Infinite,
            (Sensitivity::Finite(a), Sensitivity::Finite(b)) =>
                Sensitivity::Finite(a * b),
        }
    }

    pub fn leq(&self, other: &Sensitivity) -> bool {
        match (self, other) {
            (Sensitivity::Zero, _) => true,
            (_, Sensitivity::Infinite) => true,
            (Sensitivity::Finite(a), Sensitivity::Finite(b)) => a <= b,
            _ => false,
        }
    }
}
```

**Phase 4 checkpoint tests:**

```
-- 1-Lipschitz identity
λx. x : Lip(ℝ, ℝ, 1)

-- 2-Lipschitz doubling
λx. add(x, x) : Lip(ℝ, ℝ, 2)

-- Composition multiplies sensitivities
-- If f : Lip(B, C, k₁) and g : Lip(A, B, k₂)
-- then f ∘ g : Lip(A, C, k₁ · k₂)
```

---

## 4. Approach 2: Forking an Existing Cubical Checker {#approach-2}

Instead of building from scratch, start from an existing cubical type checker and add SCTT's smooth and sensitivity layers.

### 4.1 Option A: Forking cctt (Haskell)

[cctt](https://github.com/AndrasKovacs/cctt) by András Kovács is the fastest cubical type checker in existence. It implements Cartesian cubical type theory with exactly the interval algebra SCTT needs.

**What cctt gives you for free:**
- Complete Cartesian cubical type theory
- Defunctionalized closures and NbE
- The Sub/Force pattern
- Glue types and univalence
- Can compute the Brunerie number (the standard cubical benchmark)

**What you need to add:**
- Smooth primitives (D, ε, tangent bundles)
- The rewrite rule engine for ε² = 0
- Sensitivity types (Lip, !_k)

#### Step-by-step:

1. **Clone and build cctt:**
   ```bash
   git clone https://github.com/AndrasKovacs/cctt
   cd cctt
   cabal build
   cabal test
   ```

2. **Study the architecture.** The key files:
   - `Syntax.hs` — Term representation
   - `Values.hs` — Semantic domain
   - `Eval.hs` — Interpretation and reduction
   - `Conversion.hs` — Conversion checking
   - `Elaboration.hs` — Surface-to-core translation

3. **Add `D` and `SmoothReal` as new type formers.** In `Syntax.hs`, add constructors. In `Values.hs`, add value constructors. In `Eval.hs`, add interpretation cases.

4. **Implement the rewrite engine.** Add a `RewriteRule` type and modify the interpreter to check rules at each `App` node. Start with a single global `mul(ε, ε) = 0` rule and later add LRTT-style local scoping.

5. **Add sensitivity annotations.** These are mostly a type-level addition — extend the type checker to track sensitivities.

**Advantages:**
- You start from a working, tested, high-performance codebase
- The cubical layer (80% of the complexity) is already done
- You can run cctt's existing test suite to catch regressions

**Disadvantages:**
- You must learn Haskell and cctt's architecture
- cctt is research code — documentation is minimal
- Modifying someone else's evaluator is harder than writing your own
- Upstream changes require manual merging

### 4.2 Option B: Forking cooltt (OCaml)

[cooltt](https://github.com/RedPRL/cooltt) is a Cartesian cubical type checker in OCaml, developed by the RedPRL team (Sterling, Angiuli, Favonia, and collaborators).

**Advantages over cctt:**
- Cleaner separation of concerns
- More extensively documented
- Associated with the normalization proof (Sterling & Angiuli, LICS 2021)
- OCaml's module system provides natural architectural boundaries

**Disadvantages vs. cctt:**
- Slower (by roughly 2–5x)
- Less actively maintained
- Smaller community

#### Modification strategy:

The same as cctt: add smooth primitives, rewrite engine, and sensitivity types. cooltt's module system makes this slightly easier — each layer can live in its own module.

```ocaml
(* In a new Smooth module *)
module Smooth = struct
  type d_type = D

  type rewrite_rule = {
    pattern: pattern;
    replacement: term;
    scope: rule_scope;
  }

  let nilsquare_rule = {
    pattern = App (Mul, [Var 0; Var 0]);
    replacement = Lit 0.0;
    scope = Local "smooth_block";
  }

  let try_rewrite rules scope value =
    List.find_map (fun rule ->
      match rule.scope with
      | Local id when not (List.mem id scope) -> None
      | _ -> match_and_apply rule.pattern rule.replacement value
    ) rules
end
```

### 4.3 Option C: Starting from Mini-TT or pi-forall

If cctt and cooltt feel too complex as starting points, begin from a simpler dependent type checker and add cubical structure yourself:

- **[Mini-TT](https://github.com/AndrasKovacs/minitt-rs)** — A tiny (< 1000 lines) dependent type checker in Rust by Kovács. No cubical features, but the NbE architecture is clean.
- **[pi-forall](https://github.com/sweirich/pi-forall)** — A pedagogical dependent type checker in Haskell by Stephanie Weirich. Well-documented, designed for learning.
- **[Elaboration Zoo](https://github.com/AndrasKovacs/elaboration-zoo)** — A series of increasingly complex type checkers by Kovács, from simple to fancy.

The trade-off: you get a simpler, more understandable starting point, but you must implement the cubical layer yourself (which is the hardest part).

---

## 5. Approach 3: Rewrite Rules in an Existing Proof Assistant {#approach-3}

Instead of building your own checker, encode SCTT within an existing proof assistant using its rewrite rule or axiom mechanisms.

### 5.1 Option A: Rocq (Coq) with the Rewster

Rocq (formerly Coq) has experimental support for user-defined rewrite rules via the Rewster (Leray, Gilbert, Tabareau, and Winterhalter, ITP 2024). This is the most mature path for integrating SCTT into an existing system.

#### Step-by-step:

1. **Install Rocq with rewrite rule support:**
   ```bash
   opam pin coq https://github.com/coq/coq.git#rewster
   ```

2. **Define the smooth real line axiomatically:**
   ```coq
   Axiom SmoothR : Type.
   Axiom smooth_zero : SmoothR.
   Axiom smooth_one : SmoothR.
   Axiom smooth_add : SmoothR -> SmoothR -> SmoothR.
   Axiom smooth_mul : SmoothR -> SmoothR -> SmoothR.

   Axiom D : Type.
   Axiom D_embed : D -> SmoothR.
   Coercion D_embed : D >-> SmoothR.
   ```

3. **Add the nilsquare rewrite rule:**
   ```coq
   Rewrite Rule nilsquare :=
     | smooth_mul (D_embed ?e) (D_embed ?e) => smooth_zero.
   ```

4. **Build the Kock-Lawvere axiom:**
   ```coq
   Axiom kock_lawvere :
     forall (f : D -> SmoothR),
     exists! (a b : SmoothR),
       forall (e : D), f e = smooth_add a (smooth_mul b e).

   Definition deriv (f : SmoothR -> SmoothR) (x : SmoothR) : SmoothR :=
     let g := fun (e : D) => f (smooth_add x e) in
     projT1 (projT2 (kock_lawvere g)).
   ```

5. **Combine with Cubical (if available):**
   ```coq
   From HoTT Require Import Basics.
   (* Path types via the HoTT library *)
   (* Transport replaces coe, ap replaces path application *)
   ```

**Advantages:** Massive library, mechanized type preservation checking, strong community.

**Disadvantages:** No native cubical types, performance limited by general-purpose reducer, rewrite rules still experimental.

### 5.2 Option B: Agda with Rewrite Pragmas

Agda has `{-# REWRITE #-}` pragmas that install user-defined computation rules:

```agda
postulate
  ℝ : Set
  _+ℝ_ : ℝ → ℝ → ℝ
  _*ℝ_ : ℝ → ℝ → ℝ
  0ℝ : ℝ

postulate
  D : Set
  embed : D → ℝ

postulate
  nilsquare : (ε : D) → (embed ε) *ℝ (embed ε) ≡ 0ℝ

{-# REWRITE nilsquare #-}
```

**With Cubical Agda, you get path types too:**

```agda
{-# OPTIONS --cubical #-}

open import Cubical.Foundations.Everything

postulate
  ℝ : Type
  D : Type
  embed : D → ℝ
  nilsquare : (ε : D) → embed ε * embed ε ≡ 0ℝ
```

**Warning:** Agda's rewrite pragmas do *not* check confluence. Non-linear rewrites like nilsquare can silently break confluence. Use this for prototyping, not for production proofs.

### 5.3 Option C: Lean 4 with Custom Tactics

Lean 4 does not have rewrite rules in the Rocq/Agda sense, but its powerful metaprogramming system allows encoding SCTT via tactics and custom elaboration:

```lean
class SmoothType (α : Type) where
  tangent : Type
  deriv : (α → α) → (α → tangent)

@[simp] theorem nilsquare (ε : D) : ε * ε = 0 := by
  exact D.nilsquare_axiom ε
```

**Advantages:** Powerful metaprogramming, good performance, growing Mathlib.

**Disadvantages:** No native cubical structure, `@[simp]` is not a computation rule, less type-theoretic purity.

---

## 6. Approach 4: Encoding in Dedukti/Lambdapi {#approach-4}

Dedukti is a logical framework based on the λΠ-calculus modulo rewriting. It is designed precisely for encoding arbitrary type theories via rewrite rules. Lambdapi is its modern frontend.

### 6.1 Why Dedukti?

1. **Rewrite rules are first-class.** Every computation rule in SCTT becomes a Dedukti rewrite rule.
2. **Multiple type theories can coexist.** Encode MLTT, cubical types, and smooth types as separate modules.
3. **Cross-system interoperability.** Proofs from Rocq, Agda, and Lean can be translated to Dedukti.

### 6.2 Step-by-Step Encoding

#### Layer 1: Dependent Types

```lambdapi
constant symbol Type : TYPE;
constant symbol Term : Type → TYPE;

constant symbol Pi [A : Type] : (Term A → Type) → Type;
constant symbol lam [A : Type] [B : Term A → Type] :
  (Π (x : Term A), Term (B x)) → Term (Pi B);
constant symbol app [A : Type] [B : Term A → Type] :
  Term (Pi B) → Π (x : Term A), Term (B x);

rule app (lam &f) &a ↪ &f &a;
```

#### Layer 2: Cubical Structure via 2LTT

Following Barras and Maestracci (LFMTP 2020):

```lambdapi
constant symbol I : TYPE;
constant symbol i0 : I;
constant symbol i1 : I;

// External equality (the 2LTT key insight)
constant symbol Eq_ext [A : Type] : Term A → Term A → TYPE;
constant symbol refl_ext [A : Type] [a : Term A] : Eq_ext a a;
constant symbol cast_ext [A : Type] [B : Type] :
  Eq_ext A B → Term A → Term B;
rule cast_ext refl_ext &x ↪ &x;

// Path types with external boundary conditions
constant symbol PathP :
  (I → Type) → Π (A : Type), Term A → Term A → Type;
constant symbol path_lam [A : I → Type] [a b : _] :
  (Π (i : I), Term (A i)) → Term (PathP A _ a b);
constant symbol path_app [A : I → Type] [a b : _] :
  Term (PathP A _ a b) → Π (i : I), Term (A i);

rule path_app (path_lam &f) &i ↪ &f &i;
```

#### Layer 3: Smooth Structure

```lambdapi
constant symbol SmoothR : Type;
constant symbol sr_mul : Term SmoothR → Term SmoothR → Term SmoothR;
constant symbol sr_zero : Term SmoothR;

constant symbol D_type : Type;
constant symbol d_embed : Term D_type → Term SmoothR;

// THE nilsquare rule
rule sr_mul (d_embed &e) (d_embed &e) ↪ sr_zero;
```

#### Layer 4: Sensitivity Types

```lambdapi
constant symbol Sens : TYPE;
constant symbol sens_zero : Sens;
constant symbol sens_one : Sens;
constant symbol sens_inf : Sens;
constant symbol sens_add : Sens → Sens → Sens;
constant symbol sens_mul : Sens → Sens → Sens;
constant symbol Lip : Type → Type → Sens → Type;

rule sens_add sens_zero &k ↪ &k;
rule sens_mul sens_one &k ↪ &k;
rule sens_mul sens_zero _ ↪ sens_zero;
```

---

## 7. Approach 5: Two-Level Type Theory Staging {#approach-5}

This approach uses two-level type theory (2LTT) to separate SCTT's static (type-checking) and dynamic (runtime) phases. Following Kovács (ICFP 2022/2024):

### 7.1 Architecture

```
┌─────────────────────────────────────────────────────────┐
│  Outer (Static) Level                                    │
│  - Full SCTT with cubical, smooth, sensitivity types     │
│  - Type checking, proof verification                     │
│                                                          │
│  ┌───────────────────────────────────────────────────┐  │
│  │  Inner (Runtime) Level                             │  │
│  │  - Ordinary typed lambda calculus                   │  │
│  │  - No cubical/smooth/sensitivity overhead           │  │
│  │  - Efficient compiled code                          │  │
│  └───────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────┘
```

### 7.2 Key Idea: Staging the Nilsquare Rule

The nilsquare rule is only needed during type checking. At runtime, infinitesimals have been compiled away:

```haskell
-- Outer level: SCTT with nilsquare
outerDerivative :: SmoothFn R R -> R -> R
outerDerivative f x =
  let epsilon = freshNilsquare
      result = f (x + epsilon)
  in extractCoefficient result

-- Inner level: compiled to dual-number AD
innerDerivative :: (Double -> Double) -> Double -> Double
innerDerivative f x = dualPartOf (f (Dual x 1.0))
```

### 7.3 Implementation Outline

```haskell
data Sort = Static | Dynamic

data Term
  = Var Ix
  | Lam Sort Ty Term
  | App Term Term
  | Pi Sort Ty Ty
  | Lift Term      -- runtime code as a static value
  | Splice Term    -- embed static computation into runtime
  | Quote Term     -- create a code value
  -- SCTT constructs (all Static)
  | PathP Term Term Term
  | Coe Term Term Term Term
  | DType
  | Deriv Term
  | Lip Term Term Term
  -- Runtime constructs (all Dynamic)
  | DoubleLit Double
  | DualNum Term Term
```

**The compilation pipeline:**
1. Write SCTT code at the outer level
2. The type checker verifies all properties
3. The staging compiler erases all outer-level constructs
4. The inner level is compiled to efficient native code

**Advantages:** Clear separation of verification and execution, fast runtime.

**Disadvantages:** Complex compiler pipeline, must implement staging correctly.

---

## 8. Approach 6: Embedded DSL in a Host Language {#approach-6}

Build SCTT as a domain-specific language embedded in an existing typed programming language.

### 8.1 Haskell eDSL (Deep Embedding)

```haskell
{-# LANGUAGE GADTs, DataKinds, TypeFamilies #-}

data Ty (l :: Nat) where
  Universe :: Ty (S l)
  PiTy     :: Ty l -> (Value l -> Ty l) -> Ty l
  PathTy   :: Ty l -> Value l -> Value l -> Ty l
  SmoothR  :: Ty Z
  DTy      :: Ty Z
  LipTy    :: Ty l -> Ty l -> Double -> Ty l

data Value (l :: Nat) where
  VLam    :: (Value l -> Value l) -> Value l
  VPath   :: (Interval -> Value l) -> Value l
  VReal   :: Double -> Value Z
  VD      :: Double -> Value Z

-- Nilsquare: ε * ε = 0
mul :: Value Z -> Value Z -> Value Z
mul (VD e1) (VD e2)
  | e1 == e2  = VReal 0.0
mul (VReal x) (VReal y) = VReal (x * y)
mul _ _ = error "type error in mul"
```

### 8.2 Rust eDSL (Procedural Macros)

```rust
// User writes:
sctt! {
    fn derivative(f: Smooth(Real, Real), x: Real) -> Real {
        let epsilon: D = fresh_nilsquare();
        let result = f(x + epsilon);
        extract_coefficient(result)
    }
}

// The macro expands to dual-number AD
```

### 8.3 Python eDSL (Runtime Checking)

```python
from sctt import *

@smooth(domain=R, codomain=R)
def square(x):
    return x * x

@lipschitz(bound=2.0)
def double(x: R) -> R:
    return x + x
```

---

## 9. Approach 7: The Shallow Embedding (Quickest to Demo) {#approach-7}

A shallow embedding represents SCTT concepts directly as host-language types and functions. This is the fastest way to get a working demo.

### 9.1 Python Shallow Embedding

```python
import numpy as np
from dataclasses import dataclass
from typing import Callable

@dataclass(frozen=True)
class Dual:
    """Dual number: represents x + εy where ε² = 0."""
    real: float
    infinitesimal: float

    def __add__(self, other):
        if isinstance(other, Dual):
            return Dual(self.real + other.real,
                       self.infinitesimal + other.infinitesimal)
        return Dual(self.real + other, self.infinitesimal)

    def __mul__(self, other):
        if isinstance(other, Dual):
            # (a + bε)(c + dε) = ac + (ad + bc)ε
            # Note: bdε² = 0 (nilsquare!)
            return Dual(
                self.real * other.real,
                self.real * other.infinitesimal +
                self.infinitesimal * other.real
            )
        return Dual(self.real * other, self.infinitesimal * other)

    def __radd__(self, other): return self.__add__(other)
    def __rmul__(self, other):
        return Dual(other * self.real, other * self.infinitesimal)


def derivative(f: Callable, x: float) -> float:
    """Compute f'(x) using the nilsquare axiom (dual numbers)."""
    result = f(Dual(x, 1.0))
    return result.infinitesimal


@dataclass
class Path:
    """A path in a type: a continuous function from [0,1]."""
    func: Callable[[float], any]
    start: any
    end: any

    def __call__(self, t: float):
        assert 0.0 <= t <= 1.0
        return self.func(t)

    def __matmul__(self, other: 'Path') -> 'Path':
        """Path composition: self @ other"""
        assert self.end == other.start
        def composed(t):
            if t <= 0.5: return self.func(2 * t)
            else: return other.func(2 * t - 1)
        return Path(composed, self.start, other.end)


def lipschitz_check(f, bound: float, test_points: list[float]):
    """Empirically verify that f is k-Lipschitz."""
    for x in test_points:
        for y in test_points:
            if x != y:
                ratio = abs(f(x) - f(y)) / abs(x - y)
                if ratio > bound + 1e-10:
                    raise ValueError(
                        f"f is not {bound}-Lipschitz: "
                        f"|f({x}) - f({y})| / |{x} - {y}| = {ratio}"
                    )
    return True


# === DEMO ===
print(derivative(lambda x: x**2, 3.0))          # → 6.0
print(derivative(np.sin, 0.0))                    # → 1.0

p = Path(lambda t: t, 0.0, 1.0)
q = Path(lambda t: 1.0 + t, 1.0, 2.0)
pq = p @ q
print(pq(0.0), pq(0.5), pq(1.0))               # → 0.0, 1.0, 2.0

lipschitz_check(lambda x: 2*x, bound=2.0,
                test_points=[0, 0.5, 1, 2, -1])  # passes
```

### 9.2 Julia Shallow Embedding

Julia's multiple dispatch makes SCTT encodings particularly clean:

```julia
struct Dual{T<:Real} <: Number
    value::T
    epsilon::T
end

Base.:+(a::Dual, b::Dual) = Dual(a.value + b.value, a.epsilon + b.epsilon)
Base.:*(a::Dual, b::Dual) = Dual(
    a.value * b.value,
    a.value * b.epsilon + a.epsilon * b.value
)
# Note: ε² term dropped — this IS ε² = 0

Base.sin(d::Dual) = Dual(sin(d.value), cos(d.value) * d.epsilon)
Base.exp(d::Dual) = Dual(exp(d.value), exp(d.value) * d.epsilon)

derivative(f, x) = f(Dual(x, one(x))).epsilon
```

---

## 10. Cross-Cutting Concerns {#cross-cutting}

Regardless of which approach you choose, these concerns apply.

### 10.1 Testing Strategy

SCTT testing has three tiers:

**Tier 1: Unit tests** — Test individual components in isolation.

```rust
#[test]
fn test_nilsquare() {
    let eps = Value::D(1.0);
    let result = mul(eps.clone(), eps);
    assert_eq!(result, Value::Real(0.0));
}

#[test]
fn test_path_endpoints() {
    let path = mk_path_lam(|i| match i {
        I0 => Value::Real(0.0),
        I1 => Value::Real(1.0),
        _ => panic!("unexpected interval value"),
    });
    assert_eq!(path_app(&path, I0), Value::Real(0.0));
    assert_eq!(path_app(&path, I1), Value::Real(1.0));
}
```

**Tier 2: Golden tests** — Complete SCTT programs with expected type-checking results.

```
-- test/golden/derivative.sctt
-- Expected: type checks successfully

def square : C∞(ℝ, ℝ) := λ x. mul(x, x)
def dsquare : C∞(ℝ, ℝ) := deriv square

#reduce dsquare(3)
-- Expected output: 6
```

**Tier 3: Property-based tests** — Generate random SCTT terms and verify invariants.

```rust
#[quickcheck]
fn nbe_roundtrip(term: Term) -> bool {
    let env = Env::new();
    let val = interpret(&env, &term);
    let quoted = quote(0, &val);
    let val2 = interpret(&env, &quoted);
    conversion(0, &val, &val2)
}

#[quickcheck]
fn coe_identity(ty: Type, val: Value) -> bool {
    let result = compute_coe(&ty, I0, I0, val.clone());
    conversion(0, &result, &val)
}
```

### 10.2 The Brunerie Number Benchmark

The standard benchmark for cubical type checkers is computing the Brunerie number — the `n` such that π₄(S³) ≅ ℤ/nℤ (the answer is 2).

For SCTT, extend this with:

1. **Brunerie number** — tests the cubical layer
2. **Derivative of x³ at x=2 → 12** — tests the smooth layer
3. **Lipschitz bound of sin → 1.0** — tests the sensitivity layer
4. **Neural ODE convergence** — tests all three layers interacting

### 10.3 Benchmarking

| Metric | Target | How to Measure |
|--------|--------|----------------|
| Type checking time | < 1s for standard programs | `time cargo test` |
| Brunerie number | < 10s | Dedicated benchmark |
| Memory usage | < 1 GB for standard programs | `valgrind --tool=massif` |
| Term size after NbE | No exponential blowup | Count `Arc<Term>` allocations |
| Rewrite rule application | O(1) per rule attempt | Profile `try_rewrite` |

### 10.4 Documentation

Every design decision should be documented contemporaneously:

```markdown
## Decision: Use de Bruijn levels in values, indices in terms

**Date:** 2026-04-06
**Status:** Accepted

**Context:** We need a variable representation for both syntax (Term)
and semantics (Value).

**Options considered:**
1. de Bruijn indices everywhere
2. Locally nameless (names + indices)
3. Indices in syntax, levels in values

**Decision:** Option 3.

**Rationale:** Levels are stable under context extension.
Indices are natural for syntax.

**Consequences:**
- Must convert between levels and indices at the interpret/quote boundary
- The conversion is: index = depth - level - 1
```

---

## 11. Decision Matrix: Choosing Your Approach {#decision-matrix}

### By Goal

| Your Goal | Best Approach | Runner-Up |
|-----------|--------------|-----------|
| Publish a paper on SCTT | 1 (ground-up) or 3 (Rocq) | 4 (Dedukti) |
| Build a production proof assistant | 1 (ground-up) | 2 (fork cctt) |
| Prototype quickly to test ideas | 7 (shallow) or 6 (eDSL) | 3 (Agda) |
| Teach SCTT in a course | 7 (shallow Python) | 6 (eDSL) |
| Verify an existing formalization | 3 (Rocq or Agda) | 4 (Dedukti) |
| Cross-system interoperability | 4 (Dedukti) | 3 (Rocq) |
| Performance research | 1 (ground-up) or 5 (2LTT) | 2 (fork cctt) |
| Certified ML applications | 1 or 5 (2LTT staging) | 6 (Haskell eDSL) |

### By Background

| Your Background | Start With | Graduate To |
|----------------|------------|-------------|
| Systems programmer (Rust/C++) | 1 (ground-up Rust) | — |
| Functional programmer (Haskell/OCaml) | 2 (fork cctt/cooltt) | 1 (ground-up) |
| Proof assistant user (Agda/Coq/Lean) | 3 (rewrite rules) | 4 (Dedukti) |
| ML/data scientist (Python) | 7 (shallow Python) | 6 (eDSL) |
| Type theory researcher | Any | 1 or 4 |
| Student | 7 → 6 → 3 | 1 or 2 |

### By Time Budget

| Time Available | Approach |
|---------------|----------|
| 1 afternoon | 7 (shallow embedding in Python) |
| 1 week | 6 (eDSL) or 3 (Agda with postulates) |
| 1 month | 3 (Rocq with Rewster) or 2 (fork cctt) |
| 3 months | 1 (ground-up through Phase 2) |
| 6+ months | 1 (ground-up complete) or 5 (2LTT staging) |

### Approach Compatibility

These approaches can be combined:

```
  7 (shallow)  →  6 (eDSL)  →  1 (ground-up)
        ↓              ↓
  3 (Agda/Rocq) → 4 (Dedukti) → cross-system verification
        ↓
  5 (2LTT)  →  production staging compiler
```

Start anywhere. Each approach teaches you something the others don't.

---

## 12. Worked Examples {#worked-examples}

### 12.1 Derivative of a Polynomial (All Approaches)

Given `f(x) = x² + 3x + 1`, compute `f'(2) = 7`.

**Approach 1 (Rust):**
```rust
let f = |x: Value| {
    smooth_add(
        smooth_add(smooth_mul(x.clone(), x.clone()), smooth_mul(lit(3.0), x.clone())),
        lit(1.0)
    )
};
let result = compute_derivative(&f, &lit(2.0));
assert_eq!(result, Value::Real(7.0));
```

**Approach 3 (Agda):**
```agda
f : ℝ → ℝ
f x = x *ℝ x +ℝ 3ℝ *ℝ x +ℝ 1ℝ

_ : deriv f 2ℝ ≡ 7ℝ
_ = refl
```

**Approach 7 (Python):**
```python
f = lambda x: x**2 + 3*x + 1
print(derivative(f, 2.0))   # → 7.0
```

### 12.2 Lipschitz-Bounded Neural Network Layer

The full SCTT experience: cubical + smooth + sensitivity.

```sctt
def layer
  (W : Matrix n m ℝ)
  (b : Vec m ℝ)
  (σ : Lip(ℝ, ℝ, 1))
  : Lip(Vec n ℝ, Vec m ℝ, ‖W‖_op)
  := λ x. map σ (matvec W x + b)

def two_layers
  (W₁ : Matrix n m ℝ) (b₁ : Vec m ℝ) (σ₁ : Lip(ℝ, ℝ, 1))
  (W₂ : Matrix m k ℝ) (b₂ : Vec k ℝ) (σ₂ : Lip(ℝ, ℝ, 1))
  : Lip(Vec n ℝ, Vec k ℝ, ‖W₁‖_op · ‖W₂‖_op)
  := comp_lip (layer W₂ b₂ σ₂) (layer W₁ b₁ σ₁)

def robustness_certificate
  (net : Lip(Vec n ℝ, Vec k ℝ, K))
  (x : Vec n ℝ) (δ : ℝ)
  (hδ : margin net x ≥ δ)
  : ∀ (pert : Vec n ℝ), ‖pert‖ < δ/K →
    classify(net(x + pert)) = classify(net(x))
  := by
    intro pert hpert
    exact lipschitz_preserves_class net x pert hpert hδ
```

---

## 13. Troubleshooting and Common Pitfalls {#troubleshooting}

### 13.1 "My reducer loops" (Most Common Bug)

**Symptom:** Type checking hangs on certain terms.

**Cause:** Almost always one of:
1. `coe` at an unknown type doesn't produce a neutral — it tries to structurally distribute and loops.
2. The rewrite engine applies a rule, creates a new redex, applies again...
3. `hcom` at a constant type family doesn't simplify.

**Fix:** Add a fuel/depth counter to the interpreter. When exceeded, return a neutral term, then inspect the trace:

```rust
fn interpret_bounded(env: &Env, term: &Term, depth: usize) -> Result<Value, String> {
    if depth > MAX_REDUCTION_DEPTH {
        return Err(format!("Reduction depth exceeded at {:?}", term));
    }
    // ... pass depth + 1 to recursive calls
}
```

### 13.2 "Conversion checking is too slow"

| Cause | Fix |
|-------|-----|
| No head-normal-form short circuit | Compare heads before recursing into subterms |
| Quoting to compare | Use direct `Value`-to-`Value` comparison |
| No sharing | Use `Arc::ptr_eq` as a fast path |
| Eta expansion everywhere | Only eta-expand at function/pair types |
| Cofibration solver is expensive | Cache implication results |

### 13.3 "The nilsquare rule breaks confluence"

**Fix:** Follow the LRTT discipline (Chapter 18):
1. Scope the nilsquare rule locally.
2. Do not install commutativity/associativity as rewrite rules.
3. Ensure the reducer is call-by-need so that ε² = 0 fires before beta expansion creates problematic terms.

### 13.4 "coe in Glue types is wrong"

**Cause:** `coe` in `Glue` is the hardest case in any cubical implementation. The standard reference is ABCFHL §6.

**Fix:** There are four sub-cases depending on whether the cofibration is trivially true, false, or unknown. Read ABCFHL §6 and implement exactly what it says. There is no shortcut.

### 13.5 "Sensitivity checking rejects valid programs"

**Fix:** Implement sensitivity subtyping: if f is k-Lipschitz, it is also k'-Lipschitz for any k' ≥ k. Also implement common simplifications: `0 + k = k`, `1 · k = k`, `max(k, k) = k`.

### 13.6 "I don't know where to start"

The minimum path:

1. **Day 1:** Implement the Python shallow embedding (§9.1). Run the derivative example.
2. **Week 1:** Work through Phase 1 of Approach 1 (§3.2). You now have a dependent type checker.
3. **Week 2-3:** Add path types and reflexivity (Phase 2 start). Baby cubical.
4. **Week 4:** Add the nilsquare rule (Phase 3). The core of SCTT.
5. **Beyond:** Kan operations, Glue types, sensitivity. Each is self-contained.

---

## 14. What to Build First: The Minimal Viable SCTT {#minimal-viable}

If time is scarce, build only what demonstrates SCTT's unique value.

### The Core Demo (3 Things)

1. **A path between two equal things** — cubical layer works.
2. **A derivative computed via ε² = 0** — smooth layer works.
3. **A Lipschitz bound verified by the type checker** — sensitivity layer works.

If all three work and interact correctly, you have a minimal viable SCTT.

### The Canonical Test Suite

Every SCTT implementation should pass these tests:

```
--- Basic dependent types ---
✓ Identity function: λA. λx. x : Π(A : Type). A → A
✓ Dependent pair: (Nat, 42) : Σ(A : Type). A
✓ Church booleans type-check

--- Cubical layer ---
✓ Reflexivity: λi. x : Path A x x
✓ Path application: (λi. x) @ i0 ≡ x
✓ Transport: coe in constant family is identity
✓ Function extensionality: derivable from coe in Pi

--- Smooth layer ---
✓ Nilsquare: ε * ε = 0 for ε : D
✓ Kock-Lawvere: f(ε) = a + bε for f : D → ℝ
✓ Derivative of x²: deriv(λx. x²) 3 = 6
✓ Chain rule: deriv(f ∘ g) = (deriv f ∘ g) * deriv g

--- Sensitivity layer ---
✓ Identity is 1-Lipschitz
✓ Doubling is 2-Lipschitz
✓ Composition multiplies bounds
✓ Parallel addition adds bounds

--- Cross-layer interactions ---
✓ Path between Lipschitz-equivalent functions
✓ Derivative of a path (smooth + cubical)
✓ Lipschitz bound on a derivative (smooth + sensitivity)
✓ All three layers in one term (the full SCTT experience)
```

### Closing Thoughts

Building a type theory implementation is one of the most intellectually rewarding things a programmer can do. You will understand types, proofs, computation, and mathematics at a level that no amount of reading can provide.

SCTT is ambitious — it unifies three independently complex theories. But the decomposition into layers means you can build it incrementally, testing each layer before adding the next. The approaches in this guide give you many entry points and many paths forward.

Start wherever makes sense for you. Build the simplest thing that works. Test it. Then make it better.

---

*Previous: [Chapter 19: Connecting to the Ecosystem](./chapter_19.md) ←*

*Next: [Appendix A: Mathematical Background](./appendix_a.md) →*
