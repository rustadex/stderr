//! src/lib/esc.rs
//! # Escape Codes & Terminal Styling (`esc`)
//!
//! This module provides the core building blocks for terminal styling,
//! including custom colors, text styles, box-drawing characters, and glyphs.
//!
//! The primary types exported from here are [`colors::Color`], [`style::Style`], and [`glyphs::Glyph`].
#[path = "esc/colors.rs"]
pub mod colors;

#[path = "esc/glyphs.rs"]
pub mod glyphs;

#[path = "esc/style.rs"]
pub mod style;

#[path = "esc/boxes.rs"]
pub mod boxes;
