//-- ./src/ui/custom_widgets/toast.rs

// #![allow(unused)] // For beginning only.

//! A custom widget for showing toast messages
//! ---

use ratatui::{layout, style, text, widgets};

use crate::{domain, ui};

pub struct ToastWidget {
    pub(crate) toast: domain::Toast,
}

impl ToastWidget {
    pub fn init(toast: domain::Toast) -> Self {
        Self {
            toast
        }
    }
}

impl widgets::Widget for ToastWidget {
    /// [Required] Render the custom widget using the assigned area and terminal buffer
    fn render(self, area: layout::Rect, buf: &mut ratatui::buffer::Buffer) {
        // Calculate widget layout area / position
        let top_right = ui::helpers::top_right(40, 4, area);

        // Set the toast style based on the toast kind
        let (toast_title, toast_style) = match self.toast.kind {
            domain::ToastKind::Error => {
                let toast_style = style::Style::default()
                    .fg(style::Color::Red)
                    .add_modifier(style::Modifier::BOLD);
                let toast_title = text::Span::styled("Error", toast_style);
                (toast_title, toast_style)
            }
            domain::ToastKind::Info => {
                let toast_style = style::Style::default()
                    .fg(style::Color::LightBlue)
                    .add_modifier(style::Modifier::BOLD);
                let toast_title = text::Span::styled("Info", toast_style);
                (toast_title, toast_style)
            }
            domain::ToastKind::Notification => {
                let toast_style = style::Style::default()
                    .fg(style::Color::White)
                    .add_modifier(style::Modifier::BOLD);
                let toast_title = text::Span::styled("Notification", toast_style);
                (toast_title, toast_style)
            }
            domain::ToastKind::Success => {
                let toast_style = style::Style::default()
                    .fg(style::Color::LightGreen)
                    .add_modifier(style::Modifier::BOLD);
                let toast_title = text::Span::styled("Success", toast_style);
                (toast_title, toast_style)
            }
            domain::ToastKind::Warning => {
                let toast_style = style::Style::default()
                    .fg(style::Color::LightRed)
                    .add_modifier(style::Modifier::BOLD);
                let toast_title = text::Span::styled("Warning", toast_style);
                (toast_title, toast_style)
            }
        };

        // Setup widget block
        let block = widgets::Block::bordered()
            .title(toast_title)
            .style(toast_style);

        // Define the paragraph to be rendered
        let paragraph = widgets::Paragraph::new(self.toast.message)
            .wrap(widgets::Wrap { trim: true });

        // Clear/reset a certain area to allow overdrawing of toast.
        widgets::Clear.render(top_right, buf);

        // Render the widget block
        paragraph.block(block).render(top_right, buf);
    }
}
