use if_chain::if_chain;
use miette::Result;
use std::fs;
use std::io::Read;
use syn::{self, File, Item, Lit, Meta, MetaList, NestedMeta};

use clap::Parser;
use colored::Colorize;

mod error;
mod lints;
mod flags;

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

        let syntax = syn::parse_file(&src).expect("Unable to parse file");

        if_chain! {
            if let Some(shebang) = &syntax.shebang;
            if shebang[2..].trim().to_lowercase() == "is tested";
            then {
                println!("[{}] {}", filename.bright_cyan().bold(), "Testing enabled".green());
				show_flags(shebang);
                match check_tests(&args.input, &syntax) {
                    Ok(_) => {println!("[{}] {}", filename.bright_cyan().bold(), "Tests checked!".green())},
                    Err(e) => {return Err(e)}
                }
            } else {
                println!("[{}] {}", filename.bright_cyan().bold(), "Testing disabled".red())
            }
        }
    }

	println!("{}", "All tests were checked!".green());
    Ok(())
}

fn check_tests(workdir: &str, file: &File) -> Result<()> {
    use Item::*;
    for item in &file.items {
        let attrs = match item {
            Const(x) => &x.attrs,
            Enum(x) => &x.attrs,
            ExternCrate(x) => &x.attrs,
            Fn(x) => &x.attrs,
            ForeignMod(x) => &x.attrs,
            Impl(x) => &x.attrs,
            Macro(x) => &x.attrs,
            Macro2(x) => &x.attrs,
            Mod(x) => &x.attrs,
            Static(x) => &x.attrs,
            Struct(x) => &x.attrs,
            Trait(x) => &x.attrs,
            TraitAlias(x) => &x.attrs,
            Type(x) => &x.attrs,
            Union(x) => &x.attrs,
            Use(x) => &x.attrs,
            _ => {
                todo!()
            }
        };
    }

    Ok(())
}
