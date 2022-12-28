use proc_macro::TokenStream;
use syn::{parse_macro_input, AttributeArgs};

use darling::FromMeta;

#[derive(Debug, FromMeta)]
struct MacroArgs(String);

#[proc_macro_attribute]
pub fn is_tested(args: TokenStream, input: TokenStream) -> TokenStream {
	let attr_args = parse_macro_input!(args as AttributeArgs);
	let _args = match MacroArgs::from_nested_meta(&attr_args[0]) {
		Ok(_) => {}
		Err(e) => { return TokenStream::from(e.write_errors()) }
	};
	input
}