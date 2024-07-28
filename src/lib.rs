//-- ./src/lib.rs

/// Application.
pub mod app;

/// Terminal events handler.
pub mod event;

/// Widget renderer.
pub mod ui;

/// Terminal user interface.
pub mod tui;

/// Event handler.
pub mod handler;

/// Error handler
// mod errors;

/// Tonic Client RPC files
pub mod rpc;

// Application tracing
mod tracing;
pub use tracing::Tracer;
