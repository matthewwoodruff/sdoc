use features::common::{HELP_TEXT, AUTO_COMPLETE, run};

#[test]
fn show_help_message_when_no_arguments_are_supplied() {
    run(&[]).output(HELP_TEXT).succeeds();
}

#[test]
fn show_help_message_when_unknown_command_is_given() {
    run(&["unknown-command"]).output(HELP_TEXT).exits_with(1);
}

#[test]
fn show_autocomplete_for_available_commands() {
    run(&[]).env("AUTO_COMPLETE", "1").output(AUTO_COMPLETE).succeeds();
}

#[test]
fn execute_a_sub_command() {
    run(&["sub", "print"]).output("hello world from sub command").succeeds();
}

#[test]
fn execute_a_command_with_alias() {
    run(&["p"]).output("hello world").succeeds();
}


