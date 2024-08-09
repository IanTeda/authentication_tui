//-- ./src/lib.rs

/// Configuration file parsing
mod config;
pub use config::Config;

/// Tui Error enums
mod error;
pub use error::TuiError;

/// Terminal events handler.
pub mod event;

/// Event handlers.
pub mod handlers;

/// TUI application prelude
mod prelude;
pub use prelude::TuiResult;

/// Proto client
mod proto;
pub use proto::rpc;
pub use proto::RpcClient;

/// Application states
pub mod states;

/// Terminal user interface.
pub mod tui;

/// Widget renderer.
pub mod ui;

