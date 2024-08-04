//-- ./src/lib.rs

/// Application.
mod app;
pub use app::App;
pub use app::AppResult;

/// Terminal events handler.
pub mod event;

/// Tui Error enums
mod error;
pub use error::TuiError;

/// Event handler.
pub mod handler;

mod proto;
pub use proto::rpc;
pub use proto::RpcClient;

/// Widget renderer.
pub mod ui;

/// Terminal user interface.
pub mod tui;
