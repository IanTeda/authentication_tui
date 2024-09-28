//-- ./src/state/mod.rs

//! Module for storing the application temporary state
//! ---

/// App state module
mod app;
mod backend;
mod toast;

pub struct State {
    /// The TUI application state
    pub app: app::AppState,

    /// Toast message vector que
    pub toast: toast::ToastState,

    /// The authentication backend server state
    pub backend: backend::BackendState,
}

impl Default for State {
    /// Default settings used to write to file if config file not found
    fn default() -> Self {
        let app = app::AppState::default();
        let backend = backend::BackendState::default();
        let toast = toast::ToastState::default();

        Self { app, backend, toast }
    }
}
