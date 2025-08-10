//! Core stderr functionality - basic logging without extensions

use std::fmt::{Display, Debug};
use std::io::{self, Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use crate::esc::colors::Color as ESC;

use crate::utils::helpers::{term_width, env};

/// Logging levels for the core logger
pub enum LogLevel {
    Okay,
    Info,
    Note,
    Warn,
    Error,
    Debug,
    Trace,
    Magic,
    Silly,
    DevLog,
}

/// Configuration flags
pub enum OptionFlag {
    Quiet,
    Dev,
    Debug,
    Trace,
    Silly,
}

/// Core configuration for stderr
#[derive(Debug, Clone, Default)]
pub struct StderrConfig {
    pub quiet: bool,
    pub dev: bool,
    pub debug: bool,
    pub trace: bool,
    pub silly: bool,
}

impl StderrConfig {
    /// Creates configuration from environment variables
    pub fn from_env() -> Self {
        Self {
            quiet: env("QUIET_MODE").is_ok(),
            debug: env("DEBUG_MODE").is_ok(),
            dev: env("DEV_MODE").is_ok(),
            trace: env("TRACE_MODE").is_ok(),
            silly: env("SILLY_MODE").is_ok(),
        }
    }
}

/// Debug printer wrapper for pretty-printing Debug types
pub struct DebugPrinter<'a> {
    inner: &'a mut Stderr,
}

impl<'a> DebugPrinter<'a> {
    pub fn warn<T: Debug>(&mut self, value: &T) {
        self.inner.warn_debug(value);
    }

    pub fn info<T: Debug>(&mut self, value: &T) {
        self.inner.info_debug(value);
    }

    pub fn error<T: Debug>(&mut self, value: &T) {
        self.inner.error_debug(value);
    }

    pub fn trace<T: Debug>(&mut self, value: &T) {
        self.inner.trace_debug(value);
    }

    pub fn silly<T: Debug>(&mut self, value: &T) {
        self.inner.silly_debug(value);
    }

    pub fn magic<T: Debug>(&mut self, value: &T) {
        self.inner.magic_debug(value);
    }

    pub fn okay<T: Debug>(&mut self, value: &T) {
        self.inner.okay_debug(value);
    }

    pub fn note<T: Debug>(&mut self, value: &T) {
        self.inner.note_debug(value);
    }

    pub fn devlog<T: Debug>(&mut self, value: &T) {
        self.inner.devlog_debug(value);
    }

    pub fn debug<T: Debug>(&mut self, value: &T) {
        self.inner.debug_debug(value);
    }
}

/// Core stderr struct with basic logging functionality
pub struct Stderr {
    pub(crate) config: StderrConfig,
    pub(crate) writer: StandardStream,
    pub(crate) width: usize,
    pub(crate) label: Option<String>,
    
    // Context tracking for banner display
    pub(crate) current_context: Option<String>,
    
    // Glyph customization
    pub(crate) glyphs: GlyphSet,
    
    // Feature-specific state (only compiled in when features are enabled)
    #[cfg(feature = "trace")]
    pub(crate) last_trace_func: Option<String>,
}

/// Customizable glyph set for different logging functions
#[derive(Debug, Clone)]
pub struct GlyphSet {
    pub info: &'static str,
    pub warn: &'static str,
    pub error: &'static str,
    pub okay: &'static str,
    pub trace: &'static str,
    pub debug: &'static str,
    pub magic: &'static str,
}

impl Default for GlyphSet {
    fn default() -> Self {
        Self {
            info: "\u{03BB}",      // λ
            warn: "\u{25B3}",      // △
            error: "\u{2715}",     // ✕
            okay: "\u{2713}",      // ✓
            trace: "\u{2026}",     // …
            debug: "\u{232C}",     // ⌬
            magic: "\u{21AF}",     // ↯
        }
    }
}

impl Default for Stderr {
    fn default() -> Self {
        Self::new()
    }
}

