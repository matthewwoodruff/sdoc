use features::common::{AUTO_COMPLETE, HELP_TEXT_WITHOUT_BUILTINS, run};

#[test]
fn show_help_message_without_builtin_commands_when_no_arguments_are_supplied() {
    run(&[]).output(HELP_TEXT_WITHOUT_BUILTINS).succeeds();
}

#[test]
fn show_help_message_without_builtin_commands_when_unknown_command_is_given() {
    run(&["unknown-command"]).output(HELP_TEXT_WITHOUT_BUILTINS).exits_with(1);
}

#[test]
fn show_autocomplete_for_available_commands() {
    run(&[]).env("AUTO_COMPLETE", "1").output(AUTO_COMPLETE).succeeds();
}

#[test]
fn show_autocomplete_for_a_sub_command() {
    run(&["sub"])
        .env("AUTO_COMPLETE", "2")
        .output("\
help
edit
view
config
print
").succeeds();
}

#[test]
fn execute_a_sub_command() {
    run(&["sub", "print"]).output("hello world from sub command").succeeds();
}

#[test]
fn execute_a_command_with_alias() {
    run(&["p"]).output("hello world").succeeds();
}


