use sctt_cartesian::syntax::*;
use sctt_cartesian::smooth::*;

#[test]
fn smoothness_meet_basic() {
    assert_eq!(Smoothness::CInfty.meet(Smoothness::C(3)), Smoothness::C(3));
}

#[test]
fn diff_decreases_smoothness() {
    assert_eq!(diff_order(Smoothness::CInfty), Some(Smoothness::CInfty));
    assert_eq!(diff_order(Smoothness::C(3)), Some(Smoothness::C(2)));
    assert_eq!(diff_order(Smoothness::C(0)), None);
    assert_eq!(diff_order(Smoothness::Continuous), None);
}

#[test]
fn integral_increases_smoothness() {
    assert_eq!(integral_order(Smoothness::Continuous), Smoothness::C(1));
    assert_eq!(integral_order(Smoothness::C(2)), Smoothness::C(3));
    assert_eq!(integral_order(Smoothness::CInfty), Smoothness::CInfty);
}

#[test]
fn smooth_path_elaboration() {
    let sp = Term::smooth_path(Smoothness::CInfty, Term::Nat, Term::Zero, Term::succ(Term::Zero));
    let elaborated = elaborate_smooth_path(&sp).unwrap();
    assert!(matches!(elaborated, Term::PathType { .. }));
}

#[test]
fn smooth_path_elaboration_non_smooth_returns_none() {
    assert!(elaborate_smooth_path(&Term::Nat).is_none());
}

#[test]
fn smoothness_compatible_ok() {
    assert!(check_smoothness_compatible(Smoothness::C(2), Smoothness::CInfty).is_ok());
    assert!(check_smoothness_compatible(Smoothness::C(2), Smoothness::C(3)).is_ok());
    assert!(check_smoothness_compatible(Smoothness::Continuous, Smoothness::C(0)).is_ok());
}

#[test]
fn smoothness_incompatible_err() {
    assert!(check_smoothness_compatible(Smoothness::C(3), Smoothness::C(2)).is_err());
    assert!(check_smoothness_compatible(Smoothness::CInfty, Smoothness::C(5)).is_err());
}
