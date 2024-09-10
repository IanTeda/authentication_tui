//-- ./src/ui/custom_widgets/mod.rs

//! Custom widgets module.
//!
//! # Reference
//!
//! - [Create custom widgets](https://ratatui.rs/recipes/widgets/custom/)
//! ---

/// Display a toast message
mod toast;
/// Custom widget to display a toast message
pub use toast::ToastWidget;
