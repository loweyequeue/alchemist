use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};

use crate::cli::terminal;
use crate::config::{locate_config, parse_config, set_cwd_to_config_dir, CONFIG_FILE};
use crate::error::{AlchemistErrorType, Result};
use crate::tasks::RunnableTask;

#[derive(Subcommand, Debug)]
pub(crate) enum SubCommands {
    #[command(about = "Create a template file at current dir or at a specified location")]
    Init { target: Option<PathBuf> },
    #[command(alias = "ls", about = "List all available commands (alias: ls)")]
    List {},
    #[command(about = "Run list of given tasks")]
    Run { tasks: Vec<String> },
}

#[derive(Parser, Debug)]
#[clap(author, about)]
pub(crate) struct CliArgs {
    #[command(subcommand)]
    pub command: SubCommands,
}

pub(crate) fn run_tasks(tasks: Vec<String>) -> Result<()> {
    let config_file_path = locate_config()?;
    let alchemist_config = parse_config(&config_file_path)?;
    set_cwd_to_config_dir(&config_file_path)?;

    for t in tasks {
        if let Some(task) = alchemist_config.tasks.get(&t) {
            task.run(t, &alchemist_config)?
        }
    }
    Ok(())
}

pub(crate) fn create_template_config(target: Option<PathBuf>) -> Result<()> {
    let target_dir = target.unwrap_or_else(|| Path::new(".").to_path_buf());
    if !(target_dir.exists() && target_dir.is_dir()) {
        return AlchemistErrorType::CLIError.build_result(format!(
            "Template init dir '{}' does not exist (or is not a directory).",
            target_dir.display()
        ));
    }

    let template_path = target_dir.join(CONFIG_FILE);
    if template_path.exists() {
        return AlchemistErrorType::CLIError.build_result(format!(
            "Refusing to create config file, since '{}' already exists!",
            template_path.display()
        ));
    }

    let template_content = include_bytes!("alchemist.template");
    if let Ok(mut template_file) = File::create(template_path) {
        template_file.write_all(template_content).or_else(|_| {
            AlchemistErrorType::CLIError.build_result("Could not write template file.")
        })?;
    } else {
        return AlchemistErrorType::CLIError.build_result("Could not create template file.");
    }

    Ok(())
}

pub(crate) fn list_available_tasks() -> Result<()> {
    let config_file_path = locate_config()?;
    let alchemist_config = parse_config(&config_file_path)?;

    let mut task_names = Vec::from_iter(alchemist_config.tasks.keys());

    if task_names.is_empty() {
        terminal::warn("No tasks configured!");
        return Ok(());
    }

    task_names.sort();

    terminal::ok("Available tasks:");
    for task_name in task_names {
        println!("\t{}", task_name);
    }

    Ok(())
}
