// esc/style.rs
use termcolor::{ColorSpec};

/// A namespace for custom text styles.
pub struct Style;

impl Style {
  /// Returns a ColorSpec for bold text.
  pub fn bold() -> ColorSpec {
      let mut spec = ColorSpec::new();
      spec.set_bold(true);
      spec
  }
  
  /// Returns a ColorSpec for italic text.
  pub fn italic() -> ColorSpec {
      let mut spec = ColorSpec::new();
      spec.set_italic(true);
      spec
  }

  /// Returns a ColorSpec for inverted text (background and foreground swapped).
  pub fn invert() -> ColorSpec {
      let mut spec = ColorSpec::new();
      spec.set_invert(true);
      spec
  }


}
