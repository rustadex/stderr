// examples/table_test_driver.rs

use stderr::{Stderr, BorderStyle, flag_table, term_width};
use std::io::Result;

fn main() -> Result<()> {
    let mut log = Stderr::new();
    
    println!("=== Table Formatting Test ===\n");
    
    // Test 1: Simple table (like bookdb ls output)
    log.info("Test 1: Simple Table (BookDB ls style)");
    log.simple_table(&[
        &["Name", "Type", "Size"],
        &["config.env", "file", "1.2KB"],
        &["secrets/", "dir", "--"],
        &["api_keys.json", "file", "856B"],
        &["logs/", "dir", "--"],
    ])?;
    
    println!("\n");
    
    // Test 2: Project listing
    log.info("Test 2: Project Listing");
    log.simple_table(&[
        &["Project", "Keystores", "Last Updated"],
        &["myapp", "3", "2024-01-15"],
        &["work", "7", "2024-01-14"], 
        &["personal", "12", "2024-01-13"],
    ])?;
    
    println!("\n");
    
    // Test 3: List with bullet points
    log.info("Test 3: Bullet List (Keys)");
    let keys = ["API_KEY", "DB_PASSWORD", "JWT_SECRET", "REDIS_URL"];
    log.list(&keys, "→")?;
    
    println!("\n");
    
    // Test 4: Numbered list
    log.info("Test 4: Numbered List (Steps)");
    let steps = [
        "Create new project", 
        "Set up keystore",
        "Add environment variables",
        "Test configuration"
    ];
    log.numbered_list(&steps)?;
    
    println!("\n");
    
    // Test 5: Column layout (like ls output)
    log.info("Test 5: Column Layout (ls style)");
    let items = [
        "config", "secrets", "api_keys", "database", 
        "redis", "jwt", "oauth", "webhooks",
        "logging", "monitoring", "alerts", "backup"
    ];
    log.columns(&items, 4)?;
    
    println!("\n");
    
    // Test 6: Flag table (bitmap display)
    log.info("Test 6: Flag Table (Bitmap)");
    let flags: u32 = 0b1010_1100_0011_0101;
    let labels = &[
        "READ", "WRITE", "EXEC", "DELETE",
        "CREATE", "UPDATE", "LIST", "ADMIN",
        "DEBUG", "TRACE", "BACKUP", "RESTORE", 
        "MONITOR", "ALERT", "SYNC", "ASYNC"
    ];
    
    let current_term_width = term_width();
    let table_string = flag_table(flags, labels, BorderStyle::Light, current_term_width);
    println!("{}", table_string);
    
    println!();
    
    // Test 7: Different border styles
    log.info("Test 7: Box Styles");
    
    log.box_light("Light border box\nMultiple lines supported")?;
    println!();
    
    log.box_heavy("Heavy border box\nFor important messages")?;
    println!();
    
    log.box_double("Double border box\nFor critical alerts")?;
    println!();
    
    // Test 8: Banner
    log.banner("Test Banner", '=')?;
    log.info("Content under the banner");
    
    println!("\n=== Table Test Complete ===");
    
    Ok(())
}
