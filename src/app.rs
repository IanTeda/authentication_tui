//-- ./src/app.rs

// #![allow(unused)] // For beginning only.

//! Holds the state and application logic
//! ---


use crate::{state, Config, TuiError};

/// Application result type to keep errors consistent
pub type AppResult<T> = std::result::Result<T, TuiError>;

/// Application.
#[derive(Debug, Clone, PartialEq)]
pub struct App {
    /// Application running state
    pub running: bool,

    /// Application configuration state
    pub config: Config,

    /// Backend state
    pub backend: state::Backend,

    /// counter
    pub counter: u8,

    /// Popup state
    pub popup: state::Popup,

    /// Toast message state
    pub toast: state::Toast,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(config: Config) -> Self {
        let running = true;
        let backend = state::Backend::default();
        let counter = 0;
        let popup = state::Popup::default();
        let toast = state::Toast::default();

        Self {
            running,
            config,
            backend,
            counter,
            popup,
            toast,
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        self.toast.show = true;
    }

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
