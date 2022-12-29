use super::Pass;
use miette::{Diagnostic, SourceSpan, NamedSource};
use syn::{Item, __private::ToTokens, spanned::Spanned};
use thiserror::Error;
use miette::Result;


#[derive(Debug, Error, Diagnostic)]
#[error("oops!")]
#[diagnostic(
	code(E002)
)]
pub struct Emptiness {
	#[source_code]
	src: NamedSource,
	#[label("Right here")]
	span: SourceSpan,
}

impl Pass for Emptiness {
	fn check_item(item: &Item) -> Result<()> {
		if let Item::Fn(func) = item {
			if func.block.stmts.is_empty() {
				let span = item.span().start();
				Err(Emptiness {
					src: NamedSource::new("bad_file.rs", item.to_token_stream().to_string()),
					span: (span.line, span.column).into()
				})?;
			}
		}
		Ok(())
	}
}
