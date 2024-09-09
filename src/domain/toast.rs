//-- ./src/domain/toast.rs

// #![allow(unused)] // For development only

//! Define the toast message domain
//! ---

use std::time;

/// Enum list of Toast message kinds
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ToastKind {
    Error,
    Info,
    Notification,
    Success,
    Warning,
}

/// Toast message type structure
#[derive(Debug, Clone, PartialEq)]
pub struct Toast {
    /// Define what type of toast message this is. I will change the color.
    pub(crate) kind: ToastKind,

    /// Toast message to be shown
    pub(crate) message: String,

    /// Set the instant in time the toast message was shown
    pub(crate) shown_at: time::Instant
}

impl Toast {
    /// Create a new toast instance
    pub fn new(message: String) -> Self {
        let kind = ToastKind::Notification;
        let shown_at = time::Instant::now();

        Self {
            kind,
            message,
            shown_at,
        }
    }

    /// Set the kind of toast message
    pub fn kind(mut self, kind: ToastKind) -> Self {
        self.kind = kind;
        self
    }

    /// Set the time shown to now
    pub fn shown_now(mut self) -> Self {
        self.shown_at = time::Instant::now();
        self
    }
}

//TODO: Write some tests