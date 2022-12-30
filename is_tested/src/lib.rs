use proc_macro::TokenStream;
use proc_macro_error::{emit_call_site_error, proc_macro_error};
use syn::{parse_macro_input, AttributeArgs, Lit, NestedMeta};

const FORBIDDEN_PATHS: [&str; 4] = ["todo", "none", "fixme", ""];

#[proc_macro_error]
#[proc_macro_attribute]
pub fn is_tested(args: TokenStream, input: TokenStream) -> TokenStream {
    let attr_args = parse_macro_input!(args as AttributeArgs);
    if let NestedMeta::Lit(Lit::Str(litstr)) = &attr_args[0] {
        let value = litstr.value();
        if FORBIDDEN_PATHS.contains(&&value[..]) {
            emit_call_site_error!(
                "You need to put the path for where the tests are.";
                help = "try with: `#[is_tested(\"tests/myfunction.rs\")]`";
            );
        }
    } else {
        emit_call_site_error!(
            "You need to put the path for where the tests are.";
            help = "try with: `#[is_tested(\"tests/myfunction.rs\")]`";
        );
    }
    input
}
