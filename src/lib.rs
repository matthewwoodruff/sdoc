#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_yaml;

macro_rules! s {
    () => (String::new());
    ($($arg:tt)*) => (String::from($($arg)*));
}

pub mod commands;
pub mod model;
pub mod config;
pub mod dto;
pub mod util;
pub mod workflow;
#[cfg(test)]
pub mod test_helper;

use std::env;
use std::path::PathBuf;
use config::{Context, FileConfigSource};
use dto::Request;
use dto::Response;
use commands::node;

pub fn run() {
    let args: Vec<String> = build_args();
    let request = build_request(&args);
    let context = Context::init(get_commands_directory(), &FileConfigSource);
    let workflow = if request.autocomplete_enabled() {
        node::execute_auto_complete(request, &context)
    } else {
        node::execute(request, &context)
    };

    std::process::exit(
        match workflow::run_workflow(workflow, &workflow::SystemRunner) {
            Response::Ok => 0,
            Response::Err(v) => v
        })
}

fn build_args() -> Vec<String> {
    let mut args: Vec<String> = env::args().collect();

    if let Ok(a) = env::var("CLI_NAME") {
        args.remove(0);
        args.insert(0, a)
    }

    args
}

fn build_request(args: &Vec<String>) -> Request {
    let completed : Option<usize> = env::var("AUTO_COMPLETE")
        .map(|s| Some(s.parse().unwrap()))
        .unwrap_or(None);

    Request::new(args, completed)
}

fn get_commands_directory() -> PathBuf {
    env::var("COMMANDS_DIRECTORY")
        .map(|a| PathBuf::from(a))
        .expect("COMMANDS_DIRECTORY - Environment variable should be set")
}