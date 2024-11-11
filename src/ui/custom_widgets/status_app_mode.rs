use ratatui::{layout, style, widgets};

use crate::domain;

/// Footer status widget
#[derive(Debug, PartialEq, Default)]
pub struct StatusAppModeWidget {
    app_mode: domain::AppMode,
}

impl StatusAppModeWidget {
    /// Initiate a new status widget
    pub fn init(app_mode: domain::AppMode) -> Self {
        Self { app_mode }
    }
}

impl widgets::Widget for StatusAppModeWidget {
    /// [Required] Render the custom widget using the assigned area and terminal buffer
    fn render(self, area: layout::Rect, buf: &mut ratatui::buffer::Buffer) {
        // Set backend server status and style
        let (status, style) = match self.app_mode {
            domain::AppMode::Normal => {
                let status = "Normal";
                let style = style::Style::default()
                    .fg(style::Color::White)
                    // .bg(style::Color::Green)
                    .add_modifier(style::Modifier::BOLD);
                (status, style)
            }
            domain::AppMode::Input => {
                let status = "::Input::";
                let style = style::Style::default()
                    .fg(style::Color::LightGreen)
                    .add_modifier(style::Modifier::BOLD);
                (status, style)
            }
        };

        // Construct a new block widget
        let block = widgets::Block::new();

        // Construct a new paragraph
        let paragraph = widgets::Paragraph::new(status).left_aligned().style(style);

        // Render paragraph in the block
        paragraph.block(block).render(area, buf);
    }
}
