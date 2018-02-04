use workflow::Work;
use workflow::Instruction::{Display, SystemCommand, ExitCode};
use model::Executable;
use dto::Request;
use dto::Response::{Ok, Err};
use config::Context;

pub fn execute(request: Request, context: &Context) -> Work {
    Work::instruction(
        request.next()
            .current
            .and_then(|rc| context.find(&rc, true))
            .map(|command| {
                if let Executable::Script(ref b) = command.command_type {
                    return SystemCommand(format!("$EDITOR {}/{}", context.directory.display(), b), true);
                }
                ExitCode(Err(1))
            })
            .unwrap_or_else(|| ExitCode(Err(1))))
}

pub fn auto_complete(request: Request, context: &Context) -> Work {
    Work::instruction(
        request.next()
            .current
            .and_then(|rc| context.find(&rc, false))
            .map(|_| ExitCode(Ok))
            .unwrap_or_else(|| {
                let s = format!("{}", context.get_commands().iter()
                    .filter(|c| {
                        if let Executable::Script(_) = c.command_type {
                            return true;
                        }
                        false
                    })
                    .fold(s!(), |a, b| format!("{}{}\n", a, &b.name)));

                Display(s, Ok)
            }))
}