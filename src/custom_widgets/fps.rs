use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Style, Stylize},
    text::Span,
    widgets::{Paragraph, Widget},
};

pub struct FpsWidget {
    pub ticks_per_second: f64,
    pub frames_per_second: f64,
}

impl FpsWidget {
    pub fn init(ticks_per_second: f64, frames_per_second: f64) -> Self {
        Self {
            ticks_per_second,
            frames_per_second,
        }
    }
}

impl Widget for FpsWidget {
    /// [Required] Render the custom widget using the assigned area and terminal buffer
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Calculate top right
        let [top, _] = Layout::vertical([Constraint::Length(1), Constraint::Min(0)])
            .areas(area);

        // Construct fps message
        let message = format!(
            "{:.2} ticks/sec, {:.2} FPS",
            self.ticks_per_second, self.frames_per_second
        );

        let span = Span::styled(message, Style::new().dim());

        let paragraph = Paragraph::new(span).right_aligned();

        // frame.render_widget(paragraph, top);
        // paragraph

        // Ok(())
    }
}
