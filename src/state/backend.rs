//-- ./src/app/backend.rs

//! Application model for the Backend state
//! ---

use chrono::{DateTime, Utc};

/// Backend state model
#[derive(Debug, Clone, PartialEq)]
pub struct Backend {
    /// Access token returned during login
    pub access_token: Option<String>,

    /// When was the access token received
    pub access_on: Option<DateTime<Utc>>,

    /// Refresh token (session) return by the backend
    pub refresh_token: Option<String>,

    /// When was the refresh token received
    pub refresh_on: Option<DateTime<Utc>>,

    /// Is the backend online
    pub is_online: bool,

    /// When was the backed last checked for being online
    pub is_online_checked: Option<DateTime<Utc>>,

    /// Are we logged into the back end
    pub is_logged_in: bool,
}

impl Default for Backend {
    /// Default instance of Backend state
    fn default() -> Self {
        let access_token = None;
        let access_on = None;
        let refresh_token = None;
        let refresh_on = None;
        let is_online = false;
        let is_online_checked = None;
        let is_logged_in = false;

        Self { 
            access_token, 
            access_on, 
            refresh_token, 
            refresh_on, 
            is_online, 
            is_online_checked,
            is_logged_in,
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
    fn confirm_default_tokens() -> Result<()> {
        let default_token = Backend::default();

        assert_eq!(default_token.access_token, None);
        assert_eq!(default_token.access_on, None);
        assert_eq!(default_token.refresh_token, None);
        assert_eq!(default_token.refresh_on, None);
        assert!(!default_token.is_online);
        assert_eq!(default_token.is_online_checked, None);
        assert!(!default_token.is_logged_in);

        Ok(())
    }
}