//-- ./src/domain/mod.rs

//! Application domain types
//! ---

mod action;
pub use action::Action;

/// Toast message type module
mod toast;
/// Toast message type instance
pub use toast::Toast;
/// Toast message kinds
pub use toast::ToastKind;
