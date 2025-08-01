# Rustadex Stderr (`rdx-stderr`) v.0.5.0


An opinionated, ergonomic library for building beautiful and informative
command-line interfaces in Rust.

This crate provides the `Stderr` logger, a powerful tool inspired by
years of handcrafted Bash scripts.

[![Crates.io Version](https://img.shields.io/crates/v/rdx-stderr.svg)](https://crates.io/crates/rdx-stderr)
[![License: MIT OR Apache-2.0](https://img.shields.io/crates/l/rdx-stderr.svg)](https://github.com/rustadex/stderr/blob/main/LICENSE-MIT)
[![MSRV](https://img.shields.io/badge/msrv-1.70.0-blue.svg)](https://blog.rust-lang.org/2023/06/01/Rust-1.70.0.html)


<p align="center">
  <img src="https://raw.githubusercontent.com/rustadex/stderr/main/.github/assets/pretty.png" alt="A demo of the rdx-stderr logger in action" width="600">
</p>

<p align="center">
  <img src="https://raw.githubusercontent.com/rustadex/stderr/main/.github/assets/boxes.png" alt="A demo of the rdx-stderr logger in action" width="600">
</p>

> Internally referenced as `stderr`


---

## 🚀 What's New (v0.5)

* **Global/static logger:**  Just `use stderr::logger;` — instant everywhere logging
* **Macro logging:**  `qinfo!`, `qwarn!`, `qerror!`, `qpretty!` — log anything, anywhere, styled
* **Pretty debug/inspect:**  `.inspect().info(&obj)`, `.warn_debug(&my_struct)` for Rust-native pretty dumps
* **Custom glyphs & colors:**  Unicode and ANSI at your command
* **Boxed/grid prompts:**  `confirm_builder` with styles, borders, heavy boxes
* **Composable config:**  Toggle `quiet`, `trace`, `debug`, etc., at runtime
* **Bash-inspired, CLI-native:**  The fastest way to make terminal output *sing*
* **Trait-based, extendable:**  Easy to wire in your own glyph sets or color themes

---

## ⚡ Philosophy: Fast, Expressive, Bash DNA

* Terminal output should be beautiful and actionable, not just noise
* Defaults are high signal, but you can theme, override, and hack anything
* Built to match (and outclass) custom Bash stderr libs

---

## ✦ Features

* 🌈 Drop-in `Stderr` logger or global `logger`
* 🔥 Macro suite: `qinfo!`, `qwarn!`, `qerror!`, `qdebug!`, `qpretty!`
* 🧩 Rich debug & inspect: struct pretty-print, multi-level
* 🎨 Color & Unicode glyphs: built-in sets, composable
* 🔧 Builder-style config, runtime toggles, chained setters
* 📦 Boxed, gridded, and styled prompts (even ASCII art)
* 🌀 Ergonomic API — str, Display, Debug, all accepted
* 🚦 Bash/CLI workflow: just log, pretty-print, confirm

---

## 🚀 Quickstart

```rust
use stderr::Stderr;

fn main() {
    let mut log = Stderr::new();
    log.info("Launch successful!");
}
```

### With Global Logger

```rust
use stderr::logger;

fn main() {
    logger.info("This hits everywhere");
    logger.warn("Watch out");
}
```

---

## 🧩 Macro Logging

```rust
qinfo!("Quant mode engaged");
qwarn!("Something's spicy");
qerror!(my_struct); // Auto pretty-prints if Debug
qpretty!("⚡", &payload); // Boxed, multiline pretty-dump
```


---

## 🔍 Debug, Inspect, Pretty Print

```rust
#[derive(Debug)]
struct State { x: u8, tag: String }

let state = State { x: 42, tag: "active".into() };

// Method: pretty debug
logger.info_debug(&state); // Multiline, formatted
logger.inspect().warn(&state); // Via inspect view
```

---

## 🎨 Color, Glyphs, Style

```rust
use stderr::{Color, Glyph, Style};

let mut log = Stderr::new();
log.okay("All systems go");
println!("Using a glyph: {}", Glyph::PAWN);
```

---

## ✅ Prompts & Boxed Confirm

```rust
let mut log = Stderr::new();
if log.confirm("Engage quantum mode?")?.unwrap_or(false) {
    log.okay("Do it.");
}

let critical = "ERASE DISK?\nNo undo.";
if log.confirm_builder(critical)
      .boxed(true)
      .style(BorderStyle::Heavy)
      .ask()?
      .unwrap_or(false) {
    log.warn("This is irreversible");
}
```

<p align="center">
  <img src="https://raw.githubusercontent.com/rustadex/stderr/main/.github/assets/grid.png" alt="A demo of the rdx-stderr logger in action" width="600">
</p>


## Tree

```bash
├── Cargo.toml
└── src
    ├── lib.rs          <-- The Public API definition
    └── lib
        ├── esc.rs      <-- The `esc` module root
        ├── esc
        │   ├── color.rs
        │   ├── glyphs.rs
        │   └── style.rs
        ├── utils       <-- The `utils` module root
        │   ├── grid.rs
        │   └── flag.rs
        └── stderr.rs   <-- The `stderr` module implementation
        └── utils.rs
```

## License

This project is licensed under either of:

*   **Apache License, Version 2.0** ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
*   **MIT license** ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
