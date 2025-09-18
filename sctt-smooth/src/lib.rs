//! # Extended Smooth Operations for SCTT
//!
//! This crate extends the core smooth operations with additional
//! mathematical structures and computational tools.

pub use sctt_core::smooth::*;

/// Extended smooth operations
pub mod extended {
    //! Extended smooth functionality
}

/// GPU-accelerated smooth computations
#[cfg(feature = "gpu-acceleration")]
pub mod gpu {
    //! GPU acceleration for smooth operations
}