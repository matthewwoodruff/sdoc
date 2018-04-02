use assert_cli::{Assert, Environment};

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

pub static EDIT_USAGE: &'static str = "
Usage: sdoc edit <command>

Edit a command

Dependencies:
  EDITOR      Set this environment variable to the editor of your choice

";

pub struct Harness {
    assert: Assert,
    env: Environment
}

impl Harness {
    pub fn env(self, key: &str, value: &str) -> Self {
        Harness {
            env: self.env.insert(key, value),
            ..self
        }
    }
    pub fn output(self, output: &str) -> Self {
        Harness {
            assert: self.assert.stdout().is(output),
            ..self
        }
    }
    pub fn exits_with(self, code: i32) {
        self.assert.with_env(&self.env).fails_with(code).unwrap();
    }
    pub fn succeeds(self) {
        self.assert.with_env(&self.env).succeeds().unwrap();
    }
}

pub fn run(args: &[&str]) -> Harness {
    Harness {
        assert: Assert::main_binary().with_args(args),
        env: Environment::inherit()
            .insert("COMMANDS_DIRECTORY", "tests/data")
            .insert("CLI_NAME", "sdoc")
            .insert("EDITOR", "")
    }
}
