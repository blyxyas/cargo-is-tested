use std::process::Command;

fn main() {
    Command::new("cargo")
        .args(&["install", "--path", ".."])
        .status()
        .expect("Failed to install `is_tested_plugin`");
}
