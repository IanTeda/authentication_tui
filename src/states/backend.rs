//-- ./src/app/backend.rs

//! Backend server model for the Backend state
//! ---

use std::{net, time};

/// Backend state model
#[derive(Debug, Clone, PartialEq)]
pub struct Backend {
    /// The http address with port of the backend server to connect to
    pub address: net::SocketAddr,

    /// Access token returned during login
    pub access_token: Option<String>,

    /// When was the access token received
    pub access_on: Option<time::Instant>,

    /// Refresh token (session) return by the backend
    pub refresh_token: Option<String>,

    /// When was the refresh token received
    pub refresh_on: Option<time::Instant>,

    /// Is the backend online
    pub is_online: bool,

    /// When was the backend last checked for being online
    pub status_checked_on: Option<time::Instant>,

    /// Are we logged into the back end
    pub is_logged_in: bool,
}

impl Backend {
    pub fn new(address: net::SocketAddr) -> Self {

        let access_token = None;
        let access_on = None;
        let refresh_token = None;
        let refresh_on = None;
        let is_online = false;
        let status_checked_on = None;
        let is_logged_in = false;

        Self { 
            address, 
            access_token, 
            access_on, 
            refresh_token, 
            refresh_on, 
            is_online, 
            status_checked_on, 
            is_logged_in,
        }
    }
}

#[cfg(test)]
mod tests {
    // #![allow(unused)] // For development only

    use fake::Fake;
    use fake::faker::internet::en::IPv4;
    use rand::prelude::*;

    // Bring current module into scope
    use super::*;

    // Override with more flexible error
    pub type Result<T> = core::result::Result<T, Error>;
    pub type Error = Box<dyn std::error::Error>;

    #[test]
    fn confirm_default_tokens() -> Result<()> {
        //-- Setup and Fixtures (Arrange)
        let random_ip: net::Ipv4Addr = IPv4().fake();
        let random_port = rand::thread_rng().gen_range(0..1000);
        let random_socket = net::SocketAddr::new(net::IpAddr::V4(random_ip), random_port);

        //-- Execute Function (Act)
        let backend_state = Backend::new(random_socket);

        //-- Checks (Assertions)
        assert_eq!(backend_state.address, random_socket);
        assert_eq!(backend_state.access_token, None);
        assert_eq!(backend_state.access_on, None);
        assert_eq!(backend_state.refresh_token, None);
        assert_eq!(backend_state.refresh_on, None);
        assert!(!backend_state.is_online);
        assert_eq!(backend_state.status_checked_on, None);
        assert!(!backend_state.is_logged_in);

        // -- Return
        Ok(())
    }
}