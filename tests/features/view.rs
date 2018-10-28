use features::common::{SCRIPT_WITH_EXTRA_NEW_LINE, run};

#[test]
fn enable_viewing_a_shell_command() {
    run(&["view", "print"]).success().stdout("echo hello world\n");
}

#[test]
fn enable_viewing_a_script_command() {
    run(&["view", "script"]).success().stdout(SCRIPT_WITH_EXTRA_NEW_LINE);
}

#[test]
fn show_auto_complete() {
    run(&["view"])
        .env("AUTO_COMPLETE", "2")
        .success()
        .stdout("\
print
min-args
deps
com-dep
script

");
}

#[test]
fn only_allow_viewing_of_script_and_shell_commands() {
    run(&["view", "edit"]).code(1).stdout("");
}

#[test]
fn return_non_zero_exit_code_when_viewing_non_existing_command() {
    run(&["view", "unknown-command"]).code(1).stdout("");
}
