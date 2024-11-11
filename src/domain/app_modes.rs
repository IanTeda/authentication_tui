use core::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum AppMode {
    Normal,
    Input,
}

/// Imply default backend status
impl Default for AppMode {
    fn default() -> Self {
        AppMode::Normal
    }
}

impl fmt::Display for AppMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppMode::Normal => write!(f, "Normal"),
            AppMode::Input => write!(f, "Input"),
        }
    }
}
