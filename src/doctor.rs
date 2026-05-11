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
        print!(
            "Checking {}, {}",
            command_to_check.command, command_to_check.description
        );

        let output = Command::new("ls")
            .arg("-l")
            .output()
            .expect("Failed to execute command");

        // Check exit status
        if output.status.success() {
            // Convert stdout to string
            let stdout = String::from_utf8_lossy(&output.stdout);
            println!("Success! Output:\n{}", stdout);

            // Optionally handle stderr
            let stderr = String::from_utf8_lossy(&output.stderr);
            if !stderr.is_empty() {
                eprintln!("Warnings/Errors:\n{}", stderr);
            }
        } else {
            eprintln!("Command failed with status: {}", output.status);
        }
    }
}
