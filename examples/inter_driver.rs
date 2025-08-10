// examples/interactive_test_driver.rs

use stderr::{Stderr, BorderStyle, Color as ESC};
use std::io::Result;

fn main() -> Result<()> {
    let mut log = Stderr::new();
    
    println!("=== Interactive Features Test ===\n");
    
    // Test 1: Simple confirmation
    log.info("Test 1: Simple Confirmation");
    match log.confirm("Do you want to continue with the test?")? {
        Some(true) => log.okay("User confirmed!"),
        Some(false) => log.warn("User declined"),
        None => log.info("User quit"),
    }
    
    println!();
    
    // Test 2: Boxed confirmation
    log.info("Test 2: Boxed Confirmation");
    let boxed_prompt = "This is a critical operation.\nAll data will be permanently deleted.\nAre you absolutely sure?";
    
    match log.confirm_builder(boxed_prompt)
        .boxed(true)
        .style(BorderStyle::Heavy)
        .ask()? {
        Some(true) => log.okay("Critical operation confirmed"),
        Some(false) => log.warn("Critical operation cancelled"),  
        None => log.info("User quit critical operation"),
    }
    
    println!();
    
    // Test 3: Colored confirmation
    log.info("Test 3: Colored Confirmation");
    match log.confirm_builder("Enable debug mode?")
        .prompt_color(ESC::CYAN)
        .ask()? {
        Some(true) => log.okay("Debug mode enabled"),
        Some(false) => log.warn("Debug mode disabled"),
        None => log.info("Debug mode unchanged"),
    }
    
    println!();
    
    // Test 4: Different border styles
    log.info("Test 4: Border Style Variations");
    
    // Light border
    match log.confirm_builder("Use light border style?")
        .boxed(true)
        .style(BorderStyle::Light)
        .ask()? {
        Some(true) => log.okay("Light border confirmed"),
        _ => log.info("Light border skipped"),
    }
    
    // Double border  
    match log.confirm_builder("Use double border style?")
        .boxed(true)
        .style(BorderStyle::Double)
        .prompt_color(ESC::YELLOW)
        .ask()? {
        Some(true) => log.okay("Double border confirmed"),
        _ => log.info("Double border skipped"),
    }
    
    println!();
    
    // Test 5: Help system
    log.info("Test 5: Help Display");
    let help_text = r#"BookDB Help System

USAGE:
    bookdb <command> [args...] [flags...] [<context>]

CORE COMMANDS:
    getv <KEY>          Get variable value  
    setv <KEY=VALUE>    Set variable value
    ls [type]           List items
    cursor              Show current context
    
EXAMPLES:
    bookdb getv API_KEY
    bookdb setv PORT=3000 @myapp.VAR.config  
    bookdb ls keys
    
For more information, see the documentation."#;
    
    log.help(help_text)?;
    
    println!();
    
    // Test 6: Error simulation with confirmation
    log.info("Test 6: Error Recovery Confirmation");
    log.error("Simulated error: Database connection failed");
    
    match log.confirm_builder("Would you like to retry the connection?")
        .prompt_color(ESC::RED)
        .ask()? {
        Some(true) => {
            log.info("Retrying connection...");
            log.okay("Connection restored!");
        },
        Some(false) => log.warn("Connection retry cancelled"),
        None => log.info("User quit retry prompt"),
    }
    
    println!();
    
    // Test 7: Multi-step workflow
    log.info("Test 7: Multi-step Workflow");
    log.banner("Database Setup Wizard", '=')?;
    
    let steps = [
        ("Create database", "Create a new database instance?"),
        ("Configure users", "Set up user accounts and permissions?"),  
        ("Initialize data", "Load initial data and schema?"),
        ("Start service", "Start the database service?"),
    ];
    
    for (step_name, prompt) in &steps {
        match log.confirm_builder(prompt)
            .prompt_color(ESC::BLUE)
            .ask()? {
            Some(true) => log.okay(&format!("✓ {}", step_name)),
            Some(false) => {
                log.warn(&format!("✗ {} skipped", step_name));
                break;
            },
            None => {
                log.info("Setup cancelled");
                break;
            }
        }
    }
    
    log.banner("Setup Complete", '=')?;
    
    println!("\n=== Interactive Test Complete ===");
    println!("Note: In automated environments, confirmations return true by default");
    
    Ok(())
}
