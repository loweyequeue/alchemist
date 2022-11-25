use std::env::{current_dir, set_current_dir};

use std::path::PathBuf;
use std::{collections::HashMap, fs};

use serde::Deserialize;

use crate::cli::terminal;
use crate::error::{AlchemistErrorType, Result};
use crate::tasks::*;

pub const CONFIG_FILE: &str = "alchemist.toml";

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
    let workingdir = current_dir().or_else(|_| {
        AlchemistErrorType::CurrentDirIsInvalid
            .build_result("The current directory does not exist or you have no permissions for it.")
    })?;

    workingdir
        .ancestors()
        .map(|p| p.to_path_buf().join(CONFIG_FILE))
        .find(|p| p.exists() && p.is_file())
        .ok_or_else(|| {
            AlchemistErrorType::NoConfigFileError.with_message(format!(
                "'{}' does not exist or is not a file.",
                CONFIG_FILE
            ))
        })
    // workingdir
    //     .ancestors()
    //     .map(|p| p.to_path_buf().join(CONFIG_FILE))
    //     .filter(|c| c.exists() && c.is_file())
    //     .next()
    //     .ok_or(AlchemistErrorType::NoConfigFileError.with_message(format!(
    //         "'{}' does not exist or is not a file.",
    //         CONFIG_FILE
    //     )))
}

pub fn parse_config(config_file_path: &PathBuf) -> Result<AlchemistConfig> {
    terminal::debug(format!("searching for {}", CONFIG_FILE));

    let config_file_content: String = fs::read_to_string(config_file_path).or_else(|_| {
        AlchemistErrorType::ConfigParseError.build_result("Could not read the config file")
    })?;
    toml::from_str(&config_file_content)
        .or_else(|_| AlchemistErrorType::ConfigParseError.build_result("Invalid configuration."))
}

// allow because we might reuse the PathBuf in the future
#[allow(clippy::ptr_arg)]
pub fn set_cwd_to_config_dir(config_file_path: &PathBuf) -> Result<()> {
    let config_location = config_file_path.parent().ok_or_else(|| {
        AlchemistErrorType::CurrentDirIsInvalid.with_message("No access to config parent directory")
    })?;
    set_current_dir(config_location).or_else(|_| {
        AlchemistErrorType::CurrentDirIsInvalid.build_result("Can not move to project root.")
    })?;
    Ok(())
}
