use core::workflow::{Work, Action};
use core::workflow::Instruction::{Display, SystemCommand};
use core::model::Executable;
use core::dto::{Request, Response};
use core::config::Context;

pub fn execute(request: Request, context: &Context) -> Work {
    Work::action(
        request.next()
            .current
            .and_then(|rc| context.find(&rc, true))
            .map(|command| {
                if let Executable::Script(ref b) = command.command_type {
                    return Action::instruction(SystemCommand(format!("$EDITOR {}/{}", context.exec_directory.display(), b), true));
                }
                Action::just_response(Response::Err(18))
            })
            .unwrap_or_else(|| Action::just_response(Response::Err(18))))
}

pub fn auto_complete(request: Request, context: &Context) -> Work {
    Work::action(
        request.next()
            .current
            .and_then(|rc| context.find(&rc, false))
            .map(|_| Action::just_response(Response::Ok))
            .unwrap_or_else(|| {
                let s = format!("{}", context.get_commands().iter()
                    .filter(|c| {
                        if let Executable::Script(_) = c.command_type {
                            return true;
                        }
                        false
                    })
                    .fold(String::new(), |a, b| format!("{}{}\n", a, &b.command)));

                Action::instruction(Display(s)).response(Response::Ok)
            }))
}
