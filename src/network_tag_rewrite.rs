use std::error::Error;
use std::path::Path;
use std::process::Command;

use inquire::Text;
use owo_colors::OwoColorize;

use crate::context::ApplicationContext;
use crate::git_actions::commit_to_directory;

const NETWORK_IN_DOCKER_COMPOSE: &str = "dj_forge_network";

pub fn replace_network_in_docker_compose(
    context: &ApplicationContext,
) -> Result<(), Box<dyn Error>> {
    println!("Context: {:?}", context);

    let base_directory = Path::new(context.target())
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("unknown");

    if context.verbose() {
        println!(
            "Base directory of target path to be used as default value for network tag: {}",
            base_directory
        );
    }

    let network_tag_prompt = format!("Choose network tag, default {}", base_directory.bold());
    let network_tag = Text::new(&network_tag_prompt)
        .with_default(base_directory)
        .prompt()
        .unwrap();

    println!("tag: {}", network_tag);
    //
    // Sed to update network Tag
    //
    let docker_compose_location = format!("{}/docker-compose.yml", context.target());
    let mut replace_command_binding = Command::new("sed");
    let replace_command = replace_command_binding
        .arg("-i")
        .arg(format!("s/{}/{}/g", NETWORK_IN_DOCKER_COMPOSE, network_tag))
        .arg(&docker_compose_location);

    if context.verbose() {
        println!("Executing: {:?}", replace_command);
    }

    let output = replace_command
        .output()
        .expect("Failed to execute port mapping replace command");

    // Check exit status
    if output.status.success() {
        if context.verbose() {
            // Convert stdout to string
            let stdout = String::from_utf8_lossy(&output.stdout);
            println!("Success! Output:\n{}", stdout);
        }

        // Optionally handle stderr
        let stderr = String::from_utf8_lossy(&output.stderr);
        if !stderr.is_empty() {
            eprintln!("Warnings/Errors:\n{}", stderr);
        }
    } else {
        eprintln!("Command failed with status: {}", output.status);
        let error_message = String::from_utf8_lossy(&output.stderr);
        eprintln!("Command failed with error: {}", error_message.red());

        return Err(format!(
            "Could not map network tag in {}",
            docker_compose_location.bold()
        )
        .into());
    }

    //
    // Commit changes
    //
    let commit_message = format!(
        "Replaced network tag to {} in {}",
        &network_tag, &docker_compose_location,
    );

    if context.verbose() {
        println!("Commiting with {}", commit_message.bold());
    }

    commit_to_directory(context.target(), &commit_message)?;

    Ok(())
}
