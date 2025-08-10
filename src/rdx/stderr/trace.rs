//! Hierarchical tracing extension for stderr
//! 
//! This module adds sophisticated tracing capabilities inspired by the bash
//! FUNCNAME array, with visual hierarchy using box-drawing characters.

use super::stderr::{Stderr, OptionFlag};
use crate::esc::colors::Color as ESC;

#[cfg(feature = "trace")]
impl Stderr {
    /// Enhanced hierarchical trace with manual function name
    /// 
    /// This creates a visual tree structure showing function call hierarchy:
    /// λ┄┄┄[function_name]
    ///     ┆
    ///     └┄┄> message
    ///     └┄┄>> continuation message
    pub fn trace_fn(&mut self, func_name: &str, msg: &str) {
        if !self.config.trace { return; }
        self.hierarchical_trace(func_name, msg);
    }

    /// Automatic function name detection (requires auto-fn-names feature)
    /// Note: This is only useful when called from within a #[named] function
    #[cfg(feature = "auto-fn-names")]
    pub fn trace_auto(&mut self, msg: &str) {
        if !self.config.trace { return; }
        // This will only work if called from within a #[named] function
        self.hierarchical_trace("auto", msg);
    }

    /// Fallback for trace_auto when auto-fn-names is disabled
    #[cfg(not(feature = "auto-fn-names"))]
    pub fn trace_auto(&mut self, msg: &str) {
        // Just use regular trace if function names aren't available
        self.trace(msg);
    }

    /// Internal hierarchical trace implementation
    fn hierarchical_trace(&mut self, func_name: &str, msg: &str) {
        if self.check_flag(OptionFlag::Quiet) { return; }

        let same_func = match &self.last_trace_func {
            Some(last) if last == func_name => true,
            _ => false,
        };

        if same_func {
            // Continuation of the same function call
            let formatted = format!("\t└┄┄>> {}", msg);
            self.trace(&formatted);
        } else {
            // Start of a new function branch
            let header = format!("λ┄┄┄[{}]", func_name);
            // Print header and message on separate lines with connectors
            let formatted = format!("{}\n\t┆\n\t└┄┄> {}", header, msg);
            self.trace(&formatted);
            self.last_trace_func = Some(func_name.to_string());
        }
    }

    /// Create a trace scope that automatically traces entry and exit
    /// 
    /// Returns a guard that will log function exit when dropped
    pub fn trace_scope(&mut self, func_name: &str) -> TraceScope<'_> {
        if self.config.trace {
            self.trace_fn(func_name, "entering");
        }
        TraceScope::new(self, func_name)
    }

    /// Reset trace state (useful for testing or context switches)
    pub fn reset_trace_state(&mut self) {
        self.last_trace_func = None;
    }

    /// Get current trace function (for debugging)
    pub fn current_trace_func(&self) -> Option<&str> {
        self.last_trace_func.as_deref()
    }
}

/// RAII guard for automatic function exit tracing
pub struct TraceScope<'a> {
    stderr: &'a mut Stderr,
    func_name: String,
    should_trace: bool,
}

impl<'a> TraceScope<'a> {
    fn new(stderr: &'a mut Stderr, func_name: &str) -> Self {
        let should_trace = stderr.config.trace; // Read before borrowing
        Self {
            stderr,
            func_name: func_name.to_string(),
            should_trace,
        }
    }

    /// Add a step within this function scope
    pub fn step(&mut self, msg: &str) {
        if self.should_trace {
            self.stderr.trace_fn(&self.func_name, msg);
        }
    }

    /// Add a step with debug information
    pub fn step_debug<T: std::fmt::Debug>(&mut self, msg: &str, value: &T) {
        if self.should_trace {
            let formatted = format!("{}: {:#?}", msg, value);
            self.stderr.trace_fn(&self.func_name, &formatted);
        }
    }
}

impl<'a> Drop for TraceScope<'a> {
    fn drop(&mut self) {
        if self.should_trace {
            self.stderr.trace_fn(&self.func_name, "exiting");
        }
    }
}

/// Additional trace convenience methods
#[cfg(feature = "trace")]
impl Stderr {
    /// Trace with explicit level indication
    pub fn trace_level(&mut self, level: u8, func_name: &str, msg: &str) {
        if !self.config.trace { return; }
        
        let indent = "  ".repeat(level as usize);
        let formatted = format!("{}└┄ [{}] {}", indent, func_name, msg);
        self.trace(&formatted);
    }

    /// Trace function entry (useful for manual instrumentation)
    pub fn trace_enter(&mut self, func_name: &str) {
        self.trace_fn(func_name, "→ entering");
    }

    /// Trace function exit (useful for manual instrumentation)
    pub fn trace_exit(&mut self, func_name: &str) {
        self.trace_fn(func_name, "← exiting");
    }

    /// Trace function exit with return value
    pub fn trace_exit_with<T: std::fmt::Debug>(&mut self, func_name: &str, return_value: &T) {
        let msg = format!("← exiting with: {:#?}", return_value);
        self.trace_fn(func_name, &msg);
    }

    /// Labelled trace helpers (like your bash _make_lbl function)
    pub fn trace_add(&mut self, msg: &str) {
        self.trace_labelled("+", ESC::GREEN, msg);
    }

    pub fn trace_sub(&mut self, msg: &str) {
        self.trace_labelled("-", ESC::RED, msg);
    }

    pub fn trace_found(&mut self, msg: &str) {
        self.trace_labelled("✻", ESC::BLUE, msg);
    }

    pub fn trace_done(&mut self, msg: &str) {
        self.trace_labelled("✔", ESC::GREEN, msg);
    }

    pub fn trace_item(&mut self, msg: &str) {
        self.trace_labelled("⟐", ESC::PURPLE, msg);
    }

    /// Internal helper for labelled traces
    fn trace_labelled(&mut self, label: &str, color: termcolor::Color, msg: &str) {
        if !self.config.trace { return; }
        
        let _ = self.set_fg(color);
        let formatted_prefix = format!("\t└┄┄[ {} ]", label);
        let _ = self.print_with_prefix(color, &formatted_prefix, msg);
        let _ = self.reset();
    }
}
