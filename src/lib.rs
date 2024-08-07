//-- ./src/lib.rs

/// Application.
mod app;
pub use app::App;
pub use app::AppResult;

/// Configuration file parsing
mod config;
pub use config::Config;


/// Tui Error enums
mod error;
pub use error::TuiError;

/// Terminal events handler.
pub mod event;

/// Event handler.
pub mod handler;

mod proto;
pub use proto::rpc;
pub use proto::RpcClient;

mod state;

/// Widget renderer.
pub mod ui;

/// Terminal user interface.
pub mod tui;
