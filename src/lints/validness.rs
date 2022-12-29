use super::{super::span, Pass};
use miette::Result;
use miette::{Diagnostic, NamedSource, SourceSpan};
use syn::{Item, __private::ToTokens, spanned::Spanned};
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
#[error("Not a valid item")]
#[diagnostic(code(NOT_VALID_ITEM))]
pub struct ItemValidness {
    #[source_code]
    src: NamedSource,
    #[label("This token")]
    span: SourceSpan,
}

impl Pass for ItemValidness {
    fn check_item(filename: &str, item: &Item) -> Result<()> {
        if let Item::Verbatim(verbatim) = item {
			dbg!(&verbatim.span().start().line);
            if verbatim.span().start().line != 1 {
                return Err(ItemValidness {
                    src: NamedSource::new(filename, verbatim.to_string()),
                    span: span!(verbatim),
                }
                .into());
            }
        }
        Ok(())
    }
}
