// ------------------------------------
// Main Entry point of the Application
// -----------------------------------

// Imports

mod utilz;
use utilz::{clear_console, header, pswg};

// Main Logic

fn main() {
    clear_console();
    pswg("Whootey".to_string());
    println!("Hello, world!");
    header("Wilka");
}
