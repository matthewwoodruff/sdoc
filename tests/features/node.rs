use features::common::{AUTO_COMPLETE, HELP_TEXT_WITHOUT_BUILTINS, run};

#[test]
fn show_help_message_without_builtin_commands_when_no_arguments_are_supplied() {
    run(&[]).success().stdout(HELP_TEXT_WITHOUT_BUILTINS);
}

#[test]
fn show_help_message_without_builtin_commands_when_unknown_command_is_given() {
    run(&["unknown-command"]).code(1).stdout(HELP_TEXT_WITHOUT_BUILTINS);
}

#[test]
fn show_autocomplete_for_available_commands() {
    run(&[]).env("AUTO_COMPLETE", "1").success().stdout(AUTO_COMPLETE);
}

#[test]
fn show_autocomplete_for_a_sub_command() {
    run(&["sub"])
        .env("AUTO_COMPLETE", "2")
        .success()
        .stdout("\
help
edit
view
config
print

");
}

#[test]
fn execute_a_sub_command() {
    run(&["sub", "print"]).success().stdout("hello world from sub command\n");
}

#[test]
fn execute_a_command_with_alias() {
    run(&["p"]).success().stdout("hello world\n");
}


