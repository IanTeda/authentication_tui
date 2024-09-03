
use ratatui::{prelude::*, widgets};
use tokio::sync::mpsc::UnboundedSender;

use crate::{handlers, prelude::*, Config, ui};

#[derive(Default)]
pub struct Container {
    command_tx: Option<UnboundedSender<handlers::Action>>,
    config: Config,
}

impl Container {
    pub fn new() -> Self {
        Self::default()
    }
}

impl ui::Component for Container {
    fn register_action_handler(&mut self, tx: UnboundedSender<handlers::Action>) -> Result<()> {
        self.command_tx = Some(tx);
        Ok(())
    }

    fn register_config_handler(&mut self, config: Config) -> Result<()> {
        self.config = config;
        Ok(())
    }

    fn update(&mut self, action: handlers::Action) -> Result<Option<handlers::Action>> {
        match action {
            handlers::Action::Tick => {
                // add any logic here that should run on every tick
            }
            handlers::Action::Render => {
                // add any logic here that should run on every render
            }
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        // frame.render_widget(Paragraph::new("hello world"), area);
        //             let area = frame.area();
        frame.render_widget(
                widgets::Paragraph::new("Hello Ratatui! (press 'q' to quit)")
                    .white()
                    .on_blue(),
                area,
            );
        Ok(())
    }
}