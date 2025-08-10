// examples/context_test_driver.rs

use stderr::Stderr;

fn main() {
    let mut log = Stderr::new();
    
    println!("=== Context System Test ===\n");
    
    // Test initial state (no context)
    log.info("Starting with no context set");
    
    println!("\n--- Setting First Context ---");
    
    // Set first context - should show banner
    log.set_context("@myapp.VAR.config");
    log.info("Context set to myapp config");
    log.okay("This should appear under the context banner");
    
    println!("\n--- Same Context (No Banner) ---");
    
    // Same context - should NOT show banner
    log.set_context("@myapp.VAR.config");
    log.info("Same context - no banner should appear");
    
    println!("\n--- Different Context ---");
    
    // Different context - should show new banner
    log.set_context("@work.VAR.secrets");
    log.warn("Changed to work secrets context");
    log.info("This is under the new context");
    
    println!("\n--- Complex Context Chain ---");
    
    // Test longer context chain
    log.set_context("@production.database.VAR.credentials");
    log.error("Complex context chain test");
    
    println!("\n--- Ephemeral Context Testing ---");
    
    // Test ephemeral context (should not change persistent cursor)
    log.info("About to test ephemeral context");
    
    // Simulate ephemeral usage (manual for now since % parsing not in basic stderr)
    log.info("Simulating ephemeral context - cursor should remain unchanged");
    
    println!("\n--- Context Clearing ---");
    
    // Clear context
    log.clear_context();
    log.info("Context cleared - back to neutral");
    
    println!("\n--- Rapid Context Changes ---");
    
    // Test rapid context switching
    let contexts = [
        "@app1.VAR.config",
        "@app2.VAR.secrets", 
        "@app3.VAR.settings",
        "@app1.VAR.config",  // Back to first - should show banner again
    ];
    
    for context in &contexts {
        log.set_context(context);
        log.info(&format!("Switched to {}", context));
    }
    
    println!("\n=== Context Test Complete ===");
}
