//-- ./src/config.rs

// #![allow(unused)] // For development only

//! Tui application configuration module
//! 
//! Parse the configuration file into a struct. If no configuration file exists
//! write a file using the Config struct default vales
//! ---

use std::{fs, net, path};

use crate::{app::AppResult, TuiError};

/// Tui application configuration
///
/// ## References
///
/// * [crates-tui/src/config.rs](https://github.com/ratatui-org/crates-tui/blob/main/src/config.rs)
/// ---
#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Tui {
    /// The directory to use for storing application configuration ( etc.).
    pub config_folder: path::PathBuf,

    /// The directory to use for storing application data (logs etc.).
    pub data_folder: path::PathBuf,

    /// The file used to store the application configuration
    pub config_file: path::PathBuf,
    // The log level the application is running on [error, warn, info, debug, trace]
    // #[serde_as(as = "NoneAsEmptyString")]
    // pub log_level: tracing::level_filters::LevelFilter,

    // pub tick_rate: f64,

    // pub frame_rate: f64,

    // pub key_refresh_rate: f64,

    // pub enable_mouse: bool,

    // pub enable_paste: bool,

    // pub prompt_padding: u16,

    // pub key_bindings: KeyBindings,

    // pub color: Base16Palette,
}

impl Default for Tui {
    /// Default initiation of the Tui config struct
    fn default() -> Self {
        let project_directory =
            directories::ProjectDirs::from("com", "ianteda", "authentication");

        let config_folder =
            project_directory.clone().unwrap().config_dir().to_owned();

        let data_folder = project_directory
            // .ok_or(Error::Static("Error reading Project Directory"))?
            .unwrap()
            .data_dir()
            .to_owned();

        let config_file = config_folder.join("config.toml");

        Self {
            config_folder,
            data_folder,
            config_file,
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Backend {
    /// IP address of the backend server
    pub ip: net::IpAddr,

    /// Port used by the backend server endpoints
    pub port: u16,

    /// Optional default email address
    pub default_email: Option<String>,
}

impl Default for Backend {
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

#[derive(Debug, Clone, PartialEq, Default, serde::Deserialize, serde::Serialize)]
pub struct Config {
    /// Configure the Tui application
    pub tui: Tui,

    /// Backend configuration
    pub backend: Backend,
}

impl Config {
    pub fn parse() -> AppResult<Self> {
        // Build config directory path
        let config_directory =
            directories::ProjectDirs::from("com", "ianteda", "authentication_tui")
                .ok_or(TuiError::Static(
                    "Failed to determine application configuration directory",
                ))?
                .config_dir()
                .to_path_buf();

        // Append config file to directory path
        let config_file = config_directory.join("config.toml");

        // If config file does not exist write the default config file
        if !config_file.exists() {
            tracing::info!(
                "Configuration file not found: {}",
                config_file.display()
            );

            write_default_config(&config_directory, &config_file)?;
        };

        // Build config by first adding config.toml and overwriting with
        // Environmental variables if they are present
        let config: Config = config::Config::builder()
            // Add config.toml as source
            .add_source(config::File::from(config_file))
            // -- Environmental variables
            // Add in settings from environment variables (with a prefix of BACKEND
            // and '_' as separator). E.g. `AUTHENTICATION_APPLICATION_PORT=5001 would
            // set `settings.application.port`
            .add_source(
                config::Environment::with_prefix("AUTHENTICATION_TUI")
                    .prefix_separator("_")
                    .separator("_"),
            )
            .build()?
            .try_deserialize()?;

        Ok(config)
    }
}

/// Write a default configuration file to file passed into the function
/// 
/// # Parameters
/// 
/// * `config_file` - The PathPuf of the config file location to write to
/// ---
fn write_default_config(config_directory: &path::PathBuf, config_file: &path::PathBuf) -> AppResult<()> {
    // Initiate default config
    let default_config = Config::default();

    // Recursively create a directory and all of its parent components if they are missing.
    fs::create_dir_all(config_directory)?;

    // Write default struct to file
    fs::write(config_file, toml::to_string(&default_config)?)?;

    tracing::info!(
        "Default configuration file written to: {}",
        config_file.display()
    );

    Ok(())
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
        let config_directory = path::PathBuf::from("/tmp/authentication_tui_test");
        let config_file = path::PathBuf::from("/tmp/authentication_tui_test/test_config_file.toml");

        // Clean up previous test file if required
        if config_file.exists() {
            fs::remove_dir_all(&config_directory)?;
        };

        let default_config = Config::default();

        //-- Execute Function (Act)
        write_default_config(&config_directory, &config_file)?;

        //-- Checks (Assertions)
        let config: Config = config::Config::builder()
            .add_source(config::File::from(config_file.clone()))
            .build()?
            .try_deserialize()?;

        // Confirm parsed config equals the default config
        assert_eq!(config, default_config);

        // Clean up test file
        if config_file.exists() {
            fs::remove_dir_all(&config_directory)?;
        };

        //-- Return
        Ok(())
    }

}
