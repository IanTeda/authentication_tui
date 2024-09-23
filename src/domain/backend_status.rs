//-- ./src/domain/backend_status.rs

// #![allow(unused)] // For beginning only.

//! The statuses the backend authentication server can be in
//! ---

use core::fmt;

/// Authentication backend server status
#[derive(Debug, Clone, PartialEq)]
pub enum BackendStatus {
    /// Backend server can not be reached, so must be off line
    Offline,

    /// Backend server can be reached, but we are not logged in
    Online,

    /// Backend server can be reached and we are logged in
    LoggedIn,
}

/// Imply default backend status
impl Default for BackendStatus {
    fn default() -> Self { BackendStatus::Offline }
}

impl fmt::Display for BackendStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match self {
           BackendStatus::Offline => write!(f, "Offline"),
           BackendStatus::Online => write!(f, "Online"),
           BackendStatus::LoggedIn => write!(f, "Logged In"),
       }
    }
}