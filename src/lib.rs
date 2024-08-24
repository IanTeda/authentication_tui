//-- ./src/lib.rs

//! Application library name space
//! ---
mod app;
pub use app::App;

pub mod state;

mod terminal;
pub use terminal::Terminal;