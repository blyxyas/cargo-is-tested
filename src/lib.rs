//!<h1 align="center">cargo-is-tested</h1>
//!<div align="center">
//!	<a href="https://github.com/blyxyas/cargo-is-tested">
//!		<img src="https://img.shields.io/badge/github--9cf?style=for-the-badge&logo=github" />
//!	</a>
//!	<a href="https://crates.io/crates/cargo-is-tested">
//!		<img src="https://img.shields.io/badge/Crates.io--fc8d62?style=for-the-badge&labelColor=555555&logo=rust">
//!	</a>
//!	<a href="https://docs.rs/cargo-is-tested">
//!		<img src="https://img.shields.io/badge/Docs.rs--66c2a5?style=for-the-badge&logo=docs.rs">
//!	</a>
//!</div>
//!<br>
//!
//!`cargo-is-tested` is a way to check which of your items are tested in which aren't, very configurable and with a scripting-friendly API ready to use with your Git hooks.
//!
//!Also, is uses *✨ pretty colors ✨* for error reporting.
//!
//!##### ⚠️ This project is in a WIP state. Don't expect some things to work.
//!
//!# Example
//!
//!To check if all functions are tested in the current directory.
//!
//!```bash
//!$ cargo is-tested .
//!```
//!
//!```toml
//!# Cargo.toml
//!# [...]
//!
//![dependencies]
//!is_tested = "*" # Check crates.io for the current version of the crate.
//!
//!# [...]
//!```
//!
//!```rust, ignore
//!// src/main.rs
//!
//!#! is-tested emptiness
//!// Yes, it uses shebangs to enable testing and flags!
//!
//!use is_tested::is_tested;
//!
//!#[is_tested("test/myfunction_testing_path.rs")]
//!fn my_function() {
//!	// [...]
//!}
//!```
//!
//!Then, it will check if `test/myfunction_testing_path.rs` exists, if it doesn't, it will output an error.
//!
//!If the file exists, the program checks all lints against your test, assuring the best quality possible.
//!
//!<div align="center">
//!<img src="./assets/output-screenshot.png" height="300" width="auto" />
//!</div>
//!
//!## Installation and usage
//!
//!##### ⚠️ Installation isn't currently possible because the project isn't published yet.
//!
//!To get started using `cargo-is-tested`, install the binary.
//!
//!```bash
//!$ cargo install cargo-is-tested
//!```
//!
//!Now [document yourself](https://docs.rs/cargo-is-tested/latest/cargo-is-tested/lints) about all the lints you can apply to your tests.
//!
//!* *strict* (Activates all lints, default)
//!* [*emptiness*](https://docs.rs/cargo-is-tested/latest/cargo-is-tested/lints/emptiness)
//!* [*validness*](https://docs.rs/cargo-is-tested/latest/cargo-is-tested/lints/validness)
//!
//!More lints will be added with time.
//!
//!---
//!
//!Once you know the lints you want to apply, import the attribute `is_tested`, then choose a struct, function or any item that you want to test, and add to that item the attribute `#[is_tested("<path to the test>.rs")]`
//!
//!The result should be something like:
//!
//!```rust, ignore
//!#! is-tested strict
//!
//!use is_tested::is_tested;
//!
//!#[is_tested("tests/mystruct.rs")]
//!struct MyStruct<'a> {
//!	string: &'a str
//!}
//!```
//!
//!Don't worry, the attribute won't change anything in your code, it's just a marker for a later-parser to know that you're testing the item.
//!
//!---
//!
//!It's time to run the parser, it will read check that all tested items are tested, and with the correct code quality dicted using the lints.
//!
//!```rust, ignore
//!// tests/mystruct.rs
//!
//!use mycrate::MyStruct;
//!
//!fn main() {
//!	// [...]
//!}
//!```
//!
//!```bash
//!$ cargo is-tested .
//!```
//!
//!This will check that all tests are well written. You can use flags to customize your experience, for example, use `--structs` to check that all structs have tests associated, or use `--test` to, if all tests are confirmed, run `cargo test` automatically.

use std::ops::Range;

pub mod error;
pub mod flags;
pub mod lints;

#[cfg(feature = "suggestions")]
pub fn did_you_mean<'a, T, I>(field: &str, alternatives: I) -> Option<String>
where
    T: AsRef<str> + 'a,
    I: IntoIterator<Item = &'a T>,
{
    let mut candidate: Option<(f64, &str)> = None;
    for pv in alternatives {
        let confidence = ::strsim::jaro_winkler(field, pv.as_ref());
        if confidence > 0.8 && (candidate.is_none() || (candidate.as_ref().unwrap().0 < confidence))
        {
            candidate = Some((confidence, pv.as_ref()));
        }
    }
    candidate.map(|(_, candidate)| candidate.into())
}

#[cfg(not(feature = "suggestions"))]
pub fn did_you_mean<'a, T, I>(field: &str, alternatives: I) -> Option<String>
where
    T: AsRef<str> + 'a,
    I: IntoIterator<Item = &'a T>,
{
    None
}

#[macro_export]
macro_rules! impl_warn {
    ($ty: ty) => {
        impl ::miette::ReportHandler for $ty {
            fn debug(
                &self,
                error: &(dyn Diagnostic),
                f: &mut core::fmt::Formatter<'_>,
            ) -> core::fmt::Result {
                if f.alternate() {
                    return core::fmt::Debug::fmt(error, f);
                }
                write!(f, "{}", error)?;
                Ok(())
            }
        }
    };
}

#[macro_export]
macro_rules! maybe_warn {
    ($e: expr, $args: expr) => {
        if $e.severity() == Some(Severity::Warning) && !$args.deny_warnings {
            println!("WARN: {:?}", $e);
        } else {
            return Err($e);
        }
    };
}