impl Stderr {
    /// Creates a new logger with environment-based configuration
    pub fn new() -> Self {
        Self {
            config: StderrConfig::from_env(),
            writer: StandardStream::stderr(ColorChoice::Auto),
            width: term_width(),
            label: None,
            current_context: None,
            glyphs: GlyphSet::default(),
            #[cfg(feature = "trace")]
            last_trace_func: None,
        }
    }

    /// Creates logger with custom configuration
    pub fn with_config(config: StderrConfig) -> Self {
        Self {
            config,
            writer: StandardStream::stderr(ColorChoice::Auto),
            width: term_width(),
            label: None,
            current_context: None,
            glyphs: GlyphSet::default(),
            #[cfg(feature = "trace")]
            last_trace_func: None,
        }
    }

    /// Customize the glyph set for this logger
    pub fn with_glyphs(mut self, glyphs: GlyphSet) -> Self {
        self.glyphs = glyphs;
        self
    }

    /// Set individual glyphs
    pub fn set_glyph(&mut self, level: LogLevel, glyph: &'static str) {
        match level {
            LogLevel::Info => self.glyphs.info = glyph,
            LogLevel::Warn => self.glyphs.warn = glyph,
            LogLevel::Error => self.glyphs.error = glyph,
            LogLevel::Okay => self.glyphs.okay = glyph,
            LogLevel::Trace => self.glyphs.trace = glyph,
            LogLevel::Debug => self.glyphs.debug = glyph,
            LogLevel::Magic => self.glyphs.magic = glyph,
            _ => {} // Others don't have configurable glyphs yet
        }
    }

    // --- Label Management ---
    
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn set_label(&mut self, label: impl Into<String>) {
        self.label = Some(label.into());
    }

    pub fn clear_label(&mut self) {
        self.label = None;
    }

    // --- Context Management ---
    
    /// Set context and show banner if it changed
    pub fn set_context(&mut self, context: &str) {
        let context_str = context.to_string();
        
        // Only show banner if context actually changed
        if self.current_context.as_ref() != Some(&context_str) {
            self.context_banner(&context_str);
            self.current_context = Some(context_str);
        }
    }

    /// Clear current context
    pub fn clear_context(&mut self) {
        self.current_context = None;
    }

    /// Display context change banner
    fn context_banner(&mut self, context: &str) {
        if self.check_flag(OptionFlag::Quiet) { return; }
        
        let msg = format!(" Context: {} ", context);
        let width = self.width.min(60); // Cap banner width
        let msg_len = msg.chars().count();
        
        if msg_len >= width {
            let _ = writeln!(&mut self.writer, "--- {} ---", context);
            return;
        }
        
        let total_fill = width - msg_len;
        let left_fill = total_fill / 2;
        let right_fill = total_fill - left_fill;
        
        let left_bar = "-".repeat(left_fill);
        let right_bar = "-".repeat(right_fill);
        
        let _ = self.set_fg(ESC::BLUE);
        let _ = writeln!(&mut self.writer, "{}{}{}", left_bar, msg, right_bar);
        let _ = self.reset();
    }

    // --- Configuration ---
    
    pub fn set_quiet(&mut self, quiet: bool) {
        self.config.quiet = quiet;
    }

    pub fn set_debug(&mut self, debug: bool) {
        self.config.debug = debug;
    }

    pub fn set_trace(&mut self, trace: bool) {
        self.config.trace = trace;
    }

    pub fn set_silly(&mut self, silly: bool) {
        self.config.silly = silly;
    }

    pub fn set_dev(&mut self, dev: bool) {
        self.config.dev = dev;
    }

    pub fn check_flag(&self, flag: OptionFlag) -> bool {
        match flag {
            OptionFlag::Quiet => self.config.quiet,
            OptionFlag::Dev => self.config.dev,
            OptionFlag::Debug => self.config.debug,
            OptionFlag::Trace => self.config.trace,
            OptionFlag::Silly => self.config.silly,
        }
    }

