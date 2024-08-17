//-- ./src/handlers/mod.rs

//! Handlers for events
//! ---

/// Handle key events
mod key_events;
pub use key_events::key_events;

/// Handle tick event
mod tick;
pub use tick::tick;

pub mod crossterm;
// pub use crossterm::CrosstermEventHandler;