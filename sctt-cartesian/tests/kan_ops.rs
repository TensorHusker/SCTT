use std::sync::Arc;
use sctt_cartesian::syntax::*;
use sctt_cartesian::dim::*;
use sctt_cartesian::cof::*;
use sctt_cartesian::value::*;
use sctt_cartesian::evaluate::*;
use sctt_cartesian::kan::kan_smoothness;

#[test]
fn coe_trivial_same_endpoints() {
    // coe 0->0 [i.Nat] Zero = Zero
    let t = Term::coe(Dim::Zero, Dim::Zero, Term::Nat, Term::Zero);
    let v = evaluate(&Env::new(), &t);
    assert!(matches!(&*v, Value::Zero));
}

#[test]
fn coe_constant_line() {
    // coe 0->1 [i.Nat] Zero = Zero  (Nat doesn't use i)
    let t = Term::coe(Dim::Zero, Dim::One, Term::Nat, Term::Zero);
    let v = evaluate(&Env::new(), &t);
    assert!(matches!(&*v, Value::Zero));
}

#[test]
fn hcom_trivial_same_endpoints() {
    // hcom 0->0 Nat [] Zero = Zero
    let t = Term::hcom(Dim::Zero, Dim::Zero, Term::Nat, vec![], Term::Zero);
    let v = evaluate(&Env::new(), &t);
    assert!(matches!(&*v, Value::Zero));
}

#[test]
fn hcom_satisfied_boundary() {
    // hcom 0->1 Nat [(Top, j.S(Zero))] Zero = S(Zero)
    let t = Term::hcom(
        Dim::Zero, Dim::One, Term::Nat,
        vec![BdryBranch { cof: Cof::Top, body: Arc::new(Term::succ(Term::Zero)) }],
        Term::Zero,
    );
    let v = evaluate(&Env::new(), &t);
    assert!(matches!(&*v, Value::Succ(_)));
}

#[test]
fn hcom_stuck_no_satisfied_branch() {
    // Wrap in a PathLam to bind a dim variable, then test that hcom stays stuck
    // when the branch cofibration (i=0) is not trivially true.
    // PathLam [i. hcom 0->1 Nat [(i=0, j.S(Zero))] Zero]
    let inner_hcom = Term::hcom(
        Dim::Zero, Dim::One, Term::Nat,
        vec![BdryBranch {
            // DimIndex(0) refers to the PathLam's dim var (i) in the
            // cofibration context (cof is evaluated before the branch body
            // binds its own dim var).
            cof: Cof::eq_dim(Dim::Var(DimIndex(0)), Dim::Zero),
            body: Arc::new(Term::succ(Term::Zero)),
        }],
        Term::Zero,
    );
    let t = Term::path_lam(inner_hcom);
    let v = evaluate(&Env::new(), &t);
    // The result is a PathLam -- apply it at dim 1 to get a stuck hcom
    let applied = do_path_app(v, DimVal::One);
    assert!(matches!(&*applied, Value::HCom(..)));
}

#[test]
fn kan_smoothness_meet() {
    assert_eq!(kan_smoothness(Smoothness::CInfty, Smoothness::C(3)), Smoothness::C(3));
    assert_eq!(kan_smoothness(Smoothness::C(2), Smoothness::C(5)), Smoothness::C(2));
}
