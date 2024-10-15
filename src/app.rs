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
        let render =
            handlers::RenderEventHandler::init(config.clone(), actions.action_sender.clone());

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

        // Enter terminal raw mode
        terminal.enter()?;

        // Send application init action
        self.actions.action_sender.send(domain::Action::Init)?;

        //-- 3. Run the main application loop
        while self.state.app.is_running {
            // // Render the TUI to the terminal
            // self.render(&mut terminal)?;

            // Map crossterm events into actions
            self.actions.handle_events(&mut terminal.events).await?;

            // Update the app based on the action
            self.update(&mut terminal).await?;
        }

        //-- 4. Restore terminal screen on exit
        terminal.restore()?;

        Ok(())
    }

    // /// Render the terminal user interface
    // fn render(&mut self, terminal: &mut Terminal) -> Result<()> {
    //     terminal.draw(|frame| {
    //         ui::layout::render(self.config.clone(), &mut self.state, frame)
    //     })?;

    //     Ok(())
    // }

    async fn update(
        &mut self,
        terminal: &mut crate::Terminal,
    ) -> Result<()> {
        while let Ok(action) = self.actions.next_action() {
            // Check
            if action != domain::Action::Tick && action != domain::Action::Render {
                tracing::debug!("{action:?}");
            }

            // Match action
            match action {
                domain::Action::Init => {}
                domain::Action::Tick => {
                    self.tick.handle_event(&mut self.state).await
                }
                domain::Action::Render => {
                    // Render the TUI to the terminal
                    self.render.handle_event(&mut self.state, terminal)
                }
                domain::Action::Quit => self.state.app.is_running = false,
                _ => {}
            }
        }

        Ok(())
    }
}
