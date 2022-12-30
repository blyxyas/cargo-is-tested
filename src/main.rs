#![feature(proc_macro_internals)]
use if_chain::if_chain;
use cargo_is_tested::lints::check_lints;
use miette::Result;
use std::fs;
use syn::{self, File};

use clap::Parser;
use colored::Colorize;

use cargo_is_tested::error::ErrorKind;

use cargo_is_tested::flags::check_flags;

#[derive(Parser)]
#[command(bin_name = "cargo", name = "cargo")]
enum Cargo {
    IsTested(IsTested),
}

#[derive(clap::Args)]
#[command(author, version, about)]
struct IsTested {
    input: String,
    #[arg(short, long, default_value = "")]
	/// A list of all the lints applied.
	/// Example:
	/// 
	/// cargo is-tested my_project --lints validness,emptiness
	/// 
	/// These lints will be applied with the lints specified in your tests after the shebang.
    lints: Vec<String>,
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
                return Err(ErrorKind::UnexpectedToken {
                    filename: filename.to_owned(),
                }
                .into())
            }
        };

        if_chain! {
            if let Some(shebang) = &syntax.shebang;
            if shebang.to_lowercase().contains("is-tested");
            then {
                println!("\t[{}] {}", filename.bright_cyan().bold(), "Testing enabled".green());

                let flags = match check_flags(filename, shebang) {
                    Some(Ok(flags)) => flags,
                    Some(Err(e)) => {return Err(e)}
                    None => Vec::new()
                };

                match check_tests(&src, filename, &syntax, flags) {
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

fn check_tests(src: &str, filename: &str, file: &File, flags: Vec<String>) -> Result<()> {
    check_lints(src, filename, file, flags)
}
