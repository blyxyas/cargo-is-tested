//! \[**Warn**\] Lints when items are empty.
//!
//! ### Example
//!
//! ```rust, ignore
//! fn main() {}
//! ```
//!
//! Will trigger this lint.

use crate::impl_warn;

use super::{super::span, Pass};
use miette::Result;
use miette::{Diagnostic, NamedSource, SourceSpan};
use syn::{spanned::Spanned, Item};
use thiserror::Error;

#[rustfmt::skip]
#[derive(Debug, Error, Diagnostic)]
#[error("Empty items aren't recommended")]
#[diagnostic(
	code(EMPTY_ITEM),
	severity(Warning)
)]
pub struct Emptiness {
    #[source_code]
    src: NamedSource,
    #[label("right here")]
    span: SourceSpan,
}

impl Pass for Emptiness {
    fn check_items(source: &str, filename: &str, items: &Vec<Item>) -> Result<()> {
        for item in items {
            if let Item::Fn(func) = item {
                if func.block.stmts.is_empty() {
                    Err(Emptiness {
                        src: NamedSource::new(filename, source.to_owned()),
                        span: span!(func.sig),
                    })?;
                }
            }
        }
        Ok(())
    }
}

impl_warn! { Emptiness }