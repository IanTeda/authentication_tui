//-- ./src/domain/mod.rs

//! Application domain types
//! ---
mod action;
pub use action::Action;

mod app_modes;
pub use app_modes::AppMode;

mod backend_status;
pub use backend_status::BackendStatus;

/// Toast message type module
mod toast;
mod event;
pub use event::Event;

/// Toast message type instance
pub use toast::Toast;
/// Toast message kinds
pub use toast::ToastKind;
