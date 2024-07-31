//-- ./src/app.rs

//! Holds the state and application logic
//! ---

// use std::error;

use std::sync::Arc;

use crate::{Error, Config};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Error>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,

    pub config: Arc<Config>,

    /// counter state
    pub counter: u8,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> AppResult<Self> {
        // Set app as running on new instance
        let running = true;

        let config = Config::parse()?;
        let config = Arc::new(config);

        let counter = 0;

        let app = Self{ 
            running,
            config,
            counter,
        };

        Ok(app)
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
