use core::workflow::Work;
use core::workflow::Instruction::{Display, SystemCommand, ExitCode};
use core::model::Executable;
use core::dto::{Request, Response};
use core::config::Context;

pub fn execute(request: Request, context: &Context) -> Work {
    Work::instruction(
        request.next()
            .current
            .and_then(|rc| context.find(&rc, true))
            .map(|command| {
                match command.command_type {
                    Executable::Script(ref b) =>
                        SystemCommand(format!("less {}/{}", context.exec_directory.display(), b), true),
                    Executable::Shell(ref b) =>
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
                        match c.command_type {
                            Executable::Script(_) => true,
                            Executable::Shell(_) => true,
                            _ => false
                        }
                    })
                    .fold(s!(), |a, b| format!("{}{}\n", a, &b.name)));

                Display(s, Response::Ok)
            }))
}
