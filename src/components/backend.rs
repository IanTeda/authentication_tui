//-- ./src/ui/container.rs

#![allow(unused)] // For beginning only.

//! The main UI container component
//! ---

use ratatui::{prelude::*, widgets};
use tokio::sync::mpsc::UnboundedSender;

use crate::{components, domain, handlers, prelude::*, Config};

#[derive(Default)]
pub struct BackendComponent {
    command_tx: Option<UnboundedSender<domain::Action>>,
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
        tx: UnboundedSender<domain::Action>,
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
        action: domain::Action,
    ) -> Result<Option<domain::Action>> {
        match action {
            domain::Action::Tick => {
                // add any logic here that should run on every tick
            }
            domain::Action::Render => {
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
