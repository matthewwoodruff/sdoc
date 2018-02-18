use dto::Request;
use workflow::Work;
use workflow::Instruction::SystemCommand;
use config::Context;
use std::path::PathBuf;
use config::FileConfigSource;
use commands::shell::execute_shell;

#[test]
fn should_build_a_system_command_with_commands_directory_in_path() {
    let directory = PathBuf::from(s!("dm"));
    let config_source = FileConfigSource;
    let context = Context {
        directory,
        config: vec![],
        resolved_commands: vec![],
        config_source: &config_source
    };

    let args = vec![s!("a"), s!("a b"), s!("c")];

    let request = Request::new(&args, None);
    let command = s!("command");

    let work = execute_shell(&command, request, &context);

    assert_eq!(work, Work::instruction(SystemCommand(s!("PATH=\"$PATH:dm\" command 'a b' 'c'"), true)))
}