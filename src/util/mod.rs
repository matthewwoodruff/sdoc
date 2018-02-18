#[cfg(test)]
mod test;

use std::process::{Stdio, Command};
use dto::Response;

pub fn run_system_command(command: &String, output: bool) -> Response {
    let out = if output {Stdio::inherit()} else {Stdio::null()};
    _run_system_command_with_output(command, out)
}

fn _run_system_command_with_output(command: &String, stdio: Stdio) -> Response {
    Command::new("sh")
        .arg("-c")
        .arg(command)
        .stdout(stdio)
        .status()
        .map(|c| {
            c.code()
                .map(|c| if c == 0 { Response::Ok } else { Response::Err(c) })
                .unwrap_or(Response::Err(13))
        })
        .unwrap_or(Response::Err(14))
}
