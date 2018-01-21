use core::model::{Dependency, Command, Section, DependencyType};
use core::commands::help::build_help;
use core::commands::help::auto_complete;
use core::config::Context;
use core::dto::Request;
use std::path::PathBuf;
use core::workflow::{Work, Instruction};
use core::test_helper::{a_command, a_context, a_section};

#[test]
fn should_build_help_text_when_command_not_found() {
    let directory = PathBuf::new();
    let context = a_context(&directory);
    let args = vec![s!("update")];
    let request = Request::new(&args, None);
    let help_text = build_help(&request, &context);

    let expected_help_text =
        r#"
Welcome to the Depot Management CLI
===================================

Usage: dm a b c <command> [args]

Built-in Commands:
  help        h     Show help for all commands or a specific command
  edit        e     Edit the implementation of a command
  edit-config ec    Edit the configuration file
  view        v     View the implementation of a command
"#;

    assert_eq!(help_text, expected_help_text);
}

#[test]
fn should_build_usage_for_command_with_dependencies() {
    let env_var_dependency = Dependency {
        value: DependencyType::Envar(s!("ENV_VAR")),
        description: s!("Set this variable")
    };

    let command = Command {
        dependencies: Some(vec![env_var_dependency]),
        usage: Some(s!("-f FILENAME -a SOMETHING VALUE")),
        ..a_command()
    };

    let section_1 = Section {
        commands: vec![command],
        ..a_section()
    };

    let directory = PathBuf::new();

    let context = Context {
        sections: vec![section_1],
        ..a_context(&directory)
    };
    let args = vec![s!("help"), s!("action")];
    let request = Request::new(&args, None).next();

    let help_text = build_help(&request, &context);

    let expected_help_text =
        r#"
Usage: dm a b c action -f FILENAME -a SOMETHING VALUE

a-description

Dependencies:
  ENV_VAR     Set this variable
"#;

    assert_eq!(help_text, expected_help_text);
}

#[test]
fn should_build_help_text_for_specific_command_with_no_dependencies() {
    let command = Command {
        command: s!("action"),
        description: s!("a-description"),
        usage: Some(s!("-f FILENAME -a SOMETHING VALUE")),
        dependencies: None,
        ..a_command()
    };

    let section_1 = Section {
        commands: vec![command],
        ..a_section()
    };

    let directory = PathBuf::new();

    let context = Context {
        sections: vec![section_1],
        ..a_context(&directory)
    };
    let args = vec![s!("help"), s!("action")];
    let request = Request::new(&args, None).next();

    let help_text = build_help(&request, &context);

    let expected_help_text =
        r#"
Usage: dm a b c action -f FILENAME -a SOMETHING VALUE

a-description
"#;

    assert_eq!(help_text, expected_help_text);
}

#[test]
fn should_show_auto_completion_when_command_not_found() {
    let command_1 = Command {
        command: s!("first-command"),
        ..a_command()
    };

    let command_2 = Command {
        command: s!("second-command"),
        ..a_command()
    };

    let section_1 = Section {
        commands: vec![command_1],
        ..a_section()
    };

    let section_2 = Section {
        commands: vec![command_2],
        ..a_section()
    };

    let directory = PathBuf::new();

    let context = Context {
        sections: vec![section_1, section_2],
        ..a_context(&directory)
    };
    let args = vec![s!("help"), s!("a")];
    let request = Request::new(&args, Some(1)).next();

    let work = auto_complete(request, &context);

    let expected_help_text =
        r#"first-command
second-command
"#;

    assert_eq!(work, Work::instruction(Instruction::Display(String::from(expected_help_text))));
}

#[test]
fn should_show_auto_completion_when_command_found() {
    let command_1 = Command {
        command: s!("first-command"),
        ..a_command()
    };

    let section_1 = Section {
        commands: vec![command_1],
        ..a_section()
    };

    let directory = PathBuf::new();

    let context = Context {
        sections: vec![section_1],
        ..a_context(&directory)
    };
    let args = vec![s!("help"), s!("first-command")];
    let request = Request::new(&args, Some(1)).next();

    let work = auto_complete(request, &context);

    let expected_help_text =
        r#"first-command
"#;

    assert_eq!(work, Work::instruction(Instruction::Display(String::from(expected_help_text))));
}

#[test]
fn should_show_nothing_when_command_found_and_auto_completion_required_for_command() {
    let command_1 = Command {
        command: s!("first-command"),
        ..a_command()
    };

    let section_1 = Section {
        commands: vec![command_1],
        ..a_section()
    };

    let directory = PathBuf::new();

    let context = Context {
        sections: vec![section_1],
        ..a_context(&directory)
    };
    let args = vec![s!("help"), s!("first-command")];
    let request = Request::new(&args, Some(1));

    let work = auto_complete(request, &context);

    assert_eq!(work, Work::instruction(Instruction::Display(s!())));
}
