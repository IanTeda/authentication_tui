//-- ./src/terminal.rs

// #![allow(unused)] // For beginning only.

//! Module for handling setting up the backend terminal secondary buffer, going
//! into Raw mode to process keys immediately without having to wait for a newline
//! so that the keys are not echoed to the user’s screen when pressed and on quitting
//! restoring the terminal.

use std::io;

pub struct Terminal {
    /// Backend terminal used to render the TUI
    pub backend: ratatui::Terminal<ratatui::backend::CrosstermBackend<io::Stdout>>,

    /// Enable mouse events in backend terminal
    mouse_enabled: bool,

    /// Enable paste events in backend terminal
    paste_enabled: bool,
}

impl Terminal {
    pub fn new() -> io::Result<Terminal> {
        // Construct a new Ratatui terminal instance
        let terminal =
            ratatui::Terminal::new(ratatui::backend::CrosstermBackend::new(io::stdout()))?;

        // Default mouse enable is false
        let mouse = false;

        // Default paste enable is false
        let paste = false;

        Ok(Self {
            backend: terminal,
            mouse_enabled: mouse,
            paste_enabled: paste,
        })
    }

    pub fn enable_mouse(mut self, mouse: bool) -> Self {
        self.mouse_enabled = mouse;
        self
    }

    pub fn enable_paste(mut self, paste: bool) -> Self {
        self.paste_enabled = paste;
        self
    }

    pub fn enter(&mut self) -> io::Result<()> {
        // Enable terminal raw mode, which turns off input and output processing by
        // the terminal. This gives the TUI application control over when to print
        // characters to the screen.
        crossterm::terminal::enable_raw_mode()?;

        // Enter into the alternate screen, which is a secondary screen that allows
        // the TUI application to render whatever it needs to, without disturbing
        // the normal output of terminal apps in the shell.
        crossterm::execute!(io::stdout(), crossterm::terminal::EnterAlternateScreen, crossterm::cursor::Hide)?;

        // Enable mouse event capture
        if self.mouse_enabled {
            crossterm::execute!(io::stdout(), crossterm::event::EnableMouseCapture)?;
        }

        // Enable paste event capture
        if self.paste_enabled {
            crossterm::execute!(io::stdout(), crossterm::event::EnableBracketedPaste)?;
        }

        // Clear the terminal window
        self.backend.clear()?;

        Ok(())
    }

    pub fn restore(mut self) -> io::Result<()> {
        // Check if raw crossterm backend raw mode is enabled
        if crossterm::terminal::is_raw_mode_enabled()? {
            
            // Flush the terminal backend
            self.backend.flush()?;

            // Disable terminal backend mouse event capture
            if self.mouse_enabled {
                crossterm::execute!(io::stdout(), crossterm::event::DisableMouseCapture)?;
            }

            // Disable terminal backend paste event capture
            if self.paste_enabled {
                crossterm::execute!(io::stdout(), crossterm::event::DisableBracketedPaste)?;
            }

            // Exit backend terminal alternate screen mode and show cursor
            crossterm::execute!(io::stdout(), crossterm::terminal::LeaveAlternateScreen, crossterm::cursor::Show)?;

            // Disable backend terminal raw mode
            crossterm::terminal::disable_raw_mode()?;
        }

        Ok(())
    }
}
