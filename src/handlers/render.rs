//-- ./src/handlers/render.rs

// #![allow(unused)] // For beginning only.

//! Application frame render events
//!
//! The frequency of this event is set in config and triggered in CrosstermEventsHandler
//! ---

use std::time;

use tokio::sync::mpsc;

use crate::{domain, prelude::*, state, ui, Terminal};

/// Keep track of the render event cycles
#[derive(Debug, Clone)]
pub struct RenderEventHandler {
    /// Application configuration
    config: crate::config::Config,

    /// Action sender
    #[allow(unused)]
    action_sender: mpsc::UnboundedSender<domain::Action>,

    /// Time since last frame
    last_frame_update: time::Instant,

    /// The number of frames
    frame_count: u32,

    /// Rolling frames per second calculation
    frames_per_second: f64,
}

impl RenderEventHandler {
    /// New TickEventHandler instance
    pub fn init(
        config: crate::config::Config,
        action_sender: mpsc::UnboundedSender<domain::Action>,
    ) -> Self {
        let last_frame_update = time::Instant::now();
        let frame_count = 0;
        let frames_per_second = 0.0;
        Self {
            config,
            action_sender,
            last_frame_update,
            frame_count,
            frames_per_second,
        }
    }

    /// What to do each render event cycle
    pub fn handle_event(
        &mut self,
        state: &mut state::State,
        terminal: &mut Terminal,
    ) {
        // Calculate the frame rate based on the last render event
        self.calculate_frame_rate();

        // Update the application state
        state.app.frames_per_second = self.frames_per_second;

        let _ = self.render_tui(state, terminal);
    }

    /// Update frame rate per second
    fn calculate_frame_rate(&mut self) {
        // Increment the frame count by 1
        self.frame_count += 1;

        // Represent now as an Instance
        let now = time::Instant::now();

        // Calculate elapsed time since the last frame
        let elapsed = (now - self.last_frame_update).as_secs_f64();

        // if the last frame elapsed time is grater than up update the component state
        if elapsed >= 1.0 {
            self.frames_per_second = self.frame_count as f64 / elapsed;
            self.last_frame_update = now;
            self.frame_count = 0;
        }
    }

    /// Return the current application frame rate
    pub fn frame_rate(self) -> f64 {
        self.frames_per_second
    }

    /// Render the Terminal User Interface (TUI)
    fn render_tui(
        &mut self,
        state: &mut crate::state::State,
        terminal: &mut Terminal,
    ) -> Result<()> {
        terminal.draw(|frame| {
            ui::layout::render(self.config.clone(), state, frame)
        })?;

        Ok(())
    }
}
