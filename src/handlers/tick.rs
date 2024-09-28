//-- ./src/handlers/tick.rs

// #![allow(unused)] // For beginning only.

//! Application tick events
//! 
//! The frequency of this event is set in config and triggered in CrosstermEventsHandler
//! ---

use std::time;

use crate::state;

const TOAST_DURATION: time::Duration = time::Duration::from_secs(3);

#[derive(Debug, Clone, PartialEq)]
pub struct TickEventHandler {
    /// Time since last tick
    last_tick_update: time::Instant,

    /// The number of ticks
    tick_count: u32,

    /// Rolling ticks per second calculation
    ticks_per_second: f64,
}

// Implement default for idiomatic completeness
impl Default for TickEventHandler {
    /// Default TickEventHandler instance
    fn default() -> Self {
        Self::new()
    }
}

impl TickEventHandler {
    /// New TickEventHandler instance
    pub fn new() -> Self {
        Self {
            last_tick_update: time::Instant::now(),
            tick_count: 0,
            ticks_per_second: 0.0,
        }
    }

    /// Update tick rate information
    fn calculate_tick_rate(&mut self) {
        // Increment the tick count by one
        self.tick_count += 1;

        // Represent now as an Instance
        let now = time::Instant::now();

        // Calculate elapsed time since last tick
        let elapsed = (now - self.last_tick_update).as_secs_f64();

        // If elapsed time is grater than one second update tick state
        if elapsed >= 1.0 {
            self.ticks_per_second = self.tick_count as f64 / elapsed;
            self.last_tick_update = now;
            self.tick_count = 0;
        }
    }

    /// Return the current application tick rate per second
    pub fn tick_rate(self) -> f64 {
        self.ticks_per_second
    }

    /// What to do each tick event cycle
    pub fn handle_event(&mut self, state: &mut state::State) {
        //-- 1. Calculate tick statistics
        // Calculate the current tick rate, based on the last tick time
        self.calculate_tick_rate();

        // Update the application tick rate state
        state.app.ticks_per_second = self.ticks_per_second;

        //-- 2. Manage toast messages
        // If we have an optional toast message wait for elapsed time to exceed
        if let Some(ref mut t) = state.toast.current {
            // If toast duration is exceeded set option to None, to display
            // the next toast message in the queue.
            if t.shown_at.elapsed() > TOAST_DURATION {
                state.toast.current = None;
            }
        // Else if we have None optional toast and there is something in
        // the toast queue pop it into the optional toast
        } else if let Some(mut t) = state.toast.queue.pop_front() {
            t.shown_at = time::Instant::now();
            state.toast.current = Some(t);
        }

    }
}