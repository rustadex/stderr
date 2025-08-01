

use stderr::{
    logger,
    Color as ESC,
    Glyph as ART,
};

#[allow(dead_code)]
#[derive(Debug)]
struct FakeStruct {
    id: u32,
    name: String,
    active: bool,
}

fn main() {
    // Use static logger
    logger.info("Starting macro driver");
    logger.okay("Started macro driver");
    logger.warn("Careful — edge case ahead");
    logger.error("Something went wrong but not really");

    // Use raw logger directly
    let mut log = logger.raw();
    log.trace("Debug only shows if DEBUG_MODE=0");
    log.debug("Debugging enabled message");
    log.trace("Devlog only shows if DEV_MODE=0");
    log.devlog("Dev-specific diagnostics");

    log.trace("Deep trace logs");
    log.magic("Mystical insights ✨");
    log.silly("Goofy stuff 🌀");

    // Pretty print object
    log.trace("Pretty Print Objects");
    let obj = FakeStruct {
        id: 42,
        name: "Quantus".into(),
        active: true,
    };

    log.print_with_prefix_debug(ESC::CYAN, ART::Xi, &obj).unwrap();

    // Showcase quiet mode toggle
    log.set_quiet(true);
    log.info("This won't show due to quiet mode");
    log.set_quiet(false);

    // Bonus: label-aware logger
    log.set_label("macro_driver");
    log.note("Logger now has a label");

    // Reset label
    log.clear_label();
    log.okay("Logger reset to neutral");

    // Custom prefix with color
    let _ = log.print_with_prefix(ESC::YELLOW2, "⚙ INIT", "Bootstrapping complete");

    println!("\n✅ All logger macros exercised.");
}
