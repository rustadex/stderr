// src/stderr_lib.rs

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

use std::io::{self, IsTerminal, Write, Error};
use terminal_size::terminal_size;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use crate::esc::boxes::{BorderStyle, BoxChars};
use crate::utils::flag::flag_table as bitmap;


// --- Standalone Utility Functions (As you wanted) ---

  /// Creates a string by repeating a character `n` times.
  pub fn repeat_char(ch: char, n: usize) -> String {
    std::iter::repeat(ch).take(n).collect()
  }

  /// Gets the terminal width from the environment or a default.
  pub fn term_width() -> usize {
      terminal_size()
        .map(|(w, _h)| w.0 as usize)
        .unwrap_or(80)
  }

  /// Reads an environment variable, returning a Result.
  pub fn env(key: &str) -> Result<String, std::env::VarError> {
    std::env::var(key)
  }

  #[allow(dead_code)] 
  pub fn readline() -> io::Result<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?; // Now this works!
    Ok(input) // If successful, wrap the result in `Ok`
  }

// --- The Main Stderr Struct ---

// --- Configuration Struct (As you liked) ---





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
        dev: env("DEV_MODE").is_ok(),
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
        config: StderrConfig::from_env(), // Smart default!
        writer: StandardStream::stderr(ColorChoice::Auto),
        width: term_width(),
      }
    }

    /// Creates a new logger with a specific, user-provided configuration.
    pub fn with_config(config: StderrConfig) -> Self {
      Self {
        config,
        writer: StandardStream::stderr(ColorChoice::Auto),
        width: term_width(),
      }
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

    // pub fn resolve_opts(mut self) -> Self {
    //   self
    // }
    
    // --- Private Helpers ---
    pub fn set_fg(&mut self, color: Color) -> io::Result<()> {
      self.writer.set_color(ColorSpec::new().set_fg(Some(color)))?;
      Ok(())
    }

    pub fn set_bold_fg(&mut self, color: Color) -> io::Result<()> { 
        self.writer.set_color(ColorSpec::new().set_fg(Some(color)).set_bold(true)) 
    }


    fn set_color_spec(&mut self, spec: &ColorSpec) -> io::Result<()> {
      self.writer.set_color(spec)
    }

    fn print_with_color(&mut self, color: Color, prefix: &str, msg: &str) -> Result<(), Error> {
        if self.config.quiet { return Ok(()); }

        self.set_fg(color)?; // Use the helper
        write!(&mut self.writer, "{}", prefix)?;
        self.writer.reset()?;
        writeln!(&mut self.writer, "{}", msg)?;
        Ok(())
    }

    // --- High-Level Logging API ---

		/// Print an error message (always displayed).
		pub fn error(&mut self, msg: &str) {
				let _ = self.print_with_color(Color::Red, "✖ ", msg);
		}

		/// Print a warning message (suppressed only in quiet mode).
		pub fn warn(&mut self, msg: &str) {
				let _ = self.print_with_color(Color::Yellow, "⚠ ", msg);
		}

		/// Print an informational message (suppressed only in quiet mode).
		pub fn info(&mut self, msg: &str) {
				let _ = self.print_with_color(Color::Blue, "ℹ ", msg);
		}

		/// Print a success/okay message (suppressed only in quiet mode).
		pub fn okay(&mut self, msg: &str) {
				let _ = self.print_with_color(Color::Green, "✔ ", msg);
		}

		/// Print a note (suppressed only in quiet mode).
		pub fn note(&mut self, msg: &str) {
				let _ = self.print_with_color(Color::Magenta, "➜ ", msg);
		}

  		/// Print a debug message (only shown when debug mode is enabled).
		pub fn debug(&mut self, msg: &str) {
				if !self.config.debug { return; }
				// Use a dim white/grey for debug messages.
				let _ = self.print_with_color(Color::White, "· ", msg);
		}

		/// Print a trace message (only shown when trace mode is enabled).
		pub fn trace(&mut self, msg: &str) {
				if !self.config.trace { return; }
				let _ = self.print_with_color(Color::Cyan, "→ ", msg);
		}

		/// Print a silly/magic message (only shown when silly mode is enabled).
		pub fn magic(&mut self, msg: &str) {
				if !self.config.silly { return; }
				let _ = self.print_with_color(Color::Magenta, "✨ ", msg);
		}


    pub fn help(&mut self, help_text: &str) -> io::Result<()> {
        if self.config.quiet {
            return Ok(());
        }
        // Re-use the powerful `boxed` function with a sensible default!
        self.boxed(help_text, BorderStyle::Light)
    }
    
    // --- Formatting and Interactive API ---
    
    pub fn banner(&mut self, msg: &str, fill_char: char) -> io::Result<()> {
        if self.config.quiet { return Ok(()); }
        
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
        spec.set_fg(Some(Color::Blue)).set_bold(true);

        self.writer.reset()?;
        write!(&mut self.writer, "{} ", left_bar)?;
        self.set_color_spec(&spec)?;
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
        if self.config.quiet { return Ok(()); }

        let chars = BoxChars::from_style(&style);
        let lines: Vec<&str> = msg.lines().collect();
        let content_width = lines.iter().map(|l| l.chars().count()).max().unwrap_or(0);
        let box_width = content_width + 2;

        let top_border = std::iter::repeat(chars.horizontal).take(box_width).collect::<String>();
        let bottom_border = &top_border; // It's the same

        self.set_fg(Color::White)?;
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
        if self.config.quiet { return Ok(()); }
        
        // 1. Generate the table string using the pure utility function.
        let table_string = bitmap(bitmask, labels, style);

        // 2. Print the generated string.
        write!(&mut self.writer, "{}", table_string)?;
        self.writer.flush()
    }


    /// A simple confirmation prompt. For more options, see `confirm_builder`.
    pub fn confirm(&mut self, prompt: &str) -> io::Result<Option<bool>> {
      // The old `confirm` is now just a shortcut to the builder's default behavior.
      self.confirm_builder(prompt).ask()
    }

    /// Creates a builder for a flexible confirmation prompt.
    ///
    /// This allows for chaining options like boxing the prompt.
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
        }
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
            // Always print the final question part, even if boxed.
            self.stderr.set_bold_fg(Color::White)?;
            if self.use_box {
                write!(&mut self.stderr.writer, "Your choice [y/n/q] > ")?;
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
