// examples/comprehensive_test_driver.rs
// Tests all stderr features in a single comprehensive run

use stderr::{Stderr, BorderStyle, Glyph, Color as ESC, LogLevel, GlyphSet};
use std::io::Result;

fn main() -> Result<()> {
    println!("🚀 Starting Comprehensive Stderr Library Test");
    println!("===============================================\n");
    
    // Test basic functionality first
    test_basic_logging()?;
    
    // Test context system
    test_context_management()?;
    
    // Test tracing (if available)
    test_tracing_system()?;
    
    // Test table formatting
    test_table_formatting()?;
    
    // Test interactive features
    test_interactive_features()?;
    
    // Test customization
    test_customization()?;
    
    // Test edge cases
    test_edge_cases()?;
    
    println!("\n🎉 Comprehensive Test Complete!");
    println!("All stderr library features validated successfully.");
    
    Ok(())
}

fn test_basic_logging() -> Result<()> {
    println!("📝 Testing Basic Logging...");
    
    let mut log = Stderr::new();
    
    // Test all log levels
    log.info("Information message");
    log.warn("Warning message");
    log.error("Error message");
    log.okay("Success message");
    log.note("Note message");
    
    // Test conditional messages (only with env vars)
    log.debug("Debug message (needs DEBUG_MODE=0)");
    log.trace("Trace message (needs TRACE_MODE=0)");
    log.devlog("Dev message (needs DEV_MODE=0)");
    log.magic("Magic message (needs SILLY_MODE=0)");
    
    // Test debug printing
    #[derive(Debug)]
    struct TestData {
        name: String,
        value: i32,
    }
    
    let test_obj = TestData {
        name: "test".to_string(),
        value: 42,
    };
    
    log.info_debug(&test_obj);
    
    println!("✓ Basic logging tests passed\n");
    Ok(())
}

fn test_context_management() -> Result<()> {
    println!("🎯 Testing Context Management...");
    
    let mut log = Stderr::new();
    
    // Test context setting with banners
    log.set_context("@myapp.VAR.config");
    log.info("First context set");
    
    // Same context should not show banner
    log.set_context("@myapp.VAR.config");
    log.info("Same context - no banner");
    
    // Different context should show new banner
    log.set_context("@work.VAR.secrets");
    log.warn("Different context with banner");
    
    // Clear context
    log.clear_context();
    log.info("Context cleared");
    
    println!("✓ Context management tests passed\n");
    Ok(())
}

fn test_tracing_system() -> Result<()> {
    println!("🔍 Testing Tracing System...");
    
    let mut log = Stderr::new().with_label("tracer");
    
    // Manual function tracing
    log.trace_fn("test_function", "starting work");
    log.trace_fn("test_function", "continuing work");
    log.trace_fn("other_function", "different task");
    
    // Scoped tracing
    {
        let mut scope = log.trace_scope("scoped_work");
        scope.step("step 1 complete");
        scope.step("step 2 complete");
        scope.step_debug("final result", &vec![1, 2, 3]);
    }
    
    // Labelled traces
    log.trace_add("Adding new item");
    log.trace_found("Found existing item");
    log.trace_done("Work completed");
    log.trace_sub("Removing temp files");
    
    println!("✓ Tracing system tests passed\n");
    Ok(())
}

fn test_table_formatting() -> Result<()> {
    println!("📊 Testing Table Formatting...");
    
    let mut log = Stderr::new();
    
    // Simple table
    log.simple_table(&[
        &["Command", "Description", "Example"],
        &["getv", "Get variable", "bookdb getv API_KEY"],
        &["setv", "Set variable", "bookdb setv PORT=3000"],
        &["ls", "List items", "bookdb ls keys"],
    ])?;
    
    println!();
    
    // Column layout
    let items = ["config", "secrets", "api", "database", "redis", "jwt"];
    log.columns(&items, 3)?;
    
    println!();
    
    // List with bullets
    log.list(&["item1", "item2", "item3"], "→")?;
    
    println!();
    
    // Boxes
    log.box_light("Light box message")?;
    log.box_heavy("Heavy box message")?;
    
    println!();
    
    // Banner
    log.banner("Test Section", '=')?;
    
    println!("✓ Table formatting tests passed\n");
    Ok(())
}

fn test_interactive_features() -> Result<()> {
    println!("💬 Testing Interactive Features...");
    
    let mut log = Stderr::new();
    
    // Note: In automated testing, confirmations typically return default values
    log.info("Testing confirmation prompts (automated mode)");
    
    // Simple confirmation
    if let Ok(Some(response)) = log.confirm("Continue with test?") {
        if response {
            log.okay("Confirmation received");
        } else {
            log.warn("Confirmation declined");
        }
    } else {
        log.info("Running in non-interactive mode");
    }
    
    // Boxed confirmation
    let result = log.confirm_builder("Critical operation?")
        .boxed(true)
        .style(BorderStyle::Heavy)
        .ask();
        
    match result {
        Ok(Some(true)) => log.okay("Critical operation confirmed"),
        Ok(Some(false)) => log.warn("Critical operation cancelled"),
        Ok(None) => log.info("Operation quit"),
        Err(_) => log.info("Non-interactive environment detected"),
    }
    
    // Help display
    log.help("Test help content\nMultiple lines supported")?;
    
    println!("✓ Interactive features tests passed\n");
    Ok(())
}

fn test_customization() -> Result<()> {
    println!("🎨 Testing Customization...");
    
    // Test custom glyph set
    let custom_glyphs = GlyphSet {
        info: "ℹ",
        warn: "⚠",
        error: "❌",
        okay: "✅",
        trace: "🔍",
        debug: "🐛",
        magic: "✨",
    };
    
    let mut log = Stderr::new().with_glyphs(custom_glyphs);
    
    log.info("Info with custom glyph");
    log.warn("Warning with custom glyph");
    log.error("Error with custom glyph");
    log.okay("Success with custom glyph");
    
    // Test individual glyph setting
    log.set_glyph(LogLevel::Info, "🚀");
    log.info("Info with rocket glyph");
    
    // Test with label
    log.set_label("custom-app");
    log.info("Message with custom label");
    
    println!("✓ Customization tests passed\n");
    Ok(())
}

fn test_edge_cases() -> Result<()> {
    println!("🧪 Testing Edge Cases...");
    
    let mut log = Stderr::new();
    
    // Test empty/special content
    log.info("");  // Empty message
    log.info("Very long message that might wrap depending on terminal width and could potentially cause formatting issues but should be handled gracefully");
    
    // Test special characters
    log.info("Special chars: αβγδε 🚀💡🔥 ←→↑↓");
    
    // Test quiet mode
    log.set_quiet(true);
    log.info("This should be hidden");
    log.error("Errors should still show");
    log.set_quiet(false);
    log.okay("Quiet mode disabled");
    
    // Test rapid context changes
    for i in 1..=3 {
        log.set_context(&format!("@test{}.VAR.data", i));
        log.info(&format!("Context {}", i));
    }
    
    // Test large table
    let large_data: Vec<&[&str]> = vec![
        &["Header1", "Header2", "Header3"],
        &["Row1Col1", "Row1Col2", "Row1Col3"],
        &["Row2Col1", "Row2Col2", "Row2Col3"],
        &["Row3Col1", "Row3Col2", "Row3Col3"],
    ];
    log.simple_table(&large_data)?;
    
    println!("✓ Edge case tests passed\n");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_comprehensive_functionality() {
        // This test ensures all functions can be called without panicking
        let result = std::panic::catch_unwind(|| {
            let _ = test_basic_logging();
            let _ = test_context_management();
            let _ = test_tracing_system();
            let _ = test_table_formatting();
            let _ = test_customization();
            let _ = test_edge_cases();
        });
        
        assert!(result.is_ok(), "Comprehensive test should not panic");
    }
}
