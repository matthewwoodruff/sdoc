use core::workflow::Work;
use core::dto::{Response, Request};
use core::commands::{help, util};
use core::config::Context;

pub fn execute(request: Request, context: &Context) -> Vec<Work> {
    match request.current {
        Some(t) => execute_next_command(request.next(), &context.next(t)),
        None => vec![Work::response(Response::Err(18))]
    }
}

fn execute_next_command(request: Request, context: &Context) -> Vec<Work> {
    match request.current.and_then(|c| context.find(c, true)) {
        Some(c) => c.execute(request, context),
        None => vec![help::execute_help(context)]
    }
}

pub fn execute_auto_complete(request: Request, context: &Context) -> Vec<Work> {
    match request.current {
        Some(t) => execute_next_command_auto_complete(request.next(), &context.next(t)),
        None => vec![Work::response(Response::Err(18))]
    }
}

fn execute_next_command_auto_complete(request: Request, context: &Context) -> Vec<Work> {
    if request.autocomplete() {
        vec![util::execute_auto_complete(context)]
    } else {
        match request.current.and_then(|c| context.find(c, true)) {
            Some(c) => c.execute_auto_complete(request, context),
            None => vec![]
        }
    }
}

