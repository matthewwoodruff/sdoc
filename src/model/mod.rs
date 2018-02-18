extern crate core;

#[cfg(test)]
mod test;

use config::Context;
use dto::{Request, Response};
use workflow::Work;
use workflow::Instruction::{Display, SystemCommand};
use workflow::Instruction;
use commands::{node, shell, help, editconfig, edit, view};

#[derive(Debug, Deserialize)]
pub struct Section {
    pub heading: String,
    pub commands: Vec<Command>
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Command {
    pub name: String,
    pub description: String,
    pub value: Value,
    pub usage: Option<String>,
    pub alias: Option<String>,
    pub dependencies: Option<Vec<Dependency>>,
    pub min_args: Option<usize>
}

impl Command {
    fn matches_alias(&self, request_command: &String) -> bool {
        self.alias.as_ref().map(|a| a.eq(request_command)).unwrap_or(false)
    }

    fn matches_command(&self, request_command: &String) -> bool {
        self.name.eq(request_command)
    }

    pub fn matches(&self, request_command: &String, match_alias: bool) -> bool {
        self.matches_command(request_command) || match_alias && self.matches_alias(request_command)
    }

    fn dependency_checks(&self, command_chain: &String) -> Vec<Work> {
        self.dependencies.as_ref()
            .unwrap_or(&vec![])
            .iter()
            .map(|d| {
                let result = match d.value {
                    DependencyType::Command(ref a) => format!("command -v \"{}\"", a),
                    DependencyType::Envar(ref a) => format!("test -n \"${}\"", a)
                };

                Work::instruction(SystemCommand(result, false))
                    .on_error(self.build_command_usage_action(command_chain, Response::Err(3)))
            })
            .collect()
    }

    fn build_command_usage_action(&self, command_chain: &String, response: Response) -> Instruction {
        Display(self.build_command_usage(command_chain), response)
    }

    pub fn build_command_usage(&self, command_chain: &String) -> String {
        let dependencies = self.dependencies.as_ref()
            .map(|d|
                format!("\nDependencies:{}\n",
                        d.iter().fold(s!(), |a, b| {
                            let dep_value = match b.value {
                                DependencyType::Envar(ref c) => c,
                                DependencyType::Command(ref c) => c
                            };

                            format!("{}\n  {:12}{}", a, dep_value, &b.description)
                        })))
            .unwrap_or(s!());

        format!("\nUsage: {} {} {}\n\n{}\n{}",
                command_chain,
                &self.name,
                self.usage.as_ref().unwrap_or(&s!()),
                &self.description,
                dependencies)
    }

    pub fn execute(&self, request: Request, context: &Context) -> Vec<Work> {
        let command_chain = &context.build_command_chain();
        let mut work : Vec<Work> = self.dependency_checks(command_chain);

        if self.min_args.map(|v| v > request.next.len()).unwrap_or(false) {
            work.push(Work::instruction(self.build_command_usage_action(command_chain, Response::Err(2))))
        } else {
            work.append(&mut self.value.execute(request, context))
        }

        work
    }

    pub fn execute_auto_complete(&self, request: Request, context: &Context) -> Vec<Work> {
        self.value.auto_complete(request, context)
    }
}

#[derive(Deserialize, Debug, PartialEq)]
pub enum DependencyType {
    #[serde(rename = "command")]
    Command(String),
    #[serde(rename = "envar")]
    Envar(String)
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Dependency {
    pub value: DependencyType,
    pub description: String
}

#[derive(Deserialize, Debug, PartialEq)]
pub enum Value {
    #[serde(rename = "node")]
    Node,
    Help,
    Edit,
    EditConfig,
    View,
    #[serde(rename = "script")]
    Script(String),
    #[serde(rename = "shell")]
    Shell(String)
}

impl Value {
    fn execute(&self, request: Request, context: &Context) -> Vec<Work> {
        match *self {
            Value::Node => node::execute(request, context),
            Value::Script(ref script) => vec![shell::execute_shell(script, request, context)],
            Value::Shell(ref shell) => vec![shell::execute_shell(shell, request, context)],
            Value::Help => vec![help::execute(request, context)],
            Value::Edit => vec![edit::execute(request, context)],
            Value::EditConfig => vec![editconfig::execute(request, context)],
            Value::View => vec![view::execute(request, context)]
        }
    }

    fn auto_complete(&self, request: Request, context: &Context) -> Vec<Work> {
        match *self {
            Value::Node => node::execute_auto_complete(request, context),
            Value::Help => vec![help::auto_complete(request, context)],
            Value::Edit => vec![edit::auto_complete(request, context)],
            Value::View => vec![view::auto_complete(request, context)],
            _ => vec![Work::response(Response::Err(15))]
        }
    }
}



