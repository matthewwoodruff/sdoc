mod script;
mod view;
mod edit;
mod help;
mod node;
mod shell;
mod config;

use assert_cli::Assert;
use std::env;
use common::{environment, expect_output, expect_output_given_env};

#[test]
fn show_command_usage_when_args_are_insufficient() {
    Assert::main_binary()
        .with_args(&["min-args"])
        .with_env(&environment())
        .fails_with(2)
        .stdout().is("
Usage: sdoc min-args <an-arg>

I require arguments

")
        .execute()
        .unwrap();
}

#[test]
fn execute_a_command_with_required_args() {
    expect_output(&["min-args", "an argument"], "The args are: an argument");
}

#[test]
fn show_command_usage_when_dependency_check_fails_for_envar() {
    Assert::main_binary()
        .with_args(&["deps"])
        .with_env(&environment())
        .fails_with(3)
        .stdout().is("
Usage: sdoc deps

I have dependency requirements

Dependencies:
  A_NAME      A name that I need

")
        .execute()
        .unwrap();
}

#[test]
fn execute_a_command_when_dependencies_are_satisfied() {
    expect_output_given_env(environment().insert("A_NAME", "Steve"), &["deps"], "The name is Steve");
}

#[test]
fn show_command_usage_when_dependency_check_fails_for_command() {
    Assert::main_binary()
        .with_args(&["com-dep"])
        .with_env(&environment())
        .fails_with(3)
        .stdout().is("
Usage: sdoc com-dep

I have command requirements

Dependencies:
  test-com    A command that I need

")
        .execute()
        .unwrap();
}

#[test]
fn execute_a_command_when_dependencies_are_satisfied_with_command() {
    let path = format!("{}{}", env::var("PATH").map(|v| format!("{}:", v)).unwrap_or("".to_owned()), "tests/data/test-bin");
    expect_output_given_env(environment().insert("PATH", path.as_str()), &["com-dep"], "The command exists");
}
