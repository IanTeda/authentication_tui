//-- ./src/ui/render.rs

#![allow(unused)] // For development only

//! Renders the widgets / UI
//!
//! ## References
//!
//! * [Rebels in the Sky](https://github.com/ricott1/rebels-in-the-sky)

use ratatui::{
    layout::{Constraint, Layout}, widgets::{Block, Borders, Paragraph}, Frame
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    let area = frame.size();

    let container = Layout::vertical([
            Constraint::Min(6),    // body
            Constraint::Length(4), //footer
        ])
        .split(area);

    frame.render_widget(
        Paragraph::new("Top").block(Block::new().borders(Borders::ALL)),
        container[0],
    );

    let footer = Layout::horizontal([
        Constraint::Percentage(90),
        Constraint::Percentage(12),
        ])
    .split(container[1]);

    frame.render_widget(
        Paragraph::new("Footer Left").block(Block::new().borders(Borders::ALL)),
        footer[0],
    );

    frame.render_widget(
        Paragraph::new("Footer Right").block(Block::new().borders(Borders::ALL)),
        footer[1],
    );
}