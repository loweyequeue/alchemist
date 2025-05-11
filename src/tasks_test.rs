use super::*;

use crate::error::ErrorContext;

use indexmap::IndexMap;

//
// BasicTask tests:
//

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
            tasks: IndexMap::new(),
        },
    );
    assert_eq!(ret, Result::Ok(()));
}

#[test]
fn basic_task_env_vars() {
    let tmpdir = tempfile::tempdir().unwrap();
    let tmpfile = tmpdir.path().join("output.txt");
    let basic = AlchemistBasicTask {
        command: "sh".to_string(),
        args: Some(vec![
            "-c".to_string(),
            format!("echo $ALCHEMIST_TASKS_TEST_VAR > {}", &tmpfile.display()).to_string(),
        ]),
        env: Some(HashMap::from([(
            "ALCHEMIST_TASKS_TEST_VAR".to_string(),
            "VAR_VALUE".to_string(),
        )])),
        hide: None,
    };
    let ret = basic.run(
        "name",
        &AlchemistConfig {
            tasks: IndexMap::new(),
        },
    );

    assert!(ret.is_ok());
    assert!(tmpfile.exists());
    let tmpdata = std::fs::read(tmpfile).unwrap();
    let output = std::str::from_utf8(&tmpdata).unwrap();
    assert_eq!("VAR_VALUE\n", output);
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
            tasks: IndexMap::new(),
        },
    );
    assert_eq!(
        ret,
        Result::Err(AlchemistError::AssertionErrorVariant(ErrorContext(
            AssertionError(
                "While running basic task name, command `sh -c false` failed (non-zero exit code)."
                    .to_string()
            ),
            None
        )))
    );
}

#[test]
fn basic_task_spawn_failure() {
    let basic = AlchemistBasicTask {
        command: "/etc/passwd".to_string(),
        args: None,
        env: None,
        hide: None,
    };
    let ret = basic.run(
        "name",
        &AlchemistConfig {
            tasks: IndexMap::new(),
        },
    );
    let alchem_err = ret.as_ref().err().unwrap();
    let kind = match alchem_err {
        AlchemistError::IOErrorVariant(ErrorContext(v, _s)) => Some(v.kind()),
        _ => None,
    };
    assert_eq!(Some(std::io::ErrorKind::PermissionDenied), kind);

    let err_context_str = match alchem_err {
        AlchemistError::IOErrorVariant(ErrorContext(_v, s)) => s.to_owned(),
        _ => None,
    };
    assert_eq!(Some("Starting basic task name with command `/etc/passwd` either not found or insufficient permissions to run.".to_string()), err_context_str);
}

//
// ShellTask tests:
//

#[test]
fn shell_task() {
    let shell = AlchemistShellTask {
        shell_script: "true".to_string(),
        hide: None,
    };
    let ret = shell.run(
        "name",
        &AlchemistConfig {
            tasks: IndexMap::new(),
        },
    );
    assert_eq!(ret, Result::Ok(()));
}

#[test]
fn shell_task_nonzero_exit_code() {
    let shell = AlchemistShellTask {
        shell_script: "false".to_string(),
        hide: None,
    };
    let ret = shell.run(
        "name",
        &AlchemistConfig {
            tasks: IndexMap::new(),
        },
    );
    assert_eq!(
        ret,
        Result::Err(AlchemistError::AssertionErrorVariant(ErrorContext(
            AssertionError("Shell script 'name' exited with non-zero exit code.".to_string()),
            None
        )))
    );
}

//
// SerialTask tests:
//

#[test]
fn serial_task_empty() {
    let serial = AlchemistSerialTasks {
        serial_tasks: Vec::new(),
        hide: None,
    };
    let ret = serial.run(
        "name",
        &AlchemistConfig {
            tasks: IndexMap::new(),
        },
    );
    assert!(ret.is_ok());
}

#[test]
fn serial_tasks_ok() {
    let tmpdir = tempfile::tempdir().unwrap();
    let tmpfile = tmpdir.path().join("output.txt");
    let serial = AlchemistSerialTasks {
        serial_tasks: vec!["one".to_string(), "two".to_string()],
        hide: None,
    };
    let mut tasks: IndexMap<String, AlchemistTaskType> = IndexMap::new();

    tasks.insert(
        "one".to_string(),
        AlchemistBasicTask {
            command: "sh".to_string(),
            args: Some(vec![
                "-c".to_string(),
                format!("echo one >> {}", &tmpfile.display()).to_string(),
            ]),
            env: None,
            hide: None,
        }
        .into(),
    );
    tasks.insert(
        "two".to_string(),
        AlchemistBasicTask {
            command: "sh".to_string(),
            args: Some(vec![
                "-c".to_string(),
                format!("echo two >> {}", &tmpfile.display()).to_string(),
            ]),
            env: None,
            hide: None,
        }
        .into(),
    );

    let ret = serial.run("name", &AlchemistConfig { tasks: tasks });
    assert!(ret.is_ok());
    assert!(tmpfile.exists());
    let tmpdata = std::fs::read(tmpfile).unwrap();
    let output = std::str::from_utf8(&tmpdata).unwrap();
    assert_eq!("one\ntwo\n", output);
}

