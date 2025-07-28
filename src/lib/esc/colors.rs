
// esc/colors.rs
use termcolor::Color as TermColor;

/// A namespace for the custom color palette.
pub struct Color;

// some of my preferred colors
impl Color {
  pub const RED2: TermColor = TermColor::Ansi256(197);
  pub const RED: TermColor = TermColor::Ansi256(1);
  pub const ORANGE: TermColor = TermColor::Ansi256(214);
  pub const GREEN: TermColor = TermColor::Ansi256(10);
  pub const CYAN: TermColor = TermColor::Cyan;
  pub const PURPLE: TermColor = TermColor::Ansi256(213);
}

//   let mut writer = StandardStream::stdout(ColorChoice::Auto);

// This is crucial after a series of `write!` calls.
//writer.reset()?;
    
// Finally, print the newline.
//writeln!(&mut writer)?;


