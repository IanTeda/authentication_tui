//-- ./src/ui/container.rs

#![allow(unused)] // For beginning only.

//! The main UI container component
//! ---


use crossterm::event as crossterm;
use ratatui::{prelude::*, widgets};
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    components, custom_widgets, domain, handlers, prelude::*, ui, Config
};

#[derive(Default)]
pub struct FooterComponent {
    command_tx: Option<UnboundedSender<domain::Action>>,
    config: Config,
    backend_status: bool,
}

impl FooterComponent {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update_backend_status(&mut self) -> Result<()> {
        let status = true;
        self.backend_status = status;

        Ok(())
    }
}

impl components::Component for FooterComponent {
    // fn register_action_handler(
    //     &mut self,
    //     tx: UnboundedSender<domain::Action>,
    // ) -> Result<()> {
    //     self.command_tx = Some(tx);
    //     Ok(())
    // }

    // fn register_config_handler(&mut self, config: Config) -> Result<()> {
    //     self.config = config;
    //     Ok(())
    // }

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
            domain::Action::UpdateBackendStatus => {
                self.backend_status = true;
                // self.update_backend_status();
                // let toast = domain::Toast::new("Backend status updated").kind(domain::ToastKind::Notification);
                // Some(domain::Action::Toast(toast))
            }
            _ => {}
        }
        Ok(None)
    }

    fn handle_key_event(
        &mut self,
        key_event: crossterm::KeyEvent,
    ) -> Result<Option<domain::Action>> {
        let action = match key_event.code {
            crossterm::KeyCode::Char('r') => {
                // Build toast instance
                let toasty = domain::Toast::new("This a toast message")
                    .kind(domain::ToastKind::Error);

                // Return action for update
                domain::Action::Toast(toasty)
            }
            crossterm::KeyCode::Esc => domain::Action::ClearToast,
            _ => domain::Action::Nil,
        };

        Ok(Some(action))
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let footer_area = ui::helpers::footer(area);

        // Split up the footer rectangle
        let (footer_left, status_area) = {
            let split = layout::Layout::horizontal([
                layout::Constraint::Min(24),    // Left
                layout::Constraint::Length(12), // Right
            ])
            .split(footer_area);
            (split[0], split[1])
        };

        frame.render_widget(
            widgets::Paragraph::new("Footer").white().on_black(),
            footer_left,
        );

        let status_widget = custom_widgets::StatusWidget::init(self.backend_status);
        frame.render_widget(status_widget, status_area);

        Ok(())
    }
}
