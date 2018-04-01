extern crate assert_cli;

use assert_cli::{Assert, Environment};

static HELP_TEXT: &'static str = "
Usage: sdoc <command> [args]

Management:
  help        h     Show help for all commands or a specific command
  edit        e     Edit a command
  view        v     View a command
  config      c     Edit configuration file

Node Commands:
  sub               A sub command

Commands:
  print       p     Prints hello world
  min-args          I require arguments
  deps              I have dependency requirements
  com-dep           I have command requirements
  script            A simple script";

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

fn environment() -> Environment {
    Environment::inherit()
        .insert("COMMANDS_DIRECTORY", "tests/data")
        .insert("CLI_NAME", "sdoc")
}
