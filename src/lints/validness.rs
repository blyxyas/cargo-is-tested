use crate::error::ErrorKind;

use super::{super::span, Pass};
use miette::Result;
use miette::{Diagnostic, NamedSource, SourceSpan};
use syn::{spanned::Spanned, Item};
use thiserror::Error;

#[rustfmt::skip]
#[derive(Debug, Error, Diagnostic)]
#[error("Not a valid item")]
#[diagnostic(
	code(NOT_VALID_ITEM),
	severity(warning)
)]
pub struct ItemValidness {
    #[source_code]
    src: NamedSource,
    #[label("this token")]
    span: SourceSpan,
}

impl Pass for ItemValidness {
    fn check_items(source: &str, filename: &str, items: &Vec<Item>) -> Result<()> {
        for item in items {
            if let Item::Verbatim(verbatim) = item {
                if verbatim.span().start().line != 1 {
                    return Err(ErrorKind::FileParseError {
                        src: NamedSource::new(filename, source.to_owned()),
                        span: span!(verbatim),
                        note: Some(verbatim.to_string()),
                    }
                    .into());
                }
            }
        }
        Ok(())
    }
}
