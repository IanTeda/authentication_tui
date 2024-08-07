//-- ./src/tracing.rs

//! Tracing initiator
//!
//! # References
//! * [ratatui/examples/tracing.rs](https://github.com/ratatui-org/ratatui/blob/main/examples/tracing.rs)
//! * [Setup Logging with tracing](https://ratatui.rs/recipes/apps/log-with-tracing/)
//! ---

use std::fs;

use tracing_appender::non_blocking::WorkerGuard;

use crate::app::AppResult;

#[allow(dead_code)]
pub struct Tracer(WorkerGuard);

impl Tracer {
    /// Initialize the tracing subscriber to log to a file
    ///
    /// This function initialises the tracing subscriber to log to a file named `tracing.log` in the
    /// current directory. The function returns a [`WorkerGuard`] that must be kept alive for the
    /// duration of the program to ensure that logs are flushed to the file on shutdown. The logs are
    /// written in a non-blocking fashion to ensure that the logs do not block the main thread.
    pub fn init() -> AppResult<Self> {
        //TODO: Log to system folders
        let file = fs::File::create("tracing.log")?;

        let (non_blocking, guard) = tracing_appender::non_blocking(file);

        // By default, the subscriber is configured to log all events with a level of `DEBUG` or higher,
        // but this can be changed by setting the `RUST_LOG` environment variable.
        // TODO: Can do better than this, improve format of log line
        let env_filter = tracing_subscriber::EnvFilter::builder()
            .with_default_directive(tracing::Level::DEBUG.into())
            .from_env_lossy();

        tracing_subscriber::fmt()
            .with_writer(non_blocking)
            .with_env_filter(env_filter)
            .init();

        tracing::info!("Starring tracing");

        Ok(Self(guard))
    }
}
