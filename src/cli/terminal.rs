use colored::Colorize;

pub const INFO: &str = "ℹ︎";
pub const ERROR: &str = "✘";
pub const OK: &str = "✔︎";
pub const WARNING: &str = "‼︎";
#[cfg(debug_assertions)]
pub const DEBUG: &str = "⌗";

fn message_prefix<S: ToString>(icon: S) -> String {
    format!("{}{}{}", "[".dimmed(), icon.to_string(), "]".dimmed())
}

// TODO: FIXME: Continue here, colored lib needs to go away
// make custom basic color stuff (maybe as const fns)
// MAYBE: check if owo-colors lib is better
let ERROR_PREFIX: String = message_prefix(ERROR.red().bold());

pub fn ok<S: ToString>(message: S) {
    println!(
        "{}{}{}",
        message_prefix(OK.green().bold()),
        ": ".dimmed(),
        message.to_string()
    );
}
pub fn warn<S: ToString>(message: S) {
    println!(
        "{}{}{}",
        message_prefix(WARNING.yellow().bold()),
        ": ".dimmed(),
        message.to_string()
    );
}

pub fn error(err: crate::error::AlchemistError) {
    eprintln!("{}", err)
}

pub fn info<S: ToString>(message: S) {
    println!(
        "{}{}{}",
        message_prefix(INFO.cyan().bold()),
        ": ".dimmed(),
        message.to_string()
    )
}

#[allow(unused_variables)]
pub fn debug<S: ToString>(message: S) {
    #[cfg(debug_assertions)]
    println!(
        "{}{}{}",
        message_prefix(DEBUG.magenta().bold()),
        ": ".dimmed(),
        message.to_string()
    )
}
