//
//Main Entry point of the Application
//

// Imports 
 
 mod utilz;
 use utilz::{clear_console,header,pswg};

// Main Logic

fn main() {
    pswg("Whootey".to_string());
    clear_console();
    println!("Hello, world!");
    header("Wilka");
}
