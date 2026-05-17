use std::process::Command;

use owo_colors::OwoColorize;

use crate::common_strings::FORGE_UNICODE;

const FILES_WITH_PORT_MAPPING: [&str; 2] = ["docker-compose.yml", ".github/workflows/pytest.yml"];

pub fn replace_text_in_target_folder(
    target_directory: &str,
    source_port_number: &str,
    target_port_number: &str,
    replace_only_mapped_port: bool,
) -> bool {
    println!("{FORGE_UNICODE}Inside port replacer {source_port_number} ➡️ {target_port_number}");

    if source_port_number == target_port_number {
        println!("No change in ports, continuing...");
        return false;
    }

    if replace_only_mapped_port {
        replace_text_in_target_folder_for_port_mapping(
            target_directory,
            source_port_number,
            target_port_number,
        );

        return true;
    }

    replace_text_in_target_folder_excluding_compose_port_mapping(
        target_directory,
        source_port_number,
        target_port_number,
    );

    for file_with_port_mapping in FILES_WITH_PORT_MAPPING {
        println!("{}", file_with_port_mapping);

        let target_location = format!("{}/{}", target_directory, file_with_port_mapping);
        let mut replace_command = Command::new("sed");
        replace_command
            .arg("-i")
            .arg(format!(
                "s/{}:{}/{}:{}/g",
                source_port_number, source_port_number, source_port_number, target_port_number
            ))
            .arg(&target_location);

        println!("Executing: {:?}", replace_command);

        let output = replace_command
            .output()
            .expect("Failed to execute port mapping replace command");

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
            let error_message = String::from_utf8_lossy(&output.stderr);
            eprintln!("Command failed with error: {}", error_message.red());
        }
    }

    true
}

fn replace_text_in_target_folder_excluding_compose_port_mapping(
    target_directory: &str,
    source_port_number: &str,
    target_port_number: &str,
) -> bool {
    let mut replace_command = Command::new("find");
    replace_command
        .arg(target_directory)
        .arg("-type")
        .arg("f")
        .arg("-not")
        .arg("-path")
        .arg("./.git/*")
        .arg("-not")
        .arg("-path")
        .arg("./node_modules/*")
        .arg("-not")
        .arg("-name")
        .arg("docker-compose.yml")
        .arg("-not")
        .arg("-name")
        .arg("ruff.xml")
        .arg("-not")
        .arg("-name")
        .arg("pytest.yml")
        .arg("-not")
        .arg("-name")
        .arg("uv.lock")
        .arg("-exec")
        .arg("sed")
        .arg("-i")
        .arg(format!("s/{}/{}/g", source_port_number, target_port_number))
        .arg(format!("{}", "{}"))
        .arg("+");
    println!("Executing: {:?}", replace_command);

    let output = replace_command.output().expect("Failed to execute command");

    // Check exit status
    if output.status.success() {
        // Convert stdout to string
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("Success! Output:\n{}", stdout);

        // Optionally handle stderr
        let stderr = String::from_utf8_lossy(&output.stderr);
        if !stderr.is_empty() {
            eprintln!("Warnings/Errors:\n{}", stderr);

            return false;
        }
    } else {
        eprintln!("Command failed with status: {}", output.status);
        let error_message = String::from_utf8_lossy(&output.stderr);
        eprintln!("Command failed with error: {}", error_message);

        return false;
    }

    true
}

fn replace_text_in_target_folder_for_port_mapping(
    target_directory: &str,
    source_port_number: &str,
    target_port_number: &str,
) -> bool {
    let mut replace_command = build_sed_command(target_directory, source_port_number, target_port_number);
    println!("Executing: {:?}", replace_command);

    let output = replace_command.output().expect("Failed to execute command");

    // Check exit status
    if output.status.success() {
        // Convert stdout to string
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("Success! Output:\n{}", stdout);

        // Optionally handle stderr
        let stderr = String::from_utf8_lossy(&output.stderr);
        if !stderr.is_empty() {
            eprintln!("Warnings/Errors:\n{}", stderr);

            return false;
        }
    } else {
        eprintln!("Command failed with status: {}", output.status);
        let error_message = String::from_utf8_lossy(&output.stderr);
        eprintln!("Command failed with error: {}", error_message);

        return false;
    }

    true
}

fn build_sed_command(target_directory: &str, source_port_number: &str, target_port_number: &str) -> Command {
    let mut command = Command::new("find");
    command
        .arg(target_directory)
        .arg("-type")
        .arg("f")
        .arg("-not")
        .arg("-path")
        .arg("./.git/*")
        .arg("-not")
        .arg("-path")
        .arg("./node_modules/*")
        .arg("-not")
        .arg("-name")
        .arg("uv.lock")
        .arg("-not")
        .arg("-name")
        .arg("ruff.xml")
        .arg("-exec")
        .arg("sed")
        .arg("-i")
        .arg(format!("s/{}/{}/g", source_port_number, target_port_number))
        .arg(format!("{}", "{}"))
        .arg("+");

    command
}
