//-- ./src/handlers/action.rs

// #![allow(unused)] // For beginning only.

//! Module for managing the action task channels
//!
//! ---

use tokio::sync::mpsc;

use crate::{domain, handlers, prelude::*};

// #[derive(Debug)]
pub struct ActionHandler {
    /// Action sender channel.
    pub action_sender: mpsc::UnboundedSender<domain::Action>,

    /// Action receiver channel.
    pub action_receiver: mpsc::UnboundedReceiver<domain::Action>,

    /// Map key events to actions.
    keys: crate::handlers::KeyEventHandler,
}

impl Default for ActionHandler {
    /// Default settings used to write to file if config file not found
    fn default() -> Self {
        // Initiate send receive event channels
        let (sender, receiver) = mpsc::unbounded_channel();

        let keys = handlers::KeyEventHandler::default();

        Self {
            action_sender: sender,
            action_receiver: receiver,
            keys,
        }
    }
}

impl ActionHandler {
    /// Transform an application (terminal) event into an Action an then add to the que.
    pub async fn handle_events(
        &mut self,
        app_mode: domain::AppMode,
        terminal_events: &mut handlers::CrosstermEventsHandler,
    ) -> Result<()> {
        // Clone the task sender channel
        let action_sender = self.action_sender.clone();

        // Get the next event from the terminal events que
        let Some(event) = terminal_events.next_event().await else {
            return Ok(());
        };

        // Match the event to an Action
        let action = match event {
            // crate::handlers::event::Event::Closed => todo!(),
            // handlers::Event::Error => Action::Error,
            domain::Event::Init => domain::Action::Init,
            domain::Event::FocusGained => domain::Action::Resume,
            domain::Event::FocusLost => domain::Action::Resume,
            domain::Event::Key(key) => self.keys.handle_event(app_mode, key),
            // crate::handlers::event::Event::Mouse(_) => todo!(),
            domain::Event::Paste(s) => domain::Action::Paste(s),
            domain::Event::Quit => domain::Action::Quit,
            domain::Event::Render => domain::Action::Render,
            domain::Event::Resize(x, y) => domain::Action::Resize(x, y),
            domain::Event::Tick => domain::Action::Tick,
            // Every other event to Nil
            _ => domain::Action::Nil,
        };

        // Send action to the que
        action_sender.send(action)?;

        Ok(())
    }

    // pub fn add_toast(&mut self, toast: domain::Toast) -> Result<()> {
    //     // Clone the task sender channel
    //     let action_sender = self.action_sender.clone();

    //     // Build the toast action
    //     let action = domain::Action::Toast(toast);

    //     // Send action to the que
    //     action_sender.send(action)?;

    //     Ok(())
    // }

    /// Get the next Action in the que.
    // TODO: Can I be an option return?
    pub fn next_action(&mut self) -> Result<domain::Action> {
        // I need to be try_recv(), as the pool might be empty
        let action = self.action_receiver.try_recv()?;
        Ok(action)
    }
}
