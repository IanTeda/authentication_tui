//-- ./src/app.rs

#![allow(unused)] // For beginning only.

//! Holds the state and application logic
//! ---

use chrono::{DateTime, Utc};

use crate::TuiError;

/// Application result type to keep errors consistent
pub type AppResult<T> = std::result::Result<T, TuiError>;

pub struct Tokens {
    pub access_token: String,
    pub refresh_token: String,
    pub login_on: DateTime<Utc>,
}

#[derive(Debug)]
pub struct Popup {
    pub show: bool,
    pub title: String,
    pub message: String,
}

impl Default for Popup {
    fn default() -> Self {
        Self {
            show: false,
            title: String::from("Popup Title"),
            message: String::from("Something important to tell you"),
        }
    }
}


/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,

    /// counter
    pub counter: u8,

    pub backend_is_online: bool,

    pub popup: Popup,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            counter: 0,
            backend_is_online: false,
            popup: Popup::default(),
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn increment_counter(&mut self) {
        if let Some(res) = self.counter.checked_add(1) {
            self.counter = res;
        }
    }

    pub fn decrement_counter(&mut self) {
        if let Some(res) = self.counter.checked_sub(1) {
            self.counter = res;
        }
    }
}
