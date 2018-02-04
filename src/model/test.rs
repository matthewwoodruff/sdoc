use config::Context;
use workflow::{Work, Instruction};
use dto::Request;
use serde_yaml;
use super::*;
use test_helper::{a_command, a_context};
use model::Executable::Shell;

mod command {
    extern crate core;

    use super::*;

    #[test]
    fn should_match_command_name() {
        let command = a_command();

        let expected_command = s!("action");
        assert_eq!(command.matches(&expected_command, true), true);
    }

    #[test]
    fn should_match_command_alias() {
        let command = a_command();

        let expected_command = s!("a");
        assert_eq!(command.matches(&expected_command, true), true);
    }

    #[test]
    fn should_not_match_command_name() {
        let command = a_command();

        let expected_command = s!("abc");
        assert_eq!(command.matches(&expected_command, true), false);
    }

    #[test]
    fn should_deserialize_node_command() {
        let yaml_string =
            r#"---
name: stack
description: a description
type: Node
"#;
        let actual_command: Command = serde_yaml::from_str(yaml_string).expect("Failed to parse yaml");

        assert_eq!(actual_command.command_type, Executable::Node);
    }

    #[test]
    fn should_deserialize_full_command() {
        let yaml_string =
            r#"---
name: update
description: a description
min_args: 1
type:
    Script: update.sh
usage: <name>
dependencies:
    - value:
        Envar: NAME
      description: this is required
alias: h
"#;
        let d = Dependency {
            value: DependencyType::Envar(s!("NAME")),
            description: s!("this is required")
        };

        let expected_command = Command {
            name: s!("update"),
            description: s!("a description"),
            command_type: Executable::Script(s!("update.sh")),
            usage: Some(s!("<name>")),
            alias: Some(s!("h")),
            min_args: Some(1),
            dependencies: Some(vec![d])
        };

        let actual_command: Command = serde_yaml::from_str(yaml_string).expect("Failed to parse yaml");

        assert_eq!(actual_command, expected_command);
    }

    #[test]
    fn should_deserialize_shell_command() {
        let yaml_string =
            r#"---
name: update
description: a description
type:
    Shell: update.sh
"#;

        let actual_command: Command = serde_yaml::from_str(yaml_string).expect("Failed to parse yaml");

        assert_eq!(actual_command.command_type, Executable::Shell(s!("update.sh")));
    }

    #[test]
    fn should_execute_command() {
        let works = run(&a_command());

        assert_eq!(works, vec![Work::instruction(Instruction::SystemCommand(s!("dm stack "), true))])
    }

    #[test]
    fn should_check_dependencies_before_executing_command() {
        let dependency = Dependency {
            value: DependencyType::Envar(s!("EDITOR")),
            description: s!("Your favourite editor")
        };

        let command = Command {
            command_type: Shell(s!("echo hello world")),
            dependencies: Some(vec![dependency]),
            ..a_command()
        };

        let actual_work = run(&command);

        let dependency_check = Work::instruction(SystemCommand(s!("test -n \"$EDITOR\""), false))
            .on_error(Display(command.build_command_usage(&s!("dm a b c")), Response::Err(3)));
        let command_work = Work::instruction(SystemCommand(s!("echo hello world "), true));

        assert_eq!(actual_work, vec![dependency_check, command_work])
    }
}

fn run(command: &Command) -> Vec<Work> {
    let context = Context {
        resolved_commands: vec![s!("dm"), s!("a"), s!("b"), s!("c")],
        ..a_context()
    };
    let args: Vec<String> = vec![];
    let request = Request::new(&args, None);

    command.execute(request, &context)
}
