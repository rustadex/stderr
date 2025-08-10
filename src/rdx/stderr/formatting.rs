//! Formatting features for stderr - tables, boxes, banners, advanced layouts

use std::io;
use termcolor::ColorSpec;
use super::core::{Stderr, OptionFlag};
use crate::esc::boxes::{BorderStyle, BoxChars};
use crate::esc::colors::Color as ESC;
use crate::utils::helpers::{repeat_char, term_width};
use crate::utils::flag::flag_table;

/// Trait for types that can be displayed as table rows
pub trait TableRow {
    fn columns(&self) -> Vec<String>;
}

impl TableRow for Vec<String> {
    fn columns(&self) -> Vec<String> {
        self.clone()
    }
}

impl TableRow for &[&str] {
    fn columns(&self) -> Vec<String> {
        self.iter().map(|s| s.to_string()).collect()
    }
}

#[cfg(feature = "formatting")]
impl Stderr {
    /// Creates a banner with the specified fill character
    pub fn banner(&mut self, msg: &str, fill_char: char) -> io::Result<()> {
        if self.check_flag(OptionFlag::Quiet) { return Ok(()); }

        let msg_len = msg.chars().count() + 2; // account for one space on each side
        if msg_len >= self.width {
            writeln!(&mut self.writer, " {} ", msg)?;
            return Ok(());
        }
        let total_fill = self.width - msg_len;
        let left_fill = total_fill / 2;
        let right_fill = total_fill - left_fill;
        let left_bar = repeat_char(fill_char, left_fill);
        let right_bar = repeat_char(fill_char, right_fill);

        let mut spec = ColorSpec::new();
        spec.set_fg(Some(ESC::BLUE)).set_bold(true);

        self.writer.reset()?;
        write!(&mut self.writer, "{} ", left_bar)?;
        self.set_color(&spec)?;
        write!(&mut self.writer, "{}", msg)?;
        self.writer.reset()?;
        writeln!(&mut self.writer, " {}", right_bar)?;

        Ok(())
    }

    /// Renders a message in a box with the specified border style
    pub fn boxed(&mut self, msg: &str, style: BorderStyle) -> io::Result<()> {
        if self.check_flag(OptionFlag::Quiet) { return Ok(()); }

        let chars = BoxChars::from_style(&style);
        let lines: Vec<&str> = msg.lines().collect();
        let content_width = lines.iter().map(|l| l.chars().count()).max().unwrap_or(0);
        let box_width = content_width + 2;

        let top_border = std::iter::repeat(chars.horizontal).take(box_width).collect::<String>();
        let bottom_border = &top_border; // It's the same

        self.set_fg(ESC::WHITE)?;
        writeln!(&mut self.writer, "{}{}{}", chars.top_left, top_border, chars.top_right)?;
        for line in &lines {
            writeln!(&mut self.writer, "{} {:<width$} {}", chars.vertical, line, chars.vertical, width = content_width)?;
        }
        writeln!(&mut self.writer, "{}{}{}", chars.bottom_left, bottom_border, chars.bottom_right)?;
        self.writer.reset()
    }

    /// Renders a message in a box with light, single-line borders.
    pub fn box_light(&mut self, msg: &str) -> io::Result<()> {
        self.boxed(msg, BorderStyle::Light)
    }

    /// Renders a message in a box with heavy, bold borders.
    pub fn box_heavy(&mut self, msg: &str) -> io::Result<()> {
        self.boxed(msg, BorderStyle::Heavy)
    }

    /// Renders a message in a box with double-line borders.
    pub fn box_double(&mut self, msg: &str) -> io::Result<()> {
        self.boxed(msg, BorderStyle::Double)
    }

    /// Simple table formatter for basic data display
    /// Useful for BookDB's ls commands
    pub fn simple_table(&mut self, rows: &[&[&str]]) -> io::Result<()> {
        if self.check_flag(OptionFlag::Quiet) { return Ok(()); }
        if rows.is_empty() { return Ok(()); }

        // Calculate column widths
        let num_cols = rows[0].len();
        let mut col_widths = vec![0; num_cols];
        
        for row in rows {
            for (i, cell) in row.iter().enumerate() {
                if i < col_widths.len() {
                    col_widths[i] = col_widths[i].max(cell.chars().count());
                }
            }
        }

        // Print rows
        for (row_idx, row) in rows.iter().enumerate() {
            let mut line = String::new();
            for (col_idx, cell) in row.iter().enumerate() {
                if col_idx < col_widths.len() {
                    line.push_str(&format!("{:<width$}", cell, width = col_widths[col_idx]));
                    if col_idx < row.len() - 1 {
                        line.push_str("  "); // Column separator
                    }
                }
            }
            
            // Highlight header row
            if row_idx == 0 {
                self.set_bold_fg(ESC::BLUE)?;
                writeln!(&mut self.writer, "{}", line)?;
                self.reset()?;
                
                // Add separator line under header
                let separator: String = col_widths.iter()
                    .map(|&w| "-".repeat(w))
                    .collect::<Vec<_>>()
                    .join("  ");
                self.set_fg(ESC::GREY)?;
                writeln!(&mut self.writer, "{}", separator)?;
                self.reset()?;
            } else {
                writeln!(&mut self.writer, "{}", line)?;
            }
        }
        
        Ok(())
    }

