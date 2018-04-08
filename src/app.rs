use commands::node;
use config::Context;
use dto::{Request, Response};
use model::Section;
use std::{env, path::PathBuf, process};
use workflow;

pub fn run_app<'a>(commands_directory: PathBuf, args: Vec<String>, config_source: fn(path: &PathBuf) -> Vec<Section>) {
    let request = build_request(&args);
    let context = Context::init(commands_directory, config_source);
    let workflow = if request.autocomplete_enabled() {
        node::execute_auto_complete(request, &context)
    } else {
        node::execute(request, &context)
    };

    process::exit(
        match workflow::run_workflow(workflow, &workflow::SystemRunner) {
            Response::Ok => 0,
            Response::Err(v) => v
        })
}

fn build_request(args: &Vec<String>) -> Request {
    let completed: Option<usize> = env::var("AUTO_COMPLETE")
        .map(|s| Some(s.parse().unwrap()))
        .unwrap_or(None);

    Request::new(args, completed)
}