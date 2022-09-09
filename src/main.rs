mod error;

use serde::Deserialize;
use std::{collections::HashMap, fs, path::Path};

use colored::Colorize;

use error::{AlchemistError, AlchemistErrorType, Result};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const CONFIG_FILE: &str = "alchemist.toml";

pub trait RunnableTask {
    fn run(&self) -> Result<()>;
}

#[derive(Debug, Deserialize)]
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
    sub_tasks: Vec<String>,
}

impl RunnableTask for AlchemistBasicTask {
    fn run(&self) -> Result<()> {
        Err(AlchemistError::new(
            AlchemistErrorType::ConfigParseError,
            "It not workingz",
        ))
    }
}

impl AlchemistSerialTasks {
    pub fn to_basic_tasks() -> AlchemistSerialTasks {
        AlchemistSerialTasks {
            sub_tasks: Vec::new(),
        }
    }
}

impl RunnableTask for AlchemistSerialTasks {
    fn run(&self) -> Result<()> {
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
        Err(_) => panic!("Invalid configuration!!!!"), // TODO: custom errors and handling instead of panic
    };

    for (task_name, unknown_task) in alchemist_config.tasks.iter() {
        match unknown_task {
            AlchemistTaskType::AlchemistBasicTask(task) => {
                println!("Basic: {:#?}", task_name);
                let _ = task.run()?;
            }
            AlchemistTaskType::AlchemistSerialTasks(task) => {
                println!("SerialTasks: {:#?}", task_name);
                let _ = task.run()?;
            }
        }
    }
    Ok(())
}

fn main() {
    match do_main() {
        Ok(_) => println!("all donzos, veri gud"),
        Err(e) => eprintln!("{}", e),
    }
}