    // --- Low-Level Output Methods ---
    
    pub fn set_color(&mut self, spec: &ColorSpec) -> io::Result<()> {
        if self.check_flag(OptionFlag::Quiet) { return Ok(()); }
        self.writer.set_color(spec)
    }

    pub fn set_fg(&mut self, color: Color) -> io::Result<()> {
        self.writer.set_color(ColorSpec::new().set_fg(Some(color)))
    }

    pub fn set_bg(&mut self, color: Color) -> io::Result<()> {
        self.writer.set_color(ColorSpec::new().set_bg(Some(color)))
    }

    pub fn set_bold_fg(&mut self, color: Color) -> io::Result<()> {
        self.writer.set_color(ColorSpec::new().set_fg(Some(color)).set_bold(true))
    }

    pub fn write(&mut self, msg: impl Display) -> io::Result<()> {
        if self.check_flag(OptionFlag::Quiet) { return Ok(()); }
        write!(&mut self.writer, "{}", msg)
    }

    pub fn reset(&mut self) -> io::Result<()> {
        if self.check_flag(OptionFlag::Quiet) { return Ok(()); }
        self.writer.reset()
    }

    pub fn newline(&mut self) -> io::Result<()> {
        if self.check_flag(OptionFlag::Quiet) { return Ok(()); }
        writeln!(&mut self.writer)?;
        Ok(())
    }

    pub fn print(&mut self, msg: &str) -> io::Result<()> {
        if self.check_flag(OptionFlag::Quiet) { return Ok(()); }
        writeln!(&mut self.writer, "{}", msg)?;
        self.writer.reset()
    }

    pub fn print_with_prefix(&mut self, color: Color, prefix: impl Display, msg: &str) -> io::Result<()> {
        if self.check_flag(OptionFlag::Quiet) { return Ok(()); }
        
        self.set_fg(color)?;
        let formatted_prefix = match &self.label {
            Some(label) => format!("[{}][{}]", label, prefix),
            None => format!("[{}]", prefix),
        };

        write!(&mut self.writer, "{} ", formatted_prefix)?;
        writeln!(&mut self.writer, "{}", msg)?;
        self.writer.reset()
    }

    // --- Core Logging Methods ---
    
    pub fn fatal(&mut self, msg: &str) -> ! {
        let _ = self.error(msg);
        std::process::exit(1);
    }

    pub fn error(&mut self, msg: &str) {
        let _ = self.print_with_prefix(ESC::RED, self.glyphs.error, msg);
    }

    pub fn warn(&mut self, msg: &str) {
        let _ = self.print_with_prefix(ESC::ORANGE, self.glyphs.warn, msg);
    }

    pub fn info(&mut self, msg: &str) {
        let _ = self.print_with_prefix(ESC::BLUE, self.glyphs.info, msg);
    }

    pub fn okay(&mut self, msg: &str) {
        let _ = self.print_with_prefix(ESC::GREEN, self.glyphs.okay, msg);
    }

    pub fn note(&mut self, msg: &str) {
        let _ = self.print_with_prefix(ESC::BLUE, "\u{2192}", msg); // →
    }

    pub fn debug(&mut self, msg: &str) {
        if !self.config.debug { return; }
        let _ = self.print_with_prefix(ESC::CYAN, self.glyphs.debug, msg);
    }

    pub fn devlog(&mut self, msg: &str) {
        if !self.config.dev { return; }
        let _ = self.print_with_prefix(ESC::RED2, self.glyphs.debug, msg);
    }

    pub fn trace(&mut self, msg: &str) {
        if !self.config.trace { return; }
        let _ = self.print_with_prefix(ESC::GREY, self.glyphs.trace, msg);
    }

    pub fn magic(&mut self, msg: &str) {
        if !self.config.silly { return; }
        let _ = self.print_with_prefix(ESC::PURPLE, self.glyphs.magic, msg);
    }

