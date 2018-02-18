#[cfg(test)]
mod test;

use std::path::PathBuf;
use std::fs::File;
use serde_yaml;
use model::{Command, Section};
use commands;

pub trait ConfigSource {
    fn get_config(&self, path: &PathBuf) -> Vec<Section>;
}

pub struct Context<'a> {
    pub directory: PathBuf,
    pub config: Vec<Section>,
    pub resolved_commands: Vec<String>,
    pub config_source: &'a ConfigSource,
}

impl<'a> Context<'a> {
    pub fn get_commands(&self) -> Vec<&Command> {
        self.config.iter()
            .flat_map(|s| &s.commands)
            .collect()
    }

    pub fn find(&self, request_command: &String, match_alias: bool) -> Option<&Command> {
        self.config.iter()
            .flat_map(|s| &s.commands)
            .find(|c| c.matches(request_command, match_alias))
    }

    pub fn build_command_chain(&self) -> String {
        self.resolved_commands.join(" ")
    }

    pub fn next(&self, request_command: &String) -> Context {
        let resolved_command = self.find(request_command, true)
            .map(|c| &c.name)
            .unwrap_or(request_command)
            .to_owned();

        let directory = self.directory.join(&resolved_command);
        let mut resolved_commands = self.resolved_commands.to_vec();
        resolved_commands.push(resolved_command);

        Context {
            config: self.config_source.get_config(&directory),
            directory,
            resolved_commands,
            config_source: self.config_source,
        }
    }

    pub fn init(directory: PathBuf, config_source: &'a ConfigSource) -> Context<'a> {
        Context {
            directory,
            config: vec![],
            resolved_commands: vec![],
            config_source,
        }
    }
}

pub struct FileConfigSource;

impl ConfigSource for FileConfigSource {
    fn get_config(&self, path: &PathBuf) -> Vec<Section> {
        let x = path.join(s!("commands.yaml"));
        let mut v: Vec<Section> = serde_yaml::from_reader(File::open(x)
            .expect("Failed to open file")).unwrap();

        v.insert(0, commands::get_management_commands());
        v
    }
}


