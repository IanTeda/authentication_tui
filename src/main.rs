//-- ./src/main.rs

//! TUI entry point
//! ---

use authentication_tui::{handlers, Config, TuiResult};
use authentication_tui::tui::Tui;
use authentication_tui::states;
use authentication_tui::tracing;
use authentication_tui::handlers::crossterm::{self, CrosstermEventHandler};

#[tokio::main]
async fn main() -> TuiResult<()> {
// async fn tokio_main() -> TuiResult<()> {
    // Parse application configuration file
    let config = Config::parse()?;

    // Start tracing (log)
    let data_directory = config.clone().tui.data_directory;
    tracing::init(data_directory)?;

    let tick_rate = config.clone().tui.tick_rate;
    let frame_rate = config.clone().tui.frame_rate;
    let crossterm_event_handler = CrosstermEventHandler::new(tick_rate, frame_rate);
    let mut tui = Tui::new(crossterm_event_handler)?;
    tui.init()?;

    // Create an application.
    let mut app = states::AppState::new(config);

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        
        // Handle events.
        match tui.events.next().await? {
            // Event::Error(_) => {},
            crossterm::Event::Tick => handlers::tick(&mut app).await?,
            // Event::Key(key_event) => handle_key_events(key_event, &mut app).await?,
            crossterm::Event::Key(key_event) => handlers::key_events(key_event, &mut app).await?,
            crossterm::Event::Mouse(_) => {}
            crossterm::Event::Paste(_) => {}
            crossterm::Event::Resize(_, _) => {}
            _ => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}


// #[tokio::main]
// async fn main() -> TuiResult<()> {
//   if let Err(e) = tokio_main().await {
//     eprintln!("{} error: Something went wrong", env!("CARGO_PKG_NAME"));
//     Err(e)
//   } else {
//     Ok(())
//   }
// }