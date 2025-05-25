#[cfg(test)]
#[path = "interface_test.rs"]
mod interface_test;

use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use crate::cli::terminal;
use crate::config::{CONFIG_FILE, locate_config, parse_config, set_cwd_to_config_dir};
use crate::error::{AssertionError, Result, ResultContext};
use crate::tasks::{RunnableTask, TaskDescription};

use clap::{CommandFactory, Parser};
use once_cell::sync::Lazy;
use owo_colors::OwoColorize;
use regex::Regex;
use terminal_size::{Height, Width, terminal_size};
use unicode_segmentation::UnicodeSegmentation;

const INDENT_TASK_CONTENT: usize = 6;
const TERMINAL_WIDTH_DEFAULT: usize = 80;

#[derive(Parser, Debug)]
#[clap(author, about)]
pub(crate) struct CliArgs {
    #[arg(short, long, help="Lists all available commands in the current project. Use -v(v) for more detailed output\n    -v\tShow what each task does\n    -vv\tExpand ShellScriptTasks", conflicts_with_all=["init", "shell_complete", "commands"])]
    pub list: bool,

    #[arg(short, long, action = clap::ArgAction::Count, hide = true, conflicts_with_all=["init", "shell_complete", "commands"])]
    pub verbose: u8,

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
    terminal::info(format!(
        "Using alchemist file: {}",
        config_file_path.display()
    ));
    let alchemist_config = parse_config(&config_file_path)?;
    set_cwd_to_config_dir(&config_file_path)?;

    for t in tasks {
        match alchemist_config.tasks.get(&t) {
            Some(task) => {
                task.run(t, &alchemist_config)?;
            }
            None => terminal::warn(format!("Task '{}' does not exist!", t)),
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
        &completion_dir,
    )
    .expect("could not write completions file");
}

fn grapheme_length(s: &str) -> usize {
    s.graphemes(true).count()
}

static ANSI_ESCAPE_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\x1B\[(?P<params>[0-9;]+)(?P<code>[A-Za-z])").unwrap());

fn extract_ansi_escapes(s: &str) -> Vec<AnsiEscape> {
    let mut results = Vec::new();
    ANSI_ESCAPE_RE.captures_iter(s).for_each(|cap| {
        let fullmatch = (&cap[0]).to_string();
        let params = cap["params"].to_string();
        let code = cap["code"].chars().next().unwrap();
        results.push(AnsiEscape {
            fullmatch,
            params: params.to_string(),
            code,
        });
    });
    results
}

fn assert_only_ansi_colors(s: &str) {
    extract_ansi_escapes(s).iter().for_each(|escape| {
        if escape.code != 'm' {
            panic!("Unsupported ANSI code: {}", escape.code);
        }
    });
}

fn grapheme_length_colored<S: ToString>(s: S) -> usize {
    let s = s.to_string();
    assert_only_ansi_colors(&s);
    let s = ANSI_ESCAPE_RE.replace_all(&s, "");
    s.graphemes(true).count()
}

#[derive(Debug)]
struct AnsiEscape {
    fullmatch: String,
    params: String,
    code: char,
}

// let text = "\x1B[1;31mHello\x1B[0m World \x1B[4mUnderlined\x1B[0m";
//
// let escapes: Vec<AnsiEscape> = re
//     .captures_iter(text)
//     .map(|cap| AnsiEscape {
//         full: cap[0].to_string(),
//         params: cap["params"].to_string(),
//         code: cap["code"].chars().next().unwrap(), // code is 1 char
//     })
//     .collect();
//
// for escape in escapes {
//     println!("{:?}", escape);
// }

fn graphemes_in_range_safe(s: &str, start: Option<usize>, end: Option<usize>) -> String {
    if start.is_none() && end.is_none() {
        return s.to_string();
    }

    let mut result = String::new();
    let us_start = start.unwrap_or(0);
    let graphemes = s.graphemes(true).skip(us_start);
    match end {
        Some(us_end) => {
            let nr = us_end.saturating_sub(us_start);
            graphemes.take(nr).for_each(|g| {
                result.push_str(g);
            });
        }
        None => {
            graphemes.for_each(|g| {
                result.push_str(g);
            });
        }
    }
    result
}

pub(crate) fn list_available_tasks(verbose: u8) -> Result<()> {
    let config_file_path = locate_config()?;
    let alchemist_config = parse_config(&config_file_path)?;

    if alchemist_config.tasks.is_empty() {
        terminal::warn("No tasks configured!");
        return Ok(());
    }

    // Filtering of hidden tasks unless verbose flag(s) are given.
    let task_names = alchemist_config
        .tasks
        .iter()
        .filter(|(_, v)| v.is_shown() || verbose >= 1)
        .map(|(k, v)| (k, v.describe()))
        .collect::<Vec<(&String, TaskDescription)>>();

    println!(" ┌──────────────────┐");
    println!(" │ Available tasks: │");
    println!(" ├──────────────────┘");
    if verbose > 0 {
        println!(" │");
    }

    let num_tasks = task_names.len();

    let usable_terminal_width = match terminal_size() {
        Some((Width(terminal_w), Height(_))) => terminal_w as usize,
        _ => TERMINAL_WIDTH_DEFAULT,
    } - INDENT_TASK_CONTENT;

    for (i, (task_name, description)) in task_names.into_iter().enumerate() {
        let (entry_prefix, desc_prefix) = if i == num_tasks - 1 {
            (" └", "  ")
        } else {
            (" ├", " │")
        };
        println!(
            "{} {} · {}",
            entry_prefix,
            task_name.bold(),
            description.task_type.yellow()
        );
        let desc = match verbose {
            0 => continue,
            1 => match description.description.len() {
                0..=5 => description.description,
                _ => [
                    &description.description[0..2],
                    &["...".blue().to_string()],
                    &description.description[description.description.len() - 2..],
                ]
                .concat(),
            },
            _ => description.description,
        };
        for line in desc {
            // Use graphemes for correct length and slicing (and prevent panic
            // via breaking up utf-8 unicode characters):
            let line_len = grapheme_length(&line);
            if line_len > usable_terminal_width {
                let line_part_len = (usable_terminal_width - 5) / 2;
                println!(
                    "{}    {}{}{}",
                    desc_prefix,
                    graphemes_in_range_safe(&line, None, Some(line_part_len)),
                    " ... ".blue(),
                    graphemes_in_range_safe(&line, Some(line_len - line_part_len), None)
                );
            } else {
                println!("{}    {}", desc_prefix, line);
            }
        }
        println!("{}", desc_prefix);
    }

    Ok(())
}
