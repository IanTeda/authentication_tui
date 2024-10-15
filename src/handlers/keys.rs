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
    pub fn handle_event(&self, key_event: crossterm::KeyEvent) -> domain::Action {
        match key_event.code {
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

            // Escape from the tui application
            crossterm::KeyCode::Esc => domain::Action::ClearToast,

            // All other key events have nil action
            _ => domain::Action::Nil,
        }
    }
}
