mod script;
mod view;
mod edit;
mod help;
mod node;
mod config;
mod common;
//mod init;

use std::env;
use features::common::run;

#[test]
fn show_command_usage_when_args_are_insufficient() {
    run(&["min-args"])
        .code(2)
        .stdout("
Usage: example-cli min-args <an-arg>

I require arguments

");
}

#[test]
fn execute_a_command_with_required_args() {
    run(&["min-args", "an argument"])
        .success()
        .stdout("The args are: an argument\n");
}

#[test]
fn show_command_usage_when_dependency_check_fails_for_envar() {
    run(&["deps"])
        .code(3)
        .stdout("
Usage: example-cli deps

I have dependency requirements

Dependencies:
  A_NAME      A name that I need

");
}

#[test]
fn execute_a_command_when_dependencies_are_satisfied() {
    run(&["deps"]).env("A_NAME", "Steve").success().stdout("The name is Steve\n");
}

#[test]
fn show_command_usage_when_dependency_check_fails_for_command() {
    run(&["com-dep"])
        .code(3)
        .stdout("
Usage: example-cli com-dep

I have command requirements

Dependencies:
  test-com    A command that I need

");
}

#[test]
fn execute_a_command_when_dependencies_are_satisfied_with_command() {
    let path = format!("{}{}", env::var("PATH").map(|v| format!("{}:", v)).unwrap_or("".to_owned()), "tests/data/bin");
    run(&["com-dep"]).env("PATH", path.as_str()).success().stdout("The command exists\n");
}
