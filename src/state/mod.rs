//-- ./src/state/mod.rs

//! Module for storing the application temporary state
//! ---

/// App state module
mod app;

pub struct State {
    // The application state
    pub app: app::App,
}

impl Default for State {
    /// Default settings used to write to file if config file not found
    fn default() -> Self {
        let app = app::App::default();

        Self { app }
    }
}
