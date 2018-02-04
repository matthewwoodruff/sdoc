use config::Context;
use dto::{Request, Response};
use model::{Command, Section};
use workflow::Work;
use workflow::Instruction::Display;
use commands::util::build_auto_complete;

pub fn execute(request: Request, context: &Context) -> Work {
    let request = request.next();
    build_help(&request, context)
}

pub fn auto_complete(request: Request, context: &Context) -> Work {
    let request = request.next();
    Work::instruction(Display(auto_complete_build(request, context), Response::Ok))
}

pub fn execute_help(request: Request, context: &Context) -> Work {
    build_help(&request, context)
}

fn auto_complete_build(request: Request, context: &Context) -> String {
    request.current
        .and_then(|rc| context.find(&rc, false))
        .map(|_| s!())
        .unwrap_or_else(|| build_auto_complete(context))
}

pub fn build_help(request: &Request, context: &Context) -> Work {
    Work::instruction(match request.current {
        Some(rc) => {
            context.find(&rc, true)
                .map(|c| Display(c.build_command_usage(&context.build_command_chain()), Response::Ok))
                .unwrap_or_else(|| Display(build_full_help(context), Response::Err(1)))
        }
        _ => Display(build_full_help(context), Response::Ok)
    })
}

pub fn build_full_help(context: &Context) -> String {
    let sections: Vec<String> = context.config.iter()
        .map(|s| format_section(s))
        .collect();
    format!("\n{}\n\n{}",
            format!("Usage: {} <command> [args]", context.resolved_commands.join(" ")),
            sections.join("\n"))
}

fn format_section(section: &Section) -> String {
    let commands: Vec<String> = section.commands.iter()
        .map(|c| format_command(c))
        .collect();
    format!("{}:\n  {}\n", section.heading, commands.join("\n  "))
}

fn format_command(command: &Command) -> String {
    format!("{:12}{:6}{}",
            command.name,
            command.alias.as_ref().unwrap_or(&String::new()),
            command.description)
}
