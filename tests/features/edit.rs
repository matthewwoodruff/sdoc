use features::common::{run, SCRIPT, EDIT_USAGE};

#[test]
fn show_auto_complete() {
    run(&["edit"]).env("AUTO_COMPLETE", "2").output("script").succeeds();
}

#[test]
fn return_non_zero_exit_code_when_editing_a_non_existing_command() {
    run(&["edit", "unknown-command"]).output(EDIT_USAGE).exits_with(3);
}

#[test]
fn return_non_zero_exit_code_when_editor_is_not_set() {
    run(&["edit", "script"]).output(EDIT_USAGE).exits_with(3);
}

#[test]
fn only_allow_editing_of_script_commands() {
    run(&["edit", "print"]).env("EDITOR", "vim").output("").exits_with(1);
}

#[test]
fn allow_editor_to_be_configured() {
    run(&["edit", "script"]).env("EDITOR", "cat").output(SCRIPT).succeeds();
}