use stderr::{readline};

fn main() {
    println!("Please enter your name:");
    // Assuming you make a standalone `readline` function
    match readline() {
        Ok(name) => println!("Hello, {}!", name.trim()),
        Err(e) => eprintln!("Error reading line: {}", e),
    }
}
