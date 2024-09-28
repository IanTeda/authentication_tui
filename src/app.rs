//-- ./src/app.rs

// #![allow(unused)] // For beginning only.

//! The TUI application module
//! ---

use crate::handlers;
pub(crate) use crate::{domain, prelude::*, state, ui, Terminal};

// #[derive(Debug)]
pub struct App {
    /// Application state
    state: state::State,

    /// Application configuration
    config: crate::config::Config,

    /// Application tick event
    tick: crate::handlers::TickEventHandler,

    /// Application render event
    render: crate::handlers::RenderEventHandler,
}

impl App {
    /// Construct a new app instance
    pub fn new(config: crate::config::Config) -> Result<Self> {
        // Construct a default application state
        let state = state::State::default();

        // Construct a new tick event handler
        let tick = handlers::TickEventHandler::new();

        // Construct a new render event handler
        let render = handlers::RenderEventHandler::new();

        Ok(Self {
            state,
            config,
            tick,
            render,
        })
    }

    /// Run the TUI application
    pub async fn run(&mut self) -> Result<()> {
        //-- 1. Build the terminal backend
        // Initiate a new backend terminal
        let mut terminal =
            Terminal::new(self.config.app.tick_rate, self.config.app.frame_rate)?;

        // Enter terminal raw mode
        terminal.enter()?;

        //-- 3. Run the main application loop
        // The TUI application main loop
        while self.state.app.is_running {
            // Render the TUI to the terminal
            self.render(&mut terminal)?;

            // Match crossterm backend terminal events for action
            match terminal.events.next().await? {
                domain::Event::Tick => self.tick.handle_event(&mut self.state),
                domain::Event::Render => self.render.handle_event(&mut self.state),
                domain::Event::Key(key_event) => {
                    handlers::handle_event(key_event, &mut self.state)?
                }
                domain::Event::Mouse(_) => {}
                domain::Event::Resize(_, _) => {}
                _ => {}
            }
        }

        //-- 4. Restore terminal screen on exit
        terminal.restore()?;

        Ok(())
    }

    /// Render the terminal user interface
    fn render(&mut self, terminal: &mut Terminal) -> Result<()> {
        terminal.draw(|frame| {
            ui::layout::render(
                self.config.clone(), 
                &mut self.state, 
                frame
            )
        })?;

        Ok(())
    }
}
