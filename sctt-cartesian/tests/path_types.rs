use std::sync::Arc;
use sctt_cartesian::conv::conv;
use sctt_cartesian::evaluate::evaluate;
use sctt_cartesian::value::*;
use sctt_cartesian::syntax::*;
use sctt_cartesian::dim::*;
use sctt_cartesian::check::TypeChecker;

// ═══════════════════════════════════════════════════════════════════════════
// Conversion tests
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn conv_nat_eq_nat() {
    assert!(conv(0, 0, &Value::Nat, &Value::Nat));
}

#[test]
fn conv_nat_neq_zero() {
    assert!(!conv(0, 0, &Value::Nat, &Value::Zero));
}

#[test]
fn conv_beta_reduced() {
    // (lambda x. x) Nat  should be convertible with  Nat
    let t1 = Term::app(Term::lambda(Term::var(0)), Term::Nat);
    let t2 = Term::Nat;
    let v1 = evaluate(&Env::new(), &t1);
    let v2 = evaluate(&Env::new(), &t2);
    assert!(conv(0, 0, &v1, &v2));
}

#[test]
fn conv_eta_function() {
    // lambda x. f x  should be convertible with  f
    // Build: f is a free variable (neutral at level 0)
    let f_ty = Arc::new(Value::Pi(
        Arc::new(Value::Nat),
        Closure {
            env: Env::new(),
            body: Arc::new(Term::Nat),
        },
    ));
    let f_val = Value::Neutral(
        Arc::new(Neutral::Var(TermLevel(0))),
        f_ty,
    );

    // lambda x. f x  =  Lambda(Closure that applies Var(0) to fresh arg)
    // In value land, build: Lambda(closure: env=[f], body = App(#1, #0))
    // where #1 refers to f (shifted) and #0 is the bound var
    let eta_expanded = Value::Lambda(Closure {
        env: Env::new().extend_term(Arc::new(f_val.clone())),
        body: Arc::new(Term::app(Term::var(1), Term::var(0))),
    });

    // At term_lvl=1 (f is at level 0), the eta-expanded lambda should
    // be convertible with f itself.
    assert!(conv(1, 0, &eta_expanded, &f_val));
}

#[test]
fn conv_universe_levels() {
    let u0 = Value::Universe(Level::zero());
    let u1 = Value::Universe(Level::new(1).unwrap());
    assert!(conv(0, 0, &u0, &u0));
    assert!(!conv(0, 0, &u0, &u1));
}

#[test]
fn conv_zero_eq_zero() {
    assert!(conv(0, 0, &Value::Zero, &Value::Zero));
}

#[test]
fn conv_succ_eq() {
    let s1 = Value::Succ(Arc::new(Value::Zero));
    let s2 = Value::Succ(Arc::new(Value::Zero));
    assert!(conv(0, 0, &s1, &s2));
}

#[test]
fn conv_succ_neq() {
    let s1 = Value::Succ(Arc::new(Value::Zero));
    let s2 = Value::Succ(Arc::new(Value::Succ(Arc::new(Value::Zero))));
    assert!(!conv(0, 0, &s1, &s2));
}

#[test]
fn conv_pair_eq() {
    let p1 = Value::Pair(Arc::new(Value::Zero), Arc::new(Value::Nat));
    let p2 = Value::Pair(Arc::new(Value::Zero), Arc::new(Value::Nat));
    assert!(conv(0, 0, &p1, &p2));
}

#[test]
fn conv_pair_neq() {
    let p1 = Value::Pair(Arc::new(Value::Zero), Arc::new(Value::Nat));
    let p2 = Value::Pair(Arc::new(Value::Nat), Arc::new(Value::Zero));
    assert!(!conv(0, 0, &p1, &p2));
}

