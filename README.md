# Rustadex Stderr (`rdx-stderr`)


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

> Internally referenced as `stderr`

## Using Stderr

```rust

use stderr::{Stderr, StderrConfig}; // import StderrConfig or use default

fn main() {
    let config = StderrConfig {
        quiet: true,
        debug: false,
        trace: false,
        silly: false,
    };

    let mut stderr = Stderr::with_config(config); //can also just call it logger

    stderr.info("This will not be printed.").unwrap();
}

//or simple

use stderr::Stderr;

fn main() {
    let mut logger = Stderr::new();
    logger.info("This is so clean!").unwrap();
}
```


## Use Color, Style, Glyphs
```rust
// The `use` paths are simple and clean, hiding the `pkgs` directory.
use stderr::{Stderr, Color, Style, Glyph};

//using unicode glyphs and color escapes
fn main() {
    let mut stderr = Stderr::new();
    let _red = Color::RED;
    let _pawn = Glyph::PAWN;
    // ...
}

//using stderr logger
fn main() {
    let mut stderr = Stderr::new();
    stderr.okay("Module structure is clear and API is flattened!").unwrap();
    println!("Using a glyph: {}", Glyph::PAWN);

}
```
## Confirm

```rust
let mut stderr = Stderr::new();
if stderr.confirm("Proceed?")?.unwrap_or(false) {
    // ...
}

```
## Boxed Confirm

```rust
let mut stderr = Stderr::new();
let prompt = "This is a critical action.\nAll data will be erased.\nAre you absolutely sure?";

// Use the builder to create a much more impactful prompt.
if stderr.confirm_builder(prompt)
      .boxed(true)
      .style(BorderStyle::Heavy) // Use a heavy border for emphasis
      .ask()?
      .unwrap_or(false)
{
    stderr.warn("Erasing all data...")?;
}
```

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
        ├── utils       <-- The `esc` module root
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
