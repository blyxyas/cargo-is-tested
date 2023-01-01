//! This is a supporting library for the [`cargo-is-tested`](https://crates.io/crates/cargo-is-tested). There is no interest in this crate unless the `cargo-is-tested` crate is being used.

use proc_macro::TokenStream;
use syn::{parse_macro_input, AttributeArgs};
use darling::FromMeta;


#[derive(Debug, FromMeta)]
struct MacroArgs {
	#[darling(default)]
	#[allow(dead_code)]
	reason: Option<String>
}

/// The is_not_tested attribute is the helper function for the [`cargo-is-tested`] crate.
/// It serves as a marker, to declare that an item not being tested is by design.
/// If you're not using the [`cargo-is-tested`] binary, this attribute is totally useless.
/// 
/// [`cargo-is-tested`]: https://crates.io/crates/cargo-is-tested
#[proc_macro_attribute]
pub fn is_not_tested(_args: TokenStream, input: TokenStream) -> TokenStream {
	let attr_args = parse_macro_input!(_args as AttributeArgs);

	let _arguments = match MacroArgs::from_list(&attr_args) {
		Ok(v) => v,
		Err(e) => {
			return TokenStream::from(e.write_errors());
		}
	};

	input
}