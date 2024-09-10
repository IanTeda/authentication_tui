//-- ./src/handlers/mod.rs

//! Handlers module
//! ---

/// Action enum and handler
mod action;
pub use action::Action;
pub use action::ActionHandler;

/// Event enum and handler
mod event;
pub use event::Event;
pub use event::EventLoopHandler;
