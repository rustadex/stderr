// in src/lib/utils/flag.rs

use crate::esc::boxes::{BorderStyle, BoxChars};

/// Generates a string representation of a bitmask as a multi-row table,
/// dynamically adjusting columns to fit the terminal width.
///
/// This is a pure function that returns the formatted table as a `String`.
///
/// # Example
///
/// ```
/// // In the calling code:
/// use my_crate::utils::flag::flag_table;
/// use my_crate::term_width; // Or however you access it
///
/// let bitmask = 0b10101010;
/// let labels = &["a", "b", "c", "d", "e", "f", "g", "h"];
/// let style = BorderStyle::Double;
///
/// // 1. Get the terminal width once.
/// let width = term_width();
/// // 2. Pass it to the function.
/// let table_string = flag_table(bitmask, labels, style, width);
/// println!("{}", table_string);
/// ```
pub fn flag_table<T>(bitmask: T, labels: &[&str], style: BorderStyle, term_width: usize) -> String
where
  T: std::ops::Shr<usize, Output = T> + std::ops::BitAnd<T, Output = T> + From<u8> + Copy + PartialEq,
{
    let total_labels = labels.len();
    if total_labels == 0 { return String::new(); }

    let chars = BoxChars::from_style(&style);

    let required_width_for_one_row = 3 + (total_labels * 5) + 1;

    let labels_per_chunk = if required_width_for_one_row <= term_width {
        total_labels
    } else {
        (total_labels + 1) / 2
    };

    if labels_per_chunk == 0 { return String::new(); }

    let mut output = String::new();

    for (chunk_index, label_chunk) in labels.chunks(labels_per_chunk).enumerate() {
        let num_cols = label_chunk.len();
        let start_bit_index = chunk_index * labels_per_chunk;

        let h_four = chars.horizontal.repeat(4);
        let top_border = format!(" {}{}{}", chars.top_left, h_four, std::iter::repeat(format!("{}{}", chars.top_t, h_four)).take(num_cols - 1).collect::<String>());
        let mid_border = format!(" {}{}{}", chars.left_t, h_four, std::iter::repeat(format!("{}{}", chars.cross, h_four)).take(num_cols - 1).collect::<String>());
        let bot_border = format!(" {}{}{}", chars.bottom_left, h_four, std::iter::repeat(format!("{}{}", chars.bottom_t, h_four)).take(num_cols - 1).collect::<String>());

        let mut index_row = format!(" {}", chars.vertical);
        let mut value_row = format!(" {}", chars.vertical);
        let mut label_row = format!(" {}", chars.vertical);

        for (i_in_chunk, &label) in label_chunk.iter().enumerate() {
            let bit_index = start_bit_index + i_in_chunk;

            index_row.push_str(&format!(" {:02} {}", bit_index, chars.vertical));
            let val = if (bitmask >> bit_index) & T::from(1u8) == T::from(1u8) { 1 } else { 0 };
            value_row.push_str(&format!("  {} {}", val, chars.vertical));
            label_row.push_str(&format!(" {:>2.2} {}", label, chars.vertical));
        }

        if chunk_index > 0 {
            output.push('\n');
        }

        output.push_str(&format!("{}{}\n", top_border, chars.top_right));
        output.push_str(&format!("{}\n", index_row));
        output.push_str(&format!("{}{}\n", mid_border, chars.right_t));
        output.push_str(&format!("{}\n", value_row));
        output.push_str(&format!("{}\n", label_row));
        output.push_str(&format!("{}{}\n", bot_border, chars.bottom_right));
    }

    output
}
