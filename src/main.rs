//! TODO:
//!
//! - [ ] fix some debug variable names like `z` if there are any at all

mod cli;

mod config;
mod error;
mod tasks;
use std::env;

use clap::Parser;
use colored::Colorize;

use crate::cli::interface;
use crate::cli::interface::{CliArgs, SubCommands};
use crate::cli::terminal;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    println!("{} version {}\n", "alchemist".green(), VERSION.yellow());
    let args = CliArgs::parse();
    match args.command {
        SubCommands::Run { tasks } => match interface::run_tasks(tasks) {
            Ok(_) => terminal::ok("Finished running all given tasks."),
            Err(e) => terminal::error(e),
        },
        SubCommands::Init { target } => match interface::create_template_config(target) {
            Ok(_) => terminal::ok("Created template file!"),
            Err(e) => terminal::error(e),
        },
        SubCommands::List {} => match interface::list_available_tasks() {
            Ok(_) => (),
            Err(e) => terminal::error(e),
        },
    };
}
