use features::common::{HELP_TEXT, run, EDIT_USAGE};

static PRINT_HELP: &'static str = "\nUsage: sdoc print\n\nPrints hello world\n\n";

#[test]
fn show_help_for_a_specific_command() {
    run(&["help", "print"]).output(PRINT_HELP).succeeds();
}

#[test]
fn show_help_for_a_specific_command_alias() {
    run(&["help", "p"]).output(PRINT_HELP).succeeds();
}

#[test]
fn show_help_message_when_help_is_ran() {
    run(&["help"]).output(HELP_TEXT).succeeds();
}

#[test]
fn show_help_for_help_command() {
    run(&["help", "help"]).output("\nUsage: sdoc help [command]\n\nShow help for all commands or a specific command\n\n").succeeds();
}

#[test]
fn show_help_for_view_command() {
    run(&["help", "view"]).output("\nUsage: sdoc view\n\nView a command\n\n").succeeds();
}

#[test]
fn show_help_for_edit_command() {
    run(&["help", "edit"]).output(EDIT_USAGE).succeeds();
}

#[test]
fn show_help_for_config_command() {
    run(&["help", "config"]).output("
Usage: sdoc config

Edit configuration file

Dependencies:
  EDITOR      Set this environment variable to the editor of your choice

").succeeds();
}

#[test]
fn show_auto_complete() {
    run(&["help"])
        .env("AUTO_COMPLETE", "2")
        .output("\
help
edit
view
config
sub
print
min-args
deps
com-dep
script
").succeeds();
}

#[test]
fn return_non_zero_exit_code_when_asking_for_help_foir_nonexisting_command() {
    run(&["help", "unknown-command"]).output(HELP_TEXT).exits_with(1);
}