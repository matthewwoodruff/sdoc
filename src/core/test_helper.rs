use core::model::{Executable, Command, Section};
use core::config::{Context, get_builtin_commands, FileConfigSource};
use std::path::PathBuf;

pub static CONFIG_SOURCE: FileConfigSource = FileConfigSource;

pub fn a_context<'a>() -> Context<'a> {
    Context {
        directory: PathBuf::new(),
        config: vec![get_builtin_commands()],
        resolved_commands: vec![s!("dm"), s!("a"), s!("b"), s!("c")],
        config_source: &CONFIG_SOURCE
    }
}

pub fn a_section() -> Section {
    Section {
        heading: s!("Heading"),
        commands: vec![a_command()]
    }
}

pub fn a_command() -> Command {
    Command {
        name: s!("action"),
        description: s!("a-description"),
        command_type: Executable::Shell(s!("dm stack")),
        usage: None,
        alias: Some(s!("a")),
        dependencies: None,
        min_args: None,
    }
}