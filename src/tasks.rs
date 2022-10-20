use std::process::Command;

use crate::config::AlchemistConfig;
use crate::error::{AlchemistErrorType, Result};

use crate::cli;

use serde::Deserialize;
pub trait RunnableTask {
    fn run<T: ToString>(&self, task_name: T, config: &AlchemistConfig) -> Result<()>;
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
    fn run<T: ToString>(&self, task_name: T, _config: &AlchemistConfig) -> Result<()> {
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
                return AlchemistErrorType::CommandFailedError.build_result(
                    format!("Starting basic task {task_name} with command `{command_str}` either not found or insufficient permissions to run."))

            }
        };
        if let Ok(exit_code) = child.wait() {
            if exit_code.success() {
                cli::ok(format!("Finished command {}", command_str));
                Ok(())
            } else {
                return AlchemistErrorType::CommandFailedError.build_result(
                    format!(
                        "While running basic task {task_name}, command `{command_str}` failed (non-zero exit code)."
                    ),
                );
            }
        } else {
            return AlchemistErrorType::CommandFailedError.build_result(format!(
                "Execution of basic task {task_name} with command `{command_str}` failed to start."
            ));
        }
    }
}

impl RunnableTask for AlchemistSerialTasks {
    fn run<T: ToString>(&self, task_name: T, config: &AlchemistConfig) -> Result<()> {
        cli::info(format!(
            "Running serial task '{}' which is a collection of {:?}",
            task_name.to_string(),
            self.serial_tasks
        ));
        for t in &self.serial_tasks {
            match config.tasks.get(t) {
                Some(v) => v.run(t, config),
                None => {
                    return AlchemistErrorType::InvalidSerialTask.build_result(format!(
                        "Serial task '{}' has an invalid subtask '{t}'",
                        task_name.to_string()
                    ))
                }
            }?;
        }
        cli::ok(format!("Finished serial task '{}'", task_name.to_string()));
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
        return match self {
            AlchemistTaskType::AlchemistBasicTask(z) => z.run(task_name, &config),
            AlchemistTaskType::AlchemistSerialTasks(z) => z.run(task_name, &config),
        };
    }
}
