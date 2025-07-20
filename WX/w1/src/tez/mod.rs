// ----------------------------------------------
// tez/mod.rs - All test functions in here
// ----------------------------------------------

// --- Imports ---
use crate::utilz::{clear_console, header, pswg};

// -- Main function ---

pub fn main_tez() {
    clear_console();
    boxy1()
}

// -- Subs

// Testing out boxy-clis
fn boxy1() {
    let t1 = "Boxy Test 1";
    pswg(t1.to_string())
}
