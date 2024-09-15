//-- ./src/ui/container.rs

// #![allow(unused)] // For beginning only.

//! The main UI container component
//! ---

use ratatui::prelude::*;
use tokio::sync::mpsc::UnboundedSender;

use crate::{handlers, prelude::*, ui, Config};

#[derive(Default)]
pub struct Container {
    action_tx: Option<UnboundedSender<handlers::Action>>,
    config: Config,

    /// UI components that get plugged in
    pub components: Vec<Box<dyn ui::Component>>,
}

impl Container {
    pub fn new() -> Self {
        let footer_component = ui::FooterComponent::new();
        let home_component = ui::HomeComponent::new();
        let components: Vec<Box<dyn ui::Component>> =
            vec![Box::new(footer_component), Box::new(home_component)];

        Self {
            components,
            ..Default::default()
        }
    }
}

impl ui::Component for Container {
    fn register_action_handler(
        &mut self,
        tx: UnboundedSender<handlers::Action>,
    ) -> Result<()> {
        self.action_tx = Some(tx.clone());
        for component in self.components.iter_mut() {
            component.register_action_handler(tx.clone())?;
        }
        Ok(())
    }

    fn register_config_handler(&mut self, config: Config) -> Result<()> {
        self.config = config.clone();
        for component in self.components.iter_mut() {
            component.register_config_handler(config.clone());
        }
        Ok(())
    }

    fn init(&mut self, area: layout::Size) -> Result<()> {
        for component in self.components.iter_mut() {
            component.init(area);
        }
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
        for component in self.components.iter_mut() {
            component.update(action.clone());
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        // Split the terminal window (container) into body and footer rectangles
        let (body_area, footer_area) = {
            let split = Layout::vertical([
                Constraint::Min(6),    // body
                Constraint::Length(1), //footer
            ])
            .split(area);
            (split[0], split[1])
        };
        self.components[1].draw(frame, body_area);
        self.components[0].draw(frame, footer_area);

        Ok(())
    }
}
