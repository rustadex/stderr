//! A helper that builds on `Stderr` to provide hierarchical trace output.
//!
//! The `TraceLogger` maintains state about the last function it printed a
//! message for and builds a simple tree‑like visualization using box‑drawing
//! characters. Inspired by a Bash implementation that leverages the
//! `FUNCNAME` array, this Rust version does not automatically inspect the call
//! stack (Rust has no direct equivalent of `FUNCNAME`). Instead, callers
//! explicitly provide the name of the current function via the provided macro.
//!
//! If you want automatic function name capture, you can use the
//! [`function_name` crate](https://docs.rs/function_name/latest/function_name/)
//! and its `#[named]` attribute. This generates a `function_name!()` macro
//! within the annotated function that expands to the function’s name, which
//! can then be passed to `TraceLogger::trace`.
//!
//! Example:
//!
//! ```
//! use stderr_lib::Stderr;
//! use trace_logger::{TraceLogger, trace_here};
//! use function_name::named;
//!
//! #[named]
//! fn my_function(logger: &mut TraceLogger) {
//!     trace_here!(logger, "starting work");
//!     // ... do stuff
//!     trace_here!(logger, "finished");
//! }
//!
//! fn main() {
//!     let mut log = Stderr::new();
//!     let mut tracer = TraceLogger::new(&mut log);
//!     my_function(&mut tracer);
//! }
//! ```
// To this:

use crate::stderr_impl::Stderr; // Or whatever you named the module in lib.rs
use crate::esc::colors::Color as StderrColor; // It's good practice to alias

use std::fmt;

/// A struct that wraps a `Stderr` and adds stateful trace formatting.
pub struct TraceLogger<'a> {
    logger: &'a mut Stderr,
    last_func: Option<String>,
}

impl<'a> TraceLogger<'a> {
    /// Create a new `TraceLogger` that writes trace messages via the given
    /// `Stderr`.
    pub fn new(logger: &'a mut Stderr) -> Self {
        Self {
            logger,
            last_func: None,
        }
    }

    /// Emit a trace message associated with the given function name. If this
    /// function differs from the previously traced function, a new branch is
    /// started. Otherwise, the message is printed as a continuation.
    pub fn trace(&mut self, func_name: &str, msg: &str) {
        let same_func = match &self.last_func {
            Some(last) if last == func_name => true,
            _ => false,
        };
        if same_func {
            // Continuation of the same function call.
            let formatted = format!("\t└┄┄>> {}", msg);
            self.logger.trace(&formatted);
        } else {
            // Start of a new function branch.
            let header = format!("λ┄┄┄[{}]", func_name);
            // Print header and message on separate lines with connectors.
            let formatted = format!("{}\n\t┆\n\t└┄┄> {}", header, msg);
            self.logger.trace(&formatted);
            self.last_func = Some(func_name.to_string());
        }
    }

    /// Emit a labelled trace message with a colour and label similar to
    /// `ftrace`, `wtrace`, etc. The label is placed inside square brackets and
    /// coloured appropriately.
    pub fn trace_label(&mut self, label: &str, color: termcolor::Color, msg: &str) {
        // Build label string with colour via the underlying logger.
        // Use the underlying logger's print_with_color to emit the coloured label.
        let prefix = format!("[{}]", label);
        // For labelled traces we don't track last_func, as they are standalone.
        let _ = self.logger.print_with_color(color, "", &format!("{} {}", prefix, msg));
    }

    /// Convenience method for fatal errors (red label).
    pub fn ftrace(&mut self, msg: &str) { self.trace_label("fatal", termcolor::Color::Red, msg); }
    /// Convenience method for errors (bright red label).
    pub fn etrace(&mut self, msg: &str) { self.trace_label("error", termcolor::Color::Ansi256(197), msg); }
    /// Convenience method for warnings (orange label).
    pub fn wtrace(&mut self, msg: &str) { self.trace_label("warn", termcolor::Color::Ansi256(214), msg); }
    /// Convenience method for success (green label).
    pub fn ptrace(&mut self, msg: &str) { self.trace_label("okay", termcolor::Color::Ansi256(10), msg); }
    /// Convenience method for info (cyan label).
    pub fn itrace(&mut self, msg: &str) { self.trace_label("info", termcolor::Color::Cyan, msg); }
    /// Convenience method for magic (purple label).
    pub fn mtrace(&mut self, msg: &str) { self.trace_label("magic", termcolor::Color::Ansi256(213), msg); }
    /// Trace addition (+).
    pub fn trace_add(&mut self, msg: &str) { self.trace_label("+", termcolor::Color::Ansi256(10), msg); }
    /// Trace subtraction (-).
    pub fn trace_sub(&mut self, msg: &str) { self.trace_label("-", termcolor::Color::Red, msg); }
    /// Trace found (✻).
    pub fn trace_found(&mut self, msg: &str) { self.trace_label("✻", termcolor::Color::Blue, msg); }
    /// Trace done (✔).
    pub fn trace_done(&mut self, msg: &str) { self.trace_label("✔", termcolor::Color::Ansi256(10), msg); }
    /// Trace generic item (⟐).
    pub fn trace_item(&mut self, msg: &str) { self.trace_label("⟐", termcolor::Color::Ansi256(213), msg); }
}


/// A macro to simplify tracing at the call site. Requires the `function_name` crate.
///
/// This macro expands to a call to `TraceLogger::trace` with the current
/// function's name. It expects a mutable `TraceLogger` as the first argument
/// and a format string with optional arguments following it.
#[macro_export]
macro_rules! trace_here {
    ($logger:expr, $($arg:tt)*) => {{
        // Requires the `function_name` crate and that the current function is
        // annotated with `#[named]` so that `function_name!()` expands to the
        // name of the function. See the crate documentation for details.
        $logger.trace(function_name!(), &format!($($arg)*));
    }};
}
