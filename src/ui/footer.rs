//-- ./src/ui/footer.rs

#![allow(unused)] // For beginning only.

//! # Footer ui layout
//!
//! Contains the layout code for rendering the footer
//! ---

use ratatui::{layout, widgets};

use crate::{domain, state, ui, Config};

// pub struct FooterLayout {}

// impl FooterLayout {
//     pub fn new() -> Self {
//         Self
//     }

//     pub fn render(config: Config, state: &mut state::State, frame: &mut ratatui::Frame){

//     }
// }

pub fn render(
    config: Config,
    state: &state::State,
    area: layout::Rect,
    frame: &mut ratatui::Frame,
) {
    let (footer_left, status_area) = {
        let split = layout::Layout::horizontal([
            layout::Constraint::Min(24),    // Left
            layout::Constraint::Length(12), // Right
        ])
        .split(area);
        (split[0], split[1])
    };

    // frame.render_widget(
    //     widgets::Paragraph::new("Footer Left").block(widgets::Block::new()),
    //     footer_left,
    // );

    let status_app_mode_widget =
        ui::custom_widgets::StatusAppModeWidget::init(state.app.mode.clone());
    frame.render_widget(status_app_mode_widget, footer_left);

    let status_widget =
        ui::custom_widgets::StatusWidget::init(state.backend.status.clone());
    frame.render_widget(status_widget, status_area);
}
