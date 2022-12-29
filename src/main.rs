use if_chain::if_chain;
use syn::ItemFn;
use std::{fs, fmt::format};
use std::io::Read;
use syn::{self, token::Extern, Error, File, Item, Meta, MetaList, Lit, LitStr, NestedMeta};

use clap::Parser;
use colored::Colorize;

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

fn check_tests(workdir: &str, file: &File) -> Result<(), String> {
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
                return Err(format!("Couldn't parse the item: {:#?}", &item));
			}
        };

		for attribute in attrs {
			if let Some(ident) = attribute.path.get_ident() {
				if ident.to_string() == "is_tested" {
					// Bingo!
					// Check that the file exists.

					if let Meta::List(MetaList { nested, .. }) = attribute.parse_meta().expect("Couldn't parse the arguments. This shouldn't have happened, please open an issue") {
						if let Some(NestedMeta::Lit(Lit::Str(litstr))) = nested.first() {
							let fileread = fs::read_to_string(format!("{}/{}", workdir, litstr.value())).expect(&format!("Couldn't read file: {}", litstr.value()));
							if fileread.is_empty() {
								return Err(format!("File {} is empty. Add some tests and comeback!", litstr.value()));
							};

							let ast = match syn::parse_file(&fileread) {
								Ok(file) => file,
								Err(e) => return Err(format!("Couldn't parse file: {}.\nParser said this:\n{}", litstr.value(), e))
							};

							for item in ast.items {
								if let Fn(func) = item {
									if func.block.stmts.is_empty() {
										return Err(format!("Function `{}` is empty, try adding some tests!", func.sig.ident.to_string()))
									}
								}
							}
						}
					}
				}
			}
		}
    }

	Ok(())
}
