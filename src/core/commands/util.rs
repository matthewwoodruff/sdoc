use core::workflow::Work;
use core::workflow::Instruction::Display;
use core::config::Context;

pub fn execute_auto_complete(context: &Context) -> Work {
    Work::instruction(Display(build_auto_complete(context)))
}

pub fn build_auto_complete(context: &Context) -> String {
    context.get_commands().iter()
        .fold(s!(), |a, b| format!("{}{}\n", a, &b.name))
}

