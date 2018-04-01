use assert_cli::Assert;
use features::common::{SCRIPT, environment, expect_output, expect_output_given_env};

static EDIT_USAGE: &'static str = "
Usage: sdoc edit <command>

Edit a command

Dependencies:
  EDITOR      Set this environment variable to the editor of your choice

";

#[test]
fn show_auto_complete() {
    expect_output_given_env(environment().insert("AUTO_COMPLETE", "2"), &["edit"], "script");
}

#[test]
fn enable_viewing_a_shell_command() {
    expect_output(&["view", "print"], "echo hello world");
}

#[test]
fn return_non_zero_exit_code_when_editing_a_non_existing_command() {
    Assert::main_binary()
        .with_args(&["edit", "unknown-command"])
        .with_env(&environment())
        .fails_with(3)
        .stdout().is(EDIT_USAGE)
        .execute()
        .unwrap();
}

#[test]
fn return_non_zero_exit_code_when_editor_is_not_set() {
    Assert::main_binary()
        .with_args(&["edit", "script"])
        .with_env(&environment())
        .fails_with(3)
        .stdout().is(EDIT_USAGE)
        .execute()
        .unwrap();
}

#[test]
fn only_allow_editing_of_script_commands() {
    Assert::main_binary()
        .with_args(&["edit", "print"])
        .with_env(&environment().insert("EDITOR", "vim"))
        .fails_with(1)
        .stdout().is("")
        .execute()
        .unwrap();
}

#[test]
fn allow_editor_to_be_configured() {
    expect_output_given_env(environment().insert("EDITOR", "cat"), &["edit", "script"], SCRIPT);
}