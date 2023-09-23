use std::env::{current_dir, set_current_dir};

use std::path::PathBuf;
use std::{collections::HashMap, fs};

use oh_no::ResultContext;
use serde::Deserialize;

use crate::cli::terminal;
use crate::error::{AssertionError, Result};
use crate::tasks::*;

pub const CONFIG_FILE: &str = "alchemist.toml";

#[derive(Debug, Deserialize, Clone)]
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
    let workingdir = current_dir().error_msg("Cannot access the current directory")?;

    workingdir
        .ancestors()
        .map(|p| p.to_path_buf().join(CONFIG_FILE))
        .find(|p| p.exists() && p.is_file())
        .ok_or_else(|| {
            AssertionError(format!(
                "'{}' does not exist or is not a file.",
                CONFIG_FILE
            ))
            .into()
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

    let config_file_content =
        fs::read_to_string(config_file_path).error_msg("Could not read the config file")?;
    let cfg = toml::from_str::<AlchemistConfig>(&config_file_content)
        .error_msg("Invalid configuration.")?;
    Ok(cfg)
}

// allow because we might reuse the PathBuf in the future
#[allow(clippy::ptr_arg)]
pub fn set_cwd_to_config_dir(config_file_path: &PathBuf) -> Result<()> {
    let config_location = config_file_path
        .parent()
        .ok_or_else(|| AssertionError(String::from("No access to config parent directory")))?;
    set_current_dir(config_location).error_msg("Can not move to project root.")?;
    Ok(())
}
