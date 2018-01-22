use core;
use core::model::{Command, Section};
use core::config::{Context, SectionSource};
use std::path::PathBuf;
use core::test_helper::{a_context, a_section, a_command};

#[test]
fn should_return_commands_for_all_sections() {
    let my_command_1 = Command { name: s!("first"), ..a_command() };
    let my_command_2 = Command { name: s!("second"), ..a_command() };
    let section_1 = Section { commands: vec![my_command_1], ..a_section() };
    let section_2 = Section { commands: vec![my_command_2], ..a_section() };

    let directory = PathBuf::new();

    let context = Context {
        sections: vec![section_1, section_2],
        ..a_context(&directory)
    };

    let actual_commands = context.get_commands();
    assert_eq!(actual_commands.len(), 2);
    assert_eq!(actual_commands[0].name, s!("first"));
    assert_eq!(actual_commands[1].name, s!("second"));
}

#[test]
fn should_build_initial_context_from_current_executable() {
    let current_executable = PathBuf::from("/a/b/c/dm");
    let section_source = SectionSource::InMemory("");
    let context = Context::init(&current_executable, &section_source);

    assert_eq!(context.resolved_commands.len(), 0);
    assert_eq!(context.directory, &PathBuf::from("/a/b/c/dm"));
    assert_eq!(context.sections.len(), 0);
}

#[test]
fn should_return_command_matching_command_name() {
    let directory = PathBuf::new();

    let context = a_context(&directory);

    let command = s!("edit");
    let actual_command = context.find(&command, false);

    assert_eq!(actual_command.unwrap(), &core::config::edit_command());
}

#[test]
fn should_return_command_matching_command_alias() {
    let directory = PathBuf::new();

    let context = a_context(&directory);

    let command = s!("e");
    let actual_command = context.find(&command, true);

    assert_eq!(actual_command.unwrap(), &core::config::edit_command());
}
