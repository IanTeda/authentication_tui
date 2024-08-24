// #![allow(unused)] // For beginning only.

use std::io::Result;

use crate::{state, Terminal};

pub struct App {
    state: state::AppState,
}

impl App {
    pub fn new() -> Result<Self> {
        let state = state::AppState::default();

        Ok(Self { state })
    }

    pub fn run(&mut self) -> Result<()> {
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
