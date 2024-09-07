//-- ./src/ui/mod.rs

//! The UI modules
//! ---

/// The main container
mod container;
pub use container::Container;

/// Component trait
mod component;
pub use component::Component;

mod custom_widgets;

/// Frame and tick rate overlay
mod fps;
pub use fps::FpsCounter;

/// A collection of common ui helper functions
mod helpers;

/// Toast message overlay
mod toast;
pub use toast::Toast;