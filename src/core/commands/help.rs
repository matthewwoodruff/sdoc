use core::config::Context;
use core::dto::{Request, Response};
use core::model::{Command, Section};
use core::workflow::Work;
use core::workflow::Instruction::Display;
use core::commands::util::build_auto_complete;

pub fn execute(request: Request, context: &Context) -> Work {
    let request = request.next();
    Work::instruction(Display(build_help(&request, context), Response::Ok))
}

pub fn auto_complete(request: Request, context: &Context) -> Work {
    let request = request.next();
    Work::instruction(Display(auto_complete_build(request, context), Response::Ok))
}

pub fn execute_help(context: &Context, response: Response) -> Work {
    Work::instruction(Display(build_full_help(context), response))
}

fn auto_complete_build(request: Request, context: &Context) -> String {
    request.current
        .and_then(|rc| context.find(&rc, false))
        .map(|_| s!())
        .unwrap_or_else(|| build_auto_complete(context))
}

pub fn build_help(request: &Request, context: &Context) -> String {
    request.current
        .and_then(|rc| context.find(&rc, true))
        .map(|c| c.build_command_usage(&context.build_command_chain()))
        .unwrap_or_else(|| build_full_help(context))
}

pub fn build_full_help(context: &Context) -> String {
    let sections: Vec<String> = context.sections.iter()
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
