mod cli;
mod error;

use std::process::Command;

use serde::Deserialize;
use std::{collections::HashMap, env, fs, path::Path};

use colored::Colorize;

use crate::error::{AlchemistError, AlchemistErrorType, Result};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const CONFIG_FILE: &str = "alchemist.toml";

/// TODO:
///
/// - [ ] Command line parsing
///   - [ ] Run tasks from commandline not everythingz.
///
/// - [ ] validate Serial tasks

pub trait RunnableTask {
    fn run<T: ToString>(&self, task_name: T) -> Result<()>;
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
/// Alchemist BasicTask type is a simple task with a command and optional args
/// Example:
/// ```
/// [tasks.my_task]
/// command = "echo"
/// args = ["hello", "world"]
/// ```
pub struct AlchemistBasicTask {
    #[allow(dead_code)]
    command: String,
    #[allow(dead_code)]
    args: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
/// Alchemist SerialTasks type can be a set of multiple basic tasks
///
/// These tasks are executed in the given order
///
///
/// Example:
/// ```
/// [tasks.my_task]
/// sub_recipes = ["my_other_task1", "my_other_task2"]
///
/// ```
pub struct AlchemistSerialTasks {
    #[allow(dead_code)]
    serial_tasks: Vec<String>,
}

impl RunnableTask for AlchemistBasicTask {
    fn run<T: ToString>(&self, task_name: T) -> Result<()> {
        let task_name = task_name.to_string();
        let mut cmd = Command::new(&self.command);
        let command_str = if let Some(args) = &self.args {
            cmd.args(args);
            format!("{} {}", &self.command, args.join(" "))
        } else {
            format!("{}", &self.command)
        };
        cli::info(format!("Running command {}", command_str));
        let mut child = match cmd.spawn() {
            Ok(child) => child,
            Err(_) => {
                return Err(AlchemistError::new(
                    AlchemistErrorType::CommandFailedError,
                    format!("Starting basic task {task_name} with command `{command_str}` either not found or insufficient permissions to run."),
                ))
            }
        };
        if let Ok(exit_code) = child.wait() {
            if exit_code.success() {
                cli::ok(format!("Finished command {}", command_str));
                Ok(())
            } else {
                return Err(AlchemistError::new(
                    AlchemistErrorType::CommandFailedError,
                    format!(
                        "While running basic task {task_name}, command `{command_str}` failed (non-zero exit code)."
                    ),
                ));
            }
        } else {
            return Err(AlchemistError::new(AlchemistErrorType::CommandFailedError, format!("Execution of basic task {task_name} with command `{command_str}` failed to start.")));
        }
    }
}

impl AlchemistSerialTasks {
    pub fn to_basic_tasks() -> Result<Vec<AlchemistBasicTask>> {
        Ok(Vec::new())
    }
}

impl RunnableTask for AlchemistSerialTasks {
    fn run<T: ToString>(&self, _task_name: T) -> Result<()> {
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
/// An enum of multiple variations of tasks within the alchemist.toml
///
///
/// This is used as multiple types for serde (de)serialization
///
/// doing this allows us to have multiple types of tasks without a complex configuration format
pub enum AlchemistTaskType {
    AlchemistBasicTask(AlchemistBasicTask),
    AlchemistSerialTasks(AlchemistSerialTasks),
}

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
    tasks: HashMap<String, AlchemistTaskType>,
}

fn do_main(tasks: Vec<String>) -> Result<()> {
    println!("{} version {}\n", "alchemist".green(), VERSION.yellow());
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
    let alchemist_config: AlchemistConfig = match toml::from_str(&config_file_content) {
        Ok(v) => v,
        Err(_) => {
            return Err(AlchemistError::new(
                AlchemistErrorType::ConfigParseError,
                "Invalid configuration.",
            ))
        }
    };

    for t in tasks {
        if let Some(v) = alchemist_config.tasks.get(&t) {
            match v {
                AlchemistTaskType::AlchemistBasicTask(z) => z.run(t)?,
                AlchemistTaskType::AlchemistSerialTasks(z) => z.run(t)?,
            }
        }
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args_os()
        .skip(1)
        .filter_map(|i| match i.to_str() {
            Some(v) => Some(v.to_owned()),
            None => None,
        })
        .collect();
    match do_main(args) {
        Ok(_) => cli::ok("all donzos, veri gud"),
        Err(e) => cli::error(e),
    }
}
