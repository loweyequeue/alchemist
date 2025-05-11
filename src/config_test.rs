use super::*;

use crate::error::{AlchemistError, ErrorContext};

#[test]
fn locate_config_in_checked_out_repo() {
    let config = locate_config().unwrap();
    let nr_of_slashes_in_path = config
        .to_string_lossy()
        .chars()
        .filter(|c| *c == '/')
        .count();

    assert!(nr_of_slashes_in_path >= 2);
    assert_eq!(
        &config.file_name().unwrap().to_string_lossy(),
        "alchemist.toml"
    );
}

#[test]
fn local_config_in_tempdir() {
    let original_cwd = current_dir().unwrap();

    let tempdir = tempfile::tempdir().unwrap();
    let config_path = tempdir.path().join("alchemist.toml");
    let cmd_dir = tempdir.path().join("nested").join("deeper");
    fs::create_dir_all(&cmd_dir).unwrap();
    set_current_dir(&cmd_dir).unwrap();

    let config_not_found = locate_config().unwrap_err();
    assert_eq!(
        config_not_found,
        AlchemistError::AssertionErrorVariant(ErrorContext(
            AssertionError("'alchemist.toml' does not exist or is not a file.".to_string()),
            None
        ))
    );

    // create file like touch from config_path
    std::fs::File::create(&config_path).unwrap();
    let config_found = locate_config().unwrap();
    assert_eq!(
        std::fs::canonicalize(config_found).unwrap(),
        std::fs::canonicalize(config_path).unwrap()
    );

    set_current_dir(original_cwd).unwrap();
}

#[test]
fn parse_config_order_of_tasks() {
    let original_cwd = current_dir().unwrap(); // save original cwd

    // Create config in a tempdir
    let tempdir = tempfile::tempdir().unwrap();
    let config_path = tempdir.path().join("alchemist.toml");
    let task_template = r#"
        [tasks.taskX]
        command = ""
        args = []"#
        .to_string()
        + "\n\n";
    let mut content = String::new();
    for i in 1..=12 {
        content.push_str(&task_template.replace("X", &i.to_string()));
    }

    fs::write(&config_path, content).unwrap();
    set_current_dir(&tempdir).unwrap();

    let config = parse_config(&config_path).unwrap();
    assert_eq!(config.tasks.len(), 12);
    for (i, (task_name, task_enum)) in config.tasks.iter().enumerate() {
        // After 12 tasks in-order, smells ok.
        assert_eq!(task_name, &format!("task{}", i + 1));
    }

    set_current_dir(original_cwd).unwrap(); // restore original cwd
}

// TODO:
//  - more test for when config not found
//  - more fns to test
