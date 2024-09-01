//-- ./src/handlers/action.rs

#![allow(unused)] // For beginning only.

//! Module for managing the action task channels
//! 
//! ---

use crossterm::event::KeyCode;
use tokio::sync::mpsc;

use crate::prelude::*;
use crate::Terminal;
use crate::handlers;
use crate::handlers::Event;

#[derive(Debug, Clone, PartialEq, Eq, strum::Display, serde::Serialize, serde::Deserialize)]
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

    /// Action handler thread.
    task: tokio::task::JoinHandle<()>,
}

impl Default for ActionHandler {
    /// Default settings used to write to file if config file not found
    fn default() -> Self {
        // Initiate send receive event channels
        let (action_tx, action_rx) = mpsc::unbounded_channel();

        // Construct the task handle for the channel
        let task = tokio::spawn(async move {});

        Self{
            action_tx,
            action_rx,
            task,
        }
    }
}

impl ActionHandler {
    pub async fn handle_events(&mut self, terminal: &mut Terminal) -> Result<()> {
        // Clone the task sender channel
        let action_tx = self.action_tx.clone();

        // Get the next event from the terminal events que
        let Some(event) = terminal.events.next().await else {
            return Ok(());
        };

        let action = match event {
            // crate::handlers::event::Event::Closed => todo!(),
            Event::Error => Action::Error,
            Event::FocusGained => Action::Resume,
            Event::FocusLost => Action::Resume,
            // crate::handlers::event::Event::Init => todo!(),
            Event::Key(key) => self.handle_key_event(key),
            // crate::handlers::event::Event::Mouse(_) => todo!(),
            Event::Paste(s) => Action::Paste(s),
            Event::Quit => Action::Quit,
            Event::Render => Action::Render,
            Event::Resize(x, y) => Action::Resize(x, y),
            Event::Tick => Action::Tick,
            _ => Action::Nil,
        };

        // Send action to the que
        action_tx.send(action)?;

        Ok(())
    }

    pub fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) -> Action {
        // Clone the task sender channel
        let action_tx = self.action_tx.clone();

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
    
    pub async fn next(&mut self) -> Option<Action> {
        self.action_rx.recv().await
    }


}