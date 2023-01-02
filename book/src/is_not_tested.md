# The (Other) Attribute

```admonish note
**This attribute will not change your item.** It's just a marker.
```

`is_not_tested` is an optional crate, and the opposite attribute to `is_tested`. It can take a reason (*optional*), and it marks that the item won't be tested, and that's intentional.

It is comparable to `#[rustfmt::skip]`.

## 📦 Installation

Write this in your `Cargo.toml` file to install `is_not_tested`:

```toml
[dependencies]
is_not_tested = "0.1.0"
```

Now, **✨ It's ready to use ✨**

---
## ❓ Usage

Pick the item that you don't want to test, import the `is_not_tested` attribute, and then use it. It can take a reason, or not.

```rust, ignore
#! is-tested strict

use is_not_tested::is_not_tested;

#[is_not_tested(reason = "Too simple to have useful tests.")]
struct MyStruct(String);
```

---

Now continue reading the main chapter, [*The Binary*](cargo_is_tested.md).