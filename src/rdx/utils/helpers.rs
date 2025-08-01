//! src/lib/helpers.rs

  use std::io::{self};
  use terminal_size::terminal_size;


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

