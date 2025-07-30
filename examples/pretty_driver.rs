// examples/pretty_driver.rs

use stderr::{
    Stderr,
    Color as ESC,
    BorderStyle,
    VERSION,
    flag_table,
    term_width
};
use std::io::Result; // Keep this for main's return type

fn main() -> Result<()> {
    let mut log = Stderr::new();

    // --- Banner Demonstration ---
    log.banner("Stderr Library Showcase", '=')?; // banner() returns Result, so `?` is correct
    println!();

    // --- "Me Macro" Demonstration ---
    // These methods do NOT return a Result in your final code, so they have NO `?` or `.unwrap()`.
    log.info("This is an informational message.");
    log.note("This is a note, for something to pay attention to.");
    log.okay("This indicates a successful operation.");
    log.warn("This is a warning. Something might be off.");
    log.error("This is an error. Something went wrong.");

    log.debug("This is a debug message. (Visible with DEBUG_MODE=1)");
    log.trace("This is a trace message. (Visible with TRACE_MODE=1)");
    log.magic("This is a magic/silly message. (Visible with SILLY_MODE=1)");
    println!();

    // --- Boxed Message Demonstration ---
    let box_text = "This text is wrapped in a box.\nIt can span multiple lines and helps\ndraw attention to critical information.";
    // The `box_*` methods return Result, so `?` is correct here.
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
        "READ", "WRITE", "EXEC", "", "DIR", "FILE", "", "",
        "SETUID", "SETGID", "STICKY", "", "BACKUP", "", "", "IMMUTABLE",
    ];

    let current_term_width = term_width();
    let table_string = flag_table(flags, labels, BorderStyle::Light, current_term_width);
    println!("{}", table_string);
    println!();

    // --- Interactive Confirmation ---
    log.banner("Interactive Prompt", '*')?;
    let prompt_text = "This is a critical action that requires confirmation.\nAll unsaved data will be lost.";

    // `confirm_builder().ask()` returns a Result, so we use `match`.
    match log.confirm_builder(prompt_text).boxed(true).style(BorderStyle::Heavy).prompt_color(ESC::ORANGE).ask() {
        Ok(Some(true)) => log.okay("User confirmed the action."),
        Ok(Some(false)) => log.warn("User denied the action."),
        Ok(None) => log.note("User quit the prompt."),
        Err(e) => log.error(&format!("Could not get confirmation: {}", e)),
    }
    println!();

    log.info(&format!("Showcase complete for stderr version {}.", VERSION));

    Ok(())
}
