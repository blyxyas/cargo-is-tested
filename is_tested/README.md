# `is_tested`

This crate provides the attribute `is_tested`, this attribute won't convert your input to anything, it's just a market for the [`cargo-is-tested`].

The crate is supposed to be used with the [`cargo-is-tested`]. **It is useless without the binary**

[`cargo-is-tested`]: https://github.com/blyxyas/cargo-is-tested

## Usage

```rust
use is_tested::is_tested;

#[is_tested(<path_to_my_function_tests.rs>)]
fn my_function() {
	// [...]
}
```