//-- ./src/app.rs

#![allow(unused)] // For beginning only.

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

    /// Action handler
    actions: crate::handlers::ActionHandler, 

    /// Application tick event
    tick: crate::handlers::TickEventHandler,

    // /// Application render event
    render: crate::handlers::RenderEventHandler,

    // /// Key event handler
    keys: crate::handlers::KeyEventHandler,
}

impl App {
    /// Construct a new app instance
    pub async fn new(config: crate::config::Config) -> Result<Self> {
        // Construct a default application state
        let state = state::State::default();

        // Construct action handler
        let actions = handlers::ActionHandler::default();

        // Build the rpc client, setting Offline if error returned
        // let rpc_client = client::RpcClient::new(config.backend.address()).await?;

        // Construct a new tick event handler
        let tick = handlers::TickEventHandler::new(config.clone());

        // // Construct a new render event handler
        let render = handlers::RenderEventHandler::new(config.clone());

        // // Construct key event handler
        let keys = handlers::KeyEventHandler::new(config.clone());

        Ok(Self {
            state,
            config,
            actions,
            // rpc_client,
            tick,
            render,
            keys,
        })
    }

    /// Run the TUI application
    pub async fn run(&mut self) -> Result<()> {
        //-- 1. Build the terminal backend
        // Initiate a new backend terminal
        let mut terminal =
            Terminal::new(self.config.app.tick_rate, self.config.app.frame_rate)?;

        // // Assign socket address for communicating with the backend
        // let rpc_server_address = self.config.backend.address();

        // // Build the rpc client, setting Offline if error returned
        // let rpc_client: Option<client::RpcClient> =
        //     match client::RpcClient::new(rpc_server_address).await {
        //         // Match call returned an ok result
        //         Ok(rpc_client) => Some(rpc_client),

        //         // Match call returned an error result
        //         Err(error) => {
        //             // Set state to Offline
        //             self.state.backend.status = domain::BackendStatus::Offline;

        //             // Provide toast of status to user
        //             let toast_message =
        //                 format!("Backend server is: {:?}", self.state.backend.status);
        //             let toast = domain::Toast::new(toast_message);
        //             self.state.toast.queue.push_back(toast);

        //             // Send error to tracing log
        //             tracing::error!("Error connecting to backend server: {}", error);

        //             // Return None
        //             None
        //         }
        //     };

        // Enter terminal raw mode
        terminal.enter()?;

        //-- 3. Run the main application loop
        // The TUI application main loop
        while self.state.app.is_running {
            // Render the TUI to the terminal
            self.render(&mut terminal)?;

            self.actions.handle_events(&mut terminal).await?;

            self.update();

            // Match crossterm backend terminal events for action
            // match terminal.events.next().await {
                // //TODO: Can we work around this clone
                // domain::Event::Tick => {
                //     self.tick
                //         .handle_event(&mut self.state)
                //         .await
                // }
                // domain::Event::Render => self.render.handle_event(&mut self.state),
                // domain::Event::Key(key_event) => {
                //     self.keys.handle_event(key_event, &mut self.state).await?
                // }
                // domain::Event::Mouse(_) => {}
                // domain::Event::Resize(_, _) => {}
                // _ => {}
            // }
        }

        //-- 4. Restore terminal screen on exit
        terminal.restore()?;

        Ok(())
    }

    /// Render the terminal user interface
    fn render(&mut self, terminal: &mut Terminal) -> Result<()> {
        terminal.draw(|frame| {
            ui::layout::render(self.config.clone(), &mut self.state, frame)
        })?;

        Ok(())
    }

    async fn update(&mut self) -> Result<()> {
        while let Some(action) = self.actions.next().await {
            if action != domain::Action::Tick && action != domain::Action::Render {
                tracing::debug!("{action:?}");
            }

             match action {
                domain::Action::Tick => self.tick.handle_event(&mut self.state).await,
                domain::Action::Quit => self.state.app.is_running = false,
                _ => {}
             }
        }

        Ok(())
    }
}
