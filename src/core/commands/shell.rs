use core::dto::Request;
use core::workflow::Work;
use core::workflow::Instruction::SystemCommand;
use core::config::Context;

pub fn execute_script(shell: &String, request: Request, context: &Context) -> Work {
    build_system_command(format!("{}/{} {}", context.exec_directory.display(), shell, quote_args(request)))
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

#[cfg(test)]
mod test {

    use super::*;
    use std::path::PathBuf;
    use core::test_helper::IN_MEMORY_SECTION_SOURCE;

    #[test]
    fn should_build_shell_system_command() {
        let args = vec![s!("a"), s!("a b"), s!("c")];

        let request = Request::new(&args, None);
        let command = s!("command");

        let work = execute_shell(&command, request);

        assert_eq!(work, Work::instruction(SystemCommand(s!("command 'a b' 'c'"), true)))
    }

    #[test]
    fn should_build_script_system_command() {
        let directory = PathBuf::from(s!("dm"));
        let context = Context {
            directory: &directory,
            exec_directory: directory.to_owned(),
            sections: vec![],
            resolved_commands: vec![],
            section_source: &IN_MEMORY_SECTION_SOURCE,
        };

        let args = vec![s!("a"), s!("a b"), s!("c")];

        let request = Request::new(&args, None);
        let command = s!("command");

        let work = execute_script(&command, request, &context);

        assert_eq!(work, Work::instruction(SystemCommand(s!("dm/command 'a b' 'c'"), true)))
    }
}





