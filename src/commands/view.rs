use workflow::Work;
use workflow::Instruction::{Display, SystemCommand, ExitCode};
use model::Value;
use dto::{Request, Response};
use config::Context;

pub fn execute(request: Request, context: &Context) -> Work {
    Work::instruction(
        request.next()
            .current
            .and_then(|rc| context.find(&rc, true))
            .map(|command| {
                match command.value {
                    Value::Script(ref b) =>
                        SystemCommand(format!("less {}/{}", context.directory.display(), b), true),
                    Value::Shell(ref b) =>
                        Display(format!("{}", b), Response::Ok),
                    _ => ExitCode(Response::Err(1))
                }
            })
            .unwrap_or_else(|| ExitCode(Response::Err(1))))
}

pub fn auto_complete(request: Request, context: &Context) -> Work {
    Work::instruction(
        request.next()
            .current
            .and_then(|rc| context.find(&rc, false))
            .map(|_| ExitCode(Response::Ok))
            .unwrap_or_else(|| {
                let s = format!("{}", context.get_commands().iter()
                    .filter(|c| {
                        match c.value {
                            Value::Script(_) => true,
                            Value::Shell(_) => true,
                            _ => false
                        }
                    })
                    .fold(s!(), |a, b| format!("{}{}\n", a, &b.name)));

                Display(s, Response::Ok)
            }))
}
