use sctt_cartesian::dim::*;
use sctt_cartesian::cof::*;
use sctt_cartesian::syntax::Term;

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
