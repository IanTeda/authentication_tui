//-- ./src/config/mod.rs

#![allow(unused)] // For development only

//! Configuration module
//!
//! Parse the configuration file into a struct. If no configuration file exists
//! write a file using the Config struct default vales
//! ---

use std::path::PathBuf;
use std::{fs, path};

use crate::cli;
use crate::prelude::*;

mod app;

#[derive(Debug, Clone, PartialEq, Default, serde::Deserialize, serde::Serialize)]
pub struct Config {
    /// Configure the TUI Application
    pub app: app::AppConfig,
}

impl Config {
    pub fn parse(args: cli::Args) -> Result<Self> {
        // Build config directory path
        let config_directory =
            directories::ProjectDirs::from("com", "ianteda", "authentication_tui")
                .ok_or(Error::Static(
                    "Failed to determine application configuration directory",
                ))?
                .config_dir()
                .to_path_buf();

        // Append config file to directory path
        let config_file = config_directory.join("config.toml");

        // If config file does not exist write the default config file
        if !config_file.exists() {
            // Write default config file
            write_default_config(&config_directory, &config_file)?;
        };

        // Construct config builder
        let mut builder =
            config::Config::builder().add_source(config::File::from(config_file));

        // If a config file has been parsed to the cli arguments add it as a source
        if !args.config.is_empty() {
            let cli_config = PathBuf::from(args.config);
            builder.clone().add_source(config::File::from(cli_config));
        }

        // Add environment variables (with a prefix of TUI and '_' as separator).
        // E.g. `TUI__APP_DATA_DIRECTORY=/opt/tui/data would set `app.data_directory`
        builder.clone().add_source(
            config::Environment::with_prefix("TUI")
                .prefix_separator("__")
                .separator("_"),
        );

        let config = builder.build()?;

        Ok(config.try_deserialize()?)
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
) -> std::io::Result<()> {
    // Construct default config
    let default_config = Config::default();

    // Recursively create a directory and all of its parent if they are missing.
    fs::create_dir_all(config_directory)?;

    // Write default Config struct to file
    fs::write(config_file, toml::to_string(&default_config).unwrap()).unwrap();

    tracing::info!(
        "Default configuration file written to: {}",
        config_file.display()
    );

    Ok(())
}

// #[cfg(test)]
// mod tests {
//     // #![allow(unused)] // For development only

//     // Bring current module into scope
//     use super::*;

//     // Override with more flexible error
//     pub type Result<T> = core::result::Result<T, Error>;
//     pub type Error = Box<dyn std::error::Error>;

//     #[test]
//     fn confirm_default_config_file() -> Result<()> {
//         //-- Setup and Fixtures (Arrange)
//         let config_directory = path::PathBuf::from("/tmp/authentication_tui_test");
//         let config_file = path::PathBuf::from(
//             "/tmp/authentication_tui_test/test_config_file.toml",
//         );

//         // Clean up previous test file if required
//         if config_file.exists() {
//             fs::remove_dir_all(&config_directory)?;
//         };

//         let default_config = Config::default();

//         //-- Execute Function (Act)
//         write_default_config(&config_directory, &config_file)?;

//         //-- Checks (Assertions)
//         let config: Config = config::Config::builder()
//             .add_source(config::File::from(config_file.clone()))
//             .build()?
//             .try_deserialize()?;

//         // Confirm parsed config equals the default config
//         assert_eq!(config, default_config);

//         // Clean up test file
//         if config_file.exists() {
//             fs::remove_dir_all(&config_directory)?;
//         };

//         //-- Return
//         Ok(())
//     }
// }
