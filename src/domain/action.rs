use super::{Toast, BackendStatus};

#[derive(Debug, Clone, PartialEq, strum::Display)]
pub enum Action {
    /// Ping backend server status.
    BackendStatusUpdate,
    BackendStatus(BackendStatus),
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
    ClearToast,
    Toast(Toast),
}