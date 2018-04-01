use assert_cli::{Environment,Assert};

pub static HELP_TEXT: &'static str = "
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

pub static AUTO_COMPLETE: &'static str = "\
help
edit
view
config
sub
print
min-args
deps
com-dep
script";

pub static SCRIPT: &'static str = "\
#! /bin/bash

echo I am a simple script
echo The number of args is $#
echo \"arg 1 is '$1'\"
echo \"arg 2 is '$2'\"

exit 0

";

pub fn environment() -> Environment {
    Environment::inherit()
        .insert("COMMANDS_DIRECTORY", "tests/data")
        .insert("CLI_NAME", "sdoc")
        .insert("EDITOR", "")
}

pub fn expect_output(args: &[&str], output: &str) {
    Assert::main_binary()
        .with_args(args)
        .with_env(&environment())
        .succeeds()
        .stdout().is(output)
        .execute()
        .unwrap();
}

pub fn expect_output_given_env(env: Environment, args: &[&str], output: &str) {
    Assert::main_binary()
        .with_args(args)
        .with_env(&env)
        .succeeds()
        .stdout().is(output)
        .execute()
        .unwrap();
}
