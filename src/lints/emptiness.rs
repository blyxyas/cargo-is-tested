use super::{super::span, Pass};
use miette::Result;
use miette::{Diagnostic, NamedSource, SourceSpan};
use syn::{Item, __private::ToTokens, spanned::Spanned};
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
#[error("Empty items aren't recommended")]
#[diagnostic(code(EMPTY_ITEM))]
pub struct Emptiness {
    #[source_code]
    src: NamedSource,
    #[label("Right here")]
    span: SourceSpan,
}

// 12345678
// 1 * 8
//

impl Pass for Emptiness {
    fn check_item(filename: &str, item: &Item) -> Result<()> {
        if let Item::Fn(func) = item {
            if func.block.stmts.is_empty() {
                let span_start = func.sig.span().start();
                let span_end = func.sig.span().end();
                Err(Emptiness {
                    src: NamedSource::new(filename, func.sig.to_token_stream().to_string()),
                    span: span!(span_start, span_end),
                })?;
            }
        }
        Ok(())
    }
}
