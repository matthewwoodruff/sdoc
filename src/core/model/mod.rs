#[cfg(test)]
mod test;

use core::config::Context;
use core::dto::{Request, Response};
use core::workflow::{Work, Action};
use core::workflow::Instruction::{Display, SystemCommand};
use core::commands::{node, shell, help, editconfig, edit, view};

#[derive(Debug, Deserialize, PartialEq)]
pub struct Section {
    pub heading: String,
    pub commands: Vec<Command>
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Command {
    pub command: String,
    pub description: String,
    pub command_type: Executable,
    pub usage: Option<String>,
    pub alias: Option<String>,
    pub dependencies: Option<Vec<Dependency>>,
    pub min_args: Option<usize>
}

impl Command {
    fn matches_alias(&self, request_command: &String) -> bool {
        self.alias.as_ref().map(|a| a.eq(request_command)).unwrap_or(false)
    }

    pub fn matches_command(&self, request_command: &String) -> bool {
        self.command.eq(request_command)
    }

    pub fn matches(&self, request_command: &String) -> bool {
        self.matches_command(request_command) || self.matches_alias(request_command)
    }

    fn dependency_checks(&self, context: &Context) -> Vec<Work> {
        self.dependencies.as_ref()
            .unwrap_or(&vec![])
            .iter()
            .map(|d| {
                Work::instruction(SystemCommand(s!(format!("type -t {}", d.value)), false))
                    .on_error(self.build_command_usage_action(context))
            })
            .collect()
    }

    fn build_command_usage_action(&self, context: &Context) -> Action {
        Action::new().instruction(Display(self.build_command_usage(context)))
    }

    pub fn build_command_usage(&self, context: &Context) -> String {
        let dependencies = self.dependencies.as_ref()
            .map(|d|
                format!("\nDependencies:{}\n",
                        d.iter().fold(String::new(), |a, b| format!("{}\n  {:12}{}", a, &b.value, &b.description))))
            .unwrap_or(String::new());

        format!("\nUsage: {} {} {}\n\n{}\n{}",
                &context.resolved_commands.join(" "),
                &self.command,
                self.usage.as_ref().unwrap_or(&String::new()),
                &self.description,
                dependencies)
    }

    pub fn execute(&self, request: Request, context: &Context) -> Vec<Work> {
        let mut work : Vec<Work> = self.dependency_checks(context);

        if self.min_args.map(|v| v > request.next.len()).unwrap_or(false) {
            work.push(Work::action(self.build_command_usage_action(context)))
        } else {
            work.append(&mut self.command_type.execute(request, context))
        }

        work
    }

    pub fn execute_auto_complete(&self, request: Request, context: &Context) -> Vec<Work> {
        self.command_type.auto_complete(request, context)
    }
}


#[derive(Deserialize, Debug, PartialEq)]
pub struct Dependency {
    pub value: String,
    pub description: String
}

#[derive(Deserialize, Debug, PartialEq)]
pub enum External {
    Script(String),
    Alias(String)
}

#[derive(Deserialize, Debug, PartialEq)]
pub enum Executable {
    Node,
    Help,
    Edit,
    EditConfig,
    View,
    Script(String),
    Shell(String)
}

impl Executable {
    fn execute(&self, request: Request, context: &Context) -> Vec<Work> {
        match *self {
            Executable::Node => node::execute(request, context),
            Executable::Script(ref script) => vec![shell::execute_script(script, request, context)],
            Executable::Shell(ref alias) => vec![shell::execute_shell(alias, request)],
            Executable::Help => vec![help::execute(request, context)],
            Executable::Edit => vec![edit::execute(request, context)],
            Executable::EditConfig => vec![editconfig::execute(request, context)],
            Executable::View => vec![view::execute(request, context)]
        }
    }

    fn auto_complete(&self, request: Request, context: &Context) -> Vec<Work> {
        match *self {
            Executable::Node => node::execute_auto_complete(request, context),
            Executable::Help => vec![help::auto_complete(request, context)],
            Executable::Edit => vec![edit::auto_complete(request, context)],
            Executable::View => vec![view::auto_complete(request, context)],
            _ => vec![Work::response(Response::Err(15))]
        }
    }
}



