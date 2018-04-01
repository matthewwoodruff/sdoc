use features::common::{expect_output};

#[test]
fn execute_a_script_command() {
    expect_output(&["script"], "I am a simple script
The number of args is 0
arg 1 is ''
arg 2 is ''");
}

#[test]
fn pass_arguments_to_script_correctly() {
    expect_output(&["script", "one two", "three"], "I am a simple script
The number of args is 2
arg 1 is 'one two'
arg 2 is 'three'");
}
