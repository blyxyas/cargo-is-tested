# El (Otro) Atributo

```admonish note
**Este atributo no cambiará tu item.** Es solo un marcador.
```

`is_not_tested` es una *crate* opcional, y el atributo opuesto a `is_tested`. Pueden tomar una razón (*opcional*), y marca que el item no será testeado intencionalmente.

Es comparable a `#[rustfmt::skip]`

## 📦 Instalación

Escribe esto en tu archivo `Cargo.toml` para instalar `is_not_tested`:

```toml
[dependencies]
is_not_tested = "0.1.0"
```

Ahora, **✨ ¡Está listo para usar! ✨**

---

## ❓ Uso

Elige el item que no quieres testear, importa el atributo `is_not_tested`, y luego puedes usarlo. Puede tomar un argumento (reason, la razón de porqué no hay tests), pero es opcional.

```rust, ignore
#! is-tested strict

use is_not_tested::is_not_tested;

#[is_not_tested(reason = "Too simple to have useful tests.")]
struct MyStruct(String);
```