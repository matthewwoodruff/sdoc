use dto::Request;
use workflow::Work;
use workflow::Instruction::SystemCommand;
use config::Context;
use std::path::PathBuf;
use commands::shell::execute_shell;
use test_helper::a_context;

#[test]
fn should_build_a_system_command_with_commands_directory_in_path() {
    let directory = PathBuf::from(s!("dm"));
    let context = Context {
        directory,
        ..a_context()
    };

    let args = vec![s!("a"), s!("a b"), s!("c")];

    let request = Request::new(&args, None);
    let command = s!("command");

    let work = execute_shell(&command, request.args, &context.directory);

    assert_eq!(work, Work::instruction(SystemCommand(s!("PATH=\"$PATH:dm\" command 'a b' 'c'"), true)))
}