use core::workflow::{Work, Action};
use core::workflow::Instruction::{Display, SystemCommand};
use core::model::Executable;
use core::dto::{Request, Response};
use core::config::Context;

pub fn execute(request: Request, context: &Context) -> Work {
    let request = request.next();

    Work::action(
        request.current
            .and_then(|rc| context.find(&rc, true))
            .map(|command| {
                match command.command_type {
                    Executable::Script(ref b) =>
                        Action::new().instruction(SystemCommand(format!("cat {}/{}", context.exec_directory.display(), b), true)),
                    Executable::Shell(ref b) =>
                        Action::new().instruction(Display(format!("{}", b))),
                    _ => Action::new().response(Response::Err(18))
                }
            })
            .unwrap_or_else(|| Action::new().response(Response::Err(18))))
}

pub fn auto_complete(request: Request, context: &Context) -> Work {
    let request = request.next();

    Work::action(
        request.current
            .and_then(|rc| context.find(&rc, false))
            .map(|_| Action::new().response(Response::Ok))
            .unwrap_or_else(|| {
                let s = format!("{}", context.get_commands().iter()
                    .filter(|c| {
                        match c.command_type {
                            Executable::Script(_) => true,
                            Executable::Shell(_) => true,
                            _ => false
                        }
                    })
                    .fold(String::new(), |a, b| format!("{}{}\n", a, &b.command)));

                Action::new().instruction(Display(s)).response(Response::Ok)
            }))
}
