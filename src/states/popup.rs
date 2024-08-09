//-- ./src/app/popup.rs

// #![allow(unused)] // For beginning only.

//! Application model for the Popup state
//! ---

/// Popup state model
#[derive(Debug, Clone, PartialEq)]
pub struct Popup {
    /// When true UI will draw next refresh
    pub show: bool,

    /// Title of the popup
    pub title: String,

    /// Message of the popup
    pub message: String,
}

impl Default for Popup {
    /// Default instance of Popup state
    fn default() -> Self {
        Self {
            show: false,
            title: String::from("Popup Title"),
            message: String::from("Something to tell you."),
        }
    }
}

#[cfg(test)]
mod tests {
    // #![allow(unused)] // For development only

    // Bring current module into scope
    use super::*;

    // Override with more flexible error
    pub type Result<T> = core::result::Result<T, Error>;
    pub type Error = Box<dyn std::error::Error>;


    #[test]
    fn confirm_default_popup() -> Result<()> {
        let default_popup = Popup::default();

        assert!(!default_popup.show);
        assert_eq!(default_popup.title, String::from("Popup Title"));
        assert_eq!(default_popup.message, String::from("Something to tell you."));

        Ok(())
    }
}