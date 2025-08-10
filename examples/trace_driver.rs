// examples/trace_test_driver.rs

use stderr::Stderr;

// Only import auto function names if feature is available
#[cfg(feature = "auto-fn-names")]
use function_name::named;

fn main() {
    let mut log = Stderr::new()
        .with_label("trace-test");
    
    println!("=== Hierarchical Tracing Test ===\n");
    
    // Note: Trace messages only show if TRACE_MODE=0 is set
    log.info("Set TRACE_MODE=0 to see trace messages");
    log.info("Starting trace system test");
    
    println!("\n--- Manual Function Tracing ---");
    
    // Test manual function name tracing
    test_manual_tracing(&mut log);
    
    println!("\n--- Automatic Function Tracing ---");
    
    // Test automatic function names (if feature enabled)
    #[cfg(feature = "auto-fn-names")]
    test_automatic_tracing(&mut log);
    
    #[cfg(not(feature = "auto-fn-names"))]
    {
        log.warn("auto-fn-names feature not enabled");
        log.info("Run with: cargo run --example trace_test_driver --features auto-fn-names");
    }
    
    println!("\n--- Trace Scopes ---");
    
    // Test trace scopes
    test_trace_scopes(&mut log);
    
    println!("\n--- Labelled Traces ---");
    
    // Test labelled trace helpers
    test_labelled_traces(&mut log);
    
    println!("\n--- Context + Tracing ---");
    
    // Test context with tracing
    test_context_tracing(&mut log);
    
    println!("\n=== Trace Test Complete ===");
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

fn test_trace_scopes(log: &mut Stderr) {
    log.info("=== Trace Scope Testing ===");
    
    {
        let mut scope = log.trace_scope("test_function");
        scope.step("performing step 1");
        scope.step("performing step 2");
        
        let test_data = vec![1, 2, 3];
        scope.step_debug("step 3 result", &test_data);
        // Scope automatically logs exit when dropped
    }
    
    log.trace_fn("main", "scope test complete");
}

fn test_labelled_traces(log: &mut Stderr) {
    log.info("=== Labelled Trace Testing ===");
    
    log.trace_add("Creating new database entry");
    log.trace_found("Located existing configuration");
    log.trace_item("Processing item #42");
    log.trace_done("Configuration validation complete");
    log.trace_sub("Removing temporary files");
}

fn test_context_tracing(log: &mut Stderr) {
    log.info("=== Context + Tracing Integration ===");
    
    // Set context and trace within it
    log.set_context("@myapp.VAR.config");
    log.trace_fn("resolve_context", "context set to myapp config");
    
    // Change context - should show new banner
    log.set_context("@myapp.VAR.secrets");
    log.trace_fn("resolve_context", "context changed to secrets");
    
    // Trace some work in this context
    log.trace_fn("load_secrets", "loading secret keys");
    log.trace_fn("load_secrets", "secrets loaded successfully");
    
    // Clear context
    log.clear_context();
    log.trace_fn("cleanup", "context cleared");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_trace_functions_exist() {
        let mut log = Stderr::new();
        
        // Test that trace functions don't panic
        log.trace_fn("test", "message");
        log.trace_add("add message");
        log.trace_sub("sub message");
        log.trace_found("found message");
        log.trace_done("done message");
        log.trace_item("item message");
    }
}
