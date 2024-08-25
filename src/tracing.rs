//-- ./src/tracing.rs

// #![allow(unused)] // For beginning only.

//! Tracing initiator
//!
//! # References
//! * [ratatui/examples/tracing.rs](https://github.com/ratatui-org/ratatui/blob/main/examples/tracing.rs)
//! * [Setup Logging with tracing](https://ratatui.rs/recipes/apps/log-with-tracing/)
//! ---

use std::{fs, path::PathBuf};
use tracing_error::ErrorLayer;
use tracing_subscriber::prelude::*;

use crate::prelude::*;

/// Initialize the tracing subscriber to log to a file
///
/// This function initialises the tracing subscriber to log to a file named `tracing.log` in the
/// project data directory.
pub fn init(data_directory: PathBuf) -> Result<()> {
    // Recursively create a directory and all of its parent components if they are missing.
    fs::create_dir_all(data_directory.clone())?;

    // Set log file
    let log_file = data_directory.join("tracing.log");
    let log_file = std::fs::File::create(log_file.clone())?;

    // By default, the subscriber is configured to log all events with a level of `DEBUG` or higher,
    // but this can be changed by setting the `RUST_LOG` environment variable.
    // TODO: Can do better than this, improve format of log line
    let env_filter = tracing_subscriber::EnvFilter::builder()
        .with_default_directive(tracing::Level::DEBUG.into())
        .from_env_lossy();

    let file_subscriber = tracing_subscriber::fmt::layer()
        .with_file(true)
        .with_line_number(true)
        .with_writer(log_file)
        .with_target(false)
        .with_ansi(false)
        .with_filter(env_filter);

    tracing_subscriber::registry()
        .with(file_subscriber)
        .with(ErrorLayer::default())
        .try_init()?;

    tracing::info!("Tracing (log) started.");

    Ok(())
}