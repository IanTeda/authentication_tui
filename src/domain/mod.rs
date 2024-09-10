//-- ./src/domain/mod.rs

//! Application domain types
//! ---

/// Toast message type module
mod toast;
/// Toast message type instance
pub use toast::Toast;
/// Toast message kinds
pub use toast::ToastKind;
