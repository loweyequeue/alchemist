use serde::Deserialize;
use std::{collections::HashMap, fs, path::Path};

use colored::Colorize;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const CONFIG_FILE: &str = "alchemist.toml";

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum AlchemistTaskType {
    AlchemistCollectiveTask {
        subRecipes: Vec<String>,
    },
    AlchemistBasicTask {
        command: String,
        args: Option<Vec<String>>,
    },
}

#[derive(Debug, Deserialize)]
struct AlchemistConfig {
    tasks: HashMap<String, AlchemistTaskType>,
}

fn main() {
    println!("{} version {}", "alchemist".green(), VERSION.yellow());
    println!("searching for {}", CONFIG_FILE);

    let config_file_path = Path::new(CONFIG_FILE);

    if !config_file_path.exists() {
        panic!("config file does not exist")
    }
    if !config_file_path.is_file() {
        panic!("config file is a not a file");
    }

    let config_file_content: String = fs::read_to_string(CONFIG_FILE).unwrap();
    let y: AlchemistConfig = toml::from_str(&config_file_content).unwrap();

    //println!("{:#?}", y);
    match &y.tasks["something"] {
        // TODO: without extracting keys, we want ze strukt
        AlchemistTaskType::AlchemistBasicTask { command, args } => {
            println!("basic: {:#?}, {:#?}", command, args)
        }
        AlchemistTaskType::AlchemistCollectiveTask { subRecipes } => {
            println!("collective: {:#?}", subRecipes)
        }
    }
}
