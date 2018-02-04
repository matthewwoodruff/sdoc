use util::run_system_command;
use dto::Response;

#[test]
fn should_execute_a_valid_system_command_and_return_success() {
    assert_eq!(run_system_command(&s!("echo hello world"), false), Response::Ok)
}

#[test]
fn should_execute_an_invalid_system_command_and_return_unsuccessful() {
    assert_eq!(run_system_command(&s!("invalid-command"), false), Response::Err(127))
}


