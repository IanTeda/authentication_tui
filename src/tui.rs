//-- ./src/tui.rs

//! Initialises/exits the terminal interface
//! ---

use crate::app::{App, AppResult};
use crate::event::EventHandler;
use crate::ui;
use color_eyre::config::HookBuilder;
use color_eyre::eyre;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::Backend;
use ratatui::Terminal;
use std::io;
use std::panic;
use tracing::instrument;

/// Representation of a terminal user interface.
///
/// It is responsible for setting up the terminal,
/// initialising the interface and handling the draw events.
#[derive(Debug)]
pub struct Tui<B: Backend> {
    /// Interface to the Terminal.
    terminal: Terminal<B>,
    /// Terminal event handler.
    pub events: EventHandler,
}

impl<B: Backend> Tui<B> {
    /// Constructs a new instance of [`Tui`].
    pub fn new(terminal: Terminal<B>, events: EventHandler) -> Self {
        Self { terminal, events }
    }

    /// Initialises the terminal interface.
    ///
    /// It enables the raw mode and sets terminal properties.
    pub fn init(&mut self) -> AppResult<()> {
        terminal::enable_raw_mode()?;
        crossterm::execute!(io::stderr(), EnterAlternateScreen, EnableMouseCapture)?;

        // Define a custom panic hook to reset the terminal properties.
        // This way, you won't have your terminal messed up if an unexpected error happens.

        // Build an Eyre panic hook
        // https://github.com/eyre-rs/color-eyre
        let (panic, error) = HookBuilder::default()
            .panic_section(format!(
                "This is a bug. Consider reporting it at {}",
                env!("CARGO_PKG_REPOSITORY")
            ))
            // .capture_span_trace_by_default(false)
            // .display_location_section(false)
            // .display_env_section(false)
            .into_hooks();

        let panic = panic.into_panic_hook();

        let error = error.into_eyre_hook();

        panic::set_hook(Box::new(move |info| {
            Self::reset().expect("failed to reset the terminal");
            panic(info);
        }));

        eyre::set_hook(Box::new(move |e| {
            Self::reset().expect("failed to reset the terminal");
            error(e)
        }))?;

        self.terminal.hide_cursor()?;
        self.terminal.clear()?;
        Ok(())
    }

    /// [`Draw`] the terminal interface by [`rendering`] the widgets.
    ///
    /// [`Draw`]: ratatui::Terminal::draw
    /// [`rendering`]: crate::ui::render
    pub fn draw(&mut self, app: &mut App) -> AppResult<()> {
        self.terminal.draw(|frame| ui::render(app, frame))?;
        Ok(())
    }

    /// Resets the terminal interface.
    ///
    /// This function is also used for the panic hook to revert
    /// the terminal properties if unexpected errors occur.
    #[instrument]
    fn reset() -> AppResult<()> {
        terminal::disable_raw_mode()?;
        crossterm::execute!(
            io::stderr(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        Ok(())
    }

    /// Exits the terminal interface.
    ///
    /// It disables the raw mode and reverts back the terminal properties.
    pub fn exit(&mut self) -> AppResult<()> {
        Self::reset()?;
        self.terminal.show_cursor()?;
        Ok(())
    }
}
