use ansi_term::Color::{Blue, Green, Red};
use std::fs::{create_dir, File};
use std::fs::Permissions;
use std::{io, process};
use std::io::prelude::Write;
use std::os::unix::fs::PermissionsExt;
use std::result::Result;
use std::str::FromStr;
use std::path::Path;

static YAML: &'static str = "---
 - heading: Commands
   commands:
   - name: hello
     description: Prints hello world
     type:
       shell: echo hello world";

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
        if let Ok(v) = io::stdin().read_line(&mut input).map(|_| input.trim()) {
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

fn create_dir_if_not_exists(path: &str) -> Result<(), io::Error> {
    if Path::new(path).is_dir() {
        Ok(())
    } else {
        create_dir(path)
    }
}

pub fn run_setup() {
    println!("{}", Blue.paint("sdoc init"));

    if let Answer::No = confirm("Setup a new CLI?") {
        println!("Goodbye");
        return;
    }

    let cli_name = ask(&s!("Enter your CLI name:"));
    let content = format!("#! /bin/bash -ue
dir=$(cd $( dirname \"{}\" ) && cd .. && pwd )
COMMANDS_DIRECTORY=\"$dir\" CLI_NAME='{}' sdoc \"$@\"", "${BASH_SOURCE[0]}", cli_name);

    match create_dir_if_not_exists("bin")
        .and_then(|_| File::create(format!("bin/{}", cli_name)))
        .and_then(|mut bin| {
            bin.write_all(content.as_bytes())
                .and_then(|_| bin.set_permissions(Permissions::from_mode(0o755)))
        })
        .and_then(|_| create_dir_if_not_exists(&cli_name))
        .and_then(|_| File::create(format!("{}/commands.yaml", cli_name)))
        .and_then(|mut y| y.write_all(YAML.as_bytes())) {
        Ok(_) => {
            println!("{}", Green.paint("Setup Complete"));
            println!("Execute ./bin/{} to begin. Even better, add '$(pwd)/bin' to your $PATH", cli_name);
        },
        Err(e) => {
            println!("{}: {:?}", Red.paint("Setup Failed"), e);
            process::exit(1);
        }
    };
}