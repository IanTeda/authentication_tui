//-- ./src/ui/container.rs

#![allow(unused)] // For beginning only.

//! The main UI container component
//! ---

use ratatui::{prelude::*, widgets};
use tokio::sync::mpsc::UnboundedSender;

use crate::{components, handlers, prelude::*, Config};

#[derive(Default)]
pub struct BackendComponent {
    command_tx: Option<UnboundedSender<handlers::Action>>,
    config: Config,
}

impl BackendComponent {
    pub fn new() -> Self {
        Self::default()
    }
}

impl components::Component for BackendComponent {
    fn register_action_handler(
        &mut self,
        tx: UnboundedSender<handlers::Action>,
    ) -> Result<()> {
        self.command_tx = Some(tx);
        Ok(())
    }

    fn register_config_handler(&mut self, config: Config) -> Result<()> {
        self.config = config;
        Ok(())
    }

    fn update(
        &mut self,
        action: handlers::Action,
    ) -> Result<Option<handlers::Action>> {
        match action {
            handlers::Action::Tick => {
                // add any logic here that should run on every tick
            }
            handlers::Action::Render => {
                // add any logic here that should run on every render
            }
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        Ok(())
    }
}
