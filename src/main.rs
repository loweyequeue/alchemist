mod cli;
mod config;
mod error;
mod tasks;
use std::env;

use colored::Colorize;

use crate::config::{locate_config, parse_config, set_cwd_to_config_dir};
use crate::error::Result;
use crate::tasks::RunnableTask;

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// TODO:
///
/// - [ ] help/list command with --<arg>
/// - [ ] init command to create template file

fn run_cli_tool() -> Result<()> {
    println!("{} version {}\n", "alchemist".green(), VERSION.yellow());

    let tasks: Vec<String> = env::args_os()
        .skip(1)
        .filter_map(|i| match i.to_str() {
            Some(v) => Some(v.to_owned()),
            None => None,
        })
        .collect();

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

fn main() {
    match run_cli_tool() {
        Ok(_) => cli::ok("Finished running all given tasks."),
        Err(e) => cli::error(e),
    }
}
