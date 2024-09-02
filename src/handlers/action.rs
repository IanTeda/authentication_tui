//-- ./src/handlers/action.rs

// #![allow(unused)] // For beginning only.

//! Module for managing the action task channels
//!
//! ---

use crossterm::event::KeyCode;
use tokio::sync::mpsc;

use crate::{handlers, prelude::*, Terminal};

#[derive(
    Debug, Clone, PartialEq, Eq, strum::Display, serde::Serialize, serde::Deserialize,
)]
pub enum Action {
    ClearScreen,
    Error,
    Help,
    Nil,
    Paste(String),
    Quit,
    Render,
    Resize(u16, u16),
    Resume,
    Suspend,
    Tick,
}

#[derive(Debug)]
pub struct ActionHandler {
    /// Action sender channel.
    action_tx: mpsc::UnboundedSender<Action>,

    /// Action receiver channel.
    pub action_rx: mpsc::UnboundedReceiver<Action>,
}

impl Default for ActionHandler {
    /// Default settings used to write to file if config file not found
    fn default() -> Self {
        // Initiate send receive event channels
        let (action_tx, action_rx) = mpsc::unbounded_channel();

        Self {
            action_tx,
            action_rx,
        }
    }
}

impl ActionHandler {
    /// Transform an application event into an Action an then add to the que.
    pub async fn handle_events(&mut self, terminal: &mut Terminal) -> Result<()> {
        // Clone the task sender channel
        let action_tx = self.action_tx.clone();

        // Get the next event from the terminal events que
        let Some(event) = terminal.events.next().await else {
            return Ok(());
        };

        // Match the event to an Action
        let action = match event {
            // crate::handlers::event::Event::Closed => todo!(),
            handlers::Event::Error => Action::Error,
            handlers::Event::FocusGained => Action::Resume,
            handlers::Event::FocusLost => Action::Resume,
            // crate::handlers::event::Event::Init => todo!(),
            handlers::Event::Key(key) => self.handle_key_event(key),
            // crate::handlers::event::Event::Mouse(_) => todo!(),
            handlers::Event::Paste(s) => Action::Paste(s),
            handlers::Event::Quit => Action::Quit,
            handlers::Event::Render => Action::Render,
            handlers::Event::Resize(x, y) => Action::Resize(x, y),
            handlers::Event::Tick => Action::Tick,
            // Every other event to Nil
            _ => Action::Nil,
        };

        // Send action to the que
        action_tx.send(action)?;

        Ok(())
    }

    /// Match a key event to an Action
    pub fn handle_key_event(
        &mut self,
        key_event: crossterm::event::KeyEvent,
    ) -> Action {
        // Match the key event, returning the appropriate action type
        match key_event.code {
            // crossterm::event::KeyCode::Backspace => todo!(),
            // crossterm::event::KeyCode::Enter => todo!(),
            // crossterm::event::KeyCode::Left => todo!(),
            // crossterm::event::KeyCode::Right => todo!(),
            // crossterm::event::KeyCode::Up => todo!(),
            // crossterm::event::KeyCode::Down => todo!(),
            // crossterm::event::KeyCode::Home => todo!(),
            // crossterm::event::KeyCode::End => todo!(),
            // crossterm::event::KeyCode::PageUp => todo!(),
            // crossterm::event::KeyCode::PageDown => todo!(),
            // crossterm::event::KeyCode::Tab => todo!(),
            // crossterm::event::KeyCode::BackTab => todo!(),
            // crossterm::event::KeyCode::Delete => todo!(),
            // crossterm::event::KeyCode::Insert => todo!(),
            // crossterm::event::KeyCode::F(_) => todo!(),
            // crossterm::event::KeyCode::Char(_) => todo!(),
            // crossterm::event::KeyCode::Null => todo!(),
            // crossterm::event::KeyCode::Esc => todo!(),
            // crossterm::event::KeyCode::CapsLock => todo!(),
            // crossterm::event::KeyCode::ScrollLock => todo!(),
            // crossterm::event::KeyCode::NumLock => todo!(),
            // crossterm::event::KeyCode::PrintScreen => todo!(),
            // crossterm::event::KeyCode::Pause => todo!(),
            // crossterm::event::KeyCode::Menu => todo!(),
            // crossterm::event::KeyCode::KeypadBegin => todo!(),
            // crossterm::event::KeyCode::Media(_) => todo!(),
            // crossterm::event::KeyCode::Modifier(_) => todo!(),
            KeyCode::Esc | KeyCode::Char('q') => Action::Quit,
            _ => Action::Nil,
        }
    }

    /// Get the next Action in the que.
    pub async fn next(&mut self) -> Option<Action> {
        self.action_rx.recv().await
    }
}
