//! Interactive features for stderr - prompts, confirmations, user input

use std::io::{self, IsTerminal, Write};
use termcolor::Color;
use super::core::{Stderr, OptionFlag};
use crate::esc::boxes::{BorderStyle, BoxChars};
use crate::esc::colors::Color as ESC;

#[cfg(feature = "interactive")]
impl Stderr {
    /// Simple confirmation prompt. For more options, see `confirm_builder`.
    pub fn confirm(&mut self, prompt: &str) -> io::Result<Option<bool>> {
        // The old `confirm` is now just a shortcut to the builder's default behavior.
        self.confirm_builder(prompt).ask()
    }

    /// Creates a builder for a flexible confirmation prompt.
    pub fn confirm_builder<'a>(&'a mut self, prompt: &'a str) -> ConfirmBuilder<'a> {
        ConfirmBuilder::new(self, prompt)
    }

    /// Display help text in a formatted box
    pub fn help(&mut self, help_text: &str) -> io::Result<()> {
        if self.check_flag(OptionFlag::Quiet) {
            return Ok(());
        }
        // Re-use the powerful `boxed` function with a sensible default!
        self.boxed(help_text, BorderStyle::Light)
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
        self.prompt_color = Some(color);
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

/// Trait for adding interactive extensions (if needed for modular design)
pub trait InteractiveExt {
    fn confirm(&mut self, prompt: &str) -> io::Result<Option<bool>>;
    fn confirm_builder<'a>(&'a mut self, prompt: &'a str) -> ConfirmBuilder<'a>;
    fn help(&mut self, help_text: &str) -> io::Result<()>;
}

#[cfg(feature = "interactive")]
impl InteractiveExt for Stderr {
    fn confirm(&mut self, prompt: &str) -> io::Result<Option<bool>> {
        self.confirm(prompt)
    }

    fn confirm_builder<'a>(&'a mut self, prompt: &'a str) -> ConfirmBuilder<'a> {
        self.confirm_builder(prompt)
    }

    fn help(&mut self, help_text: &str) -> io::Result<()> {
        self.help(help_text)
    }
}
