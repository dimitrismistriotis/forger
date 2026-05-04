use std::process::Command;

pub fn check_environment() {
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