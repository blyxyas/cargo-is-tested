use if_chain::if_chain;
use miette::Result;
use std::fs;
use std::io::Read;
use syn::{self, File, Item, Meta, MetaList, Lit, NestedMeta};

use clap::Parser;
use colored::Colorize;

mod lints;
mod error;

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

fn main() {
    let Cargo::IsTested(args) = Cargo::parse();
    for path in fs::read_dir(format!("{}/src", args.input))
        .expect("You need to execute this command in a workspace (no 'src' directory found).")
    {
        let raw_filename = path.unwrap().file_name();
        let filename = raw_filename.to_str().unwrap();

        let mut file =
            fs::File::open(format!("{}/src/{filename}", args.input)).expect("Unable to open file");

        let mut src = String::new();

        file.read_to_string(&mut src).expect("Unable to read file");

        let syntax = syn::parse_file(&src).expect("Unable to parse file");

        if_chain! {
            if let Some(shebang) = &syntax.shebang;
            if shebang[2..].trim().to_lowercase() == "is tested";
            then {
                println!("[{}] {}", filename.bright_cyan().bold(), "Testing enabled".green());
                match check_tests(&args.input, &syntax) {
					Ok(_) => {},
					Err(e) => {panic!("{}", e)}
				}
            } else {
                println!("[{}] {}", filename.bright_cyan().bold(), "Testing disabled".red())
            }
        }
    }
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
            _ => {todo!()}
        };
    }

	Ok(())
}
