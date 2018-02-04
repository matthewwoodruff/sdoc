#[cfg(test)]
mod test;

use std::path::PathBuf;
use std::fs::File;
use serde_yaml;
use core::model::{Command, Section, Executable, Dependency, DependencyType};

pub trait ConfigSource {
    fn get_config(&self, path: &PathBuf) -> Vec<Section>;
}

pub struct Context<'a> {
    pub directory: &'a PathBuf,
    pub exec_directory: PathBuf,
    pub config: Vec<Section>,
    pub resolved_commands: Vec<String>,
    pub config_source: &'a ConfigSource,
}

impl <'a> Context<'a> {
    pub fn get_commands(&self) -> Vec<&Command> {
        self.config.iter()
            .flat_map(|s| &s.commands)
            .collect()
    }

    pub fn find(&self, request_command: &String, match_alias: bool) -> Option<&Command> {
        self.config.iter()
            .flat_map(|s| &s.commands)
            .find(|c| {
                if match_alias { c.matches(request_command) } else { c.matches_command(request_command) }
            })
    }

    pub fn build_command_chain(&self) -> String {
        self.resolved_commands.join(" ")
    }

    pub fn exec_directory(&self, commands: &Vec<String>) -> PathBuf {
        commands.iter()
            .map(|c| format!("{}", c))
            .fold(self.directory.to_owned(), |a, b| a.join(b))
    }

    pub fn next(&self, request_command: &String) -> Context {
        let resolved_command = self.find(&request_command, true)
            .map(|c| &c.name)
            .unwrap_or(&request_command)
            .to_owned();

        let mut commands = self.resolved_commands.to_vec();
        commands.push(resolved_command);
        let exec_directory = self.exec_directory(&commands);

        Context {
            config: self.config_source.get_config(&exec_directory),
            directory: self.directory,
            resolved_commands: commands,
            exec_directory,
            config_source: self.config_source
        }
    }

    pub fn init(directory: &'a PathBuf, config_source: &'a ConfigSource) -> Context<'a> {
        Context {
            directory,
            exec_directory: directory.to_owned(),
            config: vec![],
            resolved_commands: vec![],
            config_source
        }
    }
}

pub struct FileConfigSource;
impl ConfigSource for FileConfigSource {
    fn get_config(&self, path: &PathBuf) -> Vec<Section> {
        let x = path.join(s!("commands.yaml"));
        let mut v: Vec<Section> = serde_yaml::from_reader(File::open(x)
            .expect("Failed to open file")).unwrap();

        v.insert(0, get_builtin_commands());
        v
    }
}

pub fn edit_command() -> Command {
    Command {
        command_type: Executable::Edit,
        description: s!("Edit the implementation of a command"),
        alias: Some(s!("e")),
        usage: Some(s!("<command>")),
        name: s!("edit"),
        min_args: Some(1),
        dependencies: Some(vec![
            Dependency {
                value: DependencyType::Envar(s!("EDITOR")),
                description: s!("Set this environment variable to the editor of your choice")
            }])
    }
}

pub fn help_command() -> Command {
    Command {
        command_type: Executable::Help,
        description: s!("Show help for all commands or a specific command"),
        alias: Some(s!("h")),
        usage: Some(s!("[command]")),
        name: s!("help"),
        dependencies: None,
        min_args: None
    }
}

pub fn view_command() -> Command {
    Command {
        command_type: Executable::View,
        description: s!("View the implementation of a command"),
        alias: Some(s!("v")),
        usage: None,
        name: s!("view"),
        min_args: None,
        dependencies: None
    }
}

pub fn edit_config_command() -> Command {
    Command {
        command_type: Executable::EditConfig,
        description: s!("Edit the configuration file"),
        alias: Some(s!("ec")),
        usage: None,
        name: s!("edit-config"),
        min_args: None,
        dependencies: Some(vec![
            Dependency {
                value: DependencyType::Envar(s!("$EDITOR")),
                description: s!("Set this environment variable to the editor of your choice")
            }])
    }
}

pub fn get_builtin_commands() -> Section {
    Section {
        heading: s!("Built-in Commands"),
        commands: vec![help_command(), edit_command(), edit_config_command(), view_command()]
    }
}