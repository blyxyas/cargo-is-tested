use std::env;
use std::fs;
use std::io::Read;
use std::process;
use syn;
use syn::AttrStyle;

fn main() {
    for path in fs::read_dir("./src")
        .expect("You need to execute this command in a workspace (no 'src' directory found).")
    {
        let raw_filename = path.unwrap().file_name();
		let filename = raw_filename.to_str().unwrap();
		dbg!(&filename);
        let mut file = fs::File::open(format!("src/{filename}"))
						.expect("Unable to open file");
        let mut src = String::new();
        file.read_to_string(&mut src).expect("Unable to read file");

		let syntax = syn::parse_file(&src).expect("Unable to parse file");

		for attr in syntax.attrs {
			dbg!(&attr);
			if let Some(ident) = attr.path.get_ident() {
				dbg!(&ident.to_string());
				if ident.to_string() == "check_tests" {
					if attr.tokens.to_string() == "strict" {
						dbg!("AAA");
					}
				}
			}
		}
    }
}
