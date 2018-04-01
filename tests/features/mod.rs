mod script;
mod view;
mod edit;
mod help;
mod node;
mod config;
mod common;

use std::env;
use features::common::run;

#[test]
fn show_command_usage_when_args_are_insufficient() {
    run(&["min-args"])
        .output("
Usage: sdoc min-args <an-arg>

I require arguments

").exits_with(2);
}

#[test]
fn execute_a_command_with_required_args() {
    run(&["min-args", "an argument"]).output("The args are: an argument").succeeds();
}

#[test]
fn show_command_usage_when_dependency_check_fails_for_envar() {
    run(&["deps"])
        .output("
Usage: sdoc deps

I have dependency requirements

Dependencies:
  A_NAME      A name that I need

").exits_with(3);
}

#[test]
fn execute_a_command_when_dependencies_are_satisfied() {
    run(&["deps"]).env("A_NAME", "Steve").output("The name is Steve").succeeds();
}

#[test]
fn show_command_usage_when_dependency_check_fails_for_command() {
    run(&["com-dep"])
        .output("
Usage: sdoc com-dep

I have command requirements

Dependencies:
  test-com    A command that I need

").exits_with(3);
}

#[test]
fn execute_a_command_when_dependencies_are_satisfied_with_command() {
    let path = format!("{}{}", env::var("PATH").map(|v| format!("{}:", v)).unwrap_or("".to_owned()), "tests/data/test-bin");
    run(&["com-dep"]).env("PATH", path.as_str()).output("The command exists").succeeds();
}
