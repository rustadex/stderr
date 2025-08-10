//! Stderr module - Modular logging with optional features

// Main stderr implementation in the stderr/ subdirectory
#[path = "stderr/stderr.rs"]
pub mod stderr;

// Feature-gated extension modules in the stderr/ subdirectory
#[cfg(feature = "trace")]
#[path = "stderr/trace.rs"]
pub mod trace;

#[cfg(feature = "interactive")]
#[path = "stderr/interactive.rs"]
pub mod interactive;

#[cfg(feature = "formatting")]
#[path = "stderr/formatting.rs"]
pub mod formatting;

// Static logger in the stderr/ subdirectory
#[path = "stderr/static_logger.rs"]
pub mod static_logger;

// Re-export everything from the main stderr implementation
pub use stderr::*;

// Feature-gated re-exports
#[cfg(feature = "trace")]
pub use trace::TraceScope;

#[cfg(feature = "interactive")]
pub use interactive::{ConfirmBuilder, InteractiveExt};

#[cfg(feature = "formatting")]
pub use formatting::{TableRow, FormattingExt};

// Static logger
pub use static_logger::{LOGGER as logger, StaticLogger};

// Type aliases
pub type Logger = Stderr;
pub type Config = StderrConfig;
