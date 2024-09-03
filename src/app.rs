//-- ./src/app.rs

//! The TUI application module
//! ---

// #![allow(unused)] // For beginning only.

use crate::{handlers, prelude::*, state, ui, Terminal};

// #[derive(Debug)]
pub struct App {
    /// Application state
    state: state::AppState,

    /// Application configuration
    config: crate::config::Config,

    /// Actions handler
    actions: crate::handlers::ActionHandler,

    pub components: Vec<Box<dyn ui::Component>>,
}

impl App {
    /// Create a new app instance
    pub fn new(config: crate::config::Config) -> Result<Self> {
        // Construct a default application state
        let state = state::AppState::default();

        // Construct a default action handler
        let actions = handlers::ActionHandler::default();

        // Initiate a new fps component and store it on the heap (Box) not the stack
        let fps_component = Box::new(ui::FpsCounter::new());

        let container = Box::new(ui::Container::new());

        // Built the components vector
        let components: Vec<Box<dyn ui::Component>> = vec![container, fps_component];

        Ok(Self {
            state,
            config,
            actions,
            components,
        })
    }

    /// Run the TUI application
    pub async fn run(&mut self) -> Result<()> {
        //-- 1. Build the terminal backend
        let tick_rate = self.config.app.tick_rate;
        let frame_rate = self.config.app.frame_rate;
        let mut terminal = Terminal::new(tick_rate, frame_rate)?;

        // Enter terminal raw mode
        terminal.enter()?;

        //-- 2. Plugin components
        // Pass in the terminal area into the component
        for component in self.components.iter_mut() {
            // Deref of terminal backend needed for size
            component.init(terminal.size()?)?;
        }

        // Pass in the action handler transmit channel
        for component in self.components.iter_mut() {
            component.register_action_handler(self.actions.action_tx.clone())?;
        }

        // Pass in the configuration
        for component in self.components.iter_mut() {
            component.register_config_handler(self.config.clone())?;
        }

        //-- 3. Run the main application loop
        // The TUI application main loop
        while self.state.is_running {
            // Add any new terminal events to the action handler
            self.actions
                .handle_events(&mut terminal, &mut self.components)
                .await?;

            // Update the application
            self.update(&mut terminal).await?;
        }

        //-- 4. Restore terminal screen on exit
        terminal.restore()?;

        Ok(())
    }

    /// Update the application for a given action.
    async fn update(&mut self, terminal: &mut Terminal) -> Result<()> {
        // Loop through all actions in the que.
        while let Ok(action) = self.actions.action_rx.try_recv() {
            if action != handlers::Action::Tick && action != handlers::Action::Render
            {
                tracing::debug!("{action:?}");
            }
            match action {
                // Action::Tick => {
                //     self.last_tick_key_events.drain(..);
                // }
                handlers::Action::Quit => self.state.is_running = false,
                // Action::Suspend => self.should_suspend = true,
                // Action::Resume => self.should_suspend = false,
                // Action::ClearScreen => tui.terminal.clear()?,
                // Action::Resize(w, h) => self.handle_resize(tui, w, h)?,
                handlers::Action::Render => self.render(terminal)?,
                _ => {}
            }

            // Plugin in components update function
            for component in self.components.iter_mut() {
                if let Some(action) = component.update(action.clone())? {
                    self.actions.action_tx.send(action)?
                };
            }
        }
        Ok(())
    }

    fn render(&mut self, terminal: &mut Terminal) -> Result<()> {
        // Redraw the terminal UI
        // terminal.draw()?;
        terminal.draw(|frame| {
            for component in self.components.iter_mut() {
                if let Err(err) = component.draw(frame, frame.area()) {
                    let _ = self
                        .actions
                        .action_tx
                        .send(handlers::Action::Error(format!("Failed to draw: {:?}", err)));
                }
            }
        })?;

        Ok(())
    }
}
