use model::{Value, Command, Section, default_behaviour};
use config::{Context, file_config_source};
use commands::{get_management_commands};
use std::path::PathBuf;

pub fn a_context() -> Context {
    Context {
        directory: PathBuf::new(),
        config: vec![get_management_commands()],
        resolved_commands: vec![s!("dm"), s!("a"), s!("b"), s!("c")],
        config_source: file_config_source
    }
}

pub fn a_section() -> Section {
    Section {
        heading: s!("Heading"),
        commands: vec![a_command()],
        core: false
    }
}

pub fn a_command() -> Command {
    Command {
        name: s!("action"),
        description: s!("a-description"),
        value: Some(Value::Shell(s!("dm stack"))),
        internal: default_behaviour(),
        usage: None,
        alias: Some(s!("a")),
        dependencies: None,
        min_args: None,
    }
}