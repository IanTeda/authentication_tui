//-- ./src/main.rs

// #![allow(unused)] // For beginning only.

//! Binary entry point
//! ---

use std::io::Result;

use authentication_tui::App;

fn main() -> Result<()> {
    // Construct a new TUI application instance
    let mut app = App::new()?;

    // Run the TUI app
    app.run()?;

    Ok(())
}