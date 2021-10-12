use features::common::{run_uninitialised, get_bin_path};
use tempfile;
use ansi_term::Color::{Blue, Green};
use std::{fs, path::PathBuf};
use std::io::prelude::*;
use std::env;
use std::process::Command;
use assert_cmd;

#[test]
fn should_show_correct_output_to_the_user() {

    let temp_dir = tempfile::tempdir().unwrap();
    let temp_dir_string = temp_dir.path().to_str().unwrap();
    let expected_output = format!("\
{}
Setup a new CLI in \"{}\"? (y/n)
Enter your CLI name:
{}
Execute ./bin/test-cli to begin. Even better, add '$(pwd)/bin' to your $PATH
", Blue.paint("sdoc init"), temp_dir_string, Green.paint("Setup Complete"));

    run_uninitialised(&["init", temp_dir_string])
        .stdin("y\ntest-cli\n")
        .success()
        .stdout(expected_output);
}

fn read_file(path: PathBuf) -> String {
    let mut file = fs::File::open(path).unwrap();
    let mut string = String::new();
    file.read_to_string(&mut string).unwrap();
    string
}

#[test]
fn should_create_bin_and_commands_yaml() {
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path();
    let temp_dir_string = temp_path.to_str().unwrap();

    run_uninitialised(&["init", temp_dir_string])
        .stdin("y\ntest-cli\n")
        .success();

    let bin_string = read_file(temp_path.join("bin/test-cli"));
    let yaml_string = read_file(temp_path.join("test-cli/commands.yaml"));

    let expected_bin = format!("\
#! /bin/bash -ue
dir=$(cd $( dirname \"{}\" ) && cd .. && pwd )
COMMANDS_DIRECTORY=\"$dir\" CLI_NAME='test-cli' sdoc \"$@\"", "${BASH_SOURCE[0]}");
    assert_eq!(bin_string, expected_bin);

    let expected_yaml = "\
---
- heading: Commands
  commands:
    - name: hello
      description: Prints hello world
      value:
        shell: echo hello world
      usage: ~
      alias: hw
      dependencies: ~
      min_args: ~
";
    assert_eq!(yaml_string, expected_yaml);
}

#[test]
fn should_allow_bootstrap_script_to_be_executed_successfully() {
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path();
    let temp_dir_string = temp_path.to_str().unwrap();

    run_uninitialised(&["init", temp_dir_string])
        .stdin("y\ntest-cli\n")
        .success();

    assert_cmd::Command::new(temp_path.join("bin/test-cli"))
        .assert()
        .success()
        .stdout("
Usage: test-cli <command> [args]

Commands:
  hello       hw    Prints hello world

Run 'test-cli help' for more information

");
}

#[test]
fn should_only_accept_y_or_n_when_prompting_to_setup_cli() {
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_dir_string = temp_dir.path().to_str().unwrap();
    let expected_output = format!("\
{}
Setup a new CLI in \"{}\"? (y/n)
Setup a new CLI in \"{}\"? (y/n)
Setup a new CLI in \"{}\"? (y/n)
Goodbye
", Blue.paint("sdoc init"), temp_dir_string, temp_dir_string, temp_dir_string);

    run_uninitialised(&["init", temp_dir_string])
        .stdin("bad-input\nyes\nn\n")
        .success()
        .stdout(expected_output);
}

#[test]
fn should_allow_the_user_to_cancel_setting_up_a_cli() {
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path();
    let temp_dir_string = temp_path.to_str().unwrap();
    let expected_output = format!("\
{}
Setup a new CLI in \"{}\"? (y/n)
Goodbye
", Blue.paint("sdoc init"), temp_dir_string);

    run_uninitialised(&["init", temp_dir_string])
        .stdin("n\n")
        .success()
        .stdout(expected_output);

    assert_eq!(temp_path.join("bin").is_dir(), false);
    assert_eq!(temp_path.join("test-cli").is_dir(), false);
}

#[test]
fn should_use_current_directory_when_one_is_not_specified() {
    let current_directory = env::current_dir().unwrap();

    let expected_output = format!("\
{}
Setup a new CLI in \"{}\"? (y/n)
Goodbye
", Blue.paint("sdoc init"), current_directory.to_str().unwrap());

    run_uninitialised(&["init"])
        .stdin("n\n")
        .success()
        .stdout(expected_output);
}

#[test]
fn should_require_a_cli_name() {
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_dir_string = temp_dir.path().to_str().unwrap();

    let expected_output = format!("\
{}
Setup a new CLI in \"{}\"? (y/n)
Enter your CLI name:
Enter your CLI name:
Enter your CLI name:
{}
Execute ./bin/test-cli to begin. Even better, add '$(pwd)/bin' to your $PATH
", Blue.paint("sdoc init"), temp_dir_string, Green.paint("Setup Complete"));

    run_uninitialised(&["init", temp_dir_string])
        .stdin("y\n\n  \ntest-cli")
        .success()
        .stdout(expected_output);
}

#[test]
fn show_help_message_when_no_arguments_are_supplied() {
    let mut harness = run_uninitialised(&[]);

    let cmd = get_bin_path().into_os_string().into_string().unwrap();

    harness
        .success()
        .stdout(format!("
Usage: {} <command> [args]

Commands:
  init        i     Initialise a new cli
  help        h     Show help for all commands or a specific command

Run '{} help' for more information

", cmd, cmd));
}