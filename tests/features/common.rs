use assert_cmd::prelude::*;
use std::process::{Command, Stdio};
use assert_cmd;

pub static HELP_TEXT: &'static str = "
Usage: example-cli <command> [args]

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
  script            A simple script

";

pub static HELP_TEXT_WITHOUT_BUILTINS: &'static str = "
Usage: example-cli <command> [args]

Node Commands:
  sub               A sub command

Commands:
  print       p     Prints hello world
  min-args          I require arguments
  deps              I have dependency requirements
  com-dep           I have command requirements
  script            A simple script

Run 'example-cli help' for more information

";

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
script

";

pub static SCRIPT: &'static str = "\
#! /bin/bash

echo I am a simple script
echo The number of args is $#
echo \"arg 1 is '$1'\"
echo \"arg 2 is '$2'\"

exit 0
";

pub static SCRIPT_WITH_EXTRA_NEW_LINE: &'static str = "\
#! /bin/bash

echo I am a simple script
echo The number of args is $#
echo \"arg 1 is '$1'\"
echo \"arg 2 is '$2'\"

exit 0

";

pub static EDIT_USAGE: &'static str = "
Usage: example-cli edit <command>

Edit a command

Dependencies:
  EDITOR      Set this environment variable to the editor of your choice

";

pub struct Harness {
    command: assert_cmd::Command
}

impl Harness {
    pub fn env(&mut self, key: &str, val: &str)  -> &mut Harness {
        self.command.env(key, val);
        self
    }

    pub fn stdin(&mut self, input: &str)  -> &mut Harness {
        self.command.write_stdin(input);
        self
    }

    pub fn success(&mut self) -> assert_cmd::assert::Assert {
        self.command.assert().success()
    }

    pub fn code(&mut self, code: i32) -> assert_cmd::assert::Assert {
        self.command.assert().code(code)
    }
}

pub fn run<'a>(args: &[&str]) -> Harness {
    let mut command = assert_cmd::Command::cargo_bin("sdoc").unwrap();
    command
        .env("COMMANDS_DIRECTORY", "tests/data")
        .env("CLI_NAME", "example-cli")
        .env("EDITOR", "")
        .args(args);

    Harness{ command }
}

pub fn run_uninitialised(args: &[&str]) -> Harness {
    let mut command = assert_cmd::Command::cargo_bin("sdoc").unwrap();
    command
        .args(args);

    Harness{ command }
}
