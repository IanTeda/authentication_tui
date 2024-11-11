//-- ./src/state/app.rs

// #![allow(unused)] // For beginning only.

//! Application state module
//! ---

use crate::domain;

/// The applications current state
#[derive(Debug, Clone, PartialEq)]
pub struct AppState {
    /// Is the application running
    pub is_running: bool,

    /// Application tick rate per second
    pub ticks_per_second: f64,

    /// Application frame rate per second
    pub frames_per_second: f64,

    pub mode: domain::AppMode
}

impl Default for AppState {
    /// Default settings used to write to file if config file not found
    fn default() -> Self {
        Self::new()
    }
}

impl AppState {
    /// Construct a new application state instance
    pub fn new() -> Self {
        // Set the new application run state to true
        let is_running = true;

        // Set the new application tick rate (ticks per second) to 0.0
        let ticks_per_second = 0.0;

        // Set the new application frame rate (frames per second) to 0.0
        let frames_per_second = 0.0;

        let mode = domain::AppMode::default();

        Self {
            is_running,
            ticks_per_second,
            frames_per_second,
            mode,
        }
    }
}
