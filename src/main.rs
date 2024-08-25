//-- ./src/main.rs

// #![allow(unused)] // For beginning only.

//! Binary entry point
//! ---

use authentication_tui::prelude::*;

use clap::Parser;

fn main() -> Result<()> {
    // Initiate error panic hook
    authentication_tui::error::init()?;

    // Parse command line arguments
    let args = authentication_tui::cli::Args::parse();

    // Parse application configuration file
    let config = authentication_tui::Config::parse(args)?;

    // Start tracing (log)
    let data_directory = config.clone().app.data_directory;
    authentication_tui::tracing::init(data_directory)?;

    // Construct a new TUI application instance
    let mut app = authentication_tui::App::new(config)?;

    // Run the TUI app
    app.run()?;

    Ok(())
}