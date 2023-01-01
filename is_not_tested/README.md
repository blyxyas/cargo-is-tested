# `is_not_tested`

This crate provides the attribute `is_not_tested`, this attribute won't convert your input to anything, it's just a marker for the [`cargo-is-tested`] crate.

The crate is supposed to be used with the [`cargo-is-tested`]. **It is useless without the binary**

[`cargo-is-tested`]: https://github.com/blyxyas/cargo-is-tested

## Usage

```rust
use is_not_tested::is_not_tested;

#[is_not_tested]
fn my_function() {
	// [...]
}
```