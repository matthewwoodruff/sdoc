extern crate assert_cli;

use assert_cli::Environment;

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

pub fn environment() -> Environment {
    Environment::inherit()
        .insert("COMMANDS_DIRECTORY", "tests/data")
        .insert("CLI_NAME", "sdoc")
}
