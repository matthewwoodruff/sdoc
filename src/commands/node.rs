use workflow::Work;
use dto::Request;
use commands::{help, util};
use config::Context;

pub fn execute(request: Request, context: &Context) -> Vec<Work> {
    let command = request.current.unwrap();
    execute_next_command(request.next(), &context.next(command))
}

fn execute_next_command(request: Request, context: &Context) -> Vec<Work> {
    match request.current.and_then(|c| context.find(c, true)) {
        Some(c) => c.execute(request, context),
        None => vec![help::execute_help(request, context)]
    }
}

pub fn execute_auto_complete(request: Request, context: &Context) -> Vec<Work> {
    let command = request.current.unwrap();
    execute_next_command_auto_complete(request.next(), &context.next(command))
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

