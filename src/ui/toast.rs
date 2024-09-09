//-- ./src/ui/container.rs

// #![allow(unused)] // For beginning only.

//! The toast component
//! 
//! 
//! - Diff OSs: https://stackoverflow.com/questions/55583503/is-stdtimeduration-as-precise-as-timeprecise-time-ns-from-time-crate
//! ---

use std::time;

use crossterm::event as crossterm;
use ratatui::prelude::*;

use crate::{domain, handlers, prelude::*, ui};

// #[derive(Default)]
pub struct ToastComponent {
    // The toast message being displayed. It can be none, so it is within an Option
    toast: Option<domain::Toast>,
}

impl Default for ToastComponent {
    /// When creating a default Toast component use these values
    fn default() -> Self {
        let toast = None;

        Self { toast }
    }
}

impl ToastComponent {
    pub fn new() -> Self {
        Self::default()
    }
}

impl ui::Component for ToastComponent {
    fn update(
        &mut self,
        action: handlers::Action,
    ) -> Result<Option<handlers::Action>> {
        match action {
            // Do something when we receive a tick action
            handlers::Action::Tick => {
                // Add any logic here that should run every tick interval
            }
            // Do something when we receive a render action
            handlers::Action::Render => {
                // Add any logic here that should run on every render interval
            }
            // Do something when we receive a toast action
            handlers::Action::Toast(toast) => {
                // Add toast message
                self.toast = Some(toast);
            }
            handlers::Action::ClearToast => {
                // Mutate inside the toast option
                // https://stackoverflow.com/questions/27361350/calling-a-method-on-a-value-inside-a-mutable-option
                if let Some(ref mut t) = self.toast {
                    t.show = false;
                }
            }
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
            crossterm::KeyCode::Char('t') => {
                // Build toast instance
                let toasty = domain::Toast::new("This a toast message".to_string())
                    .kind(domain::ToastKind::Error);

                // Return action for update
                handlers::Action::Toast(toasty)
            }
            crossterm::KeyCode::Esc => handlers::Action::ClearToast,
            _ => handlers::Action::Nil,
        };

        Ok(Some(action))
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        if let Some(ref mut t) = self.toast {
            let three_seconds = time::Duration::from_secs(3);
            if t.show && t.shown_at.elapsed() < three_seconds {
                let toast_widget = ui::custom_widgets::ToastWidget { 
                    toast: t.clone(), 
                };
                frame.render_widget(toast_widget, area)
            }
        }

        Ok(())
    }
}
