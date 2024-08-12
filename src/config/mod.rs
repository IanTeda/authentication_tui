//-- ./src/config.rs

// #![allow(unused)] // For development only

//! Tui application configuration module
//!
//! Parse the configuration file into a struct. If no configuration file exists
//! write a file using the Config struct default vales
//! ---

use std::{fs, path};

use crate::{TuiError, TuiResult};

mod backend;
mod tui;

#[derive(Debug, Clone, PartialEq, Default, serde::Deserialize, serde::Serialize)]
pub struct Config {
    /// Configure the Tui application
    pub tui: tui::TuiConfig,

    /// Backend configuration
    pub backend: backend::BackendConfig,
}

impl Config {
    pub fn parse() -> TuiResult<Self> {
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
fn write_default_config(
    config_directory: &path::PathBuf,
    config_file: &path::PathBuf,
) -> TuiResult<()> {
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
        let config_file = path::PathBuf::from(
            "/tmp/authentication_tui_test/test_config_file.toml",
        );

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
