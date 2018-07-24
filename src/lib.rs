extern crate ansi_term;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

use cli::run_cli;
use commands::{help_command, init, util::no_auto_complete};
use config::file_config_source;
use model::{Command, Internal, Section};
use std::env::{args, var, VarError};
use std::path::PathBuf;

macro_rules! s {
    () => (String::new());
    ($($arg:tt)*) => (String::from($($arg)*));
}

mod commands;
mod model;
mod config;
mod dto;
mod util;
mod workflow;
mod cli;

#[cfg(test)]
mod test_helper;

pub fn run() {
    let args = build_args();
    match get_commands_directory() {
        Ok(commands_directory) => run_cli(commands_directory, args, file_config_source),
        Err(_) => run_cli(PathBuf::new(), args, init_config_source)
    }
}

fn get_commands_directory() -> Result<PathBuf, VarError> {
    var("COMMANDS_DIRECTORY").map(PathBuf::from)
}

fn build_args() -> Vec<String> {
    let mut args: Vec<String> = args().collect();

    if let Ok(a) = var("CLI_NAME") {
        args.remove(0);
        args.insert(0, a)
    }

    args
}

fn init_config_source(_: &PathBuf) -> Vec<Section> {
    let init_command = Command {
        value: None,
        internal: Internal {
            execute: init::run_setup,
            auto_complete: no_auto_complete,
        },
        description: s!("Initialise a new cli"),
        alias: Some(s!("i")),
        usage: Some(s!("[directory]")),
        name: s!("init"),
        min_args: None,
        dependencies: None,
    };

    let commands = Section {
        heading: s!("Commands"),
        commands: vec![init_command, help_command()],
        core: false
    };

    vec![commands]
}