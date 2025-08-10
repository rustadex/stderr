// examples/basic_test_driver.rs

use stderr::Stderr;

fn main() {
    let mut log = Stderr::new();
    
    println!("=== Basic Stderr Library Test ===\n");
    
    // Test all basic logging levels
    log.info("Basic info logging works");
    log.warn("Warning message test");
    log.error("Error message test");
    log.okay("Success message test");
    log.note("Note message test");
    
    // Test debug/trace (only visible with env vars)
    log.debug("Debug message (visible with DEBUG_MODE=0)");
    log.trace("Trace message (visible with TRACE_MODE=0)");
    log.devlog("Devlog message (visible with DEV_MODE=0)");
    log.magic("Magic message (visible with SILLY_MODE=0)");
    
    println!("\n=== Label Testing ===");
    
    // Test with labels
    log.set_label("test-app");
    log.info("Message with label");
    log.warn("Warning with label");
    
    log.clear_label();
    log.info("Back to no label");
    
    println!("\n=== Quiet Mode Testing ===");
    
    // Test quiet mode
    log.set_quiet(true);
    log.info("This should be hidden in quiet mode");
    log.error("Errors still show in quiet mode");
    
    log.set_quiet(false);
    log.okay("Quiet mode disabled - messages visible again");
    
    println!("\n=== Debug Printing ===");
    
    // Test debug printing
    #[derive(Debug)]
    struct TestStruct {
        id: u32,
        name: String,
    }
    
    let test_obj = TestStruct {
        id: 42,
        name: "test_object".to_string(),
    };
    
    log.info_debug(&test_obj);
    log.warn_debug(&test_obj);
    
    println!("\n=== Basic Test Complete ===");
}
