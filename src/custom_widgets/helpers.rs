//-- ./src/custom_widgets/utils.rs

// #![allow(unused)] // For beginning only.

//! Collection of widget utilities

use ratatui::layout;

/// Position the widget at the top right corner
pub fn top_right(width: u16, height: u16, area: layout::Rect) -> layout::Rect {
    let toast_layout = layout::Layout::vertical([
        layout::Constraint::Length(1),
        layout::Constraint::Length(height),
        layout::Constraint::Fill(1),
    ])
    .split(area);
    
    layout::Layout::horizontal([
        layout::Constraint::Fill(1),
        layout::Constraint::Length(width),
        layout::Constraint::Length(1)
    ])
    .split(toast_layout[1])[1]
}

/// Center rect using up certain percentage of the available rect `r`
pub fn centered_rect(percent_x: u16, percent_y: u16, r: layout::Rect) -> layout::Rect {
    let popup_layout = layout::Layout::vertical([
        layout::Constraint::Percentage((100 - percent_y) / 2),
        layout::Constraint::Percentage(percent_y),
        layout::Constraint::Percentage((100 - percent_y) / 2),
    ])
    .split(r);

    layout::Layout::horizontal([
        layout::Constraint::Percentage((100 - percent_x) / 2),
        layout::Constraint::Percentage(percent_x),
        layout::Constraint::Percentage((100 - percent_x) / 2),
    ])
    .split(popup_layout[1])[1]
}