extern crate assert_cli;

mod common;

use assert_cli::Assert;
use common::{HELP_TEXT, environment};

static PRINT_HELP: &'static str = "\nUsage: sdoc print\n\nPrints hello world\n\n";

#[test]
fn show_help_for_a_specific_command() {
    Assert::main_binary()
        .with_args(&["help", "print"])
        .with_env(&environment())
        .succeeds()
        .stdout().is(PRINT_HELP)
        .execute()
        .unwrap();
}

#[test]
fn show_help_for_a_specific_command_alias() {
    Assert::main_binary()
        .with_args(&["help", "p"])
        .with_env(&environment())
        .succeeds()
        .stdout().is(PRINT_HELP)
        .execute()
        .unwrap();
}

#[test]
fn show_help_message_when_help_is_ran() {
    Assert::main_binary()
        .with_args(&["help"])
        .with_env(&environment())
        .succeeds()
        .stdout().is(HELP_TEXT)
        .execute()
        .unwrap();
}

#[test]
fn show_help_for_help_command() {
    Assert::main_binary()
        .with_args(&["help", "help"])
        .with_env(&environment())
        .succeeds()
        .stdout().is("\nUsage: sdoc help [command]\n\nShow help for all commands or a specific command\n\n")
        .execute()
        .unwrap();
}

#[test]
fn show_help_for_view_command() {
    Assert::main_binary()
        .with_args(&["help", "view"])
        .with_env(&environment())
        .succeeds()
        .stdout().is("\nUsage: sdoc view\n\nView a command\n\n")
        .execute()
        .unwrap();
}

#[test]
fn show_help_for_edit_command() {
    Assert::main_binary()
        .with_args(&["help", "edit"])
        .with_env(&environment())
        .succeeds()
        .stdout().is("\

Usage: sdoc edit <command>

Edit a command

Dependencies:
  EDITOR      Set this environment variable to the editor of your choice

")
        .execute()
        .unwrap();
}

#[test]
fn show_help_for_config_command() {
    Assert::main_binary()
        .with_args(&["help", "config"])
        .with_env(&environment())
        .succeeds()
        .stdout().is("
Usage: sdoc config

Edit configuration file

Dependencies:
  EDITOR      Set this environment variable to the editor of your choice

")
        .execute()
        .unwrap();
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