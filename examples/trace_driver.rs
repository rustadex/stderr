// examples/trace_driver.rs
// Test driver for the new hierarchical tracing system

use stderr::{Stderr, GlyphSet, LogLevel};

#[cfg(feature = "auto-fn-names")]
use function_name::named;

fn main() {
    let mut log = Stderr::new()
        .with_label("trace-test");

    // Show the banner
    log.info("Testing Hierarchical Tracing System");
    println!();

    // Test basic tracing (should work even without trace feature)
    log.trace("Basic trace message");
    
    // Test hierarchical tracing with manual function names
    test_manual_tracing(&mut log);
    println!();

    // Test automatic function names (if feature is enabled)
    #[cfg(feature = "auto-fn-names")]
    test_automatic_tracing(&mut log);
    
    // Test trace scopes
    test_trace_scope(&mut log);
    println!();
    
    // Test context changes
    test_context_changes(&mut log);
    println!();

    // Test custom glyphs
    test_custom_glyphs(&mut log);
    println!();

    // Test labelled traces
    test_labelled_traces(&mut log);
    
    test_trace_entry_exit(&mut log);
    // Test reset functionality
    test_reset_functionality(&mut log);
    
    log.okay("All tracing tests completed!");
}

fn test_manual_tracing(log: &mut Stderr) {
    log.info("=== Manual Function Name Tracing ===");
    
    // Simulate function call hierarchy
    log.trace_fn("parse_config", "starting configuration parsing");
    log.trace_fn("parse_config", "reading config file");
    log.trace_fn("parse_config", "validating settings");
    
    // Simulate nested function call
    log.trace_fn("validate_database_url", "checking database connection");
    log.trace_fn("validate_database_url", "connection successful");
    
    // Back to original function
    log.trace_fn("parse_config", "configuration loaded successfully");
}

#[cfg(feature = "auto-fn-names")]
#[named]
fn test_automatic_tracing(log: &mut Stderr) {
    log.info("=== Automatic Function Name Tracing ===");
    
    // This should show "test_automatic_tracing" as the function name
    log.trace_auto("entering function with automatic name detection");
    log.trace_auto("performing some work");
    
    // Test nested function
    nested_function_auto(log);
    
    log.trace_auto("back in main function");
    log.trace_auto("function complete");
}

#[cfg(feature = "auto-fn-names")]
#[named]
fn nested_function_auto(log: &mut Stderr) {
    log.trace_auto("this is a nested function");
    log.trace_auto("doing nested work");
}

fn test_trace_scope(log: &mut Stderr) {
    log.info("=== Trace Scope Testing ===");
    
    {
        let mut scope = log.trace_scope("test_function");
        scope.step("performing step 1");
        scope.step("performing step 2");
        scope.step_debug("step 3 result", &vec![1, 2, 3]);
        // Scope automatically logs exit when dropped
    }
    
    log.trace_fn("main", "scope test complete");
}

fn test_context_changes(log: &mut Stderr) {
    log.info("=== Context Change Testing ===");
    
    // Test context banners
    log.set_context("@myapp.VAR.config");
    log.trace_fn("resolve_context", "context set to myapp config");
    
    // Change context - should show new banner
    log.set_context("@myapp.VAR.secrets");
    log.trace_fn("resolve_context", "context changed to secrets");
    
    // Same context - should NOT show banner again
    log.set_context("@myapp.VAR.secrets");
    log.trace_fn("resolve_context", "same context, no banner");
    
    // Clear context
    log.clear_context();
    log.trace_fn("cleanup", "context cleared");
}

fn test_custom_glyphs(log: &mut Stderr) {
    log.info("=== Custom Glyph Testing ===");
    
    // Create custom glyph set
    let custom_glyphs = GlyphSet {
        info: "📋",
        warn: "⚠️",
        error: "❌",
        okay: "✅",
        trace: "🔍",
        debug: "🐛",
        magic: "✨",
    };
    
    let mut custom_log = Stderr::new().with_glyphs(custom_glyphs);
    
    custom_log.info("Info with custom emoji glyph");
    custom_log.warn("Warning with custom emoji glyph");
    custom_log.error("Error with custom emoji glyph");
    custom_log.okay("Success with custom emoji glyph");
    custom_log.trace("Trace with custom emoji glyph");
    custom_log.debug("Debug with custom emoji glyph");
    custom_log.magic("Magic with custom emoji glyph");
    
    // Test individual glyph setting
    log.set_glyph(LogLevel::Info, "🚀");
    log.info("Info with rocket glyph");
}

fn test_labelled_traces(log: &mut Stderr) {
    log.info("=== Labelled Trace Testing ===");
    
    log.trace_add("Creating new database entry");
    log.trace_found("Located existing configuration");
    log.trace_item("Processing item #42");
    log.trace_done("Configuration validation complete");
    log.trace_sub("Removing temporary files");
}

fn test_reset_functionality(log: &mut Stderr) {
    log.info("=== Reset Functionality Testing ===");
    
    // Set up some trace state
    log.trace_fn("function_a", "first call");
    log.trace_fn("function_a", "second call - should be continuation");
    
    // Check current state
    if let Some(current_func) = log.current_trace_func() {
        log.info(&format!("Current trace function: {}", current_func));
    }
    
    // Reset the trace state
    log.reset_trace_state();
    log.info("Trace state reset");
    
    // This should start a new trace branch
    log.trace_fn("function_a", "after reset - should be new branch");
}

// Test the trace entry/exit methods
fn test_trace_entry_exit(log: &mut Stderr) {
    log.info("=== Trace Entry/Exit Testing ===");
    
    log.trace_enter("my_function");
    log.trace_fn("my_function", "doing some work");
    log.trace_exit("my_function");
    
    // Test with return value
    let result = vec!["item1", "item2"];
    log.trace_exit_with("my_function", &result);
}
