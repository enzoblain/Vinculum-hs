# 🧵 Vinculum

**Seamless Haskell–Rust interoperability through automated FFI bindings and procedural macros.**

Write Haskell. Call it from Rust. Let Vinculum handle the rest.

[![Crates.io](https://img.shields.io/crates/v/vinculum-hs.svg)](https://crates.io/crates/vinculum-hs)
[![Docs.rs](https://docs.rs/vinculum-hs/badge.svg)](https://docs.rs/vinculum-hs)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

---

## Overview

Vinculum lets you call Haskell functions directly from Rust with full type safety — no manual FFI, no boilerplate. A
single attribute macro sets up the runtime, and a build script generates the bindings automatically.

> *Vinculum* — Latin for "bond" or "link."

---

## Features

|                        |                                                               |
|------------------------|---------------------------------------------------------------|
| **Zero boilerplate**   | Bindings are generated automatically from your Haskell source |
| **Type-safe**          | Generated wrappers enforce correctness at compile time        |
| **Single macro**       | `#[vinculum::main]` handles runtime initialization            |
| **Transparent builds** | Haskell compilation is orchestrated via Cargo                 |
| **Minimal overhead**   | Direct FFI calls over the C ABI                               |
| **IDE-friendly** | Compile-time generated bindings enable full IDE support: autocompletion, type checking, navigation, and inline documentation |

---

## Example

Suppose you have a Haskell module with a few arithmetic functions:

**`Math.hs`**

```haskell
module Math where

import Data.Int

add :: Int64 -> Int64 -> Int64
add a b = a + b

multiply :: Int64 -> Int64 -> Int64
multiply a b = a * b

factorial :: Int64 -> Int64
factorial 0 = 1
factorial n = n * factorial (n - 1)
```

Vinculum reads the module, generates type-safe Rust wrappers, and makes them available under
`vinculum::functions::math`:

**`main.rs`**

```rust
use vinculum_hs::functions::math::{add, factorial, multiply};

#[vinculum_hs::main]
fn main() {
    let a = 5;
    let b = 10;

    let result = add(a, b);
    println!("{a} + {b} = {result}");

    let result = multiply(a, b);
    println!("{a} * {b} = {result}");

    let result = factorial(a);
    println!("Factorial a = {result}");
}

```

**Output:**

```
add(12, 30)      = 42
multiply(6, 7)   = 42
factorial(10)    = 3628800
```

> **Configuration required:** You must specify where your Haskell files are located, either via:
> - **Environment variable** (for development): `HASKELL_DIR=path/to/haskell cargo run`
> - **Cargo.toml metadata** (for production): See [Configuring Haskell directories](#configuring-haskell-directories-for-your-application) below


No `unsafe` blocks. No manual `extern "C"` declarations. No FFI plumbing.

---

## How it works

```
┌──────────────────────────────┐
│  Rust application            │
│  math::add(12, 30)           │  ← type-safe generated wrapper
└──────────────┬───────────────┘
               │
       ┌───────▼────────┐
       │   Vinculum     │  ← code generation + runtime lifecycle
       └───────┬────────┘
               │  C ABI / FFI
       ┌───────▼────────┐
       │ Haskell runtime│  ← GHC-compiled shared library
       │ math_add(a, b) │
       └────────────────┘
```

1. **Build time** — the build script compiles your Haskell module with GHC and generates Rust binding code.
2. **Compile time** — generated wrappers are included via `functions.rs`; the type system enforces correct usage.
3. **Runtime** — `#[vinculum::main]` initializes and tears down the Haskell RTS transparently.

---

## Getting started

### Requirements

- Rust 1.93+
- GHC 9.0+ with Cabal
- Linux / MacOS / Windows

### Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
vinculum-hs = "0.1"

[package.metadata.vinculum]
haskell_directory = "src/haskell"  # Path to your Haskell modules
```

Or use the `HASKELL_DIR` environment variable at runtime:
```bash
HASKELL_DIR=src/haskell cargo run
```

---

### Run the example

To run an example, specify the Haskell source directory via the `HASKELL_DIR` environment variable:

```bash
git clone https://github.com/enzoblain/Vinculum
cd Vinculum
HASKELL_DIR=examples/haskell cargo run --example math
```

---

### Development setup

**Important:** The `crates/vinculum-hs/src/functions.rs` file is auto-generated during build. To prevent Git
conflicts and accidental commits of generated code, mark it as skip-worktree:

```bash
git update-index --skip-worktree crates/vinculum-hs/src/functions.rs
```

This tells Git to ignore local changes to the file while keeping it under version control. The file will be
regenerated automatically whenever you rebuild the project (`cargo build` or `cargo run`).

To restore tracking if needed:

```bash
git update-index --no-skip-worktree crates/vinculum-hs/src/functions.rs
```

---

## Roadmap

- [x] Automatic binding generation from Haskell modules
- [x] `#[vinculum::main]` procedural macro
- [x] Type-safe FFI wrappers
- [x] Cargo-driven build orchestration
- [ ] Haskell library support — declare Hackage dependencies in `Cargo.toml` and let Vinculum resolve and link them
  automatically
- [ ] Richer Haskell type support (`Maybe`, `Either`, `String`, …)
- [ ] Async / concurrent interoperability
- [ ] Advanced error propagation across the FFI boundary
- [ ] Extended examples and benchmarks

---

## Contributing

Contributions are welcome and appreciated.  
Feel free to open an issue to discuss changes or submit a pull request directly.

Please ensure that your contributions align with the project's goals and maintain code quality and consistency.

---

## License

This project is licensed under the MIT License.  
See the [LICENSE](LICENSE) file for details.