#[test]
fn test_serial_task_one_fail() {
    let serial = AlchemistSerialTasks {
        serial_tasks: vec!["one".to_string(), "two".to_string()],
        hide: None,
    };
    let mut tasks: IndexMap<String, AlchemistTaskType> = IndexMap::new();
    tasks.insert(
        "one".to_string(),
        AlchemistBasicTask {
            command: "sh".to_string(),
            args: Some(vec!["-c".to_string(), "true".to_string()]),
            env: None,
            hide: None,
        }
        .into(),
    );
    tasks.insert(
        "two".to_string(),
        AlchemistBasicTask {
            command: "sh".to_string(),
            args: Some(vec!["-c".to_string(), "false".to_string()]),
            env: None,
            hide: None,
        }
        .into(),
    );
    let ret = serial.run("name", &AlchemistConfig { tasks: tasks });
    assert_eq!(
        ret,
        Result::Err(AlchemistError::AssertionErrorVariant(ErrorContext(
            AssertionError(
                "While running basic task two, command `sh -c false` failed (non-zero exit code)."
                    .to_string()
            ),
            None
        )))
    );
}

//
// ParallelTasks tests:
//

#[test]
fn parallel_task_empty() {
    let parallel = AlchemistParallelTasks {
        parallel_tasks: Vec::new(),
        hide: None,
    };
    let ret = parallel.run(
        "name",
        &AlchemistConfig {
            tasks: IndexMap::new(),
        },
    );
    assert!(ret.is_ok());
}

#[test]
fn parallel_tasks_one_fail() {
    let parallel = AlchemistParallelTasks {
        parallel_tasks: vec!["one".to_string(), "two".to_string()],
        hide: None,
    };
    let mut tasks: IndexMap<String, AlchemistTaskType> = IndexMap::new();
    tasks.insert(
        "one".to_string(),
        AlchemistBasicTask {
            command: "sh".to_string(),
            args: Some(vec!["-c".to_string(), "true".to_string()]),
            env: None,
            hide: None,
        }
        .into(),
    );
    tasks.insert(
        "two".to_string(),
        AlchemistBasicTask {
            command: "sh".to_string(),
            args: Some(vec!["-c".to_string(), "false".to_string()]),
            env: None,
            hide: None,
        }
        .into(),
    );
    let ret = parallel.run("name", &AlchemistConfig { tasks: tasks });
    assert_eq!(
        ret,
        Result::Err(AlchemistError::AssertionErrorVariant(ErrorContext(
            AssertionError("One or more errors occoured in parallel tasks".to_string()),
            None
        )))
    );
}

#[test]
fn parallel_tasks_actually_run_parallel() {
    let tmpdir = tempfile::tempdir().unwrap();
    let tmpfile = tmpdir.path().join("output.txt");
    let parallel = AlchemistParallelTasks {
        parallel_tasks: vec!["one".to_string(), "two".to_string()],
        hide: None,
    };
    let mut tasks: IndexMap<String, AlchemistTaskType> = IndexMap::new();

    tasks.insert(
        "one".to_string(),
        AlchemistBasicTask {
            command: "sh".to_string(),
            args: Some(vec![
                "-c".to_string(),
                format!("sleep 0.2; echo one >> {}", &tmpfile.display()).to_string(),
            ]),
            env: None,
            hide: None,
        }
        .into(),
    );
    tasks.insert(
        "two".to_string(),
        AlchemistBasicTask {
            command: "sh".to_string(),
            args: Some(vec![
                "-c".to_string(),
                format!("echo two >> {}", &tmpfile.display()).to_string(),
            ]),
            env: None,
            hide: None,
        }
        .into(),
    );

    let ret = parallel.run("name", &AlchemistConfig { tasks: tasks });
    assert!(ret.is_ok());
    assert!(tmpfile.exists());
    let tmpdata = std::fs::read(tmpfile).unwrap();
    let output = std::str::from_utf8(&tmpdata).unwrap();
    assert_eq!("two\none\n", output);
}
