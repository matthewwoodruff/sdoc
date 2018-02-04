#[cfg(test)]
mod test;

use dto::Response;
use util;

#[derive( Debug, PartialEq)]
pub struct Work {
    instruction: Instruction,
    error: Option<Instruction>
}

impl Work {
    pub fn instruction(instruction: Instruction) -> Work {
        Work { instruction, error: None }
    }
    pub fn response(response: Response) -> Work {
        Work { instruction: Instruction::ExitCode(response), error: None }
    }
    pub fn on_error(self, instruction: Instruction) -> Work {
        Work { error: Some(instruction), ..self }
    }

    fn run(self, runner: &InstructionRunner) -> Response {
        match runner.run(&self.instruction) {
            Response::Err(error) => {
                self.error
                    .map(|a| runner.run(&a))
                    .unwrap_or(Response::Err(error))
            },
            r => r
        }
    }
}

#[derive( Debug, PartialEq, Clone)]
pub enum Instruction {
    Display(String, Response),
    SystemCommand(String, bool),
    ExitCode(Response)
}

pub trait InstructionRunner {
    fn run(&self, instruction: &Instruction) -> Response;
}

pub struct SystemRunner;
impl InstructionRunner for SystemRunner {
    fn run(&self, instruction: &Instruction) -> Response {
        match *instruction {
            Instruction::Display(ref string, ref response) => {
                println!("{}", string);
                response.clone()
            },
            Instruction::SystemCommand(ref shell, output) => util::run_system_command(shell, output),
            Instruction::ExitCode(ref response) => response.clone()
        }
    }
}

pub fn run_workflow(workflow: Vec<Work>, runner: &InstructionRunner) -> Response {
    let mut response = Response::Ok;
    'work: for work in workflow {
        response = work.run(runner);
        if let Response::Err(_) = response {
            break 'work;
        }
    }
    response
}