#[test]
fn conv_eta_pair() {
    // A neutral variable compared with Pair(fst(n), snd(n)) should be conv
    // via pair eta: n ≡ (fst n, snd n)
    let ne = Arc::new(Neutral::Var(TermLevel(0)));
    let sigma_ty = Arc::new(Value::Sigma(
        Arc::new(Value::Nat),
        Closure { env: Env::new(), body: Arc::new(Term::Nat) },
    ));
    let n = Value::Neutral(Arc::clone(&ne), Arc::clone(&sigma_ty));
    let expanded = Value::Pair(
        Arc::new(Value::Neutral(
            Arc::new(Neutral::Fst(Arc::clone(&ne), Arc::new(Value::Nat))),
            Arc::new(Value::Nat),
        )),
        Arc::new(Value::Neutral(
            Arc::new(Neutral::Snd(Arc::clone(&ne), Arc::new(Value::Nat))),
            Arc::new(Value::Nat),
        )),
    );
    assert!(conv(1, 0, &n, &expanded));
}

#[test]
fn conv_neutral_var_eq() {
    let v0 = Value::Neutral(
        Arc::new(Neutral::Var(TermLevel(0))),
        Arc::new(Value::Nat),
    );
    let v0_copy = Value::Neutral(
        Arc::new(Neutral::Var(TermLevel(0))),
        Arc::new(Value::Nat),
    );
    assert!(conv(1, 0, &v0, &v0_copy));
}

#[test]
fn conv_neutral_var_neq() {
    let v0 = Value::Neutral(
        Arc::new(Neutral::Var(TermLevel(0))),
        Arc::new(Value::Nat),
    );
    let v1 = Value::Neutral(
        Arc::new(Neutral::Var(TermLevel(1))),
        Arc::new(Value::Nat),
    );
    assert!(!conv(2, 0, &v0, &v1));
}

#[test]
fn conv_path_lam_same_body() {
    // Two path lambdas with the same body (constant Nat)
    let pl1 = Value::PathLam(DimClosure {
        env: Env::new(),
        body: Arc::new(Term::Nat),
    });
    let pl2 = Value::PathLam(DimClosure {
        env: Env::new(),
        body: Arc::new(Term::Nat),
    });
    assert!(conv(0, 0, &pl1, &pl2));
}

#[test]
fn conv_path_lam_diff_body() {
    let pl1 = Value::PathLam(DimClosure {
        env: Env::new(),
        body: Arc::new(Term::Nat),
    });
    let pl2 = Value::PathLam(DimClosure {
        env: Env::new(),
        body: Arc::new(Term::Zero),
    });
    assert!(!conv(0, 0, &pl1, &pl2));
}

// ═══════════════════════════════════════════════════════════════════════════
// Type checker tests
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn check_nat_type() {
    let mut tc = TypeChecker::new();
    let ty = tc.infer(&Term::Nat).unwrap();
    // Nat : Type_0
    assert!(matches!(&*ty, Value::Universe(l) if l.value() == 0));
}

#[test]
fn check_zero_is_nat() {
    let mut tc = TypeChecker::new();
    let ty = tc.infer(&Term::Zero).unwrap();
    assert!(matches!(&*ty, Value::Nat));
}

#[test]
fn check_succ_is_nat() {
    let mut tc = TypeChecker::new();
    let ty = tc.infer(&Term::succ(Term::Zero)).unwrap();
    assert!(matches!(&*ty, Value::Nat));
}

#[test]
fn check_type_mismatch_rejected() {
    let mut tc = TypeChecker::new();
    // Try to check Nat against Zero (Nat is not a Nat-value)
    let result = tc.check(&Term::Nat, &Value::Nat);
    // Nat infers to Universe(0), which does not convert with Nat
    assert!(result.is_err());
}

#[test]
fn check_lambda_identity() {
    let mut tc = TypeChecker::new();
    // lambda x. x  checked against  Nat -> Nat
    let pi_ty = Value::Pi(
        Arc::new(Value::Nat),
        Closure {
            env: Env::new(),
            body: Arc::new(Term::Nat),
        },
    );
    let result = tc.check(&Term::lambda(Term::var(0)), &pi_ty);
    assert!(result.is_ok());
}

