use std::io::ErrorKind::NotFound;
use std::process::Command;

#[derive(Debug)]
struct CommandAndDescription {
    command: &'static str,
    description: &'static str,
}

const COMMANDS_TO_CHECK: [CommandAndDescription; 2] = [
    CommandAndDescription {
        command: "uv",
        description: "UV from Astral",
    },
    CommandAndDescription {
        command: "npx",
        description: "NPX allows you to execute Node.js packages without permanently installing them globally",
    },
];

pub fn check_environment() {
    for command_to_check in COMMANDS_TO_CHECK {
        println!(
            "Checking {}, {}",
            command_to_check.command, command_to_check.description
        );

        let command_output = Command::new("ladskkasdkass").output();

        match command_output {
            Ok(result_ok) => {
                // Check exit status
                if result_ok.status.success() {
                    // Convert stdout to string
                    let stdout = String::from_utf8_lossy(&result_ok.stdout);
                    println!("Success! Output:\n{}", stdout);

                    // Optionally handle stderr
                    let stderr = String::from_utf8_lossy(&result_ok.stderr);
                    if !stderr.is_empty() {
                        eprintln!("Warnings/Errors:\n{}", stderr);
                    }
                } else {
                    eprintln!("Command failed with status: {}", result_ok.status);
                }
            }
            Err(command_error) => {
                // dbg!(command_error.kind());
                if command_error.kind() == NotFound {
                    eprintln!("Could not find {} installed on the system", &command_to_check.command);
                } else {
                    eprintln!("{}", command_error);
                }
            }
        }
    }
}
