//-- ./src/handlers/mod.rs

//! Handler modules

/// Handle events coming from the crossterm backend terminal
mod crossterm;
pub use crossterm::CrosstermEventsHandler;

/// Handle key events
mod keys;
pub use keys::key_events_handler;

/// Handle render events
mod render;
pub use render::render_event_handler;

/// Handle tick events
mod tick;
pub use tick::tick_event_handler;