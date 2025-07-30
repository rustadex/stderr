// in: src/bin/grid_driver.rs


use std::io;

use stderr::{print_color_grid, Stderr, Config};

fn main() -> io::Result<()> {


    // --- Program Logic ---
    println!(
        "\nPrinting 256-color grid with 6 columns...\n",

    );


    let mut logger = Stderr::new().with_config(Config::default());

    print_color_grid(&mut logger, 6)?;

    println!("\nGrid printed successfully.\n");

    Ok(())
}