#[test]
fn check_lambda_wrong_return() {
    let mut tc = TypeChecker::new();
    // lambda x. x  checked against  Nat -> Type_0
    // The body (x : Nat) does not have type Type_0
    let u0 = Term::universe(0).unwrap();
    let pi_ty = Value::Pi(
        Arc::new(Value::Nat),
        Closure {
            env: Env::new(),
            body: Arc::new(u0),
        },
    );
    let result = tc.check(&Term::lambda(Term::var(0)), &pi_ty);
    assert!(result.is_err());
}

#[test]
fn check_pi_type_formation() {
    let mut tc = TypeChecker::new();
    // Pi(Nat, Nat) : Type_0
    let pi = Term::pi(Term::Nat, Term::Nat);
    let ty = tc.infer(&pi).unwrap();
    assert!(matches!(&*ty, Value::Universe(l) if l.value() == 0));
}

#[test]
fn check_sigma_type_formation() {
    let mut tc = TypeChecker::new();
    // Sigma(Nat, Nat) : Type_0
    let sig = Term::sigma(Term::Nat, Term::Nat);
    let ty = tc.infer(&sig).unwrap();
    assert!(matches!(&*ty, Value::Universe(l) if l.value() == 0));
}

#[test]
fn check_fst_of_sigma_var() {
    let mut tc = TypeChecker::new();
    // Given x : Sigma(Nat, Nat), infer fst(x) : Nat
    let sig_val = Arc::new(Value::Sigma(
        Arc::new(Value::Nat),
        Closure {
            env: Env::new(),
            body: Arc::new(Term::Nat),
        },
    ));
    tc.extend_ctx(sig_val);
    // x is Var(0)
    let fst_term = Term::fst(Term::var(0));
    let fst_ty = tc.infer(&fst_term).unwrap();
    assert!(matches!(&*fst_ty, Value::Nat));
    tc.pop_ctx();
}

#[test]
fn check_pair_against_sigma() {
    let mut tc = TypeChecker::new();
    let sig_ty = Value::Sigma(
        Arc::new(Value::Nat),
        Closure {
            env: Env::new(),
            body: Arc::new(Term::Nat),
        },
    );
    let pair = Term::pair(Term::Zero, Term::succ(Term::Zero));
    tc.check(&pair, &sig_ty).unwrap();
}

#[test]
fn check_universe_hierarchy() {
    let mut tc = TypeChecker::new();
    // Type_0 : Type_1
    let ty = tc.infer(&Term::universe(0).unwrap()).unwrap();
    assert!(matches!(&*ty, Value::Universe(l) if l.value() == 1));
    // Type_1 : Type_2
    let ty = tc.infer(&Term::universe(1).unwrap()).unwrap();
    assert!(matches!(&*ty, Value::Universe(l) if l.value() == 2));
}

#[test]
fn check_universe_overflow() {
    let mut tc = TypeChecker::new();
    // Type_6 cannot produce Type_7 (max is 6)
    let result = tc.infer(&Term::universe(6).unwrap());
    assert!(result.is_err());
}

// ═══════════════════════════════════════════════════════════════════════════
// Path integration tests
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn check_reflexivity_path() {
    let mut tc = TypeChecker::new();
    // refl_Nat(Zero) : Path (i. Nat) Zero Zero
    // Represented as: PathLam { body: Zero } checked against PathType(i. Nat, Zero, Zero)
    let path_ty = Value::PathType(
        DimClosure {
            env: Env::new(),
            body: Arc::new(Term::Nat),
        },
        Arc::new(Value::Zero),
        Arc::new(Value::Zero),
    );
    let path_lam = Term::path_lam(Term::Zero);
    let result = tc.check(&path_lam, &path_ty);
    assert!(result.is_ok(), "reflexivity path should type-check: {:?}", result);
}

