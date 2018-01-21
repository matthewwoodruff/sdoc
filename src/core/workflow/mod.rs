use core::dto::Response;
use core::util;

#[derive( Debug, PartialEq)]
pub struct Work {
    action: Action,
    error: Option<Action>
}

impl Work {
    pub fn action(action: Action) -> Work {
        Work { action, error: None }
    }
    pub fn instruction(instruction: Instruction) -> Work {
        Work::action(Action::instruction(instruction))
    }
    pub fn response(response: Response) -> Work {
        Work::action(Action::just_response(response))
    }
    pub fn on_error(self, action: Action) -> Work {
        Work { error: Some(action), ..self }
    }
}

#[derive( Debug, PartialEq)]
pub struct Action {
    instruction: Option<Instruction>,
    response: Option<Response>
}

impl Action {
    pub fn instruction(instruction: Instruction) -> Action {
        Action { instruction: Some(instruction), response: None }
    }
    pub fn just_response(response: Response) -> Action {
        Action { instruction: None, response: Some(response) }
    }
    pub fn response(self, response: Response) -> Action {
        Action { response: Some(response), ..self }
    }
}

#[derive( Debug, PartialEq)]
pub enum Instruction {
    Display(String),
    SystemCommand(String, bool)
}

pub fn run_workflow(workflow: Vec<Work>) -> Response {
    let mut response = Response::Ok;
    'work: for work in workflow {
        if let Response::Err(error) = run_action(work.action) {
            response = work.error
                .map(|a| run_action(a))
                .unwrap_or(Response::Err(error));
            break 'work
        }
    }
    response
}

fn run_action(action: Action) -> Response {
    let response = action.response;
    action.instruction
        .map(|i| run_instruction(i))
        .map(|r| response.unwrap_or(r))
        .unwrap_or(Response::Ok)
}

fn run_instruction(instruction: Instruction) -> Response {
    match instruction {
        Instruction::Display(string) => {
            println!("{}", string);
            Response::Ok
        },
        Instruction::SystemCommand(shell, output) => {
            if output {
                util::run_system_command(shell)
            } else {
                util::run_system_command_ignoring_output(shell)
            }
        }
    }
}