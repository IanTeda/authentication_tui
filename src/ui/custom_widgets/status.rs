//-- ./src/ui/custom_widgets/status.rs

// #![allow(unused)] // For beginning only.

//! Custom widget for the Authentication backend status in the footer
//! ---

use ratatui::{layout, style, widgets};

use crate::domain;

/// Footer status widget
#[derive(Debug, PartialEq, Default)]
pub struct StatusWidget {
    backend_status: domain::BackendStatus,
}

impl StatusWidget {
    /// Initiate a new status widget
    pub fn init(backend_status: domain::BackendStatus) -> Self {
        Self { backend_status }
    }
}

impl widgets::Widget for StatusWidget {
    /// [Required] Render the custom widget using the assigned area and terminal buffer
    fn render(self, area: layout::Rect, buf: &mut ratatui::buffer::Buffer) {
        // Set backend server status and style
        let (status, style) = match self.backend_status {
            domain::BackendStatus::Offline => {
                let status = "Offline";
                let style = style::Style::default()
                    .fg(style::Color::LightRed)
                    // .bg(style::Color::Green)
                    .add_modifier(style::Modifier::BOLD);
                (status, style)
            }
            domain::BackendStatus::Online => {
                let status = "Online";
                let style = style::Style::default()
                    .fg(style::Color::LightGreen)
                    .add_modifier(style::Modifier::BOLD);
                (status, style)
            }
            domain::BackendStatus::LoggedIn => {
                let status = "Logged In";
                let style = style::Style::default()
                    .fg(style::Color::Green)
                    .add_modifier(style::Modifier::BOLD);
                (status, style)
            }
        };

        // Construct a new block widget
        let block = widgets::Block::new();

        // Construct a new paragraph
        let paragraph = widgets::Paragraph::new(status).centered().style(style);

        // Render paragraph in the block
        paragraph.block(block).render(area, buf);
    }
}
