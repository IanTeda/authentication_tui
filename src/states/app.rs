//-- ./src/state/app.rs

#![allow(unused)] // For beginning only.

//! Holds the state and application logic
//! ---

use crate::{states, Config};

/// The modes the TUI app can be in
#[derive(Debug, Clone, PartialEq)]
pub enum AppModes {
    /// View, navigate
    Normal,
    /// Edit an entry
    Edit,
    /// Confirmation popup
    Confirm,
    /// Popup 
    Popup,
    /// Toast notification, might not need this one
    Toast
}

/// Application.
#[derive(Debug, Clone, PartialEq)]
pub struct App {
    /// Application running state
    pub running: bool,

    /// Application is mode
    pub mode: states::AppModes,

    /// Application configuration state
    pub config: Config,

    /// Backend state
    pub backend: states::Backend,

    // /// Popup state
    // pub popup: states::Popup,

    /// Toast message state
    pub toast: states::Toast,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(config: Config) -> Self {
        let running = true;
        let mode = states::AppModes::Normal;
        let backend = states::Backend::default();
        // let popup = states::Popup::default();
        let toast = states::Toast::default();

        Self {
            running,
            mode,
            config,
            backend,
            // popup,
            toast,
        }
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}


#[cfg(test)]
mod tests {
    // #![allow(unused)] // For development only

    // Bring current module into scope
    use super::*;

    // Override with more flexible error
    pub type Result<T> = core::result::Result<T, Error>;
    pub type Error = Box<dyn std::error::Error>;


    #[test]
    fn confirm_default_app() -> Result<()> {
        //-- Setup and Fixtures (Arrange)
        let default_config = Config::default();

        //-- Execute Function (Act)
        let default_app = App::new(default_config);

        //-- Checks (Assertions)
        assert!(default_app.running);
        assert_eq!(default_app.mode, states::AppModes::Normal);

        // -- Return
        Ok(())
    }
}