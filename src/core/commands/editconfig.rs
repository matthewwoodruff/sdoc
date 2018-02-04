use core::workflow::Work;
use core::workflow::Instruction::SystemCommand;
use core::dto::Request;
use core::config::Context;

pub fn execute(_: Request, context: &Context) -> Work {
    Work::instruction(SystemCommand(format!("$EDITOR {}/commands.yaml", context.directory.display()), true))
}