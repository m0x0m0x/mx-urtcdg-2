//-----------------------------------------
// Will write the chapters that are being worled on
// s1.rs = Section 1 of the work
//-----------------------------------------

// --- attributes ---
#![allow(dead_code)]

// --- Imports ---
use crate::utilz::{clear_console, pswg};
use boxy_cli::prelude::*;
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
    // Method for making a deck of cards
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

    fn shuffle(&self) {}
}

// #[allow(unused_variables)]
fn s2() {
    let t1 = "7: Representing data with structs";
    pswg(t1.to_string());

    // Generate a new dec
    let deck = Deck::new();

    // Shufle the deck
    deck.shuffle();

    // Print the deck with formatting
    for card in &deck.cards {
        println!("{}", card);
    }
}

// This is like function s2 but with boxy
// Output is not that good. Abandon
fn s2_boxy() {
    let t1 = "Cards printed inside box";
    pswg(t1.to_string());

    let deck = Deck::new();

    // Concatenate all cards into one string (with a newline between each card)
    let combined_cards = deck.cards.join("\n");

    // Create a single box for all the cards
    let mut boxy = BoxyBuilder::default()
        .box_type(BoxType::Single) // Choose your border style (Simple, Double, etc.)
        .color("#32CD32") // Border color
        .padding(BoxPad::uniform(1), BoxPad::uniform(1)) // Padding around the text
        .align(BoxAlign::Left) // Center the content inside the box
        .width(40); // Set a fixed width for the box

    // Add the combined string of cards as a segment
    boxy = boxy.add_segment(&combined_cards, "#FFFFFF", BoxAlign::Left);

    // Build the box
    let mut boxy = boxy.build();

    // Display the box with all cards inside
    boxy.display();
}
