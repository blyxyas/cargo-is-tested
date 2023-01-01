#![feature(proc_macro_internals)]
use cargo_is_tested::lints::check_lints;
use cargo_is_tested::{maybe_warn, span};
use if_chain::if_chain;
use miette::{NamedSource, Result, Severity};
use std::fs;
use std::process::Command;
use syn::spanned::Spanned;
use syn::{File, ItemStruct};

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
    /// A list of all the lints applied.
    /// Example:
    ///
    /// cargo is-tested my_project --lints validness,emptiness
    ///
    /// These lints will be applied with the lints specified in your tests after the shebang.
    #[arg(short, long, default_value = "strict")]
    lints: Vec<String>,
    /// Use if you want to run `cargo test` inmediately afterwards if there aren't any errors.
    #[arg(long, default_value = "false")]
    test: bool,
    #[arg(long, default_value = "false")]
    deny_warnings: bool,
    /// Check that all structs have testing associated. You can use the `#[is_not_tested]` attribute to confirm that not testing is a concious choice.
    #[arg(long, default_value = "false")]
    structs: bool,
    /// Checks that all functions have testing associated. You can use the `#[is_not_tested]` attribute to confirm that not testing is a concious choice.
    #[arg(long, default_value = "false")]
    functions: bool,
    /// Checks that all macros have testing associated. You can use the `#[is_not_tested]` attribute to confirm that not testing is a concious choice.
    #[arg(long, default_value = "false")]
    macros: bool,
	/// Checks that all macros 2.0 have testing associated. You can use the `#[is_not_tested]` attribute to confirm that not testing is a concious choice.
    #[arg(long, default_value = "false")]
    new_macros: bool,
    /// Checks that all traits have testing associated. You can use the `#[is_not_tested]` attribute to confirm that not testing is a concious choice.
    #[arg(long, default_value = "false")]
    traits: bool,
    /// Checks that all enums have testing associated. You can use the `#[is_not_tested]` attribute to confirm that not testing is a concious choice.
    #[arg(long, default_value = "false")]
    enums: bool,
    /// Checks that all unions have testing associated. You can use the `#[is_not_tested]` attribute to confirm that not testing is a concious choice.
    #[arg(long, default_value = "false")]
    unions: bool,
    /// Checks that all types have testing associated. You can use the `#[is_not_tested]` attribute to confirm that not testing is a concious choice.
    #[arg(long, default_value = "false")]
    types: bool,
    /// Checks that all trait aliases have testing associated. You can use the `#[is_not_tested]` attribute to confirm that not testing is a concious choice.
    #[arg(long, default_value = "false")]
    trait_aliases: bool,
}

macro_rules! add_lint_by_keyword {
	($lint_list: expr => $($flag: expr, $keyword: expr)+) => {
		$(
		if $flag {
			$lint_list.push($keyword.to_owned());
		})*
	};
}

fn main() -> Result<()> {
    let Cargo::IsTested(mut args) = Cargo::parse();
    let paths = match fs::read_dir(format!("{}/src", args.input)) {
        Ok(p) => p,
        Err(e) => return Err(ErrorKind::IoError(e).into()),
    };

	add_lint_by_keyword! {
		args.lints =>
		args.structs, "structs"
		args.functions, "functions"
		args.macros, "macros"
		args.new_macros, "new-macros"
		args.traits, "traits"
		args.enums, "enums"
		args.unions, "unions"
		args.types, "types"
		args.trait_aliases, "trait-aliases"

	}

    if !args.lints.is_empty() {
        println!("Lints enabled:");
        for lint in &args.lints {
            println!("\t[{}]", lint.magenta())
        }
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

                let mut flags = match check_flags(filename, shebang) {
                    Some(Ok(flags)) => flags,
                    Some(Err(e)) => {return Err(e)}
                    None => Vec::new()
                };

				flags.append(&mut args.lints);



                match check_tests(&src, filename, &syntax, flags) {
                    Ok(_) => {println!("\t[{}] {}", filename.bright_cyan().bold(), "Tests checked!".green())},
                    Err(e) => {maybe_warn!(e, args)}
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

    if args.test {
        match Command::new("cargo").arg("test").status() {
            Ok(_) => return Ok(()),
            Err(e) => return Err(ErrorKind::IoError(e).into()),
        }
    }

    Ok(())
}

macro_rules! check_has_tests {
	(($flags: expr; $item: ident; $filename: expr; $src: ident; $it: expr;) $($keyword: expr, $ty: pat_param, $item_kind: literal, $it_ident: expr)*) => {
		$(
		if $flags.contains(&$keyword.to_owned()) {
            if let $ty = $item {
				let mut is_tested: bool = false;
                for attr in &$it.attrs {
                    if let Some(ident) = attr.path.get_ident() {
                        if &ident.to_string() == "is_tested" {
                            is_tested = true;
                        }
                    }
                }
                if !is_tested {
                    return Err(ErrorKind::ItemNotTested {
                        src: NamedSource::new($filename, $src.to_owned()),
                        item_name: $it_ident.to_string(),
                        item_kind: $item_kind.to_owned(),
                        span: span!($it, $src),
                    }
                    .into());
                };
            }
        })*
	};
}

fn check_tests(src: &str, filename: &str, file: &File, flags: Vec<String>) -> Result<()> {
    use syn::Item;
	dbg!(&flags);
	for item in &file.items {
        check_has_tests! {
            (flags; item; filename; src; it;)

            "structs", Item::Struct(it),  "struct", it.ident
            "functions", Item::Fn(it), "function", it.sig.ident
			"macros", Item::Macro(it), "macro", it.ident.as_ref().unwrap()
			"new-macros", Item::Macro2(it), "macro 2.0", it.ident
			"traits", Item::Trait(it), "trait", it.ident
			"enums", Item::Enum(it), "enum", it.ident
			"unions", Item::Union(it), "union", it.ident
			"trait-aliases", Item::TraitAlias(it), "trait alias", it.ident
        }
    }
    check_lints(src, filename, &file.items, flags)
}
