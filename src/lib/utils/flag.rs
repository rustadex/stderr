// src/lib/utils/flag.rs

//! A utility for generating a string representation of a bitmask table.

use crate::esc::boxes::{BorderStyle, BoxChars};

/// Generates a string representation of a bitmask as a multi-row table.
///
/// This is a pure function that returns the formatted table as a `String`.
///
/// # Arguments
///
/// * `bitmask`: The integer whose bits will be displayed.
/// * `labels`: A slice of string slices corresponding to the bits.
/// * `style`: The `BorderStyle` to use for the table.
pub fn flag_table<T>(bitmask: T, labels: &[&str], style: BorderStyle) -> String
where
  T: std::ops::Shr<usize, Output = T> + std::ops::BitAnd<T, Output = T> + From<u8> + Copy + PartialEq,
{
    let chars = BoxChars::from_style(&style);
    let bitcount = labels.len();
    if bitcount == 0 { return String::new(); }

    let rows = (bitcount + 15) / 16;
    let mut output = String::new();

    for j in (0..rows).rev() {
        let start = j * 16;
        let end = (start + 15).min(bitcount - 1);
        let num_cols = end - start + 1;

        // --- Build Border Strings ---
        let h_four = chars.horizontal.repeat(4);
        let top_border = format!("# {}{}{}", chars.top_left, h_four, std::iter::repeat(format!("{}{}", chars.top_t, h_four)).take(num_cols - 1).collect::<String>());
        let mid_border = format!("# {}{}{}", chars.left_t, h_four, std::iter::repeat(format!("{}{}", chars.cross, h_four)).take(num_cols - 1).collect::<String>());
        let bot_border = format!("# {}{}{}", chars.bottom_left, h_four, std::iter::repeat(format!("{}{}", chars.bottom_t, h_four)).take(num_cols - 1).collect::<String>());

        // --- Build Content Strings ---
        let mut index_row = format!("# {}", chars.vertical);
        let mut value_row = format!("# {}", chars.vertical);
        let mut label_row = format!("# {}", chars.vertical);

        for i in (start..=end).rev() {
            index_row.push_str(&format!(" {:02} {}", i, chars.vertical));
            let val = if (bitmask >> i) & T::from(1u8) == T::from(1u8) { 1 } else { 0 };
            value_row.push_str(&format!("  {} {}", val, chars.vertical));
            let label = labels.get(i).unwrap_or(&"");
            label_row.push_str(&format!(" {:>2.2} {}", label, chars.vertical));
        }
        
        // --- Append Assembled Rows to the output String ---
        output.push_str(&format!("{}{}\n", top_border, chars.top_right));
        output.push_str(&format!("{}\n", index_row));
        output.push_str(&format!("{}{}\n", mid_border, chars.right_t));
        output.push_str(&format!("{}\n", value_row));
        output.push_str(&format!("{}\n", label_row));
        output.push_str(&format!("{}{}\n", bot_border, chars.bottom_right));
    }

    output
}
