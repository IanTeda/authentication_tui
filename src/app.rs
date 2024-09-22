//-- ./src/app.rs

#![allow(unused)] // For beginning only.

//! The TUI application module
//! ---

use crate::{components, handlers, prelude::*, state, Terminal};

// #[derive(Debug)]
pub struct App {
    /// Application state
    state: state::State,

    /// Application configuration
    config: crate::config::Config,

    /// Actions handler
    actions: crate::handlers::ActionHandler,

    /// UI components that get plugged in
    pub components: Vec<Box<dyn components::Component>>,
}

impl App {
    /// Create a new app instance
    pub fn new(config: crate::config::Config) -> Result<Self> {
        // Construct a default application state
        let state = state::State::default();

        // Construct a default action handler
        let actions = handlers::ActionHandler::default();

        // Initiate a new fps component
        let fps_component = components::FpsComponent::new();

        // Initiate a new main container (body)
        // let container = components::ContainerComponent::new();

        // Initiate a new toast message component
        let toast_component = components::ToastComponent::new();

        let backend_component = components::BackendComponent::new();

        let home_component = components::HomeComponent::new();

        // Built the components vector
        let components: Vec<Box<dyn components::Component>> = vec![
            // Store components on the heap (Box) not the stack
            // Box::new(container),
            Box::new(fps_component),
            Box::new(toast_component),
            Box::new(backend_component),
            Box::new(home_component),
        ];

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
        // Initiate a new backend terminal
        let mut terminal =
            Terminal::new(self.config.app.tick_rate, self.config.app.frame_rate)?;

        // Enter terminal raw mode
        terminal.enter()?;

        //-- 2. Plugin components
        for component in self.components.iter_mut() {
            // Pass the terminal area into each component
            // Deref of terminal backend needed for size
            component.init(terminal.size()?)?;

            // Pass the action handler transmit channel into each component
            component.register_action_handler(self.actions.sender.clone())?;

            // Pass the configuration instance into each components
            component.register_config_handler(self.config.clone())?;
        }

        //-- 3. Run the main application loop
        // The TUI application main loop
        while self.state.app.is_running {
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

    /// Update the application state for a given action.
    async fn update(&mut self, terminal: &mut Terminal) -> Result<()> {
        // Loop through all actions in the que.
        while let Ok(action) = self.actions.receiver.try_recv() {
            if action != handlers::Action::Tick && action != handlers::Action::Render
            {
                tracing::debug!("{action:?}");
            }
            match action {
                // Action::Tick => {
                //     self.last_tick_key_events.drain(..);
                // }
                handlers::Action::Quit => self.state.app.is_running = false,
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
                    self.actions.sender.send(action)?
                };
            }
        }
        Ok(())
    }

    /// Render the terminal user interface
    fn render(&mut self, terminal: &mut Terminal) -> Result<()> {
        // terminal.draw(|frame| ui::layout::render(&mut self.state, frame))?;

        terminal.draw(|frame| {
            for component in self.components.iter_mut() {
                if let Err(err) = component.draw(frame, frame.area()) {
                    let _ = self.actions.sender.send(handlers::Action::Error(
                        format!("Failed to draw: {:?}", err),
                    ));
                }
            }
        })?;

        Ok(())
    }
}
