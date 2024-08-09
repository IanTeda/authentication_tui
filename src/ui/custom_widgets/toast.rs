//-- ./src/widgets/toast.rs

// #![allow(unused)] // For beginning only.

//! A widget for displaying notifications
//!
//! #### References
//!
//! * https://github.com/kaixinbaba/hg-tui/blob/main/src/widget/popup.rs
//! ---

// use ratatui::prelude::*;

use ratatui::{
    buffer::Buffer,
    layout, style, text,
    widgets::{self, Widget},
};

use crate::{states, ui::helpers};
// TODO: what goes into this struct and should we new() it?
#[derive(Debug, Clone, PartialEq)]
pub struct Toast {}

impl widgets::StatefulWidget for Toast {
    type State = states::Toast;

    /// Render the Toast
    fn render(self, area: layout::Rect, buf: &mut Buffer, state: &mut Self::State) {
        // Calculate widget layout area / position
        let widget_area = helpers::top_right(40, 4, area);

        // Clear/reset a certain area to allow overdrawing of toast.
        widgets::Clear.render(widget_area, buf);

        let (toast_title, toast_style) = match &state.kind {
            states::ToastKinds::Error => {
                let toast_style = style::Style::default()
                    .fg(style::Color::Red)
                    .add_modifier(style::Modifier::BOLD);
                let toast_title = text::Span::styled("Error", toast_style);
                (toast_title, toast_style)
            }
            states::ToastKinds::Info => {
                let toast_style = style::Style::default()
                    .fg(style::Color::LightBlue)
                    .add_modifier(style::Modifier::BOLD);
                let toast_title = text::Span::styled("Info", toast_style);
                (toast_title, toast_style)
            }
            states::ToastKinds::Notification => {
                let toast_style = style::Style::default()
                    .fg(style::Color::White)
                    .add_modifier(style::Modifier::BOLD);
                let toast_title = text::Span::styled("Notification", toast_style);
                (toast_title, toast_style)
            }
            states::ToastKinds::Success => {
                let toast_style = style::Style::default()
                    .fg(style::Color::LightGreen)
                    .add_modifier(style::Modifier::BOLD);
                let toast_title = text::Span::styled("Success", toast_style);
                (toast_title, toast_style)
            }
            states::ToastKinds::Warning => {
                let toast_style = style::Style::default()
                    .fg(style::Color::LightRed)
                    .add_modifier(style::Modifier::BOLD);
                let toast_title = text::Span::styled("Warning", toast_style);
                (toast_title, toast_style)
            }
        };

        // Get the toast message from the widget state
        let toast_message = state.message.clone();

        // Setup widget block
        let block = widgets::Block::bordered()
            .title(toast_title)
            .style(toast_style);

        // let message = format!("Toast message: {:?}", state);
        let paragraph =
            widgets::Paragraph::new(toast_message).wrap(widgets::Wrap { trim: true });

        // Render the widget block
        paragraph.block(block).render(widget_area, buf);
    }
}
