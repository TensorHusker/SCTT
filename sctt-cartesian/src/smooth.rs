//! # Smooth Extensions
//!
//! Smoothness tracking and smooth operation helpers.

use crate::syntax::{Term, Smoothness};

/// Differentiation decreases smoothness order by 1.
/// C^inf stays C^inf, C^n becomes C^(n-1), C^0 (continuous) can't be differentiated.
pub fn diff_order(s: Smoothness) -> Option<Smoothness> {
    match s {
        Smoothness::CInfty => Some(Smoothness::CInfty),
        Smoothness::C(0) => None, // C^0 = continuous, not differentiable
        Smoothness::C(n) => Some(Smoothness::C(n - 1)),
        Smoothness::Continuous => None,
    }
}

/// Integration increases smoothness order by 1.
pub fn integral_order(s: Smoothness) -> Smoothness {
    match s {
        Smoothness::CInfty => Smoothness::CInfty,
        Smoothness::C(n) => Smoothness::C(n + 1),
        Smoothness::Continuous => Smoothness::C(1),
    }
}

/// Elaborate a SmoothPath into a regular PathType.
/// SmoothPath(order, A, a, b) -> PathType([i].A, a, b)
/// The smoothness annotation is erased -- it's tracked separately.
pub fn elaborate_smooth_path(term: &Term) -> Option<Term> {
    match term {
        Term::SmoothPath { order: _, ty, start, end } => {
            Some(Term::path_type(
                (**ty).clone(),  // line: constant type family
                (**start).clone(),
                (**end).clone(),
            ))
        }
        _ => None,
    }
}

/// Check that an operation's smoothness requirement is compatible with input.
pub fn check_smoothness_compatible(
    required: Smoothness, input: Smoothness,
) -> Result<(), String> {
    // required <= input (input must be at least as smooth as required)
    match (required, input) {
        (Smoothness::Continuous, _) => Ok(()),
        (Smoothness::C(r), Smoothness::C(i)) if i >= r => Ok(()),
        (Smoothness::C(_), Smoothness::CInfty) => Ok(()),
        (Smoothness::CInfty, Smoothness::CInfty) => Ok(()),
        _ => Err(format!("smoothness violation: need {}, got {}", required, input)),
    }
}
