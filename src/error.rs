use miette::{Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;

#[macro_export]
macro_rules! span {
	($item: expr) => {{
		let span_start = $item.span().start();
		let span_end = $item.span().end();
		(span_start.line * span_start.column, (span_end.column - span_start.column) - (span_start.line * span_start.column)).into()
	}}
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

    #[error(transparent)]
    #[diagnostic(code(IO_ERROR))]
    IoError(#[from] std::io::Error),

    // #[error("Unknown flag: `{flag}`")]
    // UnknownFlag {
    //     src: NamedSource,
    //     flag: String,
    //     span: SourceSpan,
    // },

	#[error("Unexpected token")]
	#[diagnostic(
		code(UNEXPECTED_TOKEN),
		url(docsrs),
		help("Try removing the the unexpected token in `{filename:?}`")		
	)]
	UnexpectedToken {
		filename: String,
		// There's no span.
	}

}