    /// Advanced table formatter with custom row types
    pub fn table<T: TableRow>(&mut self, headers: &[&str], rows: &[T]) -> io::Result<()> {
        if self.check_flag(OptionFlag::Quiet) { return Ok(()); }
        
        // Convert to simple format and use simple_table
        let mut all_rows = vec![headers];
        let string_rows: Vec<Vec<String>> = rows.iter().map(|r| r.columns()).collect();
        let str_rows: Vec<Vec<&str>> = string_rows.iter()
            .map(|r| r.iter().map(|s| s.as_str()).collect())
            .collect();
        
        for row in &str_rows {
            all_rows.push(row);
        }
        
        // Convert to &[&str] format
        let table_data: Vec<&[&str]> = all_rows.iter().map(|r| r.as_slice()).collect();
        self.simple_table(&table_data)
    }

    /// This is a convenience wrapper around the `util::flag_table` function.
    pub fn print_flag_table<T>(&mut self, bitmask: T, labels: &[&str], style: BorderStyle) -> io::Result<()>
    where
        T: std::ops::Shr<usize, Output = T> + std::ops::BitAnd<T, Output = T> + From<u8> + Copy + PartialEq,
    {
        if self.check_flag(OptionFlag::Quiet) { return Ok(()); }
        let current_term_width = term_width();
        let table_string = flag_table(bitmask, labels, style, current_term_width);
        write!(&mut self.writer, "{}", table_string)?;
        self.writer.flush()
    }

    /// Print a list with bullet points (useful for BookDB lists)
    pub fn list(&mut self, items: &[&str], bullet: &str) -> io::Result<()> {
        if self.check_flag(OptionFlag::Quiet) { return Ok(()); }
        
        for item in items {
            writeln!(&mut self.writer, "{} {}", bullet, item)?;
        }
        Ok(())
    }

    /// Print a numbered list
    pub fn numbered_list(&mut self, items: &[&str]) -> io::Result<()> {
        if self.check_flag(OptionFlag::Quiet) { return Ok(()); }
        
        for (i, item) in items.iter().enumerate() {
            writeln!(&mut self.writer, "{}. {}", i + 1, item)?;
        }
        Ok(())
    }

    /// Print items in columns (useful for BookDB's ls output)
    pub fn columns(&mut self, items: &[&str], num_cols: usize) -> io::Result<()> {
        if self.check_flag(OptionFlag::Quiet) { return Ok(()); }
        if items.is_empty() { return Ok(()); }
        
        // Calculate column width
        let max_width = items.iter().map(|s| s.chars().count()).max().unwrap_or(0);
        let col_width = max_width + 2; // Add padding
        
        for chunk in items.chunks(num_cols) {
            let mut line = String::new();
            for item in chunk {
                line.push_str(&format!("{:<width$}", item, width = col_width));
            }
            writeln!(&mut self.writer, "{}", line.trim_end())?;
        }
        
        Ok(())
    }
}

/// Trait for adding formatting extensions (if needed for modular design)
pub trait FormattingExt {
    fn banner(&mut self, msg: &str, fill_char: char) -> io::Result<()>;
    fn boxed(&mut self, msg: &str, style: BorderStyle) -> io::Result<()>;
    fn simple_table(&mut self, rows: &[&[&str]]) -> io::Result<()>;
    fn table<T: TableRow>(&mut self, headers: &[&str], rows: &[T]) -> io::Result<()>;
    fn list(&mut self, items: &[&str], bullet: &str) -> io::Result<()>;
    fn columns(&mut self, items: &[&str], num_cols: usize) -> io::Result<()>;
}

#[cfg(feature = "formatting")]
impl FormattingExt for Stderr {
    fn banner(&mut self, msg: &str, fill_char: char) -> io::Result<()> {
        self.banner(msg, fill_char)
    }

    fn boxed(&mut self, msg: &str, style: BorderStyle) -> io::Result<()> {
        self.boxed(msg, style)
    }

    fn simple_table(&mut self, rows: &[&[&str]]) -> io::Result<()> {
        self.simple_table(rows)
    }

    fn table<T: TableRow>(&mut self, headers: &[&str], rows: &[T]) -> io::Result<()> {
        self.table(headers, rows)
    }

    fn list(&mut self, items: &[&str], bullet: &str) -> io::Result<()> {
        self.list(items, bullet)
    }

    fn columns(&mut self, items: &[&str], num_cols: usize) -> io::Result<()> {
        self.columns(items, num_cols)
    }
}
