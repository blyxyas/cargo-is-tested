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
			return Err(ItemValidness {
				src: NamedSource::new(
					filename,
					verbatim.to_string(),
				),
				span: span!(verbatim)
			}.into());
		}
		Ok(())
	}
}