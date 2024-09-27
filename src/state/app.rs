//-- ./src/state/app.rs

// #![allow(unused)] // For beginning only.

//! Application state module
//! ---

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AppState {
    pub is_running: bool,
}

impl Default for AppState {
    /// Default settings used to write to file if config file not found
    fn default() -> Self {
        let is_running = true;
        Self { is_running }
    }
}

impl AppState {
    pub fn new () -> Self {
        Self::default()
    }
}