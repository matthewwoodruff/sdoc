use std::cell::RefCell;
use super::*;

struct TestRunner {
    system_runner: SystemRunner,
    executed_instructions: RefCell<Vec<Instruction>>
}

impl InstructionRunner for TestRunner {
    fn run(&self, instruction: &Instruction) -> Response {
        let mut x = self.executed_instructions.borrow_mut();
        x.push(instruction.clone());
        self.system_runner.run(instruction)
    }
}

#[test]
fn should_return_custom_response_for_displaying_text() {
    let work = Work::instruction(Instruction::Display(s!("hello world"), Response::Err(234)));

    assert_eq!(run_workflow(vec![work], &SystemRunner), Response::Err(234))
}

#[test]
fn should_return_error_code_for_invalid_system_command() {
    let work = Work::instruction(Instruction::SystemCommand(s!("invalid-command"), false));

    assert_eq!(run_workflow(vec![work], &SystemRunner), Response::Err(127))
}

#[test]
fn should_stop_executing_workflow_when_an_instruction_fails() {
    let work_1 = Work::instruction(Instruction::SystemCommand(s!("invalid-command"), false));
    let work_2 = Work::instruction(Instruction::Display(s!("hello world"), Response::Ok));

    let runner = TestRunner {
        system_runner: SystemRunner,
        executed_instructions: RefCell::new(vec![])
    };

    assert_eq!(run_workflow(vec![work_1, work_2], &runner), Response::Err(127));
    assert_eq!(runner.executed_instructions.into_inner(), vec![Instruction::SystemCommand(s!("invalid-command"), false)]);
}

#[test]
fn should_terminate_with_custom_exit_code() {
    let failing_instruction = Instruction::SystemCommand(s!("invalid-command"), false);
    let on_error_instruction = Instruction::Display(s!("An error"), Response::Err(10000));
    let expected_instructions = vec![failing_instruction.clone(), on_error_instruction.clone()];

    let work = Work::instruction(failing_instruction).on_error(on_error_instruction);

    let runner = TestRunner {
        system_runner: SystemRunner,
        executed_instructions: RefCell::new(vec![])
    };

    assert_eq!(run_workflow(vec![work], &runner), Response::Err(10000));
    assert_eq!(runner.executed_instructions.into_inner(), expected_instructions);
}

#[test]
fn env() {
    let failing_instruction = Instruction::SystemCommand(s!("invalid-command"), false);
    let on_error_instruction = Instruction::Display(s!("echo usage"), Response::Err(456));
    let display_instruction = Instruction::Display(s!("hello world"), Response::Ok);

    let expected_instructions = vec![failing_instruction.clone(), on_error_instruction.clone()];

    let work_1 = Work::instruction(failing_instruction).on_error(on_error_instruction);
    let work_2 = Work::instruction(display_instruction);

    let runner = TestRunner {
        system_runner: SystemRunner,
        executed_instructions: RefCell::new(vec![])
    };
    let response = run_workflow(vec![work_1, work_2], &runner);

    assert_eq!(response, Response::Err(456));
    assert_eq!(runner.executed_instructions.into_inner(), expected_instructions);
}
