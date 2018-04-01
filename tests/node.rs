extern crate assert_cli;

mod common;

use assert_cli::Assert;
use common::{HELP_TEXT, AUTO_COMPLETE, environment};

#[test]
fn show_help_message_when_no_arguments_are_supplied() {
    Assert::main_binary()
        .with_env(&environment())
        .succeeds()
        .stdout().is(HELP_TEXT)
        .execute()
        .unwrap();
}

#[test]
fn show_help_message_when_unknown_command_is_given() {
    Assert::main_binary()
        .with_args(&["unknown-command"])
        .with_env(&environment())
        .fails_with(1)
        .stdout().is(HELP_TEXT)
        .execute()
        .unwrap();
}

#[test]
fn show_autocomplete_for_available_commands() {
    let environment = environment().insert("AUTO_COMPLETE", "1");
    Assert::main_binary()
        .with_env(&environment)
        .succeeds()
        .stdout().is(AUTO_COMPLETE)
        .execute()
        .unwrap();
}

#[test]
fn execute_a_sub_command() {
    Assert::main_binary()
        .with_args(&["sub", "print"])
        .with_env(&environment())
        .succeeds()
        .stdout().is("hello world from sub command")
        .execute()
        .unwrap();
}

#[test]
fn execute_a_command_with_alias() {
    Assert::main_binary()
        .with_args(&["p"])
        .with_env(&environment())
        .succeeds()
        .stdout().is("hello world")
        .execute()
        .unwrap();
}


