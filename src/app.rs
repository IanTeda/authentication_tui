#![allow(unused)] // For beginning only.

use crate::{state, Terminal};

pub type Result<T> = std::result::Result<T, crate::error::Error>;

pub struct App {
    // Temporary application state
    state: state::AppState,

    // Application configuration
    config: crate::config::Config,
}

impl App {
    pub fn new(config: crate::config::Config) -> Result<Self> {
        let state = state::AppState::default();

        Ok(Self { state, config })
    }

    pub async fn run(&mut self) -> Result<()> {
        let mut terminal = Terminal::new()?;
        terminal.enter()?;

        // The TUI main loop
        while self.state.is_running {
            terminal.draw()?;

            // Terminal events
            // Wait every 16 seconds before checking for a crossterm event
            if crossterm::event::poll(std::time::Duration::from_millis(16))? {
                if let crossterm::event::Event::Key(key) = crossterm::event::read()?
                {
                    if key.kind == crossterm::event::KeyEventKind::Press
                        && key.code == crossterm::event::KeyCode::Char('q')
                    {
                        self.state.is_running = false;
                    }
                }
            }
        }

        // Restore terminal screen
        terminal.restore()?;

        Ok(())
    }
}
