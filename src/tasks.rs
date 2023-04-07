use std::collections::HashMap;
use std::process::Command;

use crate::config::AlchemistConfig;
use crate::error::{AlchemistErrorType, Result};

use crate::cli::terminal;

use serde::Deserialize;
pub trait RunnableTask {
    fn run<S: ToString>(&self, task_name: S, config: &AlchemistConfig) -> Result<()>;
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
    env: Option<HashMap<String, String>>,
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
    fn run<S: ToString>(&self, task_name: S, _config: &AlchemistConfig) -> Result<()> {
        let task_name = task_name.to_string();
        let mut cmd = Command::new(&self.command);

        if let Some(env_var) = &self.env {
            cmd.envs(env_var);
        }

        let command_str = if let Some(args) = &self.args {
            cmd.args(args);
            format!("{} {}", &self.command, args.join(" "))
        } else {
            self.command.to_string()
        };
        terminal::info(format!("Running command {}", command_str));
        let mut child = cmd.spawn().or_else(|_| {
            AlchemistErrorType::CommandFailedError.build_result(
                format!("Starting basic task {task_name} with command `{command_str}` either not found or insufficient permissions to run.")
            )
        })?;
        let exit_code = child.wait().or_else(|_| AlchemistErrorType::CommandFailedError.build_result( format!( "While running basic task {task_name}, command `{command_str}` failed (non-zero exit code).")))?;
        if !exit_code.success() {
            return AlchemistErrorType::CommandFailedError.build_result(
                format!( "While running basic task {task_name}, command `{command_str}` failed (non-zero exit code).")
            );
        }
        terminal::ok(format!("Finished command {}", command_str));
        Ok(())
    }
}

impl RunnableTask for AlchemistSerialTasks {
    fn run<S: ToString>(&self, task_name: S, config: &AlchemistConfig) -> Result<()> {
        let task_name = task_name.to_string();
        terminal::info(format!(
            "Running serial task '{}' which is a collection of {:?}",
            task_name, self.serial_tasks
        ));
        for sub_task_name in &self.serial_tasks {
            match config.tasks.get(sub_task_name) {
                Some(task) => task.run(sub_task_name, config),
                None => {
                    return AlchemistErrorType::InvalidSerialTask.build_result(format!(
                        "Serial task '{task_name}' has an invalid subtask '{sub_task_name}'"
                    ))
                }
            }?;
        }
        terminal::ok(format!("Finished serial task '{task_name}'"));
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

impl RunnableTask for AlchemistTaskType {
    fn run<T: ToString>(&self, task_name: T, config: &AlchemistConfig) -> Result<()> {
        match self {
            AlchemistTaskType::AlchemistBasicTask(task) => task.run(task_name, config),
            AlchemistTaskType::AlchemistSerialTasks(task) => task.run(task_name, config),
        }
    }
}
