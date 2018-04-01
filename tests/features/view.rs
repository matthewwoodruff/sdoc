use features::common::{SCRIPT, run};

#[test]
fn enable_viewing_a_shell_command() {
    run(&["view", "print"]).output("echo hello world").succeeds();
}

#[test]
fn enable_viewing_a_script_command() {
    run(&["view", "script"]).output(SCRIPT).succeeds();
}

#[test]
fn show_auto_complete() {
    run(&["view"])
        .env("AUTO_COMPLETE", "2")
        .output("\
print
min-args
deps
com-dep
script
").succeeds();
}

#[test]
fn only_allow_viewing_of_script_and_shell_commands() {
    run(&["view", "edit"]).output("").exits_with(1);
}

#[test]
fn return_non_zero_exit_code_when_viewing_non_existing_command() {
    run(&["view", "unknown-command"]).output("").exits_with(1);
}
