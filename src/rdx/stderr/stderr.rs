//! src/lib/stderr.rs

//! A simple library for printing colorful, structured messages to stderr.
//!
//! This module provides a `Stderr` struct that encapsulates a set of
//! convenience methods for emitting messages with various semantic meanings
//! (e.g., warnings, errors) using cross‑platform colored output.
//!
//! The default constructor `Stderr::new()` is "smart" and automatically
//! reads its configuration from environment variables like `QUIET_MODE`
//! and `DEBUG_MODE`. This behavior can be programmatically overridden
//! using chainable builder-style methods.
//!
//! # Example
//!
//! ```
//! // In your main.rs
//! use your_crate_name::Stderr;
//!
//! fn main() -> std::io::Result<()> {
//!     // Smart constructor: reads from the environment by default.
//!     let mut log = Stderr::new();
//!     log.info("Starting up...");
//!
//!     // Override defaults programmatically using the builder pattern.
//!     let mut quiet_log = Stderr::new().quiet(true);
//!     quiet_log.warn("This will not be printed.");
//!
//!     Ok(())
//! }
//! ```

use std::fmt::Display;
use std::io::{self, IsTerminal, Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use crate::esc::boxes::{BorderStyle, BoxChars};
use crate::utils::helpers::{ repeat_char, term_width, env };
use crate::utils::flag::flag_table;
pub use crate::esc::colors::{Color as ESC};
pub use crate::esc::glyphs::{Glyph as ART};


pub use super::stderr_debug::{ DebugPrinter, LogLevel };



// --- The Main Stderr Struct ---


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
  DevLog
}

pub enum OptionFlag {
    Quiet,
    Dev,
    Debug,
    Trace,
    Silly,
}



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



/// Holds the configuration for a Stderr logger instance.
#[derive(Debug, Clone, Default)]
pub struct StderrConfig {
    pub quiet: bool,
    pub dev:   bool,
    pub debug: bool,
    pub trace: bool,
    pub silly: bool,
}

impl StderrConfig {
    /// Creates a new StderrConfig instance by reading from environment variables.
    pub fn from_env() -> Self {
      Self {
        quiet: env("QUIET_MODE").is_ok(),
        debug: env("DEBUG_MODE").is_ok(),
        dev:   env("DEV_MODE").is_ok(),
        trace: env("TRACE_MODE").is_ok(),
        silly: env("SILLY_MODE").is_ok(),
      }
    }
}



// --- The Main Stderr Struct ---

/// A configurable logger for emitting colored messages to stderr.
pub struct Stderr {
    config: StderrConfig,
    writer: StandardStream,
    width: usize,
    label: Option<String>,
}



impl Default for Stderr {
    fn default() -> Self {
        Self::new()
    }
}

impl Stderr {
    // --- Constructors and Builders ---

    /// Creates a new logger, automatically loading its configuration from
    /// the environment. This is the primary, "smart" constructor.
    pub fn new() -> Self {
      Self {
        config: StderrConfig::from_env(), // Smart default
        writer: StandardStream::stderr(ColorChoice::Auto),
        width: term_width(),
        label: Option<String>,
      }
    }

