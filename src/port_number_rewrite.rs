use std::process::Command;

use crate::common_strings::FORGE_UNICODE;

pub fn replace_text_in_target_folder(
    target_directory: &str,
    source_port_number: &str,
    target_port_number: &str,
) -> bool {
    println!("{FORGE_UNICODE}Inside port replacer {source_port_number} ➡️ {target_port_number}");

    if source_port_number == target_port_number {
        println!("No change in ports, continuing...");
        return false;
    }

    replace_text_in_target_folder_excluding_compose_port_mapping(
        target_directory,
        source_port_number,
        target_port_number,
    );

    let    docker_compose_location = format!("{}/docker-compose.yml", target_directory);
    let mut replace_in_docker_compose_command = Command::new("sed");
    replace_in_docker_compose_command
        .arg("-i")
        .arg(format!(
            "s/{}:{}/{}:{}/g",
            source_port_number, source_port_number, source_port_number, target_port_number
        ))
        .arg(&docker_compose_location);

    println!("Executing: {:?}", replace_in_docker_compose_command);

    let output = replace_in_docker_compose_command
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
        let error_message = String::from_utf8_lossy(&output.stderr);
        eprintln!("Command failed with error: {}", error_message);
    }
// .github/workflows/pytest.yml

    true
}

fn replace_text_in_target_folder_excluding_compose_port_mapping(
    target_directory: &str,
    source_port_number: &str,
    target_port_number: &str,
) -> bool {
    let mut replace_command = Command::new("find");
    replace_command
        .arg(&target_directory)
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
