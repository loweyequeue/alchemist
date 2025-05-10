//! TODO:
//!
//! - [ ] fix some debug variable names like `z` if there are any at all

mod cli;

mod config;
mod error;
mod tasks;
use std::env;

use clap::Parser;
use owo_colors::OwoColorize;

use crate::cli::interface;
use crate::cli::interface::CliArgs;
use crate::cli::terminal;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let args = CliArgs::parse();

    let config_file_path = crate::config::locate_config().ok();

    if !args.quiet {
        println!("{} version {}\n", "alchemist".green(), VERSION.yellow());
        if let Some(config_file_path) = config_file_path {
            terminal::info(format!(
                "Using alchemist file: {}\n",
                config_file_path.display().yellow()
            ));
        } else {
            terminal::warn(
                "No alchemist config file found. Please run `alchemist --init` to create one.",
            );
        }
    }

    if let Some(init_target) = args.init {
        match interface::create_template_config(init_target) {
            Ok(_) => terminal::ok("Created template file!"),
            Err(e) => terminal::error(e),
        }
        return ();
    }
    if args.list {
        if let Err(e) = interface::list_available_tasks(args.verbose) {
            terminal::error(e);
        }
        return;
    }
    if args.shell_complete {
        interface::generate_completions();
        return ();
    }
    if args.commands.len() == 0 {
        terminal::warn("No commands were provided to run. run alchemist --help for more info.");
        return ();
    }
    match interface::run_tasks(args.commands) {
        Ok(_) => terminal::ok("Finished running all given tasks."),
        Err(e) => terminal::error(e),
    }
}
