use core::model::{Executable, Command, Section};
use core::config::{Context, SectionSource, get_builtin_commands};
use std::path::PathBuf;

pub static IN_MEMORY_SECTION_SOURCE: SectionSource = SectionSource::InMemory("");

pub fn a_context<'a>(directory: &'a PathBuf) -> Context<'a> {
    Context {
        directory,
        exec_directory: directory.to_owned(),
        sections: vec![get_builtin_commands()],
        resolved_commands: vec![s!("dm"), s!("a"), s!("b"), s!("c")],
        section_source: &IN_MEMORY_SECTION_SOURCE
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