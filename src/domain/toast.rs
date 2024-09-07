//-- ./src/domain/toast.rs

// #![allow(unused)] // For development only

//! Define the toast message domain
//! ---

// TODO: Add builder pattern

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ToastKind {
    Error,
    Info,
    Notification,
    Success,
    Warning,
}

impl Default for ToastKind {
    fn default() -> Self { ToastKind::Notification }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Toast {
    pub(crate) kind: ToastKind,
    pub(crate) message: String,
}