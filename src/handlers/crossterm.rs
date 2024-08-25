//-- ./src/handlers/crossterm.rs

#![allow(unused)] // For beginning only.

//! Module for mapping crossterm terminal backend events
//! ---

use std::{sync::mpsc, thread};

use crossterm::event::{KeyEvent, MouseEvent};

/// Terminal events.
#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    Closed,
    Error,
    FocusGained,
    FocusLost,
    Init,
    Key(KeyEvent),
    Mouse(MouseEvent),
    Paste(String),
    Quit,
    Render,
    Resize(u16, u16),
    Tick,
}

pub struct CrosstermEventLoopHandler {
    /// Event sender channel.
    sender: mpsc::Sender<Event>,

    /// Event receiver channel.
    receiver: mpsc::Receiver<Event>,
    
    /// Event handler thread.
    handler: thread::JoinHandle<()>,
}

// impl CrosstermEventLoopHandler {
//     pub fn new(tick_rate: f64, frame_rate: f64) -> Self {
//         let (sender, receiver) = mpsc::unbounded_channel();


//     }
// }