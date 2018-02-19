use dto::Request;
use workflow::Work;
use workflow::Instruction::SystemCommand;
use config::Context;

pub fn execute_shell(shell: &String, request: Request, context: &Context) -> Work {
    let command = format!("PATH=\"$PATH:{}\" {} {}", context.directory.display(), shell, quote_args(request));
    Work::instruction(SystemCommand(command, true))
}

fn quote_args(request: Request) -> String {
    let args : Vec<String> = request.args
        .iter()
        .map(|a| format!("'{}'", a))
        .collect();

    args.join(" ")
}
