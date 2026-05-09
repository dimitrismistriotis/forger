use std::process::Command;

use crate::common_strings::FORGE_UNICODE;

pub fn replace_text_in_target_folder(
    target_directory: &str,
    source_port_number: &str,
    target_port_number: &str,
) {
    println!("{FORGE_UNICODE}Inside port replacer {source_port_number} ➡️ {target_port_number}");

    if source_port_number == target_port_number {
        println!("No change in ports, continuing...");
        return;
    }
    // Call a process wrapping sed to do a find and replace for the port number
    //
    // From Command line to copy/modify:
    // ❯ rg Readme target123/
    // target123/README.md
    // 1:# Readme
    //
    // Drop output code
    //
    // Need to refactor something line this:
    // The basic syntax to replace old_string with new_string in all files
    // under the current directory is:
    //
    // find . -type f -exec sed -i 's/old_string/new_string/g' {} +
    //
    // or
    //
    // Exclude Directories: Exclude hidden directories like .git or node_modules:
    // find . -type f -not -path "./.git/*" -not -path "./node_modules/*" -exec
    // sed -i 's/old/new/g' {} +
    //
    // find target123 -type f -not -path "./.git/*" -not -path "./node_modules/*" -exec  sed -i 's/Readme/ReadYou/g' {} +
    //
    // ❯ rg ReadYou target123/
    // target123/README.md
    // 1:# ReadYou
    //
    let command_to_run = format!(
        "find {} -type f -not -path \"./.git/*\" -not -path \"./node_modules/*\" -exec  sed -i 's/{}/{}/g' {} +",
        target_directory, source_port_number, target_port_number, "{}"
    );

    let output = Command::new(&command_to_run)
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
