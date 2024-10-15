//-- ./src/app.rs

// #![allow(unused)] // For beginning only.

//! The TUI application module
//! ---

use crate::handlers;
pub(crate) use crate::{domain, prelude::*, state, Terminal};

// #[derive(Debug)]
pub struct App {
    /// Application state
    state: crate::state::State,

    /// Application configuration
    config: crate::config::Config,

    /// Action handler
    actions: crate::handlers::ActionHandler,

    /// Application tick event
    tick: crate::handlers::TickEventHandler,

    /// Application render event
    render: crate::handlers::RenderEventHandler,
}

impl App {
    /// Construct a new app instance
    pub async fn new(config: crate::config::Config) -> Result<Self> {
        // Construct a default application state
        let state = state::State::default();

        // Construct action handler
        let actions = handlers::ActionHandler::default();

        // Construct a new tick event handler
        let tick = handlers::TickEventHandler::init(actions.action_sender.clone());

        // // Construct a new render event handler
        let render = handlers::RenderEventHandler::init(
            config.clone(),
            actions.action_sender.clone(),
        );

        Ok(Self {
            state,
            config,
            actions,
            tick,
            render,
        })
    }

    /// Run the TUI application
    pub async fn run(&mut self) -> Result<()> {
        //-- 1. Build the terminal backend
        // Initiate a new backend terminal
        let tick_rate = self.config.app.tick_rate;
        let frame_rate = self.config.app.frame_rate;
        let mut terminal = Terminal::new(tick_rate, frame_rate)?;

        //-- 2. Enter terminal raw mode
        terminal.enter()?;

        //-- 3. Run the main application loop
        while self.state.app.is_running {

            // Map crossterm events into actions
            self.actions.handle_events(&mut terminal.events).await?;

            // Update the app based on the action
            self.update(&mut terminal).await?;
        }

        //-- 4. Restore terminal screen on exit
        terminal.restore()?;

        Ok(())
    }

    /// Update the TUI application
    async fn update(&mut self, terminal: &mut crate::Terminal) -> Result<()> {
        // While we have a next action in the queue
        while let Ok(action) = self.actions.next_action() {
            // Check action is working
            if action != domain::Action::Tick && action != domain::Action::Render {
                tracing::debug!("{action:?}");
            }

            // Match action
            match action {
                // Terminal initiation event action
                domain::Action::Init => {}

                // Application tick action
                domain::Action::Tick => {
                    // handle tick event
                    self.tick.handle_event(&mut self.state).await
                }

                // Application render action
                domain::Action::Render => {
                    // Handle render event
                    self.render.handle_event(&mut self.state, terminal)
                }

                // Quit tui application
                domain::Action::Quit => self.state.app.is_running = false,

                domain::Action::Toast(t) => self.state.toast.queue.push_back(t),

                // Do nothing with all other actions
                _ => {}
            }
        }

        Ok(())
    }
}
