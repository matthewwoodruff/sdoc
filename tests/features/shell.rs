use features::common::expect_output;

#[test]
fn execute_a_shell_command() {
    expect_output(&["print"], "hello world");
}