//-- ./src/ui/custom_widgets/statistics.rs

// #![allow(unused)] // For beginning only.

//! A custom widget for displaying application statistics
//! ---

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Style, Stylize},
    text::Span,
    widgets::{Paragraph, Widget},
};

/// A struct to hold the application statistics
pub struct StatisticsWidget {
    pub ticks_per_second: f64,
    pub frames_per_second: f64,
}

impl StatisticsWidget {
    /// Update the application statistics to show
    pub fn update(ticks_per_second: f64, frames_per_second: f64) -> Self {
        Self {
            ticks_per_second,
            frames_per_second,
        }
    }
}

impl Widget for StatisticsWidget {
    /// [Required] Render the custom widget using the assigned area and terminal buffer
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Calculate top right
        let [top, _] = Layout::vertical([Constraint::Length(1), Constraint::Min(0)])
            .areas(area);

        // Construct fps message
        let message = format!(
            "{:.2} ticks/sec, {:.2} frames/sec",
            self.ticks_per_second, self.frames_per_second
        );

        // Construct the text span
        let span = Span::styled(message, Style::new().dim());

        // Construct the paragraph that holds the span
        let paragraph = Paragraph::new(span).right_aligned();

        // Render the constructed paragraph
        paragraph.render(top, buf);
    }
}
