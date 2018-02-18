use workflow::{Work, Instruction};
use workflow::Instruction::Display;
use config::Context;
use dto::{Request, Response};

pub fn execute_auto_complete(context: &Context) -> Work {
    Work::instruction(Display(build_auto_complete(context), Response::Ok))
}

pub fn build_auto_complete(context: &Context) -> String {
    context.get_commands().iter().fold(s!(), |a, b| format!("{}{}\n", a, &b.name))
}

pub fn no_auto_complete(_: Request, _: &Context) -> Work {
    Work::instruction(Instruction::ExitCode(Response::Ok))
}
