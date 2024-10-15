//-- ./src/handlers/keys.rs

// #![allow(unused)] // For beginning only.

//! What to do with key events
//! ---

use crate::{domain, prelude::*};
use crossterm::event as crossterm;
use tokio::sync::mpsc;

pub struct KeyEventHandler {
    /// Action sender
    action_tx: mpsc::UnboundedSender<domain::Action>,
}

impl KeyEventHandler {
    /// Construct a new key event handler
    pub fn new(action_tx: mpsc::UnboundedSender<domain::Action>) -> Self {
        Self { action_tx }
    }

    /// Handles the key events and updates the state of [`App`].
    pub async fn handle_event(&self, key_event: crossterm::KeyEvent) -> Result<()> {
        let action = match key_event.code {
            // Exit application on `q`
            crossterm::KeyCode::Char('q') => domain::Action::Quit,

            // Escape from the tui application
            crossterm::KeyCode::Esc => domain::Action::ClearToast,

            // All other key events have nil action
            _ => domain::Action::Nil,
        };

        // Send action to the que
        self.action_tx.send(action)?;

        Ok(())
    }
}

// fn toast_backend_status(state: &mut state::State) {
//     let toast_message = format!("Backend server is: {:?}", state.backend.status);
//     let toast = domain::Toast::new(toast_message);
//     state.toast.queue.push_back(toast);
// }

// /// Handles the key events and updates the state of [`App`].
// pub fn handle_event(key_event: KeyEvent, state: &mut state::State) -> Result<()> {
//     match key_event.code {
//         // Exit application on `ESC` or `q`
//         KeyCode::Char('q') => {
//             state.app.is_running = false;
//         }
//         // Exit application on `Ctrl-C`
//         KeyCode::Char('c') => {
//             if key_event.modifiers == KeyModifiers::CONTROL {
//                 state.app.is_running = false;
//             }
//         }
//         // KeyCode::Char('t') => {
//         //     let toast_message = domain::Toast::new("Added toast to queue").kind(domain::ToastKind::Error);
//         //     state.toast.queue.push_back(toast_message);
//         // }
//         KeyCode::Char('r') => {
//             // state.backend.status = domain::BackendStatus::Online;
//         }
//         KeyCode::Esc => {
//             state.toast.current = None;
//         }
//         // // Counter handlers
//         // KeyCode::Right => {
//         //     state.increment_counter();
//         // }
//         // KeyCode::Left => {
//         //     state.decrement_counter();
//         // }
//         // Other handlers you could add here.
//         _ => {}
//     }
//     Ok(())
// }
