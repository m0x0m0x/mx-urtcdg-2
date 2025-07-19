//-----------------------------------------
// Will write the chapters that are being worled on
// s1.rs = Section 1 of the work
//-----------------------------------------

//

// --- Imports ---
use crate::utilz::{clear_console, pswg};

// --- main ---
pub fn s1_main() {
    clear_console();
    s2();
}

// --- Sub ---

// Test s1 sub function
fn s1() {
    // Function header
    let t1 = "src/sec2/s1 main";
    pswg(t1.to_string())
}

// 7 - Representing data with structs
fn s2() {
    let t1 = "7: Representing data with structs";
    pswg(t1.to_string());
}
