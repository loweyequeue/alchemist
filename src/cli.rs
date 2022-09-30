use colored::Colorize;

pub const INFO: &'static str = "ℹ︎";
pub const DEBUG: &'static str = "⌗";
pub const ERROR: &'static str = "✘";
pub const OK: &'static str = "✔︎";

fn message_prefix<T: ToString>(icon: T) -> String {
    format!("{}{}{}", "[".dimmed(), icon.to_string(), "]".dimmed())
}

pub fn ok<T: ToString>(message: T) {
    println!(
        "{}{}{}",
        message_prefix(OK.green().bold()),
        ": ".dimmed(),
        message.to_string()
    );
}

pub fn error(err: crate::error::AlchemistError) {
    println!(
        "{}{}{}{}{}",
        message_prefix(ERROR.red().bold()),
        "[".dimmed(),
        err.error_type.to_string().dimmed().italic(),
        "]: ".dimmed(),
        err.error_message.to_string()
    )
}

pub fn info<T: ToString>(message: T) {
    println!(
        "{}{}{}",
        message_prefix(INFO.cyan().bold()),
        ": ".dimmed(),
        message.to_string()
    )
}

pub fn debug<T: ToString>(message: T) {
    println!(
        "{}{}{}",
        message_prefix(DEBUG.magenta().bold()),
        ": ".dimmed(),
        message.to_string()
    )
}
