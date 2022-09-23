mod cli;
mod error;

use std::process::Command;

use serde::Deserialize;
use std::{collections::HashMap, fs, path::Path};

use colored::Colorize;

use crate::error::{AlchemistError, AlchemistErrorType, Result};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const CONFIG_FILE: &str = "alchemist.toml";

/// TODO:
///
/// - [ ] Run Basic Task: Error handling & large fn body refactor;
/// - [ ] v / x / info -> use cli module;
/// - [ ] colors for messages (consistently)
/// - [ ] or validate Serial tasks

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
        println!("[{}]: Running command {}", cli::INFO, command_str);
        let error_msg =
            format!("While running basic task {task_name}, command `{command_str}` failed.");
        let mut child = match cmd.spawn() {
            Ok(child) => child,
            Err(_) => {
                return Err(AlchemistError::new(
                    AlchemistErrorType::CommandFailedError,
                    error_msg,
                ))
            }
        };
        let exit_code = child.wait().unwrap(); // TODO: handle error
        if !exit_code.success() {
            println!(":(")
        }
        Ok(())
    }
}

impl AlchemistSerialTasks {
    pub fn to_basic_tasks() -> Result<Vec<AlchemistBasicTask>> {
        Ok(Vec::new())
    }
}

impl RunnableTask for AlchemistSerialTasks {
    fn run<T: ToString>(&self, _task_name: T) -> Result<()> {
        //Err("Not Implemented".to_string())
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

fn do_main() -> Result<()> {
    println!("{} version {}", "alchemist".green(), VERSION.yellow());
    println!("searching for {}", CONFIG_FILE);

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

    for (task_name, unknown_task) in alchemist_config.tasks.iter() {
        match unknown_task {
            AlchemistTaskType::AlchemistBasicTask(task) => {
                println!("[Z]: Running task {}", task_name);
                let _ = task.run(task_name)?;
            }
            AlchemistTaskType::AlchemistSerialTasks(task) => {
                println!("SerialTasks: {:#?}", task_name);
                let _ = task.run(task_name)?;
            }
        }
    }
    Ok(())
}

fn main() {
    match do_main() {
        Ok(_) => println!(
            "{}{}{} {}",
            "[".dimmed(),
            cli::OK.green().bold(),
            "]:".dimmed(),
            "all donzos, veri gud"
        ),
        Err(e) => eprintln!("{}", e),
    }
}
