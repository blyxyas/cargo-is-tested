use proc_macro::TokenStream;
use syn::{parse_macro_input, AttributeArgs};
use darling::FromMeta;


#[derive(Debug, FromMeta)]
struct MacroArgs {
	#[darling(default)]
	reason: Option<String>
}

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