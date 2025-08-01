
//!  in src/esc/colors.rs

//! # Terminal Color Codes
//!
//! FFunctions and types for manipulating text color in the terminal.


use termcolor::Color as TermColor;



/// A namespace for the custom color palette.
pub struct Color;

// some of my preferred colors
impl Color {
  pub const VOID: TermColor = TermColor::Ansi256(61); //dark purple
  pub const FOREST: TermColor = TermColor::Ansi256(60); //dark green
  pub const OCEAN: TermColor = TermColor::Ansi256(17); //dark blue
  pub const RED2: TermColor = TermColor::Ansi256(197);
  pub const RED: TermColor = TermColor::Ansi256(1);
  pub const BLUE: TermColor  = TermColor::Cyan;
  pub const BLUE2: TermColor = TermColor::Ansi256(39);
  pub const YELLOW: TermColor = TermColor::Ansi256(11);
  pub const YELLOW2: TermColor = TermColor::Ansi256(226);
  pub const ORANGE: TermColor = TermColor::Ansi256(214);
  pub const ORANGE2: TermColor = TermColor::Ansi256(221);
  pub const GREEN: TermColor = TermColor::Ansi256(10);
  pub const GREEN2: TermColor = TermColor::Ansi256(156);
  pub const CYAN: TermColor = TermColor::Ansi256(51);
  pub const PURPLE: TermColor = TermColor::Ansi256(213);
  pub const PURPLE2: TermColor = TermColor::Ansi256(141);
  pub const BLACK0: TermColor = TermColor::Ansi256(0);
  pub const BLACK: TermColor = TermColor::Ansi256(235);
  pub const WHITE: TermColor = TermColor::Ansi256(247);
  pub const WHITE2: TermColor = TermColor::Ansi256(15);
  pub const GREY: TermColor = TermColor::Ansi256(242);
  pub const GREY2: TermColor = TermColor::Ansi256(240);
  pub const GREY3: TermColor = TermColor::Ansi256(237);
  pub const MAGENTA: TermColor = TermColor::Ansi256(13);
  pub const MAGENTA2: TermColor = TermColor::Ansi256(198);
  pub const PINK: TermColor = TermColor::Ansi256(211);
}



// pub struct TrueColor {
//     pub r: u8,
//     pub g: u8,
//     pub b: u8,
// }

//   let mut writer = StandardStream::stdout(ColorChoice::Auto);

// This is crucial after a series of `write!` calls.
//writer.reset()?;

// Finally, print the newline.
//writeln!(&mut writer)?;
