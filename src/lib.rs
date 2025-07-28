// src/lib.rs


#[path = "lib/stderr.rs"]
mod stderr;

#[path = "lib/esc.rs"]
mod esc;

#[path = "lib/utils.rs"]
mod utils;


// --- HOIST ---

pub use stderr::{Stderr, StderrConfig};


pub use esc::colors::Color;
pub use esc::style::Style;
pub use esc::glyphs::{Glyph, debug_glyphs_string}; // (Assuming your updated glyphs.rs)



pub use esc::boxes::{BorderStyle, BoxChars};


pub use utils::trace::TraceLogger;
pub use utils::flag::flag_table;

// --- ALIASES ---

pub type Logger = Stderr;
pub type Config = StderrConfig;

pub type debug_glyphs = debug_glyphs_string;

pub use utils::flag::flag_table as bitmap;
