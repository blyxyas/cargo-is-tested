use if_chain::if_chain;
use lints::check_lints;
use miette::Result;
use std::fs;
use syn::{self, File};

use clap::Parser;
use colored::Colorize;

mod error;
mod flags;
mod lints;

use error::ErrorKind;

use crate::flags::check_flags;

#[derive(Parser)]
#[command(bin_name = "cargo", name = "cargo")]
enum Cargo {
    IsTested(IsTested),
}

#[derive(clap::Args)]
#[command(author, version, about)]
struct IsTested {
    input: String,
}

fn main() -> Result<()> {
    let Cargo::IsTested(args) = Cargo::parse();
    let paths = match fs::read_dir(format!("{}/src", args.input)) {
        Ok(p) => p,
        Err(e) => return Err(ErrorKind::IoError(e).into()),
    };
    for path in paths {
        let raw_filename = path.unwrap().file_name();
        let filename = raw_filename.to_str().unwrap();

		println!("Checking [{}]", filename.bright_cyan().bold());

        let src = match fs::read_to_string(format!("{}/src/{filename}", args.input)) {
            Ok(s) => s,
            Err(e) => return Err(ErrorKind::IoError(e).into()),
        };

        let syntax = match syn::parse_file(&src) {
            Ok(syn) => syn,
            Err(_) => {
				return Err(ErrorKind::UnexpectedToken { filename: filename.to_owned() }
                .into())
            }
        };

        if_chain! {
            if let Some(shebang) = &syntax.shebang;
            if shebang.to_lowercase().contains("is-tested");
            then {
                println!("\t[{}] {}", filename.bright_cyan().bold(), "Testing enabled".green());
				
				match check_flags(filename, shebang) {
					Ok(()) => {}
					Err(e) => {return Err(e)}
				}

                match check_tests(filename, &syntax) {
                    Ok(_) => {println!("\t[{}] {}", filename.bright_cyan().bold(), "Tests checked!".green())},
                    Err(e) => {return Err(e)}
                }
            } else {
                println!("\t[{}] {}", filename.bright_cyan().bold(), "Testing disabled".red())
            }
        };
    }

    println!(
        "\n[=============================]\n\n{}",
        "All tests were checked!".green()
    );
    Ok(())
}

fn check_tests(filename: &str, file: &File) -> Result<()> {
    check_lints(filename, file)
}