//-- ./src/main.rs

// #![allow(unused)] // For beginning only.

//! Binary entry point
//! ---

use authentication_tui::prelude::*;

use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command line arguments
    let args = authentication_tui::cli::Args::parse();

    // Parse application configuration file
    let config = authentication_tui::Config::parse(args)?;

    // Initiate error panic hook
    authentication_tui::error::init(config.app.tick_rate, config.app.frame_rate)?;

    // Start tracing (log)
    let data_directory = config.clone().app.data_directory;
    authentication_tui::tracing::init(data_directory)?;

    // Initiate the TUI application
    let mut app = authentication_tui::App::new(config).await?;

    // Run the TUI application
    app.run().await?;

    Ok(())
}
