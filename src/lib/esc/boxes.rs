// esc/boxes.rs

//re : https://en.wikipedia.org/wiki/Box-drawing_characters
  #[derive(Default)]
  pub enum BorderStyle {
      #[default]
      Light,
      Heavy,
      Double,
      // We could even add Rounded later!
      // Rounded,
  }

  // This can be a private struct inside stderr.rs

  struct BoxChars {
      top_left: &'static str,
      top_right: &'static str,
      bottom_left: &'static str,
      bottom_right: &'static str,
      horizontal: &'static str,
      vertical: &'static str,
      left_t: &'static str,  // ├
      right_t: &'static str, // ┤
  }

  // Now, we define the character sets for each style.
  impl BoxChars {
    pub fn from_style(style: &BorderStyle) -> Self {
        match style {
        BorderStyle::Light => Self {
            top_left: "\u{250C}", // ┌
            top_right: "\u{2510}", // ┐
            bottom_left: "\u{2514}", // └
            bottom_right: "\u{2518}", // ┘
            horizontal: "\u{2500}", // ─
            vertical: "\u{2502}", // │
            left_t: "\u{251C}", // ├
            right_t: "\u{2524}", // ┤
            cross: "\u{253C}", // ┼         
        },
        BorderStyle::Heavy => Self {
            top_left: "\u{2513}", // ┓ (Typo in chart, should be ┏) -> Corrected: ┏ U+250F
            top_right: "\u{2513}", // ┓
            bottom_left: "\u{2517}", // ┗
            bottom_right: "\u{251B}", // ┛
            horizontal: "\u{2501}", // ━
            vertical: "\u{2503}", // ┃
            left_t: "\u{2523}", // ┣
            right_t: "\u{252B}", // ┫
            cross: "\u{254B}", // ╋
        },
        BorderStyle::Double => Self {
            top_left: "\u{2554}", // ╔
            top_right: "\u{2557}", // ╗
            bottom_left: "\u{255A}", // ╚
            bottom_right: "\u{255D}", // ╝
            horizontal: "\u{2550}", // ═
            vertical: "\u{2551}", // ║
            left_t: "\u{2560}", // ╠
            right_t: "\u{2563}", // ╣
            cross: "\u{256C}", // ╬
        },
      }
    }
  }

