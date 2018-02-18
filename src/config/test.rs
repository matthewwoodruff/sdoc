use super::*;
use test_helper::{a_context, a_section, a_command};

#[test]
fn should_return_commands_for_all_sections() {
    let my_command_1 = Command { name: s!("first"), ..a_command() };
    let my_command_2 = Command { name: s!("second"), ..a_command() };
    let section_1 = Section { commands: vec![my_command_1], ..a_section() };
    let section_2 = Section { commands: vec![my_command_2], ..a_section() };

    let context = Context {
        config: vec![section_1, section_2],
        ..a_context()
    };

    let actual_commands = context.get_commands();
    assert_eq!(actual_commands.len(), 2);
    assert_eq!(actual_commands[0].name, s!("first"));
    assert_eq!(actual_commands[1].name, s!("second"));
}

#[test]
fn should_build_initial_context_from_current_executable() {
    let current_executable = PathBuf::from("/a/b/c/dm");
    let config_source = FileConfigSource;
    let context = Context::init(current_executable, &config_source);

    assert_eq!(context.resolved_commands.len(), 0);
    assert_eq!(context.directory, PathBuf::from("/a/b/c/dm"));
    assert_eq!(context.config.len(), 0);
}

#[test]
fn should_return_command_matching_command_name() {
    let my_command = Command { name: s!("my-command"), ..a_command() };
    let section = Section { commands: vec![my_command], ..a_section() };
    let context = Context { config: vec![section], ..a_context() };

    let actual_command = context.find(&s!("my-command"), false);

    assert_eq!(actual_command.unwrap().name, s!("my-command"));
}

#[test]
fn should_return_command_matching_command_alias() {
    let my_command = Command { name: s!("my-command"), alias: Some(s!("mc")), ..a_command() };
    let section = Section { commands: vec![my_command], ..a_section() };
    let context = Context { config: vec![section], ..a_context() };

    let actual_command = context.find(&s!("mc"), true);

    assert_eq!(actual_command.unwrap().name, s!("my-command"));
}
