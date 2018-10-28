use features::common::run;

#[test]
fn execute_a_script_command() {
    run(&["script"])
        .success()
        .stdout("I am a simple script
The number of args is 0
arg 1 is ''
arg 2 is ''
");
}

#[test]
fn pass_arguments_to_script_correctly() {
    run(&["script", "one two", "three"])
        .success()
        .stdout("I am a simple script
The number of args is 2
arg 1 is 'one two'
arg 2 is 'three'
");
}
