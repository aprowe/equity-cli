mod card;
mod poker_hand;
mod poker_utils;

use card::{cards_from_str, Card};
use clap;
use poker_utils::{deck_without_cards, determine_winner, new_deck, shuffle_deck};

/// Given a game state, run simulations to determine the frequencies of winning
fn run_out(
    deck: Vec<Card>,
    hands: Vec<[Card; 2]>,
    community: Vec<Card>,
    iterations: u32,
) -> Vec<f32> {
    let mut wins = vec![0.0; hands.len()];

    for i in 0..iterations {
        let mut deck = deck.clone();
        let hands = hands.clone();
        let mut community = community.clone();
        shuffle_deck(&mut deck);

        while community.len() < 5 {
            community.push(deck.pop().unwrap());
        }

        if i % 10000 == 0 {
            println!("Iteration: {i}");
        }

        let (idx, _) = determine_winner(hands, community.try_into().unwrap());
        for i in &idx {
            wins[*i] += 1.0 / idx.len() as f32;
        }
    }

    wins.iter().map(|c| *c as f32 / iterations as f32).collect()
}

use clap::Parser;

/// Equity Calculator
#[derive(Parser, Debug)]
#[command(
    name = "equity-cli",
    version,
    about = "Simple Equity Calculator for poker"
)]
struct Args {
    /// Number of iterations
    #[arg(short, long, default_value_t = 100_000)]
    iterations: u32,

    /// Current board
    /// Cards should use two letters each
    /// Example: 5c6hQs
    #[arg(short, long, default_value = "")]
    board: String,

    /// Hands to compare.
    /// Hands should be separated by space, and use two letters for each hand
    /// such as AhAs or KdQd
    #[arg()]
    hands: Vec<String>,
}

/// Run the actual Caculation
fn run_calculation(board: Vec<Card>, hands: Vec<[Card; 2]>, iterations: u32) -> Vec<f32> {
    let deck = new_deck();

    let mut dead_cards = hands
        .iter()
        .flat_map(|h| h.iter())
        .cloned()
        .collect::<Vec<_>>();

    dead_cards.append(&mut board.clone());

    let deck = deck_without_cards(deck, dead_cards);

    run_out(deck, hands, board, iterations)
}

fn main() {
    let args = Args::parse();

    if args.hands.len() < 2 {
        panic!("You need at least 2 hands to compare");
    }

    let hands = args
        .hands
        .iter()
        .map(|h| cards_from_str(h))
        .map(|h| [h[0], h[1]])
        .collect::<Vec<[Card; 2]>>();

    let board = cards_from_str(&args.board);

    // Print out Hands it will run
    for (i, hand) in hands.iter().enumerate() {
        println!("Hand {}: {:?}", i + 1, hand);
    }

    // Print out board
    println!("Board: {:?}", board);

    println!("Running {} iterations...", args.iterations);
    let results = run_calculation(board, hands, args.iterations);

    // Print Results
    for (i, result) in results.iter().enumerate() {
        println!("Hand {}: {:.2}%", i + 1, result * 100.0);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /// Shorthand for creating pocket cards
    pub fn c(s: &str) -> [Card; 2] {
        let cards = cards_from_str(s);
        assert_eq!(cards.len(), 2);
        [cards[0], cards[1]]
    }

    #[test]
    fn test_aces_vs_kings() {
        let result = run_calculation(vec![], vec![c("AhAs"), c("KdKh")], 10_000);
        assert_eq!(result.len(), 2);

        // AA should be ~ 2
        assert!(result[0] > 0.80, "actual: {}", result[0]);
        assert!(result[1] < 0.20, "actual: {}", result[1]);
    }
}