#[test]
fn check_path_endpoint_mismatch_rejected() {
    let mut tc = TypeChecker::new();
    // PathLam { body: Zero } against PathType(i. Nat, Zero, Succ(Zero))
    // Left endpoint matches (Zero == Zero), but right doesn't (Zero != Succ(Zero))
    let path_ty = Value::PathType(
        DimClosure {
            env: Env::new(),
            body: Arc::new(Term::Nat),
        },
        Arc::new(Value::Zero),
        Arc::new(Value::Succ(Arc::new(Value::Zero))),
    );
    let path_lam = Term::path_lam(Term::Zero);
    let result = tc.check(&path_lam, &path_ty);
    assert!(result.is_err(), "endpoint mismatch should be rejected");
}

#[test]
fn check_path_type_formation() {
    let mut tc = TypeChecker::new();
    // PathType(i. Nat, Zero, Zero) : Type_0
    let path = Term::path_type(Term::Nat, Term::Zero, Term::Zero);
    let ty = tc.infer(&path).unwrap();
    assert!(matches!(&*ty, Value::Universe(l) if l.value() == 0));
}

#[test]
fn check_path_app_at_zero() {
    let mut tc = TypeChecker::new();
    // First build a constant path  <i> Zero : Path(i. Nat, Zero, Zero)
    // Then apply it at dim 0, which should give type Nat
    let path_ty = Term::path_type(Term::Nat, Term::Zero, Term::Zero);
    let _path_lam = Term::path_lam(Term::Zero);

    // We can infer PathApp type if we annotate.
    // path_app(<i>Zero, 0) : Nat
    // But since PathLam can't be inferred directly, we test via evaluate + conv.
    let env = Env::new();
    let path_app = Term::path_app(Term::path_lam(Term::Zero), Dim::Zero);
    let result = evaluate(&env, &path_app);
    assert!(matches!(&*result, Value::Zero));

    let path_app_one = Term::path_app(Term::path_lam(Term::Zero), Dim::One);
    let result_one = evaluate(&env, &path_app_one);
    assert!(matches!(&*result_one, Value::Zero));

    // Also verify the path type itself is well-formed
    let ty = tc.infer(&path_ty).unwrap();
    assert!(matches!(&*ty, Value::Universe(l) if l.value() == 0));
}

#[test]
fn check_path_left_endpoint_mismatch_rejected() {
    let mut tc = TypeChecker::new();
    // PathLam { body: Succ(Zero) } against PathType(i. Nat, Zero, Succ(Zero))
    // Left endpoint: body at 0 = Succ(Zero), expected Zero -> mismatch
    let path_ty = Value::PathType(
        DimClosure {
            env: Env::new(),
            body: Arc::new(Term::Nat),
        },
        Arc::new(Value::Zero),
        Arc::new(Value::Succ(Arc::new(Value::Zero))),
    );
    let path_lam = Term::path_lam(Term::succ(Term::Zero));
    let result = tc.check(&path_lam, &path_ty);
    assert!(result.is_err(), "left endpoint mismatch should be rejected");
}

#[test]
fn check_constant_path_succ() {
    let mut tc = TypeChecker::new();
    // <i> Succ(Zero) : Path(i. Nat, Succ(Zero), Succ(Zero))
    let one = Value::Succ(Arc::new(Value::Zero));
    let path_ty = Value::PathType(
        DimClosure {
            env: Env::new(),
            body: Arc::new(Term::Nat),
        },
        Arc::new(one.clone()),
        Arc::new(one),
    );
    let path_lam = Term::path_lam(Term::succ(Term::Zero));
    let result = tc.check(&path_lam, &path_ty);
    assert!(result.is_ok(), "constant path at Succ(Zero) should type-check: {:?}", result);
}
