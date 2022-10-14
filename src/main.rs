mod cli;
mod config;
mod error;
mod tasks;
use std::env;

use colored::Colorize;

use crate::tasks::RunnableTask;
use crate::{config::get_config, error::Result};

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// TODO:
///
/// - [ ] Search upwards for config file (not just current dir)

fn run_cli_tool() -> Result<()> {
    println!("{} version {}\n", "alchemist".green(), VERSION.yellow());

    let tasks: Vec<String> = env::args_os()
        .skip(1)
        .filter_map(|i| match i.to_str() {
            Some(v) => Some(v.to_owned()),
            None => None,
        })
        .collect();

    let alchemist_config = get_config()?;

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
