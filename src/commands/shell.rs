use dto::Request;
use workflow::Work;
use workflow::Instruction::SystemCommand;
use config::Context;

pub fn execute_script(shell: &String, request: Request, context: &Context) -> Work {
    build_system_command(format!("{}/{} {}", context.directory.display(), shell, quote_args(request)))
}

pub fn execute_shell(shell: &String, request: Request) -> Work {
    build_system_command(format!("{} {}", shell, quote_args(request)))
}

fn quote_args(request: Request) -> String {
    let args : Vec<String> = request.next
        .iter()
        .map(|a| format!("'{}'", a))
        .collect();

    args.join(" ")
}

fn build_system_command(command: String) -> Work {
    Work::instruction(SystemCommand(command, true))
}