    /// Creates a new logger with a specific, user-provided configuration.
    pub fn with_config(self, config: StderrConfig) -> Self {
      Self {
        config,
        writer: StandardStream::stderr(ColorChoice::Auto),
        label: Option<String>,
        width: term_width(),
      }
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Overrides the quiet setting.
    pub fn set_quiet(mut self, quiet: bool) -> Self {
      self.config.quiet = quiet;
      self
    }

    /// Overrides the debug setting.
    pub fn set_debug(mut self, debug: bool) -> Self {
      self.config.debug = debug;
      self
    }

    /// Overrides the trace setting.
    pub fn set_trace(mut self, trace: bool) -> Self {
      self.config.trace = trace;
      self
    }

    /// Overrides the silly setting.
    pub fn set_silly(mut self, silly: bool) -> Self {
      self.config.silly = silly;
      self
    }

    /// Overrides the silly setting.
    pub fn set_dev(mut self, dev: bool) -> Self {
      self.config.dev = dev;
      self
    }


    pub fn check_flag(&self, flag: OptionFlag) -> bool {
        match flag {
            OptionFlag::Quiet => self.config.quiet,
            OptionFlag::Dev   => self.config.dev,
            OptionFlag::Debug => self.config.debug,
            OptionFlag::Trace => self.config.trace,
            OptionFlag::Silly => self.config.silly,
        }
    }
    // pub fn resolve_opts(mut self) -> Self {
    //   self
    // }

    // --- Convenience Helpers ---
    pub fn set_bg(&mut self, color: Color) -> io::Result<()> {
        self.writer.set_color(ColorSpec::new().set_bg(Some(color)))?;
        Ok(())
    }

    pub fn set_fg(&mut self, color: Color) -> io::Result<()> {
        self.writer.set_color(ColorSpec::new().set_fg(Some(color)))?;
        Ok(())
    }

    pub fn set_bold_fg(&mut self, color: Color) -> io::Result<()> {
        self.writer.set_color(ColorSpec::new().set_fg(Some(color)).set_bold(true))
    }


    pub fn set_color(&mut self, spec: &ColorSpec) -> io::Result<()> {
        if self.check_flag(OptionFlag::Quiet){ return Ok(()); }
        self.writer.set_color(spec)
    }

    /// Prints a colored message with a prefix and a newline, then resets.
    pub fn print_with_prefix(&mut self, color: Color, prefix: impl Display, msg: &str) -> io::Result<()> {
        if self.check_flag(OptionFlag::Quiet){ return Ok(()); }
        self.set_fg(color)?; // Use the helper
        let formatted_prefix = match &self.label {
            Some(label) => format!("[{}][{}]", label, prefix),
            None => format!("[{}]", prefix),
        };

        write!(&mut self.writer, "{} ", formatted_prefix)?; 
        writeln!(&mut self.writer, "{}", msg)?;
    
        self.writer.reset()
    }


    /// Prints a simple, uncolored message with a newline and resets the terminal.
    pub fn print(&mut self, msg: &str) -> io::Result<()> {
        if self.check_flag(OptionFlag::Quiet){ return Ok(()); }
        writeln!(&mut self.writer, "{}", msg)?;
        self.writer.reset()
    }

    /// Writes a piece of text to the stream WITHOUT a newline.
    /// Use this to build up a line from multiple parts.
    pub fn write(&mut self, msg: impl Display) -> io::Result<()> {
        if self.check_flag(OptionFlag::Quiet){ return Ok(()); }
        write!(&mut self.writer, "{}", msg)
    }

    /// Resets all terminal attributes (like color and boldness) to their default.
    pub fn reset(&mut self) -> io::Result<()> {
        if self.check_flag(OptionFlag::Quiet){ return Ok(()); }
        self.writer.reset()
    }

    /// Prints a newline to the stream.
    pub fn newline(&mut self) -> io::Result<()> {
        if self.check_flag(OptionFlag::Quiet){ return Ok(()); }
        writeln!(&mut self.writer)?;
        Ok(())
    }

    // --- High-Level Logging API ---



      /// Print a log message
    pub fn txt(&mut self, msg: &str) {
      let _ = self.print(msg);
    }

    pub fn fatal(&mut self, msg: &str) -> ! {
        let _ = self.error(msg);
        std::process::exit(1);
    }


    /// Print an error message (always displayed).
    pub fn error(&mut self, msg: &str) {
      let _ = self.print_with_prefix(ESC::RED, ART::Fail, msg);
    }

    /// Print a warning message (suppressed only in quiet mode).
    pub fn warn(&mut self, msg: &str) {
      let _ = self.print_with_prefix(ESC::ORANGE, ART::Delta, msg);
    }

    /// Print an informational message (suppressed only in quiet mode).
    pub fn info(&mut self, msg: &str) {
      let _ = self.print_with_prefix(ESC::BLUE, ART::Lambda, msg);
    }

    /// Print a success/okay message (suppressed only in quiet mode).
    pub fn okay(&mut self, msg: &str) {
      let _ = self.print_with_prefix(ESC::GREEN, ART::Pass, msg);
    }

    /// Print a note (suppressed only in quiet mode).
    pub fn note(&mut self, msg: &str) {
      let _ = self.print_with_prefix(ESC::BLUE, ART::Right, msg);
    }

    /// Print a debug message (only shown when debug mode is enabled).
    pub fn debug(&mut self, msg: &str) {
      if !self.config.debug { return; }
      // Use a dim white/grey for debug messages.
      let _ = self.print_with_prefix(ESC::CYAN, ART::Boto, msg);
    }

    /// Print a dev message (only shown when dev mode is enabled).
    pub fn devlog(&mut self, msg: &str) {
      if !self.config.dev { return; }
      // Use a dim white/grey for debug messages.
      let _ = self.print_with_prefix(ESC::RED2, ART::Boto, msg);
    }


    /// Print a trace message (only shown when trace mode is enabled).
    pub fn trace(&mut self, msg: &str) {
      if self.check_flag(OptionFlag::Trace){ return; }
      let _ = self.print_with_prefix(ESC::GREY, ART::Dots, msg);
    }

    /// Print a silly/magic message (only shown when silly mode is enabled).
    pub fn magic(&mut self, msg: &str) {
      if self.check_flag(OptionFlag::Silly){ return; }          
      let _ = self.print_with_prefix(ESC::PURPLE, ART::Bolt, msg);
    }

    /// Print a silly message (only shown when silly mode is enabled).
    pub fn silly(&mut self, msg: &str) {
      if self.check_flag(OptionFlag::Silly){ return; }   
      let _ = self.print_with_prefix(ESC::MAGENTA, ART::Phi, msg);
    }

    pub fn help(&mut self, help_text: &str) -> io::Result<()> {
        if self.check_flag(OptionFlag::Quiet){
            return Ok(());
        }
        // Re-use the powerful `boxed` function with a sensible default!
        self.boxed(help_text, BorderStyle::Light)
    }


    pub fn inspect(&mut self) -> DebugPrinter {
        DebugPrinter { inner: self }
    }

    pub fn log(&mut self, level: LogLevel, msg: &str) {
      let (color, symbol) = match level {
          LogLevel::Okay  => (ESC::GREEN,   ART::Pass),
          LogLevel::Warn  => (ESC::ORANGE,  ART::Delta),
          LogLevel::Error => (ESC::RED,     ART::Fail),
          LogLevel::Info  => (ESC::BLUE,    ART::Lambda),
          LogLevel::Note  => (ESC::BLUE,    ART::Lambda),
          LogLevel::Debug => (ESC::CYAN,    ART::Boto),
          LogLevel::Trace => (ESC::GREY,    ART::Dots),
          LogLevel::Magic => (ESC::PURPLE,  ART::Bolt),
          LogLevel::Silly => (ESC::MAGENTA, ART::Phi),
          LogLevel::DevLog => (ESC::MAGENTA, ART::Boto),
        };

      let _ = self.print_with_prefix(color, symbol, msg);
    }


    pub fn print_with_prefix_debug<T: std::fmt::Debug>(
        &mut self,
        color: Color,
        prefix: impl Display,
        value: &T,
    ) -> io::Result<()> {
        if self.check_flag(OptionFlag::Quiet){ return Ok(()); }

        self.set_fg(color)?;

        let formatted_prefix = match &self.label {
            Some(label) => format!("[{}][{}]", label, prefix),
            None => format!("[{}]", prefix),
        };

        writeln!(&mut self.writer, "{} {:#?}", formatted_prefix, value)?;
        self.writer.reset()
    }


    /// Pretty-print an error struct (always displayed).
    pub fn error_debug<T: Debug>(&mut self, value: &T) {
        let _ = self.print_with_prefix_debug(ESC::RED, ART::Fail, value);
    }

    /// Pretty-print a warning struct (quiet suppressible).
    pub fn warn_debug<T: Debug>(&mut self, value: &T) {
        let _ = self.print_with_prefix_debug(ESC::ORANGE, ART::Delta, value);
    }

    /// Pretty-print an info struct (quiet suppressible).
    pub fn info_debug<T: Debug>(&mut self, value: &T) {
        let _ = self.print_with_prefix_debug(ESC::BLUE, ART::Lambda, value);
    }

    /// Pretty-print an okay struct (quiet suppressible).
    pub fn okay_debug<T: Debug>(&mut self, value: &T) {
        let _ = self.print_with_prefix_debug(ESC::GREEN, ART::Pass, value);
    }

    /// Pretty-print a note struct (quiet suppressible).
    pub fn note_debug<T: Debug>(&mut self, value: &T) {
        let _ = self.print_with_prefix_debug(ESC::BLUE, ART::Right, value);
    }

    /// Pretty-print a debug struct (only shown in debug mode).
    pub fn debug_debug<T: Debug>(&mut self, value: &T) {
        if !self.config.debug { return; }
        let _ = self.print_with_prefix_debug(ESC::CYAN, ART::Boto, value);
    }

    /// Pretty-print a devlog struct (only shown in dev mode).
    pub fn devlog_debug<T: Debug>(&mut self, value: &T) {
        if !self.config.dev { return; }
        let _ = self.print_with_prefix_debug(ESC::RED2, ART::Boto, value);
    }

    /// Pretty-print a trace struct (only shown in trace mode).
    pub fn trace_debug<T: Debug>(&mut self, value: &T) {
        if !self.config.trace { return; }
        let _ = self.print_with_prefix_debug(ESC::GREY, ART::Dots, value);
    }

    /// Pretty-print a magic struct (only shown in silly mode).
    pub fn magic_debug<T: Debug>(&mut self, value: &T) {
        if !self.config.silly { return; }
        let _ = self.print_with_prefix_debug(ESC::PURPLE, ART::Bolt, value);
    }

    /// Pretty-print a silly struct (only shown in silly mode).
    pub fn silly_debug<T: Debug>(&mut self, value: &T) {
        if !self.config.silly { return; }
        let _ = self.print_with_prefix_debug(ESC::MAGENTA, ART::Phi, value);
    }


    // --- Formatting and Interactive API ---

    pub fn banner(&mut self, msg: &str, fill_char: char) -> io::Result<()> {
        if self.check_flag(OptionFlag::Quiet){ return Ok(()); }

        let msg_len = msg.chars().count() + 2; // account for one space on each side
        if msg_len >= self.width {
            writeln!(&mut self.writer, " {} ", msg)?;
            return Ok(());
        }
        let total_fill = self.width - msg_len;
        let left_fill = total_fill / 2;
        let right_fill = total_fill - left_fill;
        let left_bar = repeat_char(fill_char, left_fill);
        let right_bar = repeat_char(fill_char, right_fill);

        let mut spec = ColorSpec::new();
        spec.set_fg(Some(ESC::BLUE)).set_bold(true);

        self.writer.reset()?;
        write!(&mut self.writer, "{} ", left_bar)?;
        self.set_color(&spec)?;
        write!(&mut self.writer, "{}", msg)?;
        self.writer.reset()?;
        writeln!(&mut self.writer, " {}", right_bar)?;

        Ok(())
    }

    /// Renders a message in a box with light, single-line borders.
    pub fn box_light(&mut self, msg: &str) -> io::Result<()> {
        self.boxed(msg, BorderStyle::Light)
    }

    /// Renders a message in a box with heavy, bold borders.
    pub fn box_heavy(&mut self, msg: &str) -> io::Result<()> {
        self.boxed(msg, BorderStyle::Heavy)
    }

    /// Renders a message in a box with double-line borders.
    pub fn box_double(&mut self, msg: &str) -> io::Result<()> {
        self.boxed(msg, BorderStyle::Double)
    }

    pub fn boxed(&mut self, msg: &str, style: BorderStyle) -> io::Result<()> {
        if self.check_flag(OptionFlag::Quiet){ return Ok(()); }

        let chars = BoxChars::from_style(&style);
        let lines: Vec<&str> = msg.lines().collect();
        let content_width = lines.iter().map(|l| l.chars().count()).max().unwrap_or(0);
        let box_width = content_width + 2;

        let top_border = std::iter::repeat(chars.horizontal).take(box_width).collect::<String>();
        let bottom_border = &top_border; // It's the same

        self.set_fg(ESC::WHITE)?;
        writeln!(&mut self.writer, "{}{}{}", chars.top_left, top_border, chars.top_right)?;
        for line in &lines {
            writeln!(&mut self.writer, "{} {:<width$} {}", chars.vertical, line, chars.vertical, width = content_width)?;
        }
        writeln!(&mut self.writer, "{}{}{}", chars.bottom_left, bottom_border, chars.bottom_right)?;
        self.writer.reset()
    }

    /// This is a convenience wrapper around the `util::bitmap` function.
    pub fn print_flag_table<T>(&mut self, bitmask: T, labels: &[&str], style: BorderStyle) -> io::Result<()>
    where
        T: std::ops::Shr<usize, Output = T> + std::ops::BitAnd<T, Output = T> + From<u8> + Copy + PartialEq,
    {
        if self.check_flag(OptionFlag::Quiet){ return Ok(()); }
        let current_term_width = term_width();
        let table_string = flag_table(bitmask, labels, style, current_term_width);
        write!(&mut self.writer, "{}", table_string)?;
        self.writer.flush()
    }


    /// A simple confirmation prompt. For more options, see `confirm_builder`.
    pub fn confirm(&mut self, prompt: &str) -> io::Result<Option<bool>> {
      // The old `confirm` is now just a shortcut to the builder's default behavior.
      self.confirm_builder(prompt).ask()
    }

    /// Creates a builder for a flexible confirmation prompt.
    pub fn confirm_builder<'a>(&'a mut self, prompt: &'a str) -> ConfirmBuilder<'a> {
      ConfirmBuilder::new(self, prompt)
    }

}


/// A builder for creating a flexible confirmation prompt.
///
/// Created via `Stderr::confirm_builder()`.
pub struct ConfirmBuilder<'a> {
    stderr: &'a mut Stderr,
    prompt: &'a str,
    use_box: bool,
    style: BorderStyle,
    prompt_color: Option<Color>,
}

