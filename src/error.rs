//-- ./src/errors.rs

// #![allow(unused)] // For development only

//! Application Error types to define response
//!
//! # References
//!
//! * [Tired-Fox/rataify](https://github.com/Tired-Fox/rataify/blob/main/src/error.rs)
//! ---

use crate::prelude::*;

/// Application error enum
#[derive(thiserror::Error, Debug)]
pub enum Error {
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

    #[error(transparent)]
    TracingSubscriber(#[from] tracing_subscriber::util::TryInitError),

    #[error(transparent)]
    ColorEyreInit(#[from] color_eyre::eyre::InstallError),

    #[error(transparent)]
    Config(#[from] config::ConfigError),

    #[error(transparent)]
    TonicTransport(#[from] tonic::transport::Error),

    #[error(transparent)]
    TonicStatus(#[from] tonic::Status),

    #[error(transparent)]
    TonicUri(#[from] tonic::codegen::http::uri::InvalidUri),

    #[error(transparent)]
    ActionSend(#[from] tokio::sync::mpsc::error::SendError<crate::domain::Action>),

    #[error(transparent)]
    TryRecv(#[from] tokio::sync::mpsc::error::TryRecvError),
    // tokio::sync::mpsc::error::TryRecvError
}

pub fn init(tick_rate: f64, frame_rate: f64) -> Result<()> {
    let (panic_hook, eyre_hook) = color_eyre::config::HookBuilder::default()
        .panic_section(format!(
            "This is a bug. Consider reporting it at {}",
            env!("CARGO_PKG_REPOSITORY")
        ))
        .capture_span_trace_by_default(false)
        .display_location_section(false)
        .display_env_section(false)
        .into_hooks();
    eyre_hook.install()?;
    std::panic::set_hook(Box::new(move |panic_info| {
        if let Ok(mut t) = crate::Terminal::new(tick_rate, frame_rate) {
            if let Err(r) = t.restore() {
                tracing::error!("Unable to exit Terminal: {:?}", r);
            }
        }

        #[cfg(not(debug_assertions))]
        {
            use human_panic::{handle_dump, metadata, print_msg};
            let metadata = metadata!();
            let file_path = handle_dump(&metadata, panic_info);
            // prints human-panic message
            print_msg(file_path, &metadata)
                .expect("human-panic: printing error message to console failed");
            eprintln!("{}", panic_hook.panic_report(panic_info)); // prints color-eyre stack trace to stderr
        }
        let msg = format!("{}", panic_hook.panic_report(panic_info));
        tracing::error!("Error: {}", strip_ansi_escapes::strip_str(msg));

        #[cfg(debug_assertions)]
        {
            // Better Panic stacktrace that is only enabled when debugging.
            better_panic::Settings::auto()
                .most_recent_first(false)
                .lineno_suffix(true)
                .verbosity(better_panic::Verbosity::Full)
                .create_panic_handler()(panic_info);
        }

        std::process::exit(libc::EXIT_FAILURE);
    }));
    Ok(())
}

/// Similar to the `std::dbg!` macro, but generates `tracing` events rather
/// than printing to stdout.
///
/// By default, the verbosity level for the generated events is `DEBUG`, but
/// this can be customised.
#[macro_export]
macro_rules! trace_dbg {
        (target: $target:expr, level: $level:expr, $ex:expr) => {{
                match $ex {
                        value => {
                                tracing::event!(target: $target, $level, ?value, stringify!($ex));
                                value
                        }
                }
        }};
        (level: $level:expr, $ex:expr) => {
                trace_dbg!(target: module_path!(), level: $level, $ex)
        };
        (target: $target:expr, $ex:expr) => {
                trace_dbg!(target: $target, level: tracing::Level::DEBUG, $ex)
        };
        ($ex:expr) => {
                trace_dbg!(level: tracing::Level::DEBUG, $ex)
        };
}
