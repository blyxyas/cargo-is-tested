# The Binary

`cargo-is-tested` is used to check your tests, ensure their quality, their abundance and their importance.

## 📦 Installation

### Using crates.io

You can install the binary using:

```bash
$	cargo install cargo-is-tested
```


### Manual installation

To install the tool, you're going to need the following.

* [Rust](https://www.rust-lang.org/tools/install)
* Cargo <sub><span style="color: gray;">(Comes with Rust)</span></sub>
* [Git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git)

#### Cloning the repo

Clone the github repository:

```
$	git clone https://github.com/blyxyas/cargo-is-tested
```

#### Install the project

```
$	cd cargo-is-tested;
		cargo install --path .
```

---

Now, **✨ It's ready to use! ✨**

You can use at any point the following command to check that all your tests are there, with the dicted quality control.

```bash
$	cargo is-tested <my project path>
```

## ❓ Usage

Using the ecosystem is simple, but requires learning what to do.

You'll need to also know how to use `is_tested` (attribute), please, read [that chapter](is_tested.md) before continuing.

---

*Okay!* I'll assume that you already know how to use the `is_tested` attribute, because you read the `is_tested` chapter.

---

The most innovative concept in this binary is the use of *shebangs* (`#!`). Yes, it turns out that Rust accepts shebangs as a valid token, even with not much functionality.

So, we can use a shebang at the start of a file to declare which lints we want (**also possible with the CLI**).

```
#! is-tested
```

You'll write this in the first line of a file to enable testing, the binary will parse this later, and **will not cause an error.**

Now, you can apply any lint you want, and each item will be subject to your linting.
To skip an item from linting, you can use the `is_not_tested` attribute. [It has its own chapter](is_not_tested.md).

```admonish example
I want to check that all my functions are checked, except my function `main`.
```

```rust, ignore
#! is-tested functions

use is_tested::is_tested;
use is_not_tested::is_not_tested;

#[is_not_tested(reason = "It's the main function, duuuh")] // Reasons are optional!
fn main() {
	// [...]
}

#[is_tested("tests/a_function.rs")]
fn a_function() {
	// [...]
}

// No testing here, Oh no!
fn another_function() {
	// [...]
}
```

The code above will cause an error, a **🌌 pretty error 🌟**, to be specific, because you specified (with that `functions` lint) that all functions must be tested.

There are lints for a lot of other items (check `cargo is-tested --help`), like structs, traits, macros...

---

### In-test lints

Some lints are applied to ensure that an item type is tested, others are applied to ensure that your tests have an amount of quality control.

For example, the lint `emptiness` will check that your tests don't contain empty functions.

---

## Error reporting

Using [miette]'s error reporting capabilities, we can send beautiful errors, so that you know in exactly which parts of your file you must resign as a programmer.

For example, if you create a function `main` that is empty, it will give you a warning, a **beautiful one**.

<div align="center">
<img src="https://raw.githubusercontent.com/blyxyas/cargo-is-tested/master/assets/output-screenshot.png" height="400" width="auto"/>
</div>

[miette]: https://github.com/zkat/miette