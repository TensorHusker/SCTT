//! # Kan Operations -- Coercion and Homogeneous Composition
//!
//! For the first draft, only trivial cases reduce:
//! - `coe r->r = id` (already handled in evaluate.rs)
//! - `coe r->s (i. A) t = t` when `A` doesn't depend on `i` (constant line)
//! - `hcom r->r = base` (already handled in evaluate.rs)
//! - `hcom` with a satisfied boundary branch evaluates to that branch
//!
//! Full computation rules for Pi/Sigma/Path are deferred.

use std::sync::Arc;
use crate::syntax::Smoothness;
use crate::value::*;
use crate::evaluate::apply_dim_closure;

/// Try to reduce `coe r->s [i.A] t` beyond the trivial `r==s` case.
/// Returns `Some(result)` if reduction fires, `None` if stuck.
pub fn try_reduce_coe(
    from: &DimVal, to: &DimVal, line_cl: &DimClosure, body: &Arc<Value>,
) -> Option<Arc<Value>> {
    // Constant line: if A doesn't depend on i, coe is identity.
    // Check by evaluating at both endpoints and comparing.
    let at_from = apply_dim_closure(line_cl, from.clone());
    let at_to = apply_dim_closure(line_cl, to.clone());
    if crate::conv::conv(0, 0, &at_from, &at_to) {
        // Line is constant between from and to -- coe is identity
        Some(Arc::clone(body))
    } else {
        None // stuck
    }
}

/// Try to reduce `hcom r->s A [branches] base` beyond the trivial `r==s` case.
/// If any branch's cofibration is satisfied (true), return that branch evaluated at `to`.
pub fn try_reduce_hcom(
    _from: &DimVal, to: &DimVal, _ty: &Arc<Value>,
    bdry: &BdryVal, _base: &Arc<Value>,
) -> Option<Arc<Value>> {
    for (cof, cl) in &bdry.branches {
        if cof.is_true() {
            return Some(apply_dim_closure(cl, to.clone()));
        }
    }
    None // no branch satisfied -- stuck
}

/// Smoothness of a Kan operation: meet of input smoothness and line smoothness.
pub fn kan_smoothness(input: Smoothness, line: Smoothness) -> Smoothness {
    input.meet(line)
}
