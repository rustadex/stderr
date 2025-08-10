//! Stderr module - Modular logging with optional features
//! 
//! This module provides a feature-gated approach to stderr functionality:
//! - Core: Basic logging (always available)
//! - Trace: Hierarchical tracing with function tracking
//! - Interactive: Prompts, confirmations, user input
//! - Formatting: Tables, boxes, banners, advanced formatting

// Core functionality (always available)
pub mod core;

// Feature-gated modules
#[cfg(feature = "trace")]
pub mod trace;

#[cfg(feature = "interactive")]
pub mod interactive;

#[cfg(feature = "formatting")]
pub mod formatting;

// Re-export core types
pub use core::{
    Stderr, StderrConfig, LogLevel, OptionFlag, GlyphSet
};

// Feature-gated re-exports
#[cfg(feature = "trace")]
pub use trace::TraceScope;

#[cfg(feature = "interactive")]
pub use interactive::{ConfirmBuilder, InteractiveExt};

#[cfg(feature = "formatting")]
pub use formatting::{TableRow, FormattingExt};

// Static logger (always available)
pub mod static_logger;
pub use static_logger::{LOGGER as logger, StaticLogger};

// Convenient type aliases
pub type Logger = Stderr;
pub type Config = StderrConfig;

// Re-export for macros
pub use crate::esc::colors::Color;
pub use crate::esc::glyphs::Glyph;

// Legacy aliases for backward compatibility
pub use core::{Stderr as StderrImpl};
pub use crate::esc::colors::Color as ESC;
pub use crate::esc::glyphs::Glyph as ART;
