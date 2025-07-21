//-----------------------------------------
// Will write the chapters that are being worled on
// s1.rs = Section 1 of the work
// Project Description
/*
Section 3 - Focuses on bank project where you had stoppped
*/
//-----------------------------------------

// --- attributes ---
#![allow(dead_code)]

// --- Imports ---
use crate::utilz::{clear_console, header, pswg};
use boxy_cli::prelude::*;
use rand::{rng, seq::SliceRandom};
use yansi::Paint;

// --- Main Function---
pub fn s31_main() {
    pswg("Section3".to_string());
}

// --- Sun functions ---

// Bank and Account Struct
#[derive(Debug)]
struct Account {
    balance: u32,
    id: i32,
    holder: String,
}
#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}
