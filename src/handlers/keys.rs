//-- ./src/handlers/keys.rs

// #![allow(unused)] // For beginning only.

//! What to do with key events
//! ---

use crate::domain;
use crossterm::event as crossterm;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct KeyEventHandler {}

impl KeyEventHandler {
    /// Handles the key events and updates the state of [`App`].
    pub fn handle_event(&self, app_mode: domain::AppMode, key_event: crossterm::KeyEvent) -> domain::Action {
        match app_mode {
            domain::AppMode::Normal => match key_event.code {
                // Exit application on `q`
                crossterm::KeyCode::Char('q') => domain::Action::Quit,

                // // Test toast message with 't'
                // crossterm::KeyCode::Char('t') => {
                //     let toast_message = "I am toasty".to_string();
                //     let toast = domain::Toast::new(toast_message);
                //     domain::Action::Toast(toast)
                // },

                // Update backend server status
                crossterm::KeyCode::Char('u') => domain::Action::BackendStatusUpdate,

                // Update backend server status
                crossterm::KeyCode::Char('m') => domain::Action::AppMode(domain::AppMode::Input),

                // Escape from the tui application
                crossterm::KeyCode::Esc => domain::Action::ClearToast,

                // All other key events have nil action
                _ => domain::Action::Nil,
            },
            domain::AppMode::Input => match key_event.code  {
                // let toasty = domain::Toast::new("Edit mode");
                // domain::Action::Toast(toasty)

                // Escape from the tui application
                crossterm::KeyCode::Esc => domain::Action::AppMode(domain::AppMode::Normal),

                // Add to input
                _ => domain::Action::Nil,

            },
        }
    }
}
