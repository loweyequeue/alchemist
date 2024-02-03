use super::*;

#[test]
fn basic_task() {
    let basic = AlchemistBasicTask {
        command: "sh".to_string(),
        args: Some(vec!["-c".to_string(), "true".to_string()]),
        env: None,
        hide: None,
    };
    let ret = basic.run(
        "name",
        &AlchemistConfig {
            tasks: HashMap::new(),
        },
    );
    assert_eq!(ret, Result::Ok(()));
}

#[test]
fn basic_task_nonzero_exit_code() {
    let basic = AlchemistBasicTask {
        command: "sh".to_string(),
        args: Some(vec!["-c".to_string(), "false".to_string()]),
        env: None,
        hide: None,
    };
    let ret = basic.run(
        "name",
        &AlchemistConfig {
            tasks: HashMap::new(),
        },
    );
    assert_eq!(
        ret,
        Result::Err(AlchemistError::AssertionErrorVariant(oh_no::ErrorContext(
            AssertionError(
                "While running basic task name, command `sh -c false` failed (non-zero exit code)."
                    .to_string()
            ),
            None
        )))
    );
}
