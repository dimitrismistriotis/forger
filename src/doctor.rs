use std::io::ErrorKind::NotFound;
use std::process::Command;

#[derive(Debug)]
struct CommandAndDescription {
    command: &'static str,
    description: &'static str,
    website: &'static str,
}

const COMMANDS_TO_CHECK: [CommandAndDescription; 2] = [
    CommandAndDescription {
        command: "uv",
        description: "UV from Astral: An extremely fast Python package and project manager, written in Rust.",
        website: "https://docs.astral.sh/uv/",
    },
    CommandAndDescription {
        command: "npx",
        description: "NPX allows you to execute Node.js packages without permanently installing them globally",
        website: "https://docs.npmjs.com/cli/v11/commands/npx",
    },
];

pub fn check_environment() {
    for command_to_check in COMMANDS_TO_CHECK {
        println!(
            "Checking {}, {}",
            command_to_check.command, command_to_check.description
        );

        let command_output = Command::new(format!("which {}", command_to_check.command)).output();

        match command_output {
            Ok(result_ok) => {
                // Check exit status
                if result_ok.status.success() {
                    // Convert stdout to string
                    let stdout = String::from_utf8_lossy(&result_ok.stdout);
                    println!("{} found, output:\n{}", command_to_check.command, stdout);

                    // Optionally handle stderr
                    // let stderr = String::from_utf8_lossy(&result_ok.stderr);
                    // if !stderr.is_empty() {
                    //    eprintln!("Warnings/Errors:\n{}", stderr);
                    //}
                } else {
                    eprintln!("Search command failed with status: {}", result_ok.status);
                }
            }
            Err(command_error) => {
                // dbg!(command_error.kind());
                if command_error.kind() == NotFound {
                    eprintln!("Could not find {} installed on the system", &command_to_check.command);
                    eprintln!("Chek their website for installation instructions: {}", &command_to_check.website);
                } else {
                    eprintln!("{}", command_error);
                }
            }
        }
    }
}
