//-- ./src/handlers/action.rs

// #![allow(unused)] // For beginning only.

//! Module for managing the action task channels
//!
//! ---

use crossterm::event as crossterm;
use tokio::sync::mpsc;

use crate::{components, domain, handlers, prelude::*, Terminal};

// TODO: I don't think we need Display derive
// TODO: Move this to a domain
#[derive(Debug, Clone, PartialEq, strum::Display)]
pub enum Action {
    /// Ping backend server status.
    UpdateBackendStatus,
    ClearScreen,
    Error(String),
    Help,
    Nil,
    Paste(String),
    Quit,
    Render,
    Resize(u16, u16),
    Resume,
    Suspend,
    Tick,
    ClearToast,
    Toast(domain::Toast),
}

#[derive(Debug)]
pub struct ActionHandler {
    /// Action sender channel.
    pub sender: mpsc::UnboundedSender<Action>,

    /// Action receiver channel.
    pub receiver: mpsc::UnboundedReceiver<Action>,
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
    pub async fn handle_events(
        &mut self,
        terminal: &mut Terminal,
        #[allow(clippy::ptr_arg)] components: &mut Vec<Box<dyn components::Component>>, //TODO: Needs more research
    ) -> Result<()> {
        // Clone the task sender channel
        let action_sender = self.sender.clone();

        // Get the next event from the terminal events que
        let Some(event) = terminal.events.next().await else {
            return Ok(());
        };

        // Clone event for passing into components
        let component_event = &event.clone();

        // Match the event to an Action
        let action = match event {
            // crate::handlers::event::Event::Closed => todo!(),
            // handlers::Event::Error => Action::Error,
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
        action_sender.send(action)?;

        for component in components.iter_mut() {
            if let Some(action) = component.handle_events(Some(component_event))? {
                action_sender.send(action)?;
            }
        }

        Ok(())
    }

    /// Match a key event to an Action
    pub fn handle_key_event(&mut self, key_event: crossterm::KeyEvent) -> Action {
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
            crossterm::KeyCode::Char('q') => Action::Quit,
            _ => Action::Nil,
        }
    }

    pub fn add_toast(&mut self, toast: domain::Toast) -> Result<()> {
        // Clone the task sender channel
        let action_sender = self.sender.clone();

        // Build the toast action
        let action = Action::Toast(toast);

        // Send action to the que
        action_sender.send(action)?;

        Ok(())
    }

    /// Get the next Action in the que.
    pub async fn next(&mut self) -> Option<Action> {
        self.receiver.recv().await
    }
}
