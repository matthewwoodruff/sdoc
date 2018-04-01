use assert_cli::Assert;
use features::common::{environment, expect_output_given_env};

#[test]
fn show_nothing_for_auto_complete() {
    expect_output_given_env(environment().insert("AUTO_COMPLETE", "2"), &["config"], "");
}

#[test]
fn allow_editor_to_be_configured() {
    expect_output_given_env(environment().insert("EDITOR", "shasum -a 256"), &["config"], "9e6aa1704493659f7b9fc420342711d5ea9fc2e3e149f1ff2d1da4ef81f8894e  tests/data/sdoc/commands.yaml");
}

#[test]
fn return_non_zero_exit_code_when_editor_is_not_set() {
    Assert::main_binary()
        .with_args(&["config"])
        .with_env(&environment())
        .fails_with(3)
        .stdout().is("
Usage: sdoc config

Edit configuration file

Dependencies:
  EDITOR      Set this environment variable to the editor of your choice

")
        .execute()
        .unwrap();
}
