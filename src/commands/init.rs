use ansi_term::Color::{Blue, Green, Red};
use config::Context;
use dto::Request;
use dto::Response;
use model::{Command, Section, Value};
use serde_yaml;
use std::{
    env,
    fs::{create_dir_all, File, Permissions},
    io::{Error, ErrorKind, prelude::Write, stdin},
    os::unix::fs::PermissionsExt,
    path::PathBuf,
    process,
    result::Result,
    str::FromStr,
};
use workflow::Instruction;
use workflow::Work;

fn default_config() -> Vec<Section> {
    let hello_world = Command {
        name: s!("hello"),
        description: s!("Prints hello world"),
        alias: Some(s!("hw")),
        value: Some(Value::Shell(s!("echo hello world"))),
        internal: None,
        usage: None,
        min_args: None,
        dependencies: None,
    };

    vec![Section {
        heading: s!("Commands"),
        commands: vec![hello_world],
    }]
}

#[derive(Debug)]
enum Answer {
    Yes,
    No,
}

impl FromStr for Answer {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        match s {
            "y" => Ok(Answer::Yes),
            "n" => Ok(Answer::No),
            _ => Err(())
        }
    }
}

fn ask(question: &String) -> String {
    let mut input = String::new();
    loop {
        println!("{}", question);
        if let Ok(v) = stdin().read_line(&mut input).map(|_| input.trim()) {
            if v.len() > 0 {
                return v.to_owned();
            }
        }
        input.clear();
    }
}

fn confirm(question: &str) -> Answer {
    let formatted_question = format!("{} (y/n)", question);
    loop {
        if let Ok(a) = ask(&formatted_question).parse::<Answer>() {
            return a;
        }
    }
}

fn create_dir_if_not_exists(path: &PathBuf) -> Result<(), Error> {
    if path.is_dir() {
        Ok(())
    } else {
        create_dir_all(path)
    }
}

pub fn run_setup(request: Request, _: &Context) -> Work {
    let no_output = Work::instruction(Instruction::ExitCode(Response::Ok));

    let directory = request.next().current
        .map(PathBuf::from)
        .unwrap_or(env::current_dir().unwrap());

    println!("{}", Blue.paint("sdoc init"));

    if let Answer::No = confirm(&format!("Setup a new CLI in {:?}?", directory)) {
        println!("Goodbye");
        return no_output;
    }

    let cli_name = ask(&s!("Enter your CLI name:"));
    let content = format!("\
#! /bin/bash -ue
dir=$(cd $( dirname \"{}\" ) && cd .. && pwd )
COMMANDS_DIRECTORY=\"$dir\" CLI_NAME='{}' sdoc \"$@\"", "${BASH_SOURCE[0]}", cli_name);

    let bin_directory = directory.join("bin");
    let bin = &bin_directory.join(&cli_name);
    let commands_directory = &directory.join(&cli_name);
    let commands_yaml = &commands_directory.join("commands.yaml");

    match create_dir_if_not_exists(&bin_directory)
        .and_then(|_| File::create(&bin))
        .and_then(|mut bin| {
            bin.write_all(content.as_bytes())
                .and_then(|_| bin.set_permissions(Permissions::from_mode(0o755)))
        })
        .and_then(|_| create_dir_if_not_exists(&commands_directory))
        .and_then(|_| File::create(&commands_yaml))
        .and_then(|mut y|
            serde_yaml::to_string(&default_config())
                .map_err(|e| Error::new(ErrorKind::Other, e))
                .and_then(|yaml| y.write_all(yaml.as_bytes()))
        ) {
        Ok(_) => {
            println!("{}", Green.paint("Setup Complete"));
            println!("Execute ./bin/{} to begin. Even better, add '$(pwd)/bin' to your $PATH", cli_name);
        }
        Err(e) => {
            println!("{}: {:?}", Red.paint("Setup Failed"), e);
            process::exit(1);
        }
    };

    no_output
}