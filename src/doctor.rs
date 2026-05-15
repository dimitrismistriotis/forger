use std::io::ErrorKind::NotFound;
use std::process::Command;

use crate::common_strings::FORGE_UNICODE;

#[derive(Debug)]
struct CommandAndDescription {
    title: &'static str,
    command: &'static str,
    description: &'static str,
    website: &'static str,
}

const COMMANDS_TO_CHECK: [CommandAndDescription; 2] = [
    CommandAndDescription {
        title: "uv",
        command: "uv -V",
        description: "UV from Astral: An extremely fast Python package and project manager, written in Rust.",
        website: "https://docs.astral.sh/uv/",
    },
    CommandAndDescription {
        title: "npx",
        command: "npx --version",
        description: "NPX allows you to execute Node.js packages without permanently installing them globally",
        website: "https://docs.npmjs.com/cli/v11/commands/npx",
    },
];

pub fn check_environment() {
    println!("{FORGE_UNICODE}Check that everything needed for forger to run is available.\n");

    for command_to_check in COMMANDS_TO_CHECK {
        println!(
            "Checking {}, {}",
            command_to_check.title, command_to_check.description
        );

        let mut parts = command_to_check.command.split_whitespace();
        let program = parts.next().expect("no command given");

        let command_output = Command::new(program)
            .args(parts) // remaining items become arguments
            .output();

        match command_output {
            Ok(result_ok) => {
                // Check exit status
                if result_ok.status.success() {
                    // Convert stdout to string
                    let stdout = String::from_utf8_lossy(&result_ok.stdout);
                    println!(
                        "{} found, Executed: {} with output: {}",
                        command_to_check.title, command_to_check.command, stdout
                    );

                    // Optionally handle stderr
                    // let stderr = String::from_utf8_lossy(&result_ok.stderr);
                    // if !stderr.is_empty() {
                    //    eprintln!("Warnings/Errors:\n{}", stderr);
                    //}
                } else {
                    eprintln!("Search command failed with status: {}", result_ok.status);
                    println!("{:#?}", result_ok);
                }
            }
            Err(command_error) => {
                // dbg!(command_error.kind());
                if command_error.kind() == NotFound {
                    eprintln!(
                        "Could not find {} installed on the system",
                        &command_to_check.command
                    );
                    eprintln!(
                        "Check their website for installation instructions: {}",
                        &command_to_check.website
                    );
                } else {
                    eprintln!("{}", command_error);
                }
            }
        }
    }
}
