use features::common::{HELP_TEXT, run, EDIT_USAGE};

static PRINT_HELP: &'static str = "\nUsage: example-cli print\n\nPrints hello world\n\n";

#[test]
fn show_help_for_a_specific_command() {
    run(&["help", "print"]).success().stdout(PRINT_HELP);
}

#[test]
fn show_help_for_a_specific_command_alias() {
    run(&["help", "p"]).success().stdout(PRINT_HELP);
}

#[test]
fn show_help_message_when_help_is_ran() {
    run(&["help"]).success().stdout(HELP_TEXT);
}

#[test]
fn show_help_for_help_command() {
    run(&["help", "help"]).success().stdout("\nUsage: example-cli help [command]\n\nShow help for all commands or a specific command\n\n");
}

#[test]
fn show_help_for_view_command() {
    run(&["help", "view"]).success().stdout("\nUsage: example-cli view\n\nView a command\n\n");
}

#[test]
fn show_help_for_edit_command() {
    run(&["help", "edit"]).success().stdout(EDIT_USAGE);
}

#[test]
fn show_help_for_config_command() {
    run(&["help", "config"])
        .success()
        .stdout("
Usage: example-cli config

Edit configuration file

Dependencies:
  EDITOR      Set this environment variable to the editor of your choice

");
}

#[test]
fn show_auto_complete() {
    run(&["help"])
        .env("AUTO_COMPLETE", "2")
        .success()
        .stdout("\
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

");
}

#[test]
fn return_non_zero_exit_code_when_asking_for_help_foir_nonexisting_command() {
    run(&["help", "unknown-command"]).code(1).stdout(HELP_TEXT);
}