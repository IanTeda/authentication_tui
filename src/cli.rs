//-- ./src/cli.rs

//! Command line interface module
//! 
//! Parse command line switches and provide help
//! 
//! ---

#[derive(Debug, clap::Parser,)]
#[command(author, version = version(), about)]
pub struct Args {
    /// location of the configuration file
    #[arg(short, long, default_value = "")]
    pub config: String,
}

const VERSION_MESSAGE: &str = concat!(
    env!("CARGO_PKG_VERSION"),
    "-",
    env!("VERGEN_GIT_DESCRIBE"),
    " (",
    env!("VERGEN_BUILD_DATE"),
    ")"
);

pub fn version() -> String {
    let author = clap::crate_authors!();

    // let current_exe_path = PathBuf::from(clap::crate_name!()).display().to_string();
    // let config_dir_path = get_config_dir().display().to_string();
    // let data_dir_path = get_data_dir().display().to_string();

    format!(
        "\
{VERSION_MESSAGE}

Authors: {author}

"
    )
}