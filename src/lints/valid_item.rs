use super::{Pass, super::span};
use miette::{Diagnostic, SourceSpan, NamedSource};
use syn::{Item, spanned::Spanned, __private::ToTokens};
use thiserror::Error;
use miette::Result;


#[derive(Debug, Error, Diagnostic)]
#[error("Not a valid item")]
#[diagnostic(code(NOT_VALID_ITEM))]
pub struct ItemValidness {
	#[source_code]
	src: NamedSource,
	#[label("This token")]
	span: SourceSpan
}

impl Pass for ItemValidness {
	fn check_item(filename: &str, item: &Item) -> Result<()> {
		if let Item::Verbatim(verbatim) = item {
			let span_start = verbatim.span().start();
			let span_end = verbatim.span().end();
			return Err(ItemValidness {
				src: NamedSource::new(
					filename,
					verbatim.to_string(),
				),
				span: span!(span_start, span_end)
			}.into());
		}
		Ok(())
	}
}