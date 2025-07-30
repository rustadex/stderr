//! src/lib.rs
//! # Rustadex Stderr (`rdx-stderr`)
//!
//! An opinionated, ergonomic library for building beautiful and informative
//! command-line interfaces in Rust.
//!
//! This crate provides the `Stderr` logger, a powerful tool inspired by
//! years of handcrafted Bash scripts.
//!
//! ## Main Structs & Configuration
//!
//! The most important types for getting started.
//!
//! ---
//!
#[path = "rdx/stderr.rs"]
pub mod stderr;

#[path = "rdx/esc.rs"]
pub mod esc;

#[path = "rdx/utils.rs"]
pub mod utils;

#[path = "rdx/meta.rs"]
pub mod meta;

// --- The Curated Public API ---

pub use stderr::{Stderr, StderrConfig, readline, repeat_char, term_width, env };

pub use esc::colors::Color;
pub use esc::glyphs::{Glyph, debug_glyphs_string};
pub use esc::style::Style;

pub use esc::boxes::{BorderStyle, BoxChars};

//pub use utils::trace::TraceLogger;

pub use utils::flag::flag_table;
pub use utils::grid::print_color_grid;

pub use meta::{STDERR_VERSION as VERSION, help_string};

// --- ALIASES ---

pub type Logger = Stderr;
pub type Config = StderrConfig;
