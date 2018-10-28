use features::common::{run, SCRIPT, EDIT_USAGE};

#[test]
fn show_auto_complete() {
    run(&["edit"]).env("AUTO_COMPLETE", "2").success().stdout("script\n\n");
}

#[test]
fn return_non_zero_exit_code_when_editing_a_non_existing_command() {
    run(&["edit", "unknown-command"]).code(3).stdout(EDIT_USAGE);
}

#[test]
fn return_non_zero_exit_code_when_editor_is_not_set() {
    run(&["edit", "script"]).code(3).stdout(EDIT_USAGE);
}

#[test]
fn only_allow_editing_of_script_commands() {
    run(&["edit", "print"]).env("EDITOR", "vim").code(1).stdout("");
}

#[test]
fn allow_editor_to_be_configured() {
    run(&["edit", "script"]).env("EDITOR", "cat").success().stdout(SCRIPT);
}