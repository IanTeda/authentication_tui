//-- ./src/widgets/toast.rs

#![allow(unused)] // For beginning only.

//! A widget for displaying notifications
//!
//! #### References
//!
//! * https://github.com/kaixinbaba/hg-tui/blob/main/src/widget/popup.rs
//! ---

use ratatui::{
    buffer::Buffer,
    layout,
    widgets::{self, Widget},
};

use crate::state;

// TODO: what goes into this struct and should we new() it?
#[derive(Debug, Clone, PartialEq)]
pub struct Toast {}

impl widgets::StatefulWidget for Toast {
    type State = state::Toast;

    /// Render the Toast
    fn render(self, area: layout::Rect, buf: &mut Buffer, state: &mut Self::State) {
        // Calculate widget layout area / position
        let widget_area = top_right(40, 5, area);

        // Clear/reset a certain area to allow overdrawing of toast.
        widgets::Clear.render(widget_area, buf);

        let toast_title = state.title.clone();
        let toast_message = state.message.clone();

        // Setup widget block
        let block = widgets::Block::bordered().title(toast_title);

        // let message = format!("Toast message: {:?}", state);
        let paragraph = widgets::Paragraph::new(toast_message);

        paragraph.block(block).render(widget_area, buf);
    }
}

fn top_right(width: u16, height: u16, area: layout::Rect) -> layout::Rect {
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
    // unimplemented!()
}
