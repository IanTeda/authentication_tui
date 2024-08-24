//-- ./src/main.rs

#![allow(unused)] // For beginning only.

//! Binary entry point
//! ---

use std::io::{self, Result};

use ratatui::{prelude::*, widgets};

fn main() -> Result<()> {
    let mut terminal = authentication_tui::Terminal::new()?;
    terminal.enter();

    // The TUI main loop
    loop {
        // Draw the UI
        // A closure (an anonymous method) with a single Frame parameter, that 
        // renders the full size of the terminal window.
        terminal.backend.draw(|frame| {
            let area = frame.area();
            frame.render_widget(
                widgets::Paragraph::new("Hello Ratatui! (press 'q' to quit)")
                    .white()
                    .on_blue(),
                area,
            );
        })?;

        // Terminal events
        // Wait every 16 seconds before checking for a crossterm event
        if crossterm::event::poll(std::time::Duration::from_millis(16))? {
            if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                if key.kind == crossterm::event::KeyEventKind::Press && key.code == crossterm::event::KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    // Restore terminal screen
   terminal.restore();

    Ok(())
}