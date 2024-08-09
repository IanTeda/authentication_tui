//-- ./src/main.rs

//! TUI entry point
//! ---

use authentication_tui::{handlers, Config, TuiResult};
use authentication_tui::event::{Event, EventHandler};
use authentication_tui::tui::Tui;
use authentication_tui::states;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;

#[tokio::main]
async fn main() -> TuiResult<()> {
    // Parse application configuration file
    let config = Config::parse()?;

    // Create an application.
    let mut app = states::App::new(config);

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next().await? {
            Event::Tick => handlers::tick(&mut app).await?,
            // Event::Key(key_event) => handle_key_events(key_event, &mut app).await?,
            Event::Key(key_event) => handlers::key_events(key_event, &mut app).await?,
            Event::Mouse(_) => {}
            Event::Paste(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
