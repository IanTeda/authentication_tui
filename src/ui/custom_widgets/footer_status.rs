//-- ./src/widgets/status.rs

// #![allow(unused)] // For beginning only.

//! A widget for displaying backend server status in the footer
//!
//! #### References
//!
//! * https://github.com/kaixinbaba/hg-tui/blob/main/src/widget/popup.rs
//! ---

// use ratatui::prelude::*;

use ratatui::{
    buffer::Buffer,
    layout, style,
    widgets::{self, Widget},
};

use crate::states;

// #[derive(Debug, Clone, Copy)]
// struct Theme {
//     text: style::Color,
//     background: style::Color,
//     highlight: style::Color,
//     shadow: style::Color,
// }

// const RED: Theme = Theme {
//     text: style::Color::Rgb(48, 16, 16),
//     background: style::Color::Rgb(144, 48, 48),
//     highlight: style::Color::Rgb(192, 64, 64),
//     shadow: style::Color::Rgb(96, 32, 32),
// };

// const GREEN: Theme = Theme {
//     text: style::Color::Rgb(16, 48, 16),
//     background: style::Color::Rgb(48, 144, 48),
//     highlight: style::Color::Rgb(64, 192, 64),
//     shadow: style::Color::Rgb(32, 96, 32),
// };

#[derive(Debug, Clone, PartialEq, Default)]
pub struct FooterStatus {}

// impl Default for FooterStatus {
//     fn default() -> Self {
//         Self {}
//     }
// }

impl widgets::StatefulWidget for FooterStatus {
    type State = states::Backend;

    fn render(self, area: layout::Rect, buf: &mut Buffer, state: &mut Self::State) {
        // Set backend server status and style
        let (status, style) = match state.is_online {
            true => {
                let status = "Online";
                let style = style::Style::default()
                    .fg(style::Color::LightGreen)
                    .add_modifier(style::Modifier::BOLD);
                (status, style)
            },
            false => {
                let status: &str = "Offline";
                let style = style::Style::default()
                    .fg(style::Color::LightRed)
                    .add_modifier(style::Modifier::BOLD);
                (status, style)
            }
        };

        let block = widgets::Block::new();

        let paragraph = widgets::Paragraph::new(status).centered().style(style);

        paragraph.block(block).render(area, buf);
    }
}
