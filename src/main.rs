use if_chain::if_chain;
use lints::check_lints;
use miette::{NamedSource, Result};
use std::fs;
use std::io::Read;
use syn::{self, spanned::Spanned, File, Item, Lit, Meta, MetaList, NestedMeta};

use clap::Parser;
use colored::Colorize;

mod error;
mod flags;
mod lints;

use error::ErrorKind;

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

        let src = match fs::read_to_string(format!("{}/src/{filename}", args.input)) {
            Ok(s) => s,
            Err(e) => return Err(ErrorKind::IoError(e).into()),
        };

        let syntax = match syn::parse_file(&src) {
            Ok(syn) => syn,
            Err(e) => {
				let span = e.span().start();
                return Err(ErrorKind::UnexpectedToken {
                    src: NamedSource::new(filename, src),
                    span: (span.line, span.column).into(),
                }
                .into())
            }
        };

        if_chain! {
            if let Some(shebang) = &syntax.shebang;
            if shebang[2..].trim().to_lowercase() == "is tested";
            then {
                println!("[{}] {}", filename.bright_cyan().bold(), "Testing enabled".green());
                match check_tests(&src, filename, &syntax) {
                    Ok(_) => {println!("[{}] {}", filename.bright_cyan().bold(), "Tests checked!".green())},
                    Err(e) => {return Err(e)}
                }
            } else {
                println!("[{}] {}", filename.bright_cyan().bold(), "Testing disabled".red())
            }
        };
    }

    println!(
        "\n[=============================]\n\n{}",
        "All tests were checked!".green()
    );
    Ok(())
}

fn check_tests(source: &str, filename: &str, file: &File) -> Result<()> {
    check_lints(filename, file)
}
