use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use crate::cli::terminal;
use crate::config::{locate_config, parse_config, set_cwd_to_config_dir, CONFIG_FILE};
use crate::error::{AssertionError, Result, ResultContext};
use crate::tasks::RunnableTask;
use clap::{CommandFactory, Parser};

#[derive(Parser, Debug)]
#[clap(author, about)]
pub(crate) struct CliArgs {
    #[arg(short, long, help="Lists all available commands in the current project", conflicts_with_all=["init", "shell_complete", "commands"])]
    pub list: bool,

    #[arg(short, long, help = "Write an alchemist example file to start a new alchemist project", conflicts_with_all=["list", "shell_complete", "commands"])]
    pub init: Option<Option<PathBuf>>,

    #[arg(short, long, help= "Writes completion files to common shells", conflicts_with_all=["list", "init", "commands"])]
    pub shell_complete: bool,

    #[arg(
        short,
        long,
        help = "Hide intro text, useful for calling alchemist recursively"
    )]
    pub quiet: bool,

    #[arg(conflicts_with_all=["list", "init", "shell_complete"])]
    pub commands: Vec<String>,
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
    let target_dir = match target {
        Some(dir) => dir,
        None => PathBuf::from("."),
    };
    if !(target_dir.exists() && target_dir.is_dir()) {
        return AssertionError(format!(
            "Template init dir '{}' does not exist (or is not a directory).",
            target_dir.display()
        ))
        .into();
    }

    let template_path = target_dir.join(CONFIG_FILE);
    if template_path.exists() {
        return AssertionError(format!(
            "Refusing to create config file, since '{}' already exists!",
            template_path.display()
        ))
        .into();
    }

    let template_content = include_bytes!("alchemist.template");
    let mut template_file =
        File::create(template_path).error_msg("Could not create template file.")?;
    template_file
        .write_all(template_content)
        .error_msg("Could not write template file.")?;
    Ok(())
}

pub(crate) fn generate_completions() {
    let home = PathBuf::from(std::env::var("HOME").expect("could not determine home dir"));
    let completion_dir = home.join(".config").join("fish").join("completions");

    if let Err(e) = std::fs::create_dir_all(&completion_dir) {
        panic!("Error: {}", e);
    }
    let mut cmd = CliArgs::command();
    clap_complete::generate_to(
        clap_complete::Shell::Fish,
        &mut cmd,
        "alchemist",
        completion_dir,
    )
    .expect("could not write completions file");
}

pub(crate) fn list_available_tasks() -> Result<()> {
    let config_file_path = locate_config()?;
    let alchemist_config = parse_config(&config_file_path)?;

    let mut task_names = alchemist_config
        .tasks
        .iter()
        .filter(|(_, v)| v.is_shown())
        .map(|(k, _)| k)
        .collect::<Vec<&String>>();

    // let mut task_names = Vec::from_iter(alchemist_config.tasks.keys());

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