// Add this impl block in `src/lib/stderr.rs`

impl<'a> ConfirmBuilder<'a> {
    /// Creates a new confirmation builder.
    fn new(stderr: &'a mut Stderr, prompt: &'a str) -> Self {
        Self {
            stderr,
            prompt,
            use_box: false, // Don't use a box by default
            style: BorderStyle::default(), // Default to Light
            prompt_color: None,
        }
    }

    pub fn prompt_color(mut self, color: Color) -> Self {
        self.prompt_color = Some(color); // <-- ADD THIS NEW METHOD
        self
    }

    /// Wraps the confirmation prompt in a box.
    pub fn boxed(mut self, use_box: bool) -> Self {
        self.use_box = use_box;
        self
    }

    /// Sets the `BorderStyle` for the box if it is enabled.
    pub fn style(mut self, style: BorderStyle) -> Self {
        self.style = style;
        self
    }

    /// Asks the user for confirmation and returns the result.
    pub fn ask(self) -> io::Result<Option<bool>> {
        if self.stderr.config.quiet { return Ok(Some(true)); }
        if !io::stdin().is_terminal() {
            return Err(io::Error::new(io::ErrorKind::Unsupported, "Cannot ask for confirmation in a non-interactive terminal."));
        }

        // If boxing is enabled, draw the box first.
        if self.use_box {
            self.stderr.boxed(self.prompt, self.style)?;
        }

        loop {

          if let Some(color) = self.prompt_color {
              // If yes, use it.
              self.stderr.set_bold_fg(color)?;
          } else {
              // If no, use the default bold white.
              self.stderr.set_bold_fg(ESC::WHITE)?;
          }
          if self.use_box {
              write!(&mut self.stderr.writer, "Your choice [y/n/q] -> ")?;
          } else {
              write!(&mut self.stderr.writer, "{} [y/n/q] > ", self.prompt)?;
          }

          self.stderr.writer.reset()?;
          self.stderr.writer.flush()?;

          let mut input = String::new();
          io::stdin().read_line(&mut input)?;

          match input.trim().chars().next().unwrap_or_default() {
            'y' | 'Y' => return Ok(Some(true)),
            'n' | 'N' => return Ok(Some(false)),
            'q' | 'Q' => return Ok(None),
            _ => {
                let _ = self.stderr.warn("Invalid input. Please try again.");
            }

          }
        }

    }
}
