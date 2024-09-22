//-- ./src/ui/container.rs

#![allow(unused)] // For beginning only.

//! The main UI container component
//! ---

use ratatui::prelude::*;
use tokio::sync::mpsc::UnboundedSender;
use crossterm::event as crossterm;

use crate::{components, domain, handlers, prelude::*, ui, Config};

#[derive(Default)]
pub struct ContainerComponent {
    action_tx: Option<UnboundedSender<handlers::Action>>,
    config: Config,

    /// UI components that get plugged in
    pub components: Vec<Box<dyn components::Component>>,
}

impl ContainerComponent {
    pub fn new() -> Self {
        let footer_component = components::FooterComponent::new();
        let home_component = components::HomeComponent::new();
        let components: Vec<Box<dyn components::Component>> =
            vec![Box::new(footer_component), Box::new(home_component)];

        Self {
            components,
            ..Default::default()
        }
    }
}

impl components::Component for ContainerComponent {
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
            let _ = component.register_config_handler(config.clone());
        }
        Ok(())
    }

    fn init(&mut self, area: layout::Size) -> Result<()> {
        for component in self.components.iter_mut() {
            let _ = component.init(area);
        }
        Ok(())
    }

    fn handle_key_event(
        &mut self,
        key_event: crossterm::KeyEvent,
    ) -> Result<Option<handlers::Action>> {
        

        let action = match key_event.code {
            crossterm::KeyCode::Char('g') => {
                // Build toast instance
                let toasty = domain::Toast::new("This a toast message")
                    .kind(domain::ToastKind::Error);

                // Return action for update
                handlers::Action::Toast(toasty)
            }
            _ => handlers::Action::Nil,
        };

        Ok(Some(action))
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
            let _ = component.update(action.clone());
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
        let _ = self.components[1].draw(frame, body_area);
        let _ = self.components[0].draw(frame, footer_area);

        Ok(())
    }
}
