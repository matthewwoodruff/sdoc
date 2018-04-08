use features::common::run_uninitialised;
use tempfile;
use ansi_term::Color::{Blue, Green};
use std::{fs, path::PathBuf};
use std::io::prelude::*;

pub static HELP_TEXT: &'static str = "
Usage: target/debug/sdoc <command> [args]

Commands:
  init        i     Initialise a new cli
  help        h     Show help for all commands or a specific command
";

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
        .input("y\ntest-cli\n")
        .output(expected_output)
        .succeeds();
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
        .input("y\ntest-cli\n")
        .succeeds();

    let bin_string = read_file(temp_path.join("bin/test-cli"));
    let yaml_string = read_file(temp_path.join("test-cli/commands.yaml"));

    let expected_bin = format!("\
#! /bin/bash -ue
dir=$(cd $( dirname \"{}\" ) && cd .. && pwd )
COMMANDS_DIRECTORY=\"$dir\" CLI_NAME='test-cli' sdoc \"$@\"", "${BASH_SOURCE[0]}");
    assert_eq!(bin_string, expected_bin);

    let asd = "\
---
- heading: Commands
  commands:
    - name: hello
      description: Prints hello world
      value:
        shell: echo hello world
      usage: ~
      alias: h
      dependencies: ~
      min_args: ~";
    assert_eq!(yaml_string, asd);
}

#[test]
fn show_help_message_when_no_arguments_are_supplied() {
    run_uninitialised(&[]).output(HELP_TEXT).succeeds();
}