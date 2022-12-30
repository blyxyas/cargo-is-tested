<h1 align="center">cargo-is-tested</h1>
<div align="center">
	<a href="https://github.com/blyxyas/cargo-is-tested">
		<img src="https://img.shields.io/badge/github--9cf?style=for-the-badge&logo=github" />
	</a>
	<a href="https://crates.io/crates/cargo-is-tested">
		<img src="https://img.shields.io/badge/Crates.io--fc8d62?style=for-the-badge&labelColor=555555&logo=rust">
	</a>
	<a href="https://docs.rs/cargo-is-tested">
		<img src="https://img.shields.io/badge/Docs.rs--66c2a5?style=for-the-badge&logo=docs.rs">
	</a>
</div>
<br>

`cargo-is-tested` is a way to check which of your items are tested in which aren't, very configurable and with a scripting-friendly API ready to use with your Git hooks.

Also, is uses *✨ pretty colors ✨* for error reporting.

##### ⚠️ This project is in a WIP state. Don't expect some things to work.

# Example

To check if all functions are tested in the current directory.

```bash
$ cargo is-tested .
```

```toml
# Cargo.toml
# [...]

[dependencies]
is_tested = "*" # Check crates.io for the current version of the crate.

# [...]
```

```rust
// src/main.rs

#! is-tested emptiness
// Yes, it uses shebangs to enable testing and flags!

use is_tested::is_tested;

#[is_tested("test/myfunction_testing_path.rs")]
fn my_function() {
	// [...]
}
```

Then, it will check if `test/myfunction_testing_path.rs` exists, if it doesn't, it will output an error.

If the file exists, the program checks all lints against your test, assuring the best quality possible.

<div align="center">
<img src="./assets/output-screenshot.png" height="300" width="auto" />
</div>

## Installation and usage

##### ⚠️ Installation ins't currently possible because the project isn't published yet.

To get started using `cargo-is-tested`, install the binary.

```bash
$ cargo install cargo-is-tested
```

Now, document yourself in 
