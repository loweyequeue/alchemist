//! TODO:
//!
//! - [ ] fix some debug variable names like `z` if there are any at all

mod cli;

mod config;
mod error;
mod tasks;
use std::env;

use clap::Parser;
use simply_colorful::Colorize;

use crate::cli::interface;
use crate::cli::interface::CliArgs;
use crate::cli::terminal;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let args = CliArgs::parse();

    if !args.quiet {
        println!("{} version {}\n", "alchemist".green(), VERSION.yellow());
    }

    if args.init.is_some() {
        match interface::create_template_config(args.init) {
            Ok(_) => terminal::ok("Created template file!"),
            Err(e) => terminal::error(e),
        }
        return ();
    }
    if args.list {
        match interface::list_available_tasks() {
            Ok(_) => (),
            Err(e) => terminal::error(e),
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
