//-----------------------------------------
// Will write the chapters that are being worled on
// s1.rs = Section 1 of the work
//-----------------------------------------

// --- attributes ---
#![allow(dead_code)]

// --- Imports ---
use crate::utilz::{clear_console, header, pswg};
use boxy_cli::prelude::*;
use rand::{rng, seq::SliceRandom};
use yansi::Paint;

// --- Main Function---
pub fn s1_main() {
    pswg("Section3".to_string());
}

// --- Sun functions ---
