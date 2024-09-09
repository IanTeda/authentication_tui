//-- ./src/app/toast.rs

// #![allow(unused)] // For beginning only.

//! Application model for toast message state
//! ---

use std::time::Instant;

#[derive(Debug, Clone, PartialEq)]
pub enum ToastKinds {
    Error,
    Info,
    Notification,
    Success,
    Warning,
}

/// Toast model state
#[derive(Debug, Clone, PartialEq)]
pub struct Toast {
    /// Type of toast
    pub kind: ToastKinds,

    /// When true UI will draw next refresh
    pub show: bool,

    /// Message of the popup
    pub message: String,

    /// Auto hide the toast message
    pub auto_hide: bool,

    /// When the toast notification was shown
    pub displayed_on: Instant,
}

impl Default for Toast {
    fn default() -> Self {
        let kind = ToastKinds::Notification;
        let show = false;
        let message = String::from("Something just happened that goes over one line to two..");
        let auto_hide = true;
        let displayed_on = Instant::now();

        Self {
            kind,
            show,
            message,
            auto_hide,
            displayed_on,
        }
    }
}

#[cfg(test)]
mod tests {
    // Bring current module into scope
    use super::*;

    // Override with more flexible error
    pub type Result<T> = core::result::Result<T, Error>;
    pub type Error = Box<dyn std::error::Error>;


    #[test]
    fn confirm_default_toast() -> Result<()> {
        //-- Setup and Fixtures (Arrange)

        //-- Execute Function (Act)
        let default_toast = Toast::default();

        //-- Checks (Assertions)
        assert_eq!(default_toast.kind, ToastKinds::Notification);
        assert!(!default_toast.show);
        assert_eq!(default_toast.message, String::from("Something just happened that goes over one line to two.."));
        assert!(default_toast.auto_hide);

        // -- Return
        Ok(())
    }
}