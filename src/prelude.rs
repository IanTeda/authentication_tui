//-- ./src/prelude.rs

//! Prelude module for including in every module

// Re-export the crate Error.
pub use crate::error::Error;

/// Application result type to keep errors consistent
pub type Result<T> = color_eyre::Result<T, crate::error::Error>;
