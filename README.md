# Alchemist

Alchemist is a tool that can easily replace your `bin/` dirs full of scripts, or unnecessarily complicated `Makefile`s.

Alchemist aims to be as powerful as possible while staying extremely simple to use.

Jump to the [Installation](#Installation) to get started

## Getting started
**NOTE:** make sure you've checked [Installation](#Installation) first!

To use alchemist for a project simply run `alchemist --init` in the project root

This will create an `alchemist.toml` file with an example of each supported task type.

Then run `alchemist --list` to see all available tasks.

Then simply run one of the tasks with `alchemist my-task`.

Or multiple tasks (serially) with `alchemist my-task1 my-task2`

Check below for a more in-depth explanation on all task types.

## Tasks

### Basic Task

A simple task, execute command X with Yn arguments.

Inside your `alchemist.toml` create a new task with at least a `command` field

```toml
[tasks.my_basic_task]
command = "pwd"
```

Other optional fields:
- args:`list` Supply a list of arguments to the `command` (`args = ["hello", "world"]`)
- hide:`bool` Hide the task from `alchemist --list` (`hide = true`)
- env:`string` Set an environment variable for this task `env = { FOO = "BAR", BAZ = "BUZZ" }`

### Serial Task

A serial task is a collection of tasks that will be executed one after the other.

Inside your `alchemist.toml` create a new task with at least a `serial_tasks` field
```toml
[tasks.multiple_tasks]
serial_tasks = ["clean", "build", "run"]
```

All tasks in the `serial_tasks` list have to exist somewhere in the `alchemist.toml`. Hidden tasks can be used.

Other optional fields:
- hide:`bool` hide the task from `alchemist --list` (`hide = true`)

### Parallel Task

A parallel task is very similar to serial task except that all tasks will be ran at the same time and then be awaited.

Inside your `alchemist.toml` create a new task with at least a `parallel_tasks` field
```toml
[tasks.build_all]
parallel_tasks = ["build_server", "build_client"]
```

All tasks in the `parallel_tasks` list have to exist somewhere in the `alchemist.toml`. Hidden tasks can be used.

Other optional fields:
- hide:`bool` hide the task from `alchemist --list` (`hide = true`)

### Shell Script

Run a good 'ol shell script (executed in `sh`).

Inside your `alchemist.toml` create a new task with at least a `shell_script` field
```toml
[tasks.shell]
shell_script = '''
VAR="World"
echo Hello ${VAR}!
'''
```

Other optional fields:
- hide:`bool` hide the task from `alchemist --list` (`hide = true`)

## Advanced usage

Parallel tasks and serial tasks can be combined to run a series of tasks at the same time and await them before running another (series of) task(s).

A small example to build 2 rust binaries in parallel and run them only when done:
```toml
[tasks.build_server_dev]
hide = true
command = "cargo"
args = ["build", "--bin", "server"]

[tasks.build_client_dev]
hide = true
command = "cargo"
args = ["build", "--bin", "client"]

[tasks.run_prebuilt_dev_server]
hide = true
command = "./target/debug/server"

[tasks.run_prebuilt_dev_client]
hide = true
command = "./target/debug/client"

[tasks.build_dev]
parallel_tasks = ["build_server_dev", "build_client_dev"]

[tasks.run_prebuilt_dev]
hide = true
parallel_tasks = ["run_prebuilt_dev_server", "run_prebuilt_dev_client"]

[tasks.run]
serial_tasks = ["build_dev", "run_prebuilt_dev"]
```

With this example you can run `alchemist run` to build 2 binaries in parallel, then once they are both done, run them both in parallel (starting the server first).

## Installation

### Requirements
- Unix (POSIX shell `sh` & filesystem)
- [cargo](https://rustup.rs/)

### Building and installing
Simply clone the repo & do a local cargo install:

```sh
git clone https://github.com/jasonverbeek/alchemist.git
cd alchemist
cargo run -- install
```

### Optional Completions
Currently only implemented for the `fish`-shell.

Run: `alchemist --shell-complete` to generate the completions.
