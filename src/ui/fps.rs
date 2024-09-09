//-- ./src/ui/fps.rs

//! A component for displaying frame and tick rates at the top right had corner
//! ---

use std::time::Instant;
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Style, Stylize},
    text::Span,
    widgets::Paragraph,
    Frame,
};

use crate::{ui, handlers, prelude::*};

/// Define component state
#[derive(Debug, Clone, PartialEq)]
pub struct FpsComponent {
    /// Time since last tick
    last_tick_update: Instant,

    /// The number of ticks
    tick_count: u32,

    /// Rolling ticks per second calculation
    ticks_per_second: f64,

    /// Time since last frame
    last_frame_update: Instant,

    /// The number of frames
    frame_count: u32,

    /// Rolling frames per second calculation
    frames_per_second: f64,
}

impl Default for FpsComponent {
    /// Default FPS Counter instance
    fn default() -> Self {
        Self::new()
    }
}

impl FpsComponent {
    /// New FPS Counter instance
    pub fn new() -> Self {
        Self {
            last_tick_update: Instant::now(),
            tick_count: 0,
            ticks_per_second: 0.0,
            last_frame_update: Instant::now(),
            frame_count: 0,
            frames_per_second: 0.0,
        }
    }

    /// Update tick rate information
    fn app_tick_rate(&mut self) -> Result<()> {
        // Increment the tick count by one
        self.tick_count += 1;
        
        // Represent now as an Instance
        let now = Instant::now();
        
        // Calculate elapsed time since last tick
        let elapsed = (now - self.last_tick_update).as_secs_f64();
        
        // If elapsed time is grater than one second update tick component state
        if elapsed >= 1.0 {
            self.ticks_per_second = self.tick_count as f64 / elapsed;
            self.last_tick_update = now;
            self.tick_count = 0;
        }
        Ok(())
    }

    /// Update frame rate information
    fn app_frame_rate(&mut self) -> Result<()> {
        // Increment the frame count by 1
        self.frame_count += 1;
        
        // Represent now as an Instance
        let now = Instant::now();
        
        // Calculate elapsed time since the last frame
        let elapsed = (now - self.last_frame_update).as_secs_f64();
        
        // if the last frame elapsed time is grater than up update the component state
        if elapsed >= 1.0 {
            self.frames_per_second = self.frame_count as f64 / elapsed;
            self.last_frame_update = now;
            self.frame_count = 0;
        }
        Ok(())
    }
}

impl ui::Component for FpsComponent {
    /// Update the state
    fn update(&mut self, action: handlers::Action) -> Result<Option<handlers::Action>> {
        match action {
            handlers::Action::Tick => self.app_tick_rate()?,
            handlers::Action::Render => self.app_frame_rate()?,
            _ => {}
        };
        Ok(None)
    }

    /// Draw the component
    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        // Set out the component position
        let [top, _] = Layout::vertical([Constraint::Length(1), Constraint::Min(0)]).areas(area);
        
        // Format the message to show on screen
        let message = format!(
            "{:.2} ticks/sec, {:.2} FPS",
            self.ticks_per_second, self.frames_per_second
        );

        // Style the text span
        let span = Span::styled(message, Style::new().dim());
        
        // Style the paragraph that holds ths text span
        let paragraph = Paragraph::new(span).right_aligned();

        // Render the widget
        frame.render_widget(paragraph, top);

        // Return ok
        Ok(())
    }
}