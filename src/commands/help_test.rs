use super::*;
use commands::help::{execute, auto_complete, build_help_without_builtins};
use config::Context;
use dto::{Request, Response};
use workflow::{Work, Instruction};
use workflow::Instruction::Display;
use test_helper::{a_command, a_context, a_section};

#[test]
fn should_build_help_text_when_command_not_found() {
    let context = a_context();
    let args = vec![s!("help"), s!("update")];
    let request = Request::new(&args, None);
    let actual_help = execute(request, &context);

    let expected_help_text = "
Usage: dm a b c <command> [args]

Management:
  help        h     Show help for all commands or a specific command
  edit        e     Edit a command
  view        v     View a command
  config      c     Edit configuration file
";

    assert_eq!(actual_help, Work::instruction(Display(s!(expected_help_text), Response::Err(1))));
}

#[test]
fn should_build_help_text_when_no_args_given() {
    let context = a_context();
    let args = vec![s!("help")];
    let request = Request::new(&args, None);
    let actual_help = execute(request, &context);

    let expected_help_text = "
Usage: dm a b c <command> [args]

Management:
  help        h     Show help for all commands or a specific command
  edit        e     Edit a command
  view        v     View a command
  config      c     Edit configuration file
";

    assert_eq!(actual_help, Work::instruction(Display(s!(expected_help_text), Response::Ok)));
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

    let context = Context {
        config: vec![section_1],
        ..a_context()
    };
    let args = vec![s!("help"), s!("action")];
    let request = Request::new(&args, None);

    let actual_help = execute(request, &context);

    let expected_help_text = "
Usage: dm a b c action -f FILENAME -a SOMETHING VALUE

a-description

Dependencies:
  ENV_VAR     Set this variable
";

    assert_eq!(actual_help, Work::instruction(Display(s!(expected_help_text), Response::Ok)));
}

#[test]
fn should_build_help_text_for_specific_command_with_no_dependencies() {
    let command = Command {
        name: s!("action"),
        description: s!("a-description"),
        usage: Some(s!("-f FILENAME -a SOMETHING VALUE")),
        dependencies: None,
        ..a_command()
    };

    let section_1 = Section {
        commands: vec![command],
        ..a_section()
    };

    let context = Context {
        config: vec![section_1],
        ..a_context()
    };
    let args = vec![s!("help"), s!("action")];
    let request = Request::new(&args, None);

    let help_text = execute(request, &context);

    let expected_help_text =
        "
Usage: dm a b c action -f FILENAME -a SOMETHING VALUE

a-description
";

    assert_eq!(help_text, Work::instruction(Display(s!(expected_help_text), Response::Ok)));
}

#[test]
fn should_show_auto_completion_when_command_not_found() {
    let command_1 = Command {
        name: s!("first-command"),
        ..a_command()
    };

    let command_2 = Command {
        name: s!("second-command"),
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

    let context = Context {
        config: vec![section_1, section_2],
        ..a_context()
    };
    let args = vec![s!("help"), s!("a")];
    let request = Request::new(&args, Some(1)).next();

    let work = auto_complete(request, &context);

    let expected_help_text =
        "first-command
second-command
";

    assert_eq!(work, Work::instruction(Instruction::Display(s!(expected_help_text), Response::Ok)));
}

#[test]
fn should_show_auto_completion_when_command_found() {
    let command_1 = Command {
        name: s!("first-command"),
        ..a_command()
    };

    let section_1 = Section {
        commands: vec![command_1],
        ..a_section()
    };

    let context = Context {
        config: vec![section_1],
        ..a_context()
    };
    let args = vec![s!("help"), s!("first-command")];
    let request = Request::new(&args, Some(1)).next();

    let work = auto_complete(request, &context);

    let expected_help_text =
        r#"first-command
"#;

    assert_eq!(work, Work::instruction(Instruction::Display(s!(expected_help_text), Response::Ok)));
}

#[test]
fn should_show_nothing_when_command_found_and_auto_completion_required_for_command() {
    let command_1 = Command {
        name: s!("first-command"),
        ..a_command()
    };

    let section_1 = Section {
        commands: vec![command_1],
        ..a_section()
    };

    let context = Context {
        config: vec![section_1],
        ..a_context()
    };
    let args = vec![s!("help"), s!("first-command")];
    let request = Request::new(&args, Some(1));

    let work = auto_complete(request, &context);

    assert_eq!(work, Work::instruction(Instruction::Display(s!(), Response::Ok)));
}

#[test]
fn should_build_help_text_without_builtin_commands() {
    let command = Command {
        name: s!("action"),
        description: s!("a-description"),
        usage: Some(s!("-f FILENAME -a SOMETHING VALUE")),
        dependencies: None,
        ..a_command()
    };

    let section_1 = Section {
        commands: vec![command],
        ..a_section()
    };

    let context = Context {
        config: vec![section_1],
        ..a_context()
    };

    let string = build_help_without_builtins(&context);

    let expected_help_text = "
Usage: dm a b c <command> [args]

Heading:
  action      a     a-description

Run 'dm a b c help' for more information
";

    assert_eq!(string , expected_help_text);
}
