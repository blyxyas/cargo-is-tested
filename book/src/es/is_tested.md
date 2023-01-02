# The Attribute

El atributo `is_tested` es usado para marcar un item con un test, es una parte esencial del ecosistema, incluso sin cambiar el item.

```admonish note
**Este atributo no cambiará tu item.** Es solo un marcador.
```

## 📦 Instalación

Puedes instalar el atributo escribiendo estó en tu archivo `Cargo.toml`:

```toml
[dependencies]
is_tested = "0.1.1"
```

Esto desatará el poder de `is_tested` en tus manos.

## ❓ Uso

Puedes usar este atributo como cualquier otro, con `#[is_tested]`. El atributo toma un argumento, una *string* que sirve como dirección al archivo en donde están los tests (desde la raíz del proyecto) de ese ítem.

Por ejemplo:

```rust, ignore
use is_tested::is_tested;

#[is_tested("tests/my_func_tests.rs")]
fn my_func() {
    // [...]
}
```

Puedes usar este atributo en *cualquier item testeable*, como structs, funciones, macros... Algunos items no están incluidos en el ecosistema porque testearlos no es útil (como `use`)

---

Ahora continúa leyendo el capítulo principal, [El Ejecutable](cargo_is_tested.md).