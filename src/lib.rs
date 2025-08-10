//! src/lib.rs
//! # Rustadex Stderr (`rdx-stderr`) - Modular Edition
//!
//! An opinionated, ergonomic library for building beautiful and informative
//! command-line interfaces in Rust.
//!
//! This crate provides the `Stderr` logger, a powerful tool inspired by
//! years of handcrafted Bash scripts, now with modular feature support.
//!
//! ## Features
//!
//! - **default**: Includes all features (`trace`, `interactive`, `formatting`, `auto-fn-names`)
//! - **minimal**: Just core logging functionality
//! - **trace**: Hierarchical function tracing with visual tree structure
//! - **interactive**: User prompts, confirmations, and interactive elements
//! - **formatting**: Tables, boxes, banners, and advanced text formatting
//! - **auto-fn-names**: Automatic function name detection for tracing
//!
//! ## Quick Start
//!
//! ```rust
//! use stderr::Stderr;
//!
//! let mut log = Stderr::new();
//! log.info("Hello, world!");
//! ```
//!
//! ## With Hierarchical Tracing
//!
//! ```rust
//! use stderr::{Stderr, qtrace_fn, qcontext};
//!
//! let mut log = Stderr::new();
//! qcontext!("@myapp.VAR.config");
//! qtrace_fn!("parse_config", "starting configuration parsing");
//! qtrace_fn!("parse_config", "config loaded successfully");
//! ```

#[path = "rdx/stderr.rs"]
pub mod stderr;

#[path = "rdx/esc.rs"]
pub mod esc;

#[path = "rdx/utils.rs"]
pub mod utils;

#[path = "rdx/meta.rs"]
pub mod meta;

#[path = "rdx/macros.rs"]
pub mod macros;

// --- Re-export the public API ---

// Core exports (always available)
pub use stderr::{
    Stderr, StderrConfig, LogLevel, OptionFlag, GlyphSet,
    logger, StaticLogger
};

// ESC and styling
pub use esc::colors::Color;
pub use esc::glyphs::{Glyph, debug_glyphs_string};
pub use esc::style::Style;
pub use esc::boxes::{BorderStyle, BoxChars};

// Utilities
pub use utils::helpers::{readline, repeat_char, term_width, env};
pub use utils::flag::flag_table;
pub use utils::grid::print_color_grid;

// Meta information
pub use meta::{STDERR_VERSION as VERSION, help_string};

// Feature-gated exports
#[cfg(feature = "trace")]
pub use stderr::TraceScope;

#[cfg(feature = "interactive")]
pub use stderr::{ConfirmBuilder, InteractiveExt};

#[cfg(feature = "formatting")]
pub use stderr::{TableRow, FormattingExt};

// --- Type Aliases for Convenience ---

pub type Logger = Stderr;
pub type Config = StderrConfig;

// --- Feature Documentation ---

#[cfg(feature = "trace")]
#[doc = "
# Hierarchical Tracing

The `trace` feature adds sophisticated function call tracing with visual hierarchy:

```rust
use stderr::{qtrace_fn, qtrace_auto, qtrace_scope};

// Manual function names
qtrace_fn!(\"my_function\", \"starting work\");
qtrace_fn!(\"my_function\", \"work complete\");

// Automatic function names (requires auto-fn-names feature)
qtrace_auto!(\"this uses the current function name automatically\");

// Scoped tracing with automatic entry/exit
qtrace_scope!(\"my_function\"); // Logs entry and exit automatically
```

Visual output:
```
λ┄┄┄[my_function]
    ┆
    └┄┄> starting work
    └┄┄>> work complete
```
"]
pub mod trace_docs {}

#[cfg(feature = "interactive")]
#[doc = "
# Interactive Features

The `interactive` feature adds user prompts and confirmations:

```rust
use stderr::Stderr;

let mut log = Stderr::new();

// Simple confirmation
if log.confirm(\"Continue?\")?.unwrap_or(false) {
    log.okay(\"Proceeding...\");
}

// Advanced confirmation with styling
if log.confirm_builder(\"Delete all files?\")
    .boxed(true)
    .style(BorderStyle::Heavy)
    .ask()?.unwrap_or(false) {
    log.warn(\"Files deleted\");
}
```
"]
pub mod interactive_docs {}

#[cfg(feature = "formatting")]
#[doc = "
# Advanced Formatting

The `formatting` feature adds tables, boxes, and banners:

```rust
use stderr::{Stderr, BorderStyle};

let mut log = Stderr::new();

// Banners
log.banner(\"Section Title\", '=')?;

// Boxes
log.box_heavy(\"Important message\")?;

// Tables (useful for BookDB ls commands)
log.simple_table(&[
    &[\"Name\", \"Type\", \"Size\"],
    &[\"config.env\", \"file\", \"1.2KB\"],
    &[\"secrets\", \"dir\", \"--\"],
])?;
```
"]
pub mod formatting_docs {}

// --- Legacy Compatibility ---

// Ensure existing code continues to work
pub use stderr::{Stderr as StderrImpl};
pub use esc::colors::Color as ESC;
pub use esc::glyphs::Glyph as ART;
