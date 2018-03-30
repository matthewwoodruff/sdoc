extern crate ansi_term;
extern crate serde;
extern crate serde_yaml;

#[macro_use]
extern crate serde_derive;

use app::run_app;
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
        .map(run_app)
        .unwrap_or_else(|_| run_setup());
}

fn get_commands_directory() -> Result<PathBuf, VarError> {
    env::var("COMMANDS_DIRECTORY")
        .map(|a| PathBuf::from(a))
}