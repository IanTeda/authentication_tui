//-- ./src/domain/event.rs

//! Application event types
//! ---

/// TUI Application events.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Event {
    Closed,
    Error,
    FocusGained,
    FocusLost,
    Init,
    Key(crossterm::event::KeyEvent),
    Mouse(crossterm::event::MouseEvent),
    Paste(String),
    Quit,
    Render,
    Resize(u16, u16),
    Tick,
}