use std::{collections::HashMap, fs, path::Path};

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

pub fn get_config() -> Result<AlchemistConfig> {
    cli::debug(format!("searching for {}", CONFIG_FILE));
    let config_file_path = Path::new(CONFIG_FILE);

    if !config_file_path.exists() {
        return Err(AlchemistError::new(
            AlchemistErrorType::NoConfigFileError,
            format!("'{}' does not exist", CONFIG_FILE),
        ));
    }
    if !config_file_path.is_file() {
        // Known bug: no symlinks, not going to fix
        return Err(AlchemistError::new(
            AlchemistErrorType::NoConfigFileError,
            format!("'{}' is not a file", CONFIG_FILE),
        ));
    }

    let config_file_content: String = fs::read_to_string(CONFIG_FILE).unwrap();
    toml::from_str(&config_file_content).or(Err(AlchemistError::new(
        AlchemistErrorType::ConfigParseError,
        "Invalid configuration.",
    )))
}
