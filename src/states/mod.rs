//-- ./src/state/mod.rs

// #![allow(unused)] // For beginning only.

//! Application states
//! ---

mod app;
pub use app::AppState;
pub use app::AppModes;

/// Application backend state
mod backend;
pub use backend::Backend;

// /// Application popup state
// mod popup;
// pub use popup::Popup;

/// Application toast state
mod toast;
pub use toast::Toast;
pub use toast::ToastKinds;