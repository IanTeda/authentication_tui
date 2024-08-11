//-- ./src/ui/render.rs

#![allow(unused)] // For development only

//! Renders the widgets / UI
//!
//! ## References
//!
//! * [Rebels in the Sky](https://github.com/ricott1/rebels-in-the-sky)

use ratatui::{
    layout::{self, Constraint, Layout},
    widgets::{self, Block, Borders, Paragraph},
    Frame,
};

use tracing_subscriber::fmt::format;

use crate::states;

use super::custom_widgets;

/// Renders the user interface widgets.
pub fn render(app: &mut states::App, frame: &mut Frame) {
    // Get the terminal window area
    let container = frame.area();

    // Split the terminal window (container) into body and footer rectangles
    let (body, footer) = {
        let split = Layout::vertical([
            Constraint::Min(6),    // body
            Constraint::Length(1), //footer
        ])
        .split(container);
        (split[0], split[1])
    };

    // Render the body rectangle
    frame.render_widget(
        Paragraph::new("Top").block(Block::new().borders(Borders::ALL)),
        body,
    );

    // Split up the footer rectangle
    let (footer_left, status_area) = {
        let split = Layout::horizontal([
            Constraint::Min(24), // Left
            Constraint::Length(12), // Right
        ])
        .split(footer);
        (split[0], split[1])
    };

    frame.render_widget(
        Paragraph::new("Footer Left").block(Block::new()),
        footer_left,
    );

    let footer_status_widget = custom_widgets::FooterStatus::default();
    frame.render_stateful_widget(footer_status_widget, status_area, &mut app.backend);

    // Render the toast notification if show is true
    if app.toast.show {
        frame.render_stateful_widget(
            custom_widgets::Toast {},
            container,
            &mut app.toast,
        );
    }
}
