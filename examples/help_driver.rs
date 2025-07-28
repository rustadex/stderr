// examples/show_help.rs

// Import the items from your library
use stderr::{help_string, Stderr};

fn main() -> std::io::Result<()> {
    // Create a logger instance
    // Personally I prefer to call it 'stderr' call it log or whatever you want
    let mut logger = Stderr::new(); 

    // Get the help text from your new function
    let help_text = help_string();

    // Use the logger's own `help` method to print it beautifully
    logger.help(&help_text)?;

    Ok(())
}
