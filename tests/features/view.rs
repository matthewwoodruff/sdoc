use assert_cli::Assert;
use common::{SCRIPT, environment, expect_output, expect_output_given_env};

#[test]
fn enable_viewing_a_shell_command() {
    expect_output(&["view", "print"], "echo hello world");
}

#[test]
fn enable_viewing_a_script_command() {
    expect_output(&["view", "script"], SCRIPT);
}

#[test]
fn show_auto_complete() {
    expect_output_given_env(environment().insert("AUTO_COMPLETE", "2"), &["view"], "\
print
min-args
deps
com-dep
script
");
}

#[test]
fn only_allow_viewing_of_script_and_shell_commands() {
    Assert::main_binary()
        .with_args(&["view", "edit"])
        .with_env(&environment())
        .fails_with(1)
        .execute()
        .unwrap();
}

#[test]
fn return_non_zero_exit_code_when_viewing_non_existing_command() {
    Assert::main_binary()
        .with_args(&["view", "unknown-command"])
        .with_env(&environment())
        .fails_with(1)
        .execute()
        .unwrap();
}
