//-- ./src/ui/container.rs

// #![allow(unused)] // For beginning only.

//! The toast component
//!
//!
//! - Diff OSs: https://stackoverflow.com/questions/55583503/is-stdtimeduration-as-precise-as-timeprecise-time-ns-from-time-crate
//! ---

use std::{collections::VecDeque, time};

use crossterm::event as crossterm;
use ratatui::prelude::*;

use crate::{domain, handlers, prelude::*, ui};

// #[derive(Default)]
pub struct ToastComponent {
    /// Toast queue to store messages as they are triggered
    queue: VecDeque<domain::Toast>,

    /// The toast message being displayed. It can be none, so it is within an Option
    toast: Option<domain::Toast>,
}

impl Default for ToastComponent {
    /// When creating a default Toast component use these values
    fn default() -> Self {
        let toast = None;
        let queue = VecDeque::new();

        Self { queue, toast }
    }
}

impl ToastComponent {
    pub fn new() -> Self {
        Self::default()
    }
}

const TOAST_DURATION: time::Duration = time::Duration::from_secs(3);

impl ui::Component for ToastComponent {
    fn update(
        &mut self,
        action: handlers::Action,
    ) -> Result<Option<handlers::Action>> {
        match action {
            // Do something when we receive a tick action
            handlers::Action::Tick => {
                //-- Add any logic here that should run every tick interval

                // If we have an optional toast message do something
                if let Some(ref mut t) = self.toast {
                    // If toast duration is exceeded set option to None, to display
                    // the next toast message in the queue.
                    if t.shown_at.elapsed() > TOAST_DURATION {
                        self.toast = None;
                    }

                // Else if we have None optional toast and there is something in
                // the toast queue pop it into the optional toast
                } else if let Some(mut t) = self.queue.pop_front() {
                    t.shown_at = time::Instant::now();
                    self.toast = Some(t);
                }
            }

            // Do something when we receive a render action
            handlers::Action::Render => {
                //-- Add any logic here that should run on every render interval
            }

            // Do something when we receive a toast action
            handlers::Action::Toast(toast) => {
                // Add toast message to queue
                self.queue.push_back(toast);
            }

            // Do something when we receive a clear toast action
            handlers::Action::ClearToast => {
                // Set the optional toast to None
                self.toast = None;
            }

            // All other actions do nothing
            _ => {}
        }
        Ok(None)
    }

    //TODO: Return doesn't need to be an option if you match all cases
    fn handle_key_event(
        &mut self,
        key_event: crossterm::KeyEvent,
    ) -> Result<Option<handlers::Action>> {
        let action = match key_event.code {
            // crossterm::KeyCode::Char('t') => {
            //     // Build toast instance
            //     let toasty = domain::Toast::new("This a toast message".to_string())
            //         .kind(domain::ToastKind::Error);

            //     // Return action for update
            //     handlers::Action::Toast(toasty)
            // }
            crossterm::KeyCode::Esc => handlers::Action::ClearToast,
            _ => handlers::Action::Nil,
        };

        Ok(Some(action))
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        // Mutate into the toast option
        // https://stackoverflow.com/questions/27361350/calling-a-method-on-a-value-inside-a-mutable-option
        if let Some(ref mut t) = self.toast {
            let toast_widget = ui::custom_widgets::ToastWidget { toast: t.clone() };
            frame.render_widget(toast_widget, area)
        }

        Ok(())
    }
}
