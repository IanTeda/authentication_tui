//-- ./src/ui/custom_widgets/mod.rs

// #![allow(unused)] // For beginning only.

//! Custom widgets module.
//!
//! # Reference
//!
//! - [Create custom widgets](https://ratatui.rs/recipes/widgets/custom/)
//! ---

mod statistics;
pub use statistics::StatisticsWidget;

/// Display a toast message
mod toast;
/// Custom widget to display a toast message
pub use toast::ToastWidget;

mod status;
pub use status::StatusWidget;
