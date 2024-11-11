//-- ./src/domain/action.rs

//! The list of application action types
//! ---

use crate::domain;

/// Application action types
/// ---
#[derive(Debug, Clone, PartialEq, strum::Display)]
pub enum Action {
    AppMode(domain::AppMode),
    /// Ping backend server status.
    BackendStatusUpdate,
    BackendStatus(domain::BackendStatus),
    ClearScreen,
    Error(String),
    Help,
    Nil,
    Paste(String),
    Quit,
    Render,
    Resize(u16, u16),
    Resume,
    Suspend,
    Tick,
    Init,
    ClearToast,
    Toast(domain::Toast),
}