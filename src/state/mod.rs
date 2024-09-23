//-- ./src/state/mod.rs

//! Module for storing the application temporary state
//! ---

/// App state module
mod app;
mod backend;
pub use backend::BackendState;

pub struct State {
    /// The TUI application state
    pub app: app::App,
}

impl Default for State {
    /// Default settings used to write to file if config file not found
    fn default() -> Self {
        let app = app::App::default();

        Self { app }
    }
}
