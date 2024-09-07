//-- ./src/ui/container.rs

//! The toast component
//! ---

use std::time;

use crossterm::event as crossterm;
use ratatui::prelude::*;

use crate::{domain, handlers, prelude::*, ui};

// #[derive(Default)]
pub struct Toast {
    /// Type of toast
    pub kind: domain::ToastKind,

    /// When true UI will draw next refresh
    pub show: bool,

    /// Message of the popup
    pub message: String,

    /// Auto hide the toast message
    pub auto_hide: bool,

    /// When the toast notification was shown
    pub displayed_on: time::Instant,
}

impl Default for Toast {
    fn default() -> Self {
        let kind = domain::ToastKind::default();
        let show = false;
        let message = "I am a toast message".to_string();
        let auto_hide = true;
        let displayed_on = time::Instant::now();

        Self {
            kind,
            show,
            message,
            auto_hide,
            displayed_on,
        }
    }
}

impl Toast {
    pub fn new() -> Self {
        Self::default()
    }
}

impl ui::Component for Toast {
    fn update(
        &mut self,
        action: handlers::Action,
    ) -> Result<Option<handlers::Action>> {
        match action {
            handlers::Action::Tick => {
                // if self.show {
                //     println!("Show toasty")
                // }
            }
            handlers::Action::Render => {
                // add any logic here that should run on every render
            }
            handlers::Action::Toast(toast) => {
                self.kind = toast.kind;
                self.message = toast.message;
                self.show = true;
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
                let toasty = domain::Toast {
                    kind: domain::ToastKind::Notification,
                    message: "This is a toast".to_string(),
                };

                // Return action for update
                handlers::Action::Toast(toasty)
            }
            crossterm::KeyCode::Esc => handlers::Action::Quit,
            _ => handlers::Action::Nil,
        };

        Ok(Some(action))
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {

        let toast = domain::Toast {
            kind: domain::ToastKind::Info,
            message: "Yea, toasty...".to_string(),
        };

        let toast_widget = ui::custom_widgets::Toast {
            toast,
            displayed_at: time::Instant::now(),
        };

        frame.render_widget(toast_widget, area);

        // // Calculate widget layout area / position
        // let top_right = ui::helpers::top_right(40, 4, area);

        // let toast_style = style::Style::default()
        //     .fg(style::Color::LightBlue)
        //     .add_modifier(style::Modifier::BOLD);

        // let toast_title = text::Span::styled("Info", toast_style);

        //         // Setup widget block
        // let block = widgets::Block::bordered()
        //     .title(toast_title)
        //     .style(toast_style);

        // // Format the message to show on screen
        // let message = "A toast to ratatui, cheers!".to_string();

        // // Style the text span
        // let span = Span::styled(message, Style::new().dim());
        
        // // Style the paragraph that holds ths text span
        // let paragraph = widgets::Paragraph::new(span).block(block);

        // // Clear/reset a certain area to allow overdrawing of toast.
        // widgets::Clear.render(top_right, frame.buffer_mut());

        // // Render the widget
        // frame.render_widget(paragraph, top_right);

        Ok(())
    }
}
