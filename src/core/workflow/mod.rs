use core::dto::Response;
use core::util;

#[derive( Debug, PartialEq)]
pub enum Instruction {
    Display(String),
    SystemCommand(String, bool)
}

#[derive( Debug, PartialEq)]
pub struct Action {
    instruction: Option<Instruction>,
    response: Option<Response>
}

impl Action {
    pub fn new() -> Action {
        Action { instruction: None, response: None }
    }

    pub fn instruction(self, instruction: Instruction) -> Action {
        Action { instruction: Some(instruction), ..self }
    }

    pub fn response(self, response: Response) -> Action {
        Action { response: Some(response), ..self }
    }
}

pub trait InstructionRunner {
    fn run(&mut self, instruction: Instruction) -> Response;
}

pub struct SystemInstructionRunner;
impl <'a> InstructionRunner for &'a SystemInstructionRunner {
    fn run(&mut self, instruction: Instruction) -> Response {
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
}

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
        Work::action(Action::new().instruction(instruction))
    }
    pub fn response(response: Response) -> Work {
        Work::action(Action::new().response(response))
    }
    pub fn on_error(self, action: Action) -> Work {
        Work { error: Some(action), ..self }
    }
}

fn run_action<T: InstructionRunner>(action: Action, instruction_runner: &mut T) -> Response {
    let response = action.response;
    action.instruction
        .map(|i| instruction_runner.run(i))
        .map(|r| response.unwrap_or(r))
        .unwrap_or(Response::Ok)
}

pub fn run<T: InstructionRunner>(workflow: Vec<Work>, mut instruction_runner: T) -> Response {
    let mut response = Response::Ok;
    'work: for work in workflow {
        if let Response::Err(error) = run_action(work.action, &mut instruction_runner) {
            response = work.error
                .map(|a| run_action(a, &mut instruction_runner))
                .unwrap_or(Response::Err(error));
            break 'work
        }
    }
    response
}
