// examples/pretty_driver.rs

use stderr::{
    Stderr,
    BorderStyle,
    Glyph,
    STDERR_VERS,
    bitmap, // The alias for flag_table
};
use std::io::Result;

fn main() -> Result<()> {
    // Create a logger instance. The builder could also be used here.
    let mut log = Stderr::new();

    // --- Banner Demonstration ---
    log.banner("Stderr Library Showcase", '=')?;
    println!(); // Add some vertical space

    // --- "Me Macro" Demonstration ---
    log.info("This is an informational message.")?;
    log.note("This is a note, for something to pay attention to.")?;
    log.okay("This indicates a successful operation.")?;
    log.warn("This is a warning. Something might be off.")?;
    log.error("This is an error. Something went wrong.")?;
    
    // Debug, Trace, and Silly messages might not show,
    // depending on environment variables.
    log.debug("This is a debug message. (Visible with DEBUG_MODE=1)")?;
    log.trace("This is a trace message. (Visible with TRACE_MODE=1)")?;
    log.magic("This is a magic/silly message. (Visible with SILLY_MODE=1)")?;
    println!();

    // --- Boxed Message Demonstration ---
    let box_text = "This text is wrapped in a box.\nIt can span multiple lines and helps\ndraw attention to critical information.";
    log.box_light("--- Light Box (Default) ---")?;
    log.box_light(box_text)?;
    println!();

    log.box_heavy("--- Heavy Box ---")?;
    log.box_heavy(box_text)?;
    println!();

    log.box_double("--- Double-line Box ---")?;
    log.box_double(box_text)?;
    println!();

    // --- Flag Table / Bitmap Demonstration ---
    log.banner("Flag Table / Bitmap", '-')?;
    let flags: u32 = 0b1001_0000_1100_0101;
    let labels = &[
        "READ", "WRITE", "EXEC", "", "DIR", "FILE", "", "", // 0-7
        "SETUID", "SETGID", "STICKY", "", "BACKUP", "", "", "IMMUTABLE", // 8-15
    ];
    // We call the hoisted alias `bitmap` here.
    let table_string = bitmap(flags, labels, BorderStyle::Light);
    // We print the string directly, since flag_table is a pure function.
    println!("{}", table_string);
    println!();


    // --- Interactive Confirmation ---
    log.banner("Interactive Prompt", '*')?;
    let prompt_text = "This is a critical action that requires confirmation.\nAll unsaved data will be lost.";
    
    // We use the builder for a more descriptive, boxed prompt.
    // In a non-interactive environment (like CI), this will return an error or default to 'yes'.
    match log.confirm_builder(prompt_text).boxed(true).style(BorderStyle::Heavy).ask() {
        Ok(Some(true)) => log.okay("User confirmed the action.")?,
        Ok(Some(false)) => log.warn("User denied the action.")?,
        Ok(None) => log.note("User quit the prompt.")?,
        Err(e) => log.error(&format!("Could not get confirmation: {}", e))?,
    }
    println!();

    log.note(&format!("Showcase complete for stderr version {}.", STDERR_VERS))?;

    Ok(())
}
