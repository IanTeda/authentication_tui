//-- ./src/lib.rs

/// Application.
pub mod app;

/// Configuration file handler
mod config;
pub use config::Config;

/// Terminal events handler.
pub mod event;

/// Widget renderer.
pub mod ui;

/// Terminal user interface.
pub mod tui;

/// Event handler.
pub mod handler;

/// Error handler
mod error;
pub use error::Error;

/// Tonic Client RPC files
mod rpc;
pub use rpc::proto;

// Application tracing
mod tracing;
pub use tracing::Tracer;
