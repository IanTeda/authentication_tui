//-- ./src/handlers/mod.rs

//! # Handler modules
//! 
//! 1. Crossterm: Handle crossterm backend terminal events
//! 2. Keys: Handle key events
//! 3. Render: Handle backend frame render events
//! 4. Tick: Handle regular application tick events

mod actions;
pub use actions::ActionHandler;

/// Handle events coming from the crossterm backend terminal
mod crossterm;
pub use crossterm::CrosstermEventsHandler;

/// Handle key events
mod keys;
// pub use keys::handle_event;
pub use keys::KeyEventHandler;

/// Handle render events
mod render;
pub use render::RenderEventHandler;
/// Handle tick events
mod tick;
pub use tick::TickEventHandler;