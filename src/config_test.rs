use crate::error::{AlchemistError, ErrorContext};

use super::*;

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

// TODO:
//  - more test for when config not found
//  - more fns to test
