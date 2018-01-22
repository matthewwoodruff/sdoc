#[cfg(test)]
mod test;

use std::process;
use core::dto::Response;

pub fn run_system_command(command: &String) -> Response {
    run_system_command_with_output(command, process::Stdio::inherit())
}

pub fn run_system_command_ignoring_output(command: &String) -> Response {
    run_system_command_with_output(command, process::Stdio::null())
}

fn run_system_command_with_output(command: &String, stdio: process::Stdio) -> Response {
    process::Command::new("bash")
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
