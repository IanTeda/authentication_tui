//-- ./src/ui.rs

// #![allow(unused)] // For development only

//! Renders the widgets / UI
//!
//! ## References
//!
//! * [Rebels in the Sky](https://github.com/ricott1/rebels-in-the-sky)

mod footer;
mod render;
pub use render::render;

/// Custom tui widget renders
pub mod custom_widgets;

/// Helper functions for positioning the widgets
pub mod helpers;