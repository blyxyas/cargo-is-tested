use std::process::Command;

fn main() {
	Command::new("cargo").args(&["install", "--path", "./is_tested_plugin"]).status().expect("Failed to install `is_tested_plugin`");
}