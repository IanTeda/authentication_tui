//-- ./src/app.rs

//! The TUI application module
//! ---

#![allow(unused)] // For beginning only.

use crate::{handlers::{self, Action}, prelude::*, state, Terminal};

pub struct App {
    // Temporary application state
    state: state::AppState,

    // Application configuration
    config: crate::config::Config,

    actions: crate::handlers::ActionHandler,
}

impl App {
    /// Create a new app instance
    pub fn new(config: crate::config::Config) -> Result<Self> {
        // Construct a default application state
        let state = state::AppState::default();

        // Construct a default action handler
        let actions = handlers::ActionHandler::default();

        Ok(Self { state, config, actions })
    }

    /// Run the TUI application
    pub async fn run(&mut self) -> Result<()> {
        // Construct a new terminal backend instance
        let tick_rate = self.config.app.tick_rate;
        let frame_rate = self.config.app.frame_rate;
        let mut terminal = Terminal::new(tick_rate, frame_rate)?;

        // Enter terminal raw mode
        terminal.enter()?;

        // terminal.draw()?;

        // // The TUI application main loop
        while self.state.is_running {

            // Add any new terminal events to the action handler
            self.actions.handle_events(&mut terminal).await?;

            // Update the application
            self.update(&mut terminal).await?;
        }

        // Restore terminal screen
        terminal.restore()?;

        Ok(())
    }

    async fn update(&mut self, terminal: &mut Terminal) -> Result<()> {
        while let Ok(action) = self.actions.action_rx.try_recv() {
            if action != handlers::Action::Tick && action != handlers::Action::Render {
                tracing::debug!("{action:?}");
            }
            match action {
                // Action::Tick => {
                //     self.last_tick_key_events.drain(..);
                // }
                Action::Quit => self.state.is_running = false,
                // Action::Suspend => self.should_suspend = true,
                // Action::Resume => self.should_suspend = false,
                // Action::ClearScreen => tui.terminal.clear()?,
                // Action::Resize(w, h) => self.handle_resize(tui, w, h)?,
                Action::Render => self.render(terminal)?,
                _ => {}
            }
        }
        Ok(())
    }

    fn render(&mut self, terminal: &mut Terminal) -> Result<()> {
        // Redraw the terminal UI
        terminal.draw()?;

        Ok(())
    }
}
