//-- ./src/ui/layout.rs

#![allow(unused)] // For beginning only.

//! The main TUI Layout module
//! ---

use ratatui::{layout, widgets};

use crate::{domain, state, Config, ui};

use super::custom_widgets;

pub fn render(config: Config, state: &mut state::State, frame: &mut ratatui::Frame) {
    // Get the terminal window area
    let terminal_area = frame.area();

    // Split the terminal window into a body and a footer rectangle
    let (body_area, footer_area) = {
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
        body_area,
    );

    //-- 2. Render the footer
    ui::footer::render(config.clone(), state, footer_area, frame);

    //-- 3. Render statistics widget if set in the config file
    // Construct the custom statistics widget
    let statistics_widget = custom_widgets::StatisticsWidget::update(
        state.app.ticks_per_second,
        state.app.frames_per_second,
    );

    // Render the statistics widget, if config is set true. I need to go last as 
    // I use the terminal area
    if config.app.show_statistics {
        frame.render_widget(statistics_widget, terminal_area);
    }

    //-- 4. Render toast message widget if there is a current one
    if let Some(ref mut current_toast) = state.toast.current {
        let toast_widget = custom_widgets::ToastWidget {
            toast: current_toast.clone(),
        };
        frame.render_widget(toast_widget, terminal_area)
    }
}
