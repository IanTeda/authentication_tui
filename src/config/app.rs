//-- ./src/config/app.rs

// #![allow(unused)] // For development only

//! Application configuration module
//! ---

use std::path;

/// Application (TUI) configuration struct
///
/// ## References
///
/// * [crates-tui/src/config.rs](https://github.com/ratatui-org/crates-tui/blob/main/src/config.rs)
/// ---
#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct AppConfig {

    /// The directory to use for storing application data (logs etc.).
    pub data_directory: path::PathBuf,

    /// The file used to store the application configuration
    pub config_file: path::PathBuf,

    /// Duration of the application loop in ticks per second i.e. number of ticks per second
    pub tick_rate: f64,

    /// Frame (redraw) of the application refresh in frames per second i.e. number of frames per second
    pub frame_rate: f64,
}


impl Default for AppConfig {
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

        let tick_rate: f64 = 4.0;

        let frame_rate: f64 = 60.0;

        Self {
            data_directory,
            config_file,
            tick_rate,
            frame_rate,
        }
    }
}