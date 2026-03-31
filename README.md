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

#[vinculum_hs::main(haskell_directory = "examples/haskell")]
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

- Rust 1.56+
- GHC 9.0+ with Cabal
- Linux or macOS

### Installation

```toml
[dependencies]
vinculum-hs = "*"
```

### Run the example

```bash
git clone https://github.com/enzoblain/Vinculum
cd Vinculum
cargo run --example math
```

---

## Project structure

```
vinculum/
├── crates/
│   ├── vinculum/               # Core framework
│   │   ├── src/
│   │   │   ├── lib.rs          # Public API
│   │   │   ├── functions.rs    # Auto-generated bindings
│   │   │   ├── runtime.rs      # Haskell RTS management
│   │   │   └── ffi/            # FFI layer
│   │   └── build_scripts/      # Build orchestration
│   └── vinculum-macros/        # #[vinculum::main] macro
├── examples/
│   └── math/
│       ├── main.rs
│       └── Math.hs
└── Cargo.toml
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

Contributions are welcome. Areas of interest: type marshalling, error handling, performance, documentation, and test
infrastructure.

---

## License

MIT — see [LICENSE](LICENSE).
