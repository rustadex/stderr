// src/lib/meta.rs


pub const STDERR_VERSION: &str = "0.4.0";

// The help text itself, as a multi-line string constant.
const HELP_TEXT: &str = "
stderr - A Rust library for rich terminal output.

This is the help text for the `stderr` library itself, not an application
using it. It demonstrates the use of the `Stderr::help()` method.

CORE CONCEPTS:
  - Ergonomic, semantic logging functions (.info(), .warn(), .error()).
  - Rich formatting utilities (.banner(), .boxed(), .bitmap()).
  - Automatic environment variable detection (DEBUG_MODE, QUIET_MODE).
  - A flexible builder pattern for programmatic configuration.
";

/// Returns the complete help string for the stderr library.
///
/// This function constructs the help message, including the current version.
pub fn help_string() -> String {
    format!("Version: {}\n{}", STDERR_VERSION, HELP_TEXT)
}
