//! src/lib/utils/grid.rs

use std::io;
use termcolor::ColorSpec; // Ensure WriteColor is in scope for the methods
use crate::stderr::Stderr; // Adjust the path to your Stderr struct

pub fn print_color_grid(logger: &mut Stderr, cols: usize) -> io::Result<()> {
    for i in 0..256 {

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
            if i == 0 || i == 8 {
                termcolor::Color::White
            } else {
                termcolor::Color::Black
            }
        } else {
            // Grayscale ramp
            if i > 243 {
                termcolor::Color::Black
            } else {
                termcolor::Color::White
            }
        };

        // Create the full color specification for this cell
        let mut spec = ColorSpec::new();
        spec.set_bg(Some(termcolor::Color::Ansi256(i as u8)));
        spec.set_fg(Some(fg_color));

        logger.set_color(&spec)?;
        logger.write(&format!(" {:<3} .", i))?;
        logger.reset()?;

        if (i + 1) % cols == 0 {
            logger.newline()?;
        } else {
            logger.write("")?;
        }
    }

    // Ensure we always end with a final newline for clean terminal output
    logger.newline()?;

    Ok(())
}
