use std::sync::Arc;
use sctt_cartesian::dim::*;
use sctt_cartesian::cof::*;
use sctt_cartesian::syntax::*;
use sctt_cartesian::value::*;
use sctt_cartesian::evaluate::*;
use sctt_cartesian::quote::*;
use sctt_cartesian::check::TypeChecker;
use sctt_cartesian::conv::conv;

#[test]
fn dim_substitution_zero() {
    let d = Dim::Var(DimIndex(0));
    let result = d.subst_dim(DimIndex(0), &Dim::Zero);
    assert_eq!(result, Dim::Zero);
}

#[test]
fn dim_substitution_miss() {
    let d = Dim::Var(DimIndex(0));
    let result = d.subst_dim(DimIndex(1), &Dim::One);
    assert_eq!(result, Dim::Var(DimIndex(0)));
}

#[test]
fn dim_is_constant() {
    assert!(Dim::Zero.is_const());
    assert!(Dim::One.is_const());
    assert!(!Dim::Var(DimIndex(0)).is_const());
}

#[test]
fn cof_trivially_true() {
    assert!(Cof::Top.is_true());
    assert!(!Cof::Bot.is_true());
}

#[test]
fn cof_eval_at_endpoint() {
    let cof = Cof::eq_dim(Dim::Var(DimIndex(0)), Dim::Zero);
    let result = cof.subst_dim(DimIndex(0), &Dim::Zero);
    assert!(result.is_true());
}

#[test]
fn cof_eval_at_wrong_endpoint() {
    let cof = Cof::eq_dim(Dim::Var(DimIndex(0)), Dim::Zero);
    let result = cof.subst_dim(DimIndex(0), &Dim::One);
    assert!(result.is_false());
}

#[test]
fn cof_conjunction() {
    let cof = Cof::and(
        Cof::eq_dim(Dim::Var(DimIndex(0)), Dim::Zero),
        Cof::eq_dim(Dim::Var(DimIndex(1)), Dim::One),
    );
    let result = cof.subst_dim(DimIndex(0), &Dim::Zero);
    assert!(!result.is_false());
}

#[test]
fn subst_identity() {
    let t = Term::lambda(Term::var(0));
    let result = t.subst_term(0, &Term::Nat);
    assert_eq!(result, Term::lambda(Term::var(0)));
}

#[test]
fn subst_free_var() {
    let t = Term::var(0);
    let result = t.subst_term(0, &Term::Nat);
    assert_eq!(result, Term::Nat);
}

#[test]
fn subst_dim_in_path_app() {
    let t = Term::path_app(Term::var(0), Dim::Var(DimIndex(0)));
    let result = t.subst_dim(DimIndex(0), &Dim::Zero);
    assert_eq!(result, Term::path_app(Term::var(0), Dim::Zero));
}

// ─── NbE Tests ───────────────────────────────────────────────────────────────

#[test]
fn env_extend_and_lookup_term() {
    let env = Env::new();
    let val = Arc::new(Value::Nat);
    let env2 = env.extend_term(val);
    assert_eq!(env2.term_depth(), 1);
}

#[test]
fn evaluate_identity() {
    let t = Term::app(Term::lambda(Term::var(0)), Term::Nat);
    let v = evaluate(&Env::new(), &t);
    assert!(matches!(&*v, Value::Nat));
}

#[test]
fn evaluate_fst_pair() {
    let t = Term::fst(Term::pair(Term::Zero, Term::succ(Term::Zero)));
    let v = evaluate(&Env::new(), &t);
    assert!(matches!(&*v, Value::Zero));
}

#[test]
fn evaluate_path_lam_app_zero() {
    let t = Term::path_app(Term::path_lam(Term::Nat), Dim::Zero);
    let v = evaluate(&Env::new(), &t);
    assert!(matches!(&*v, Value::Nat));
}

#[test]
fn nbe_roundtrip_nat() {
    let t = Term::Nat;
    let v = evaluate(&Env::new(), &t);
    let t2 = quote(0, 0, &v);
    assert_eq!(t, t2);
}

#[test]
fn nbe_beta_reduction() {
    let t = Term::app(Term::lambda(Term::var(0)), Term::Zero);
    let v = evaluate(&Env::new(), &t);
    let t2 = quote(0, 0, &v);
    assert_eq!(t2, Term::Zero);
}

// ─── Conversion Tests ───────────────────────────────────────────────────────

#[test]
fn conv_nat_self() {
    assert!(conv(0, 0, &Value::Nat, &Value::Nat));
}

#[test]
fn conv_different_constructors() {
    assert!(!conv(0, 0, &Value::Nat, &Value::Zero));
}

#[test]
fn conv_after_beta() {
    let t1 = Term::app(Term::lambda(Term::var(0)), Term::Nat);
    let t2 = Term::Nat;
    let v1 = evaluate(&Env::new(), &t1);
    let v2 = evaluate(&Env::new(), &t2);
    assert!(conv(0, 0, &v1, &v2));
}

// ─── Type Checker Tests ─────────────────────────────────────────────────────

#[test]
fn tc_nat_type() {
    let mut tc = TypeChecker::new();
    let ty = tc.infer(&Term::Nat).unwrap();
    assert!(matches!(&*ty, Value::Universe(l) if l.value() == 0));
}

#[test]
fn tc_zero_is_nat() {
    let mut tc = TypeChecker::new();
    let ty = tc.infer(&Term::Zero).unwrap();
    assert!(matches!(&*ty, Value::Nat));
}

#[test]
fn tc_type_mismatch_rejected() {
    let mut tc = TypeChecker::new();
    // Nat infers to Universe(0), which does not convert with Nat
    let result = tc.check(&Term::Nat, &Value::Nat);
    assert!(result.is_err());
}

#[test]
fn tc_app_identity() {
    let mut tc = TypeChecker::new();
    // (lambda x. x) Zero : Nat
    // The identity is not directly inferable, but App of it is.
    // Pi(Nat, Nat) -> checking lambda x.x against it, then applying.
    let pi_ty = Value::Pi(
        Arc::new(Value::Nat),
        Closure {
            env: Env::new(),
            body: Arc::new(Term::Nat),
        },
    );
    tc.check(&Term::lambda(Term::var(0)), &pi_ty).unwrap();
}

#[test]
fn tc_pair_sigma() {
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
