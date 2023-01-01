use miette::{Diagnostic, NamedSource, SourceSpan};
use proc_macro2::LineColumn;
use thiserror::Error;

pub fn get_span(src: &str, start: &LineColumn, end: &LineColumn) -> SourceSpan {
    let mut remaining_lines = end.line - 1;
    let mut bytepos_start: usize = 0;
    let mut bytepos_end: usize = 0;
	for (i, c) in src.chars().enumerate() {
        if c == '\n' {
			remaining_lines -= 1;
			if end.line - remaining_lines == start.line {
				bytepos_start = i + start.column + 1;
			} else {
				bytepos_end = i + end.column;
			}
        }
    }
    return (bytepos_start..bytepos_end).into();
}

#[macro_export]
macro_rules! span {
    ($item: expr, $source: ident) => {{
        $crate::error::get_span($source, &$item.span().start(), &$item.span().end())
    }};
}

#[derive(Error, Debug, Diagnostic)]
#[non_exhaustive]
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
    #[error("Couldn't parse this token")]
    #[diagnostic(code(FILE_PARSE_ERROR), url(docsrs))]
    FileParseError {
        #[source_code]
        src: NamedSource,
        #[label("this token")]
        span: SourceSpan,
        #[help]
        note: Option<String>,
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
    },

    #[error("This lint doesn't exist")]
    #[diagnostic(code(UNEXPECTED_LINT), url(docsrs))]
    UnknownLint {
        #[source_code]
        src: NamedSource,
        #[label("this lint")]
        span: SourceSpan,
        #[help]
        note: Option<String>,
    },

    /// This error happens when you activate the `structs` lint, and you don't test a struct.
    #[error("The {item_kind} `{item_name}` isn't tested")]
    #[diagnostic(code(ITEM_NOT_TESTED), url(docsrs))]
    ItemNotTested {
        #[source_code]
        src: NamedSource,
        item_name: String,
        item_kind: String,
        #[label("this {item_kind}")]
        span: SourceSpan,
    },
}
