//-- ./src/errors.rs

// #![allow(unused)] // For development only


/// Application Error types to define response
/// 
/// #### References
/// 
/// * [Tired-Fox/rataify](https://github.com/Tired-Fox/rataify/blob/main/src/error.rs)
/// ---

/// Application error enum
#[derive(thiserror::Error, Debug)]
pub enum TuiError {
    //-- Generic Errors
    /// For starter, to remove as code matures.
    #[error("Generic error: {0}")]
    Generic(String),

    /// For starter, to remove as code matures.
    #[error("Static error: {0}")]
    Static(&'static str),

    //-- External errors
    /// Derive IO errors
    #[error(transparent)]
    IO(#[from] std::io::Error),

    // Config errors
    #[error(transparent)]
    Config(#[from] config::ConfigError),

    #[error(transparent)]
    ColorEyre(#[from] color_eyre::eyre::InstallError),

    #[error(transparent)]   
    Toml(#[from] toml::ser::Error),

    #[error(transparent)]
    Uri(#[from] http::uri::InvalidUri),

    #[error(transparent)]
    TonicTransport(#[from] tonic::transport::Error),

    #[error(transparent)]
    TonicStatus(#[from] tonic::Status),

    #[error(transparent)]
    TracingSubscriber(#[from] tracing_subscriber::util::TryInitError),

}