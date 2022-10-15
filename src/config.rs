use std::env::{current_dir, set_current_dir};

use std::path::PathBuf;
use std::{collections::HashMap, fs};

use serde::Deserialize;

use crate::cli;
use crate::error::{AlchemistError, AlchemistErrorType, Result};
use crate::tasks::*;

const CONFIG_FILE: &str = "alchemist.toml";

#[derive(Debug, Deserialize)]
/// Contains the structure of the alchemist.toml file
///
/// Reads a toml file like the following:
/// ```
/// [tasks.task1]
/// ...
///
/// [tasks.task2]
/// ...
/// ```
pub struct AlchemistConfig {
    /// Contains a map of tasks that can be of multiple task types
    pub tasks: HashMap<String, AlchemistTaskType>,
}

pub fn locate_config() -> Result<PathBuf> {
    let workingdir = current_dir()
        .or(Err(AlchemistErrorType::CurrentDirIsInvalid.with_message(
            "The current directory does not exist or you have no permissions for it.",
        )))?;

    workingdir
        .ancestors()
        .map(|p| p.to_path_buf().join(CONFIG_FILE))
        .filter(|c| c.exists() && c.is_file())
        .next()
        .ok_or(AlchemistErrorType::NoConfigFileError.with_message(format!(
            "'{}' does not exist or is not a file.",
            CONFIG_FILE
        )))
}

pub fn parse_config(config_file_path: &PathBuf) -> Result<AlchemistConfig> {
    cli::debug(format!("searching for {}", CONFIG_FILE));

    let config_file_content: String = fs::read_to_string(config_file_path).unwrap(); // TODO
    toml::from_str(&config_file_content).or(Err(
        AlchemistErrorType::ConfigParseError.with_message("Invalid configuration.")
    ))
}

pub fn set_cwd_to_config_dir(config_file_path: &PathBuf) -> Result<()> {
    let config_location = config_file_path.parent().unwrap();
    set_current_dir(config_location).or(Err(
        AlchemistErrorType::CurrentDirIsInvalid.with_message("Can not move to project root.")
    ))?;
    Ok(())
}
