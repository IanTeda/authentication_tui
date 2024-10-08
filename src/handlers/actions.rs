//-- ./src/handlers/action.rs

// #![allow(unused)] // For beginning only.

//! Module for managing the action task channels
//! ---

use crossterm::event as crossterm;
use tokio::sync::mpsc;

use crate::{domain, prelude::*, Terminal};

#[derive(Debug)]
pub struct ActionHandler {
    /// Action sender channel.
    pub sender: mpsc::UnboundedSender<domain::Action>,

    /// Action receiver channel.
    pub receiver: mpsc::UnboundedReceiver<domain::Action>,
}

impl Default for ActionHandler {
    /// Default settings used to write to file if config file not found
    fn default() -> Self {
        // Initiate send receive event channels
        let (sender, receiver) = mpsc::unbounded_channel();

        Self { sender, receiver }
    }
}

impl ActionHandler {
    /// Transform an application event into an Action an then add to the que.
    pub async fn handle_events(&mut self, terminal: &mut Terminal) -> Result<()> {
        let Some(event) = terminal.events.next().await else {
            return Ok(());
        };

        // Clone the task sender channel
        let action_sender = self.sender.clone();

        // Match the event to an Action
        let action = match event {
            // crate::handlers::event::Event::Closed => todo!(),
            domain::Event::Error => domain::Action::Error("Error".to_string()),
            domain::Event::FocusGained => domain::Action::Resume,
            domain::Event::FocusLost => domain::Action::Resume,
            domain::Event::Init => domain::Action::Init,
            domain::Event::Key(key) => self.handle_key_event(key),
            // crate::handlers::event::Event::Mouse(_) => todo!(),
            domain::Event::Paste(s) => domain::Action::Paste(s),
            domain::Event::Quit => domain::Action::Quit,
            domain::Event::Render => domain::Action::Render,
            domain::Event::Resize(x, y) => domain::Action::Resize(x, y),
            domain::Event::Tick => domain::Action::Tick,
            // Map every other event to Nil
            _ => domain::Action::Nil,
        };

        // // Send action to the que
        action_sender.send(action)?;

        Ok(())
    }

    /// Match a key event to an Action
    pub fn handle_key_event(
        &mut self,
        key_event: crossterm::KeyEvent,
    ) -> domain::Action {
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
            crossterm::KeyCode::Char('q') => domain::Action::Quit,
            _ => domain::Action::Nil,
        }
    }

    pub fn add_toast(&mut self, toast: domain::Toast) -> Result<()> {
        // Clone the task sender channel
        let action_sender = self.sender.clone();

        // Build the toast action
        let action = domain::Action::Toast(toast);

        // Send action to the que
        action_sender.send(action)?;

        Ok(())
    }

    /// Get the next Action in the que.
    pub async fn next(&mut self) -> Option<domain::Action> {
        self.receiver.recv().await
    }
}
