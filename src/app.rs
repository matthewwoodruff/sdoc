use commands::node;
use config::{Context, FileConfigSource};
use dto::{Request, Response};
use std::{env, process, path};
use workflow;

pub fn run_app(commands_directory: path::PathBuf) {
    let args: Vec<String> = build_args();
    let request = build_request(&args);
    let context = Context::init(commands_directory, &FileConfigSource);
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

fn build_args() -> Vec<String> {
    let mut args: Vec<String> = env::args().collect();

    if let Ok(a) = env::var("CLI_NAME") {
        args.remove(0);
        args.insert(0, a)
    }

    args
}

fn build_request(args: &Vec<String>) -> Request {
    let completed: Option<usize> = env::var("AUTO_COMPLETE")
        .map(|s| Some(s.parse().unwrap()))
        .unwrap_or(None);

    Request::new(args, completed)
}