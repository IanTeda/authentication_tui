//-- ./src/ui/mod.rs

//! The UI modules
//! ---

/// Component trait
mod component;
pub use component::Component;

/// Frame and tick rate overlay
mod fps;
pub use fps::FpsCounter;

/// The main container
mod container;
pub use container::Container;