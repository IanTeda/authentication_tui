//-- ./src/handlers/keys.rs

// #![allow(unused)] // For beginning only.

//! What to do with key events
//! ---

use crate::prelude::*;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crate::state;

/// Handles the key events and updates the state of [`App`].
pub fn handle_event(key_event: KeyEvent, state: &mut state::State) -> Result<()> {
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Esc | KeyCode::Char('q') => {
            state.app.is_running = false;
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                state.app.is_running = false;
            }
        }
        // // Counter handlers
        // KeyCode::Right => {
        //     state.increment_counter();
        // }
        // KeyCode::Left => {
        //     state.decrement_counter();
        // }
        // Other handlers you could add here.
        _ => {}
    }
    Ok(())
}