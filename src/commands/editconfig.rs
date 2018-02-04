use workflow::Work;
use workflow::Instruction::SystemCommand;
use dto::Request;
use config::Context;

pub fn execute(_: Request, context: &Context) -> Work {
    Work::instruction(SystemCommand(format!("$EDITOR {}/commands.yaml", context.directory.display()), true))
}