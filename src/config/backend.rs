//-- ./src/config/backend.rs

// #![allow(unused)] // For development only

//! Authentication backend configuration module
//! ---

use std::net;

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct BackendConfig {
    /// IP address of the backend server
    pub ip: net::IpAddr,

    /// Port used by the backend server endpoints
    pub port: u16,

    /// Optional default email address
    pub default_email: Option<String>,
}

impl Default for BackendConfig {
    fn default() -> Self {
        let localhost = net::IpAddr::V4(net::Ipv4Addr::new(127, 0, 0, 1));
        let port = 8081;
        let default_email = Some(String::from("authentication@teda.id.au"));

        Self {
            ip: localhost,
            port,
            default_email,
        }
    }
}

impl BackendConfig {
    pub fn address(&self) -> net::SocketAddr {
        net::SocketAddr::new(self.ip, self.port)
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
    fn confirm_default_config_file() -> Result<()> {
        //-- Setup and Fixtures (Arrange)

        //-- Execute Function (Act)
        let default_config = BackendConfig::default();

        //-- Checks (Assertions)
        assert_eq!(default_config.ip.to_string(), "127.0.0.1");
        assert_eq!(default_config.port, 8081);
        assert_eq!(default_config.default_email.unwrap(), "authentication@teda.id.au");

        //-- Return
        Ok(())
    }
}