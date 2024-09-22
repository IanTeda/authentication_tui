//-- ./src/handlers/mod.rs

//! Handlers module
//! ---

/// Action handler
mod action;
pub use action::ActionHandler;

/// Event enum and handler
mod event;
pub use event::Event;
pub use event::EventLoopHandler;
