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

pub struct Context {
    pub directory: PathBuf,
    pub config: Vec<Section>,
    pub resolved_commands: Vec<String>,
    pub config_source: fn(path: &PathBuf) -> Vec<Section>
}

impl Context {
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
            config: (self.config_source)(&directory),
            directory,
            resolved_commands,
            config_source: self.config_source
        }
    }

    pub fn init(directory: PathBuf, config_source2: fn(path: &PathBuf) -> Vec<Section>) -> Context {
        Context {
            directory,
            config: vec![],
            resolved_commands: vec![],
            config_source: config_source2
        }
    }
}

pub fn file_config_source(path: &PathBuf) -> Vec<Section> {
    let x = path.join(s!("commands.yaml"));
    let mut v: Vec<Section> = serde_yaml::from_reader(File::open(x)
        .expect("Failed to open file")).unwrap();

    v.insert(0, commands::get_management_commands());
    v
}