    pub fn silly(&mut self, msg: &str) {
        if !self.config.silly { return; }
        let _ = self.print_with_prefix(ESC::MAGENTA, "\u{03C6}", msg); // φ
    }

    /// Get access to the debug printer interface
    pub fn inspect(&mut self) -> DebugPrinter<'_> {
        DebugPrinter { inner: self }
    }

    pub fn log(&mut self, level: LogLevel, msg: &str) {
        let (color, symbol) = match level {
            LogLevel::Okay => (ESC::GREEN, self.glyphs.okay),
            LogLevel::Warn => (ESC::ORANGE, self.glyphs.warn),
            LogLevel::Error => (ESC::RED, self.glyphs.error),
            LogLevel::Info => (ESC::BLUE, self.glyphs.info),
            LogLevel::Note => (ESC::BLUE, "\u{2192}"), // →
            LogLevel::Debug => (ESC::CYAN, self.glyphs.debug),
            LogLevel::Trace => (ESC::GREY, self.glyphs.trace),
            LogLevel::Magic => (ESC::PURPLE, self.glyphs.magic),
            LogLevel::Silly => (ESC::MAGENTA, "\u{03C6}"), // φ
            LogLevel::DevLog => (ESC::MAGENTA, self.glyphs.debug),
        };

        let _ = self.print_with_prefix(color, symbol, msg);
    }

    // --- Debug Pretty Printing ---
    
    pub fn print_with_prefix_debug<T: Debug>(
        &mut self,
        color: Color,
        prefix: impl Display,
        value: &T,
    ) -> io::Result<()> {
        if self.check_flag(OptionFlag::Quiet) { return Ok(()); }

        self.set_fg(color)?;
        let formatted_prefix = match &self.label {
            Some(label) => format!("[{}]{}", label, prefix),
            None => format!("{}", prefix),
        };

        writeln!(&mut self.writer, "{} {:#?}", formatted_prefix, value)?;
        self.writer.reset()
    }

    pub fn error_debug<T: Debug>(&mut self, value: &T) {
        let _ = self.print_with_prefix_debug(ESC::RED, self.glyphs.error, value);
    }

    pub fn warn_debug<T: Debug>(&mut self, value: &T) {
        let _ = self.print_with_prefix_debug(ESC::ORANGE, self.glyphs.warn, value);
    }

    pub fn info_debug<T: Debug>(&mut self, value: &T) {
        let _ = self.print_with_prefix_debug(ESC::BLUE, self.glyphs.info, value);
    }

    pub fn okay_debug<T: Debug>(&mut self, value: &T) {
        let _ = self.print_with_prefix_debug(ESC::GREEN, self.glyphs.okay, value);
    }

    pub fn note_debug<T: Debug>(&mut self, value: &T) {
        let _ = self.print_with_prefix_debug(ESC::BLUE, "\u{2192}", value); // →
    }

    pub fn debug_debug<T: Debug>(&mut self, value: &T) {
        if !self.config.debug { return; }
        let _ = self.print_with_prefix_debug(ESC::CYAN, self.glyphs.debug, value);
    }

    pub fn devlog_debug<T: Debug>(&mut self, value: &T) {
        if !self.config.dev { return; }
        let _ = self.print_with_prefix_debug(ESC::RED2, self.glyphs.debug, value);
    }

    pub fn trace_debug<T: Debug>(&mut self, value: &T) {
        if !self.config.trace { return; }
        let _ = self.print_with_prefix_debug(ESC::GREY, self.glyphs.trace, value);
    }

    pub fn magic_debug<T: Debug>(&mut self, value: &T) {
        if !self.config.silly { return; }
        let _ = self.print_with_prefix_debug(ESC::PURPLE, self.glyphs.magic, value);
    }

    pub fn silly_debug<T: Debug>(&mut self, value: &T) {
        if !self.config.silly { return; }
        let _ = self.print_with_prefix_debug(ESC::MAGENTA, "\u{03C6}", value); // φ
    }
}
