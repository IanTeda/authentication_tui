//-- ./src/state/mod.rs

//! Module for storing the application temporary state
//! ---

/// App state module
mod app;

/// Toast state module
mod toast;

pub struct State {
    // The application state
    pub app: app::App,

    // Toast messaging state
    pub toast: toast::Toast,
}

impl Default for State {
    /// Default settings used to write to file if config file not found
    fn default() -> Self {
        let app = app::App::default();
        let toast = toast::Toast::default();

        Self {
            app,
            toast,
        }
    }
}