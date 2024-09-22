// #![allow(unused)] // For beginning only.

mod backend;
pub use backend::BackendComponent;

/// The main container
mod container;
pub use container::ContainerComponent;

/// Component trait
mod component;
pub use component::Component;

/// Frame and tick rate overlay
mod fps;
pub use fps::FpsComponent;

/// Footer / status bar
mod footer;
pub use footer::FooterComponent;

/// Home page
mod home;
pub use home::HomeComponent;

/// Toast message overlay
mod toast;
pub use toast::ToastComponent;