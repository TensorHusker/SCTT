//! # Glue Type Stubs
//!
//! Glue types exist syntactically for future univalence support.
//! Only trivial reductions: `Glue A [Bot -> _] = A`.

use std::sync::Arc;
use crate::value::*;

/// Try to reduce a Glue type: if cofibration is Bot, the Glue is just the base type.
pub fn try_reduce_glue_type(base: &Arc<Value>, cof: &CofVal, _fiber: &Arc<Value>) -> Option<Arc<Value>> {
    if cof.is_false() {
        Some(Arc::clone(base))
    } else {
        None
    }
}

/// Try to reduce a Glue element: if cof is Bot, it's just the base element.
pub fn try_reduce_glue_elem(base: &Arc<Value>, cof: &CofVal, _fiber_elem: &Arc<Value>) -> Option<Arc<Value>> {
    if cof.is_false() {
        Some(Arc::clone(base))
    } else {
        None
    }
}
