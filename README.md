# 🧵 Vinculum

**Vinculum** is an experimental project exploring **high-performance interoperability between Haskell and Rust**, with a focus on **memory-mapped shared data** and clean FFI boundaries.

The goal is to combine:
- 🧠 **Haskell** for high-level abstractions, safety, and expressiveness
- 🦀 **Rust** for low-level control, performance, and memory safety

> *Vinculum* means “bond” or “link” in Latin — a fitting name for a project about connecting two strong ecosystems.

---

## Motivation

Haskell excels at correctness and abstraction, while Rust shines in systems programming and memory-safe low-level work.  
This project investigates a pragmatic way to **bind the two languages together** without sacrificing their respective strengths.

Key questions behind Vinculum:
- How clean can a Haskell ↔ Rust boundary be?
- Can `mmap` be used as a fast, shared communication layer?
- What is the minimal, maintainable FFI surface?
- How far can we push performance while keeping APIs ergonomic?

---

## High-Level Architecture

```
Haskell (API, abstractions)
        ^
        |  FFI (C ABI)
        v
Rust (mmap, memory management, performance)
```

- **Rust** exposes a stable C ABI (`cdylib` or `staticlib`)
- **Haskell** provides a safe, idiomatic wrapper around it
- Communication is **bi-directional**: Haskell can call Rust, and Rust can call back into Haskell when needed

---

## Repository Structure

```
vinculum/
├── rust/        # Rust library (mmap, low-level logic)
│   ├── Cargo.toml
│   └── src/
│
├── haskell/     # Haskell library (FFI + high-level API)
│   ├── vinculum.cabal
│   └── src/
│
├── include/     # C headers (if needed)
├── README.md
└── .gitignore
```

---

## Current Status

🚧 **Work in progress / experimental**

Planned milestones:
- [ ] Minimal Rust `mmap` API
- [ ] Haskell FFI bindings
- [ ] Safe resource management (`ForeignPtr`, `bracket`)
- [ ] Benchmarks
- [ ] Documentation & examples

---

## Non-Goals

- Not a general IPC framework
- Not a replacement for existing Haskell FFI tools
- Not production-ready (yet)

This project is primarily **exploratory and educational**, though it aims to stay realistic and well-engineered.

---

## Requirements

- **GHC** (modern version)
- **Cabal**
- **Rust** (stable)
- Unix-like OS recommended (Linux/macOS)

---

## Philosophy

- Minimal interfaces
- Explicit ownership
- Clear separation of concerns
- No magic
- Performance where it matters

---

## Licensing

This project is licensed under the MIT License.
