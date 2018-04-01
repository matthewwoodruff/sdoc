use assert_cli::Assert;
use common::{HELP_TEXT, AUTO_COMPLETE, environment, expect_output, expect_output_given_env};

#[test]
fn show_help_message_when_no_arguments_are_supplied() {
    expect_output(&[], HELP_TEXT);
}

#[test]
fn show_help_message_when_unknown_command_is_given() {
    Assert::main_binary()
        .with_args(&["unknown-command"])
        .with_env(&environment())
        .fails_with(1)
        .stdout().is(HELP_TEXT)
        .execute()
        .unwrap();
}

#[test]
fn show_autocomplete_for_available_commands() {
    expect_output_given_env(environment().insert("AUTO_COMPLETE", "1"), &[], AUTO_COMPLETE);
}

#[test]
fn execute_a_sub_command() {
    expect_output(&["sub", "print"], "hello world from sub command");
}

#[test]
fn execute_a_command_with_alias() {
    expect_output(&["p"], "hello world");
}


