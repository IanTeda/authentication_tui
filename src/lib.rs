//-- ./src/lib.rs

//! Application library name space
//! ---

mod app;
pub use app::App;

pub mod cli;

mod config;
pub use config::Config;

pub mod error;
pub use error::Error;

/// Event and action handlers
pub mod handlers;

pub mod prelude;

pub mod state;

mod terminal;
pub use terminal::Terminal;

pub mod tracing;