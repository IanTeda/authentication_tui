//-- ./src/config.rs

// #![allow(unused)] // For development only

//! Tui application configuration module
//!
//! Parse the configuration file into a struct. If no configuration file exists
//! write a file using the Config struct default vales
//! ---

use std::path;


/// Tui application configuration
///
/// ## References
///
/// * [crates-tui/src/config.rs](https://github.com/ratatui-org/crates-tui/blob/main/src/config.rs)
/// ---
#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct TuiConfig {
    /// The directory to use for storing application configuration ( etc.).
    pub config_directory: path::PathBuf,

    /// The directory to use for storing application data (logs etc.).
    pub data_directory: path::PathBuf,

    /// The file used to store the application configuration
    pub config_file: path::PathBuf,

    /// Duration of the application loop in milliseconds
    pub tick_rate: u64,
    // pub frame_rate: f64,

    // pub key_refresh_rate: f64,

    // pub enable_mouse: bool,

    // pub enable_paste: bool,

    // pub prompt_padding: u16,

    // pub key_bindings: KeyBindings,

    // pub color: Base16Palette,
}

impl Default for TuiConfig {
    /// Default settings used to write to file if config file not found
    fn default() -> Self {
        let project_directory =
            directories::ProjectDirs::from("com", "ianteda", "authentication_tui");

        let config_directory =
            project_directory.clone().unwrap().config_dir().to_owned();

        let data_directory = project_directory
            // .ok_or(Error::Static("Error reading Project Directory"))?
            .unwrap()
            .data_dir()
            .to_owned();

        let config_file = config_directory.join("config.toml");

        let tick_rate: u64 = 250;

        Self {
            config_directory,
            data_directory,
            config_file,
            tick_rate,
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
    fn confirm_default_config_file() -> Result<()> {
        //-- Setup and Fixtures (Arrange)

        //-- Execute Function (Act)
        let default_config = TuiConfig::default();

        //-- Checks (Assertions)
        assert_eq!(default_config.config_directory.display().to_string(), "/home/ian/.config/authentication_tui");
        assert_eq!(default_config.data_directory.display().to_string(), "/home/ian/.local/share/authentication_tui");
        assert_eq!(default_config.config_file.display().to_string(), "/home/ian/.config/authentication_tui/config.toml");
        assert_eq!(default_config.tick_rate, 250);

        //-- Return
        Ok(())
    }
}