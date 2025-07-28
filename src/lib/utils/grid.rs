use crate::stderr_impl::Stderr; // We need this to get a writer
use termcolor::{self, ColorSpec, WriteColor};
use std::io::{self, Write};


pub fn print_color_grid(logger: &mut Stderr, cols: usize) -> io::Result<()> {
    let writer = &mut logger.writer; // Get a mutable reference to the writer

    for i in 0..256 {
        // Create a color spec for the background
        let mut bg_spec = ColorSpec::new();
        bg_spec.set_bg(Some(termcolor::Color::Ansi256(i as u8)));
        
        // Determine the best foreground color (black or white) for contrast
        let fg_color = if (16..232).contains(&i) {
            // For the main color cube, simple brightness check
            let r = ((i - 16) / 36) * 51;
            let g = (((i - 16) % 36) / 6) * 51;
            let b = ((i - 16) % 6) * 51;
            if (r + g + b) > 382 { // Heuristic for brightness
                termcolor::Color::Black
            } else {
                termcolor::Color::White
            }
        } else if i < 16 {
            // For the first 16 colors, it's a mix
             if i == 0 || i == 8 { termcolor::Color::White } else { termcolor::Color::Black }
        } else {
            // Grayscale ramp
            if i > 243 { termcolor::Color::Black } else { termcolor::Color::White }
        };

        let mut spec = ColorSpec::new();
        spec.set_bg(Some(termcolor::Color::Ansi256(i as u8)));
        spec.set_fg(Some(fg_color));

        writer.set_color(&spec)?;
        // Print the color number, padded to fit nicely.
        write!(writer, " {:<3} ", i)?;

        // Reset and add a space or a newline
        writer.reset()?;
        if (i + 1) % cols == 0 {
            writeln!(writer)?;
        } else {
            write!(writer, " ")?;
        }
    }
    writeln!(writer)?; // Ensure we end with a newline
    Ok(())
}
