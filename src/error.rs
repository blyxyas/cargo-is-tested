use miette::{Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;

macro_rules! span {
    ($raw_span: ident) => {
        ($raw_span.line, $raw_span.column).into()
    };
}

macro_rules! define_errors {
	($($ident: ident => $code: expr)*) => {$(const $ident: usize = $code;)*};
}

define_errors! {
    FILE_PARSE_ERROR => 001
    FILE_DOESNT_EXIST => 002

}

#[derive(Error, Debug, Diagnostic)]
pub enum ErrorKind {
    /// A file parse error.
	/// This happens because:
	/// 
	/// 1. These tokens are forming an item not interpreted by [`syn`], the parser we use.
	/// 2. This item isn't yet implemented.
	/// 
	/// Being that Rust is a language that is being developed, at any update it could introduce a new item, therefore the list of items needs to be non exhaustive. If this error is highlighting a new Rust feature, you can [open an issue](https://github.com/blyxyas/cargo-is-tested/issues)
	/// 
	/// [More information about how `syn` parses items](https://docs.rs/syn/latest/syn/enum.Item.html)
	/// 
	/// [`syn`]: https://docs.rs/syn
	#[error("Couldn't parse error")]
	#[diagnostic(
		code(FILE_PARSE_ERROR),
		url(docsrs),
		help("Try fixing the highlighted tokens")
	)]
    FileParseError {
        #[source_code]
        src: NamedSource,
        #[label("This token")]
        span: SourceSpan,
    },
	/// The file doesn't exist (or isn't found).
	/// This error is very self expanatory, it is outputed with the filename. Just check if the file exists.
	/// 
	/// 
    #[error("File `{filename}` doesn't exist")]
	#[diagnostic(
		code(FILE_DOESNT_EXIST),
		url(docsrs),
		help("Try checking if the file exists, or creating it.")
	)]
    FileDoesntExist {
        #[source_code]
        filename: String,
    },
}