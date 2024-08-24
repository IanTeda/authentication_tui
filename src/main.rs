//-- ./src/main.rs

//! Binary entry point
//! ---

use std::io::{self, Result};

use crossterm::{event, terminal, ExecutableCommand};
use ratatui::{prelude::*, widgets};

fn main() -> Result<()> {
    // Enter into the alternate screen, which is a secondary screen that allows 
    // the TUI application to render whatever it needs to, without disturbing 
    // the normal output of terminal apps in the shell.
    io::stdout().execute(terminal::EnterAlternateScreen)?;

    // Enable terminal raw mode, which turns off input and output processing by 
    // the terminal. This gives the TUI application control over when to print 
    // characters to the screen.
    terminal::enable_raw_mode()?;

    // Build a new terminal instance
    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;

    // Clear the terminal window
    terminal.clear()?;

    // The TUI main loop
    loop {
        // Draw the UI
        // A closure (an anonymous method) with a single Frame parameter, that 
        // renders the full size of the terminal window.
        terminal.draw(|frame| {
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
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Press && key.code == event::KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    // Restore terminal screen
    io::stdout().execute(terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}