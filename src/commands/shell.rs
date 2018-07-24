use workflow::Work;
use workflow::Instruction::SystemCommand;
use std::path::PathBuf;

pub fn execute_shell(shell: &String, args: &[String], directory: &PathBuf) -> Work {
    let command = format!("PATH=\"$PATH:{}\" {} {}", directory.display(), shell, quote_args(args));
    Work::instruction(SystemCommand(command, true))
}

fn quote_args(args: &[String]) -> String {
    let out_args : Vec<String> = args
        .iter()
        .map(|a| format!("'{}'", a))
        .collect();

    out_args.join(" ")
}
