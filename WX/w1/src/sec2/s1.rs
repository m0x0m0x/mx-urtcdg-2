//-----------------------------------------
// Will write the chapters that are being worled on
// s1.rs = Section 1 of the work
//-----------------------------------------

// --- attributes ---
#![allow(dead_code)]

// --- Imports ---
use crate::utilz::{clear_console, pswg};
use yansi::Paint;

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

// Deck struct for s2
#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}
impl Deck {
    fn new() -> Self {
        // List of suits
        let suits = ["Hearts ♥️", "Diamonds ♦️"];
        let values = ["Ace", "Jack"];

        let mut cards = vec![];

        for suit in suits {
            for value in values {
                cards.push(format!("{} of {}", value.red(), suit.green()));
            }
        }

        // Instancing the Deck Struct
        Deck { cards }
    }
}

// #[allow(unused_variables)]
fn s2() {
    let t1 = "7: Representing data with structs";
    pswg(t1.to_string());

    // Create a new deck
    let deck = Deck::new();
    for card in &deck.cards {
        println!("{}", card);
    }
}
