use model::{Command, Dependency, DependencyType, Internal, Section};

pub mod edit;
pub mod editconfig;
pub mod help;
pub mod node;
pub mod shell;
pub mod util;
pub mod view;

#[cfg(test)]
mod help_test;
#[cfg(test)]
mod shell_test;

pub fn edit_command() -> Command {
    Command {
        value: None,
        internal: Some(Internal {
            execute: edit::execute,
            auto_complete: edit::auto_complete,
        }),
        description: s!("Edit a command"),
        alias: Some(s!("e")),
        usage: Some(s!("<command>")),
        name: s!("edit"),
        min_args: Some(1),
        dependencies: Some(vec![
            Dependency {
                value: DependencyType::Envar(s!("EDITOR")),
                description: s!("Set this environment variable to the editor of your choice"),
            }]),
    }
}

pub fn help_command() -> Command {
    Command {
        value: None,
        internal: Some(Internal {
            execute: help::execute,
            auto_complete: help::auto_complete,
        }),
        description: s!("Show help for all commands or a specific command"),
        alias: Some(s!("h")),
        usage: Some(s!("[command]")),
        name: s!("help"),
        dependencies: None,
        min_args: None,
    }
}

pub fn view_command() -> Command {
    Command {
        value: None,
        internal: Some(Internal {
            execute: view::execute,
            auto_complete: view::auto_complete,
        }),
        description: s!("View a command"),
        alias: Some(s!("v")),
        usage: None,
        name: s!("view"),
        min_args: None,
        dependencies: None,
    }
}

pub fn edit_config_command() -> Command {
    Command {
        value: None,
        internal: Some(Internal {
            execute: editconfig::execute,
            auto_complete: util::no_auto_complete,
        }),
        description: s!("Edit configuration file"),
        alias: Some(s!("c")),
        usage: None,
        name: s!("config"),
        min_args: None,
        dependencies: Some(vec![
            Dependency {
                value: DependencyType::Envar(s!("EDITOR")),
                description: s!("Set this environment variable to the editor of your choice"),
            }]),
    }
}

pub fn get_management_commands() -> Section {
    Section {
        heading: s!("Management"),
        commands: vec![help_command(), edit_command(), view_command(), edit_config_command()],
    }
}