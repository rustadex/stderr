// src/lib.rs

#[path = "lib/stderr.rs"]
mod stderr;

#[path = "lib/esc.rs"]
mod esc;

#[path = "lib/utils.rs"]
mod utils;

#[path = "lib/meta.rs"]
mod meta;

// --- HOIST ---

pub use stderr::{Stderr, StderrConfig};

pub use esc::colors::Color;
pub use esc::glyphs::{Glyph, debug_glyphs_string as debug_glyphs};
pub use esc::style::Style;

pub use esc::boxes::{BorderStyle, BoxChars};

//pub use utils::trace::TraceLogger;

pub use utils::flag::flag_table as bitmap;

pub use meta::{STDERR_VERSION as VERSION, help_string};

// --- ALIASES ---

pub type Logger = Stderr;
pub type Config = StderrConfig;
