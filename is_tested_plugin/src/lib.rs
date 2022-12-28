#![crate_type = "dylib"]
#![feature(rustc_private)]

extern crate rustc_driver;
extern crate rustc_ast;

extern crate rustc_lint;
#[macro_use]
extern crate rustc_session;

use rustc_lint::{EarlyContext, EarlyLintPass, LintContext};
use rustc_driver::plugin::Registry;
use rustc_ast::{Item, ItemKind};

const LINTED_VALUES: [&str; 4] = [
	"none",
	"fixme",
	"todo",
	""
];

declare_lint!(IS_TESTED, Warn, "Warn about non-tested items");
declare_lint_pass!(Pass => [IS_TESTED]);

impl<'tcx> EarlyLintPass for Pass {
	fn check_item(&mut self, cx: &EarlyContext, item: &Item) {
		if let ItemKind::Fn(_) = &item.kind {
			let mut has_tests = false;
			dbg!(&item, item.span);
			for attr in &item.attrs {
				if let Some(ident) = attr.ident() {
					if ident.as_str() == "is_tested" {
						has_tests = true;
						if let Some(value) = attr.value_str() {
							let value_str = value.as_str().to_lowercase();
							if LINTED_VALUES.contains(&&value_str[..]) {
								cx.lint(
									IS_TESTED,
									"Item isn't tested",
									|lint| lint.set_span(item.span)
								)
							}
						}
					}
				}
			}
			if !has_tests {
				cx.lint(
					IS_TESTED,
					"Item isn't tested",
					|lint| lint.set_span(item.span)
				)
			}
		}
	}
}

#[no_mangle]
fn __rustc_plugin_registrar(reg: &mut Registry) {
    reg.lint_store.register_lints(&[&IS_TESTED]);
    reg.lint_store.register_early_pass(|| Box::new(Pass));
}