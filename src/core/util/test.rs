use core::util::run_system_command;
use core::dto::Response;

#[test]
fn should_execute_a_valid_system_command_and_return_success() {
    assert_eq!(run_system_command(s!("echo hello world")), Response::Ok)
}

#[test]
fn should_execute_an_invalid_system_command_and_return_unsuccessful() {
    assert_eq!(run_system_command(s!("invalid-command")), Response::Err(127))
}


