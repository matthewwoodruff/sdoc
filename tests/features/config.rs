use features::common::run;

#[test]
fn show_nothing_for_auto_complete() {
    run(&["config"])
        .env("AUTO_COMPLETE", "2")
        .output("").succeeds();
}

#[test]
fn allow_editor_to_be_configured() {
    run(&["config"])
        .env("EDITOR", "shasum -a 256")
        .output("9e6aa1704493659f7b9fc420342711d5ea9fc2e3e149f1ff2d1da4ef81f8894e  tests/data/sdoc/commands.yaml").succeeds();
}

#[test]
fn return_non_zero_exit_code_when_editor_is_not_set() {
    run(&["config"])
        .output("
Usage: sdoc config

Edit configuration file

Dependencies:
  EDITOR      Set this environment variable to the editor of your choice

").exits_with(3);
}
