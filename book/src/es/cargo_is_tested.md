# El Ejecutable

`cargo-is-tested` es usado para probar tus tests, garantizar su calidad, abundancia e importancia.

## 📦 Instalación

### Usando crates.io

Puedes instalar el ejecutable usando:

```
$	cargo install cargo-is-tested
```

### Instalación manual

Para instalar la herramienta, necesitarás lo siguiente.

* [Rust](https://www.rust-lang.org/tools/install)
* Cargo <sub><span style="color: gray;">(Viene con Rust)</span></sub>
* [Git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git)

#### Clonando el repositorio

Clona el repositorio de Github:

```
$	git clone https://github.com/blyxyas/cargo-is-tested
```

#### Instala el proyecto

```
$	cd cargo-is-tested;
		cargo install --path .
```

---

Ahora, **✨ ¡Está listo para usar! ✨**

Ahora puedes usar en cualquier momento el siguiente comando, para comprobar que todos tus tests estén ahí, con la calidad dictada.

```
$	cargo is-tested <camino al proyecto>
```

## ❓ Uso

Usar el ecosistema es muy fácil, pero requiere saber qué hacer.

También necesitarás saber como usar `is_tested` (atributo), por favor, lee [ese capítulo](is_tested.md) antes de continuar.

---

*¡Ok!* Asumiré que ya sabes como usar el atributo `is_tested`.

---

El concepto más innovador en el ecosistema es el uso de *shebangs* (`#!`). Sí, resulta que Rust acepta *shebangs* como un *token* válido, incluso sin tener mucha funcionalidad.

Así que, tenemos que usar un *shebang* al principio de un archivo para declarar que *lints* queremos **(también posible con la CLI).**

```
#! is-tested
```

Tienes que escribir esto en la primera linea de un archivo para activar los tests, el ejecutable va a *parsear* esto después, y **no causará un error.**

Ahora, puedes aplicar cualquier *lint* que quieras, y cada item será sujeto a tus *lints*. Para omitir un item, puedes usar el atributo `is_not_tested`. [Tiene su propio capítulo](is_not_tested.md)

```admonish example
Quiero probar que todas las funcionen tienen tests asociados, excepto por la función `main`
```

```rust, ignore
#! is-tested functions

use is_tested::is_tested;
use is_not_tested::is_not_tested;

#[is_not_tested(reason = "Es la función principal, duuuh")] // Reasons son opcionales!
fn main() {
    // [...]
}

#[is_tested("tests/a_function.rs")]
fn a_function() {
    // [...]
}

// Aquí no hay tests, Oh no!
fn another_function() {
    // [...]
}
```

The código superior causará un error, un **🌌 error bonito 🌟**, porque has especificado (con el *lint* `functions`) que todas las funciones deben estár comprobadas.

Hay *lints* para muchos items (Comprueba `cargo is-tested --help`), como structs, traits, macros...

---

### Lints en los tests

Algunos *lints* son aplicados para asegurar que un tipo de item tiene tests, mientras que otros son aplicados para garantizar que tus tests tienen cierta calidad.

Por ejemplo, el *lint* `emptiness` se asegurará que tus tests no contienen funciones vacías

---

## Reporte de errores

Usando las capacidades de reporte de errores de [miette](https://github.com/zkat/miette), podemos mandar errores preciosos, para que puedas saber exáctamente que parte de tu archivo te va hacer replantearte tus habilidades.

Por ejemplo, si creas una función `main` que esté vacía, te dará un aviso, pero **uno muy bonito**.

<div align="center">
<img src="https://raw.githubusercontent.com/blyxyas/cargo-is-tested/master/assets/output-screenshot.png" height="400" width="auto"/>
</div>
