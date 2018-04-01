extern crate assert_cli;

mod common;

use assert_cli::Assert;
use common::{HELP_TEXT, environment, expect_output_given_args};

static PRINT_HELP: &'static str = "\nUsage: sdoc print\n\nPrints hello world\n\n";

#[test]
fn show_help_for_a_specific_command() {
    expect_output_given_args(&["help", "print"],PRINT_HELP);
}

#[test]
fn show_help_for_a_specific_command_alias() {
    expect_output_given_args(&["help", "p"], PRINT_HELP);
}

#[test]
fn show_help_message_when_help_is_ran() {
    expect_output_given_args(&["help"], HELP_TEXT);
}

#[test]
fn show_help_for_help_command() {
    expect_output_given_args(&["help", "help"],"\nUsage: sdoc help [command]\n\nShow help for all commands or a specific command\n\n");
}

#[test]
fn show_help_for_view_command() {
    expect_output_given_args(&["help", "view"], "\nUsage: sdoc view\n\nView a command\n\n");
}

#[test]
fn show_help_for_edit_command() {
    expect_output_given_args(&["help", "edit"], "\

Usage: sdoc edit <command>

Edit a command

Dependencies:
  EDITOR      Set this environment variable to the editor of your choice

");
}

#[test]
fn show_help_for_config_command() {
    expect_output_given_args(&["help", "config"], "
Usage: sdoc config

Edit configuration file

Dependencies:
  EDITOR      Set this environment variable to the editor of your choice

");
}

#[test]
fn show_auto_complete() {
    let environment = environment().insert("AUTO_COMPLETE", "2");

    Assert::main_binary()
        .with_args(&["help"])
        .with_env(&environment)
        .succeeds()
        .stdout().is("\
help
edit
view
config
sub
print
min-args
deps
com-dep
script
")
        .execute()
        .unwrap();
}

#[test]
fn return_non_zero_sexit_code_when_asking_for_help_foir_nonexisting_command() {
    Assert::main_binary()
        .with_args(&["help", "unknown-command"])
        .with_env(&environment())
        .fails_with(1)
        .stdout().is(HELP_TEXT)
        .execute()
        .unwrap();
}