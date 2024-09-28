//-- ./src/ui/layout.rs

#![allow(unused)] // For beginning only.

//! The main TUI Layout module
//! ---

use ratatui::{layout, widgets};

use crate::{state, Config};

use super::custom_widgets;

pub fn render(config: Config, state: &mut state::State, frame: &mut ratatui::Frame) {
    // Get the terminal window area
    let terminal_area = frame.area();

    // Split the terminal window into a body and a footer rectangle
    let (body, footer) = {
        let split = layout::Layout::vertical([
            layout::Constraint::Min(6),    // body
            layout::Constraint::Length(1), //footer
        ])
        .split(terminal_area);
        (split[0], split[1])
    };

    // Render the body rectangle
    frame.render_widget(
        widgets::Paragraph::new("Top")
            .block(widgets::Block::new().borders(widgets::Borders::ALL)),
        body,
    );

    // Split up the footer rectangle
    let (footer_left, status_area) = {
        let split = layout::Layout::horizontal([
            layout::Constraint::Min(24),    // Left
            layout::Constraint::Length(12), // Right
        ])
        .split(footer);
        (split[0], split[1])
    };

    frame.render_widget(
        widgets::Paragraph::new("Footer Left").block(widgets::Block::new()),
        footer_left,
    );

    //-- Render statistics widget if set in the config file
    // Construct the custom statistics widget
    let statistics_widget =
        custom_widgets::StatisticsWidget::update(state.app.ticks_per_second, state.app.frames_per_second);
    
    // Render the statistics widget. I need to go last as I use the terminal area
    if config.app.show_statistics {
        frame.render_widget(statistics_widget, terminal_area);
    }

    // let status_widget = custom_widgets::StatusWidget::init(state.backend.is_online);
    // frame.render_widget(status_widget, status_area);
}
