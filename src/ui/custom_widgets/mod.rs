//-- ./src/custom_widgets/mod.rs

//! Bespoke widgets not part of the TUI standard library
//! ---

/// Toast notification widget
mod toast;
pub use toast::Toast;

mod footer_status;
pub use footer_status::FooterStatus;
