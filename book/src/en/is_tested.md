# The Attribute

The `is_tested` attribute is used to mark an item with a test, it's an essential part of the ecosystem, even without changing the item.

```admonish note
**This attribute will not change your item.** It's just a marker.
```

## 📦 Installation

You can install the attribute by writing this in your `Cargo.toml` file:

```toml
[dependencies]
is_tested = "0.1.1"
```

This will unleash the power of `is_tested` in your hands.

## ❓ Usage

You can use this attribute as any other attribute is used, with `#[is_tested]`, this attribute takes one argument, a string serving as path, this path is the path (from project's root) of the tests for that item.

As an example:

```rust, ignore
use is_tested::is_tested;

#[is_tested("tests/my_func_tests.rs")]
fn my_func() {
	// [...]
}
```

You can use the attribute in **any testable item**, like structs, functions, macros...
Some items aren't included in the ecosystem because testing them isn't useful (like `use` statements).

---

Now continue reading the main chapter, [*The Binary*](cargo_is_tested.md).
