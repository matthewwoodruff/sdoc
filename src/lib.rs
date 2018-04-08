extern crate ansi_term;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

use app::run_app;
use config::file_config_source;
use init::run_setup;
use std::env;
use std::env::VarError;
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
mod app;
mod init;

#[cfg(test)]
mod test_helper;

pub fn run() {
    get_commands_directory()
        .map(|dir| run_app(dir, build_args(), file_config_source))
        .unwrap_or_else(|_| run_setup());
}

fn get_commands_directory() -> Result<PathBuf, VarError> {
    env::var("COMMANDS_DIRECTORY")
        .map(|a| PathBuf::from(a))
}

fn build_args() -> Vec<String> {
    let mut args: Vec<String> = env::args().collect();

    if let Ok(a) = env::var("CLI_NAME") {
        args.remove(0);
        args.insert(0, a)
    }

    args
}