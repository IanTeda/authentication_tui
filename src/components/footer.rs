//-- ./src/ui/container.rs

// #![allow(unused)] // For beginning only.

//! The main UI container component
//! ---

use ratatui::{prelude::*, widgets};

use crate::{components, custom_widgets, domain, prelude::*, ui};

#[derive(Default)]
pub struct FooterComponent {
    backend_status: domain::BackendStatus,
}

impl FooterComponent {
    pub fn new() -> Self {
        Self::default()
    }
}

impl components::Component for FooterComponent {
    fn update(&mut self, action: domain::Action) -> Result<Option<domain::Action>> {
        if let domain::Action::BackendStatus(status) = action {
            self.backend_status = status;
        }
        Ok(None)
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

        // TODO: Pass reference with lifetime
        let status_widget =
            custom_widgets::StatusWidget::init(self.backend_status.clone());
        frame.render_widget(status_widget, status_area);

        Ok(())
    }
}
