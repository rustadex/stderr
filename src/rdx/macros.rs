//! Enhanced macros for stderr with hierarchical tracing support

// --- Core Logging Macros (unchanged) ---

#[macro_export]
macro_rules! qlog {
    ($level:expr, $msg:expr) => {
        $crate::logger.log($level, $msg)
    };
}

#[macro_export]
macro_rules! qinfo {
    ($($arg:tt)*) => {
        $crate::logger.info(&format!($($arg)*))
    };
}

#[macro_export]
macro_rules! qwarn {
    ($($arg:tt)*) => {
        $crate::logger.warn(&format!($($arg)*))
    };
}

#[macro_export]
macro_rules! qerror {
    ($($arg:tt)*) => {
        $crate::logger.error(&format!($($arg)*))
    };
}

#[macro_export]
macro_rules! qdebug {
    ($($arg:tt)*) => {
        $crate::logger.debug(&format!($($arg)*))
    };
}

#[macro_export]
macro_rules! qokay {
    ($($arg:tt)*) => {
        $crate::logger.okay(&format!($($arg)*))
    };
}

#[macro_export]
macro_rules! qpretty {
    ($prefix:expr, $value:expr) => {
        $crate::logger.raw().print_with_prefix_debug(
            $crate::Color::MAGENTA,
            $prefix,
            &$value
        ).ok();
    };
}

// --- Enhanced Trace Macros ---

/// Simple trace (existing functionality)
#[macro_export]
macro_rules! qtrace {
    ($($arg:tt)*) => {
        $crate::logger.trace(&format!($($arg)*))
    };
}

/// Hierarchical trace with manual function name
#[cfg(feature = "trace")]
#[macro_export]
macro_rules! qtrace_fn {
    ($func_name:expr, $($arg:tt)*) => {
        $crate::logger.raw().trace_fn($func_name, &format!($($arg)*))
    };
}

/// Automatic function name tracing (requires auto-fn-names feature)
#[cfg(all(feature = "trace", feature = "auto-fn-names"))]
#[macro_export]
macro_rules! qtrace_auto {
    ($($arg:tt)*) => {
        $crate::logger.raw().trace_auto(&format!($($arg)*))
    };
}

/// Fallback when auto-fn-names is not available
#[cfg(all(feature = "trace", not(feature = "auto-fn-names")))]
#[macro_export]
macro_rules! qtrace_auto {
    ($($arg:tt)*) => {
        $crate::logger.trace(&format!($($arg)*))
    };
}

/// No-op when trace feature is disabled
#[cfg(not(feature = "trace"))]
#[macro_export]
macro_rules! qtrace_auto {
    ($($arg:tt)*) => {};
}

// --- Trace Scope Macros ---

/// Create a trace scope for automatic entry/exit logging
#[cfg(feature = "trace")]
#[macro_export]
macro_rules! qtrace_scope {
    ($func_name:expr) => {
        let _trace_scope = $crate::logger.raw().trace_scope($func_name);
    };
}

/// Automatic trace scope using current function name
#[cfg(all(feature = "trace", feature = "auto-fn-names"))]
#[macro_export]
macro_rules! qtrace_scope_auto {
    () => {
        let _trace_scope = $crate::logger.raw().trace_scope(function_name::function_name!());
    };
}

// --- Labelled Trace Macros (inspired by bash version) ---

/// Trace addition/creation operations
#[cfg(feature = "trace")]
#[macro_export]
macro_rules! qtrace_add {
    ($($arg:tt)*) => {
        $crate::logger.raw().trace_add(&format!($($arg)*))
    };
}

/// Trace removal/deletion operations
#[cfg(feature = "trace")]
#[macro_export]
macro_rules! qtrace_sub {
    ($($arg:tt)*) => {
        $crate::logger.raw().trace_sub(&format!($($arg)*))
    };
}

/// Trace found/discovery operations
#[cfg(feature = "trace")]
#[macro_export]
macro_rules! qtrace_found {
    ($($arg:tt)*) => {
        $crate::logger.raw().trace_found(&format!($($arg)*))
    };
}

/// Trace completion operations
#[cfg(feature = "trace")]
#[macro_export]
macro_rules! qtrace_done {
    ($($arg:tt)*) => {
        $crate::logger.raw().trace_done(&format!($($arg)*))
    };
}

/// Trace item/element operations
#[cfg(feature = "trace")]
#[macro_export]
macro_rules! qtrace_item {
    ($($arg:tt)*) => {
        $crate::logger.raw().trace_item(&format!($($arg)*))
    };
}

// --- Context Macros ---

/// Set context and automatically display banner if changed
#[macro_export]
macro_rules! qcontext {
    ($context:expr) => {
        $crate::logger.raw().set_context($context)
    };
}

/// Execute code within a temporary context
#[macro_export]
macro_rules! qwith_context {
    ($context:expr, $code:block) => {{
        let mut logger = $crate::logger.raw();
        let old_context = logger.current_context.clone();
        logger.set_context($context);
        let result = $code;
        if let Some(ctx) = old_context {
            logger.set_context(&ctx);
        } else {
            logger.clear_context();
        }
        result
    }};
}

// --- Feature-Gated No-Ops ---

// When trace feature is disabled, provide no-op versions
#[cfg(not(feature = "trace"))]
#[macro_export]
macro_rules! qtrace_fn {
    ($func_name:expr, $($arg:tt)*) => {};
}

#[cfg(not(feature = "trace"))]
#[macro_export]
macro_rules! qtrace_scope {
    ($func_name:expr) => {};
}

#[cfg(not(feature = "trace"))]
#[macro_export]
macro_rules! qtrace_add {
    ($($arg:tt)*) => {};
}

#[cfg(not(feature = "trace"))]
#[macro_export]
macro_rules! qtrace_sub {
    ($($arg:tt)*) => {};
}

#[cfg(not(feature = "trace"))]
#[macro_export]
macro_rules! qtrace_found {
    ($($arg:tt)*) => {};
}

#[cfg(not(feature = "trace"))]
#[macro_export]
macro_rules! qtrace_done {
    ($($arg:tt)*) => {};
}

#[cfg(not(feature = "trace"))]
#[macro_export]
macro_rules! qtrace_item {
    ($($arg:tt)*) => {};
}
