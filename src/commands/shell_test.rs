use dto::Request;
use workflow::Work;
use workflow::Instruction::SystemCommand;
use config::Context;
use std::path::PathBuf;
use config::FileConfigSource;
use commands::shell::{execute_shell, execute_script};

#[test]
fn should_build_shell_system_command() {
    let args = vec![s!("a"), s!("a b"), s!("c")];

    let request = Request::new(&args, None);
    let command = s!("command");

    let work = execute_shell(&command, request);

    assert_eq!(work, Work::instruction(SystemCommand(s!("command 'a b' 'c'"), true)))
}

#[test]
fn should_build_script_system_command() {
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

    let work = execute_script(&command, request, &context);

    assert_eq!(work, Work::instruction(SystemCommand(s!("dm/command 'a b' 'c'"), true)))
}