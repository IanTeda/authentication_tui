//-- ./src/ui/custom_widgets/status.rs

//! Backend status custom widget for the footer
//! ---

use ratatui::{layout, style, text, widgets};

/// Footer status widget
pub struct StatusWidget {
    is_online: bool
}

impl StatusWidget {
    pub fn init(is_online: bool) -> Self {
        Self {
            is_online
        }
    }
}

impl widgets::Widget for StatusWidget {
    /// [Required] Render the custom widget using the assigned area and terminal buffer
    fn render(self, area: layout::Rect, buf: &mut ratatui::buffer::Buffer) {

    }
}