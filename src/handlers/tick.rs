//-- ./src/handlers/tick.rs

// #![allow(unused)] // For beginning only.

//! Application tick events
//! 
//! The frequency of this event is set in config and triggered in CrosstermEventsHandler
//! ---

use std::time;

use crate::state;

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
        // Calculate the current tick rate, based on the last tick time
        self.calculate_tick_rate();

        // Update the application tick rate state
        state.app.ticks_per_second = self.ticks_per_second;

    }
}