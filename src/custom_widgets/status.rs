//-- ./src/ui/custom_widgets/status.rs

//! Backend status custom widget for the footer
//! ---

use ratatui::{layout, style, text, widgets};

/// Footer status widget
pub struct StatusWidget {
    is_online: bool,
}

impl StatusWidget {
    pub fn init(is_online: bool) -> Self {
        Self { is_online }
    }
}

impl widgets::Widget for StatusWidget {
    /// [Required] Render the custom widget using the assigned area and terminal buffer
    fn render(self, area: layout::Rect, buf: &mut ratatui::buffer::Buffer) {
        // Set backend server status and style
        let (status, style) = match self.is_online {
            true => {
                let status = "Online";
                let style = style::Style::default()
                    .fg(style::Color::LightGreen)
                    // .bg(style::Color::Green)
                    .add_modifier(style::Modifier::BOLD);
                (status, style)
            }
            false => {
                let status: &str = "Offline";
                let style = style::Style::default()
                    .fg(style::Color::LightRed)
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
