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

    let footer =
        Layout::horizontal([Constraint::Percentage(90), Constraint::Percentage(12)])
            .split(container[1]);

    frame.render_widget(
        Paragraph::new("Footer Left").block(Block::new().borders(Borders::ALL)),
        footer[0],
    );

    frame.render_widget(
        Paragraph::new(format!("Server Online: {}", app.backend_is_online))
            .block(Block::new().borders(Borders::ALL)),
        footer[1],
    );

    if app.popup.show {
        let block = Block::bordered().title(app.popup.title.clone());
        let area = centered_rect(60, 20, area);
        frame.render_widget(widgets::Clear, area); //this clears out the background
        let paragraph =
            Paragraph::new(app.popup.message.clone());
        let widget = paragraph.block(block).wrap(widgets::Wrap { trim: true });
        frame.render_widget(widget, area);
    }
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: layout::Rect) -> layout::Rect {
    let popup_layout = Layout::vertical([
        Constraint::Percentage((100 - percent_y) / 2),
        Constraint::Percentage(percent_y),
        Constraint::Percentage((100 - percent_y) / 2),
    ])
    .split(r);

    Layout::horizontal([
        Constraint::Percentage((100 - percent_x) / 2),
        Constraint::Percentage(percent_x),
        Constraint::Percentage((100 - percent_x) / 2),
    ])
    .split(popup_layout[1])[1]
}
