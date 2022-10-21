mod cli;
mod config;
mod error;
mod tasks;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use clap::{Parser, Subcommand};
use colored::Colorize;
use error::AlchemistErrorType;

use crate::config::{locate_config, parse_config, set_cwd_to_config_dir, CONFIG_FILE};
use crate::error::Result;
use crate::tasks::RunnableTask;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Subcommand, Debug)]
enum SubCommands {
    Init { target: Option<String> },
    List {},
    Run { tasks: Vec<String> },
}

#[derive(Parser, Debug)]
#[clap(author, about)]
struct CliArgs {
    #[command(subcommand)]
    command: SubCommands,
}

/// TODO:
///
/// - [ ] help/list command with --<arg>
/// - [ ] init command to create template file

fn run_tasks(tasks: Vec<String>) -> Result<()> {
    let config_file_path = locate_config()?;
    let alchemist_config = parse_config(&config_file_path)?;
    set_cwd_to_config_dir(&config_file_path)?;

    for t in tasks {
        if let Some(v) = alchemist_config.tasks.get(&t) {
            v.run(t, &alchemist_config)?
        }
    }
    Ok(())
}

fn create_template_config(target: Option<String>) -> Result<()> {
    let target_str = target.unwrap_or(String::from("."));
    let target_dir = Path::new(&target_str);
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

    let template_content = include_bytes!("../alchemist.template");
    if let Ok(mut template_file) = File::create(template_path) {
        template_file
            .write_all(template_content)
            .or(AlchemistErrorType::CLIError.build_result("Could not write template file."))?;
    } else {
        return AlchemistErrorType::CLIError.build_result("Could not create template file.");
    }

    Ok(())
}

fn main() {
    println!("{} version {}\n", "alchemist".green(), VERSION.yellow());
    let args = CliArgs::parse();
    match args.command {
        SubCommands::Run { tasks } => match run_tasks(tasks) {
            Ok(_) => cli::ok("Finished running all given tasks."),
            Err(e) => cli::error(e),
        },
        SubCommands::Init { target } => match create_template_config(target) {
            Ok(_) => cli::ok("Created template file!"),
            Err(e) => cli::error(e),
        },
        _ => return,
    }
}
