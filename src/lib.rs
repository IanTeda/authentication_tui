//-- ./src/lib.rs

//! Application library name space
//! ---

/// TUI Application instance
mod app;
pub use app::App;

/// Command line argument parser
pub mod cli;

/// RPC client module
pub mod client;

/// Configuration file parser
mod config;
pub use config::Config;

/// Domain type definitions
pub mod domain;

/// UI components and widgets
pub mod ui;

/// TUI Application error types
pub mod error;
pub use error::Error;

/// Event and action handlers
pub mod handlers;

/// Re-exports of key modules to include in each module
pub mod prelude;

/// The application state
pub mod state;

/// The Crossterm backend terminal module
mod terminal;
pub use terminal::Terminal;

/// Tracing (logging) module
pub mod tracing;
