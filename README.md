



## License

This project is **dual-licensed**:

- **AGPLv3** — Free for personal, academic, or non-commercial use.
- **Commercial license** — Required for any commercial, enterprise, or revenue-generating use.

Each package in the Rustadex is individually licensed under the GNU AGPLv3.

Use of the entire Codex, any bundled subset, or derivative works for commercial, enterprise, or business development purposes requires a commercial license. This includes usage in internal tools, client-facing products, hosted services, or revenue-generating projects.

We're committed to supporting individual developers, students, and the open-source community — the AGPLv3 lets you use and modify freely. If you're a company or startup, please do the right thing and reach out for licensing.


To inquire about commercial licensing, contact [qode(no-botz)ninjax[at]gmail.com].

## Tree

```bash
├── Cargo.toml
└── src
    ├── lib.rs          <-- The Public API definition
    └── pkgs
        ├── esc.rs      <-- The `esc` module root
        ├── esc
        │   ├── color.rs
        │   ├── glyphs.rs
        │   └── style.rs
        ├── utils       <-- The `esc` module root
        │   └─── trace.rs  
        └── stderr.rs   <-- The `stderr` module implementation
        └── utils.rs  
```

## Using Stderr

```rust



use RdxStderr::{Stderr, StderrConfig}; // import StderrConfig or use default

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
use stderr::{Stderr, Color, Style, Glyph, TraceLogger};

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
