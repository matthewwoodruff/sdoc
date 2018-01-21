#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_yaml;

macro_rules! s {
    () => (String::new());
    ($($arg:tt)*) => (String::from($($arg)*));
}

mod core;

use std::env;
use std::path::PathBuf;
use core::config::{SectionSource, Context};
use core::dto::Request;
use core::dto::Response;
use core::commands::node;
use core::workflow;

pub fn run() {
    let args: Vec<String> = build_args();
    let request = build_request(&args);
    let directory = get_top_level_directory();
    let section_source = SectionSource::File;
    let context = Context::init(&directory, &section_source);

    let workflow = if request.autocomplete_enabled() {
        node::execute_auto_complete(request, &context)
    } else {
        node::execute(request, &context)
    };

    std::process::exit(
        match workflow::run_workflow(workflow) {
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

fn get_top_level_directory() -> PathBuf {
    env::var("COMMANDS_DIRECTORY")
        .map(|a| PathBuf::from(a))
        .expect("COMMANDS_DIRECTORY - Environment variable should be set")
}