use crate::card::{Card, Rank, Suit};
use crate::poker_hand::{cards_to_hand, PokerHandRank};


// Get all possible combinations of 5 cards in 7
pub fn get_combinations(cards: [Card; 7]) -> Vec<[Card; 5]> {
    let mut combinations = Vec::new();

    for i in 0..3 {
        for j in i + 1..4 {
            for k in j + 1..5 {
                for l in k + 1..6 {
                    for m in l + 1..7 {
                        combinations.push([cards[i], cards[j], cards[k], cards[l], cards[m]]);
                    }
                }
            }
        }
    }

    combinations
}

// Rank all possibles hands and return the best one
pub fn get_best_hand(cards: [Card; 7]) -> PokerHandRank {
    get_combinations(cards)
        .iter()
        .map(|hand| cards_to_hand(*hand))
        .max()
        .unwrap()
}

// Create a deck of 52 cards
pub fn new_deck() -> Vec<Card> {
    use Rank::*;
    use Suit::*;
    let mut deck = Vec::new();
    for &rank in &[
        Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace,
    ] {
        for &suit in &[Spades, Hearts, Clubs, Diamonds] {
            deck.push(Card::new(rank, suit));
        }
    }
    deck
}

// Shuffle the deck in-place
pub fn shuffle_deck(deck: &mut Vec<Card>) {
    use rand::seq::SliceRandom;
    use rand::thread_rng;
    deck.shuffle(&mut thread_rng());
}

// Determine the winner of the game
// based on multiple hands and a community board
pub fn determine_winner(
    hands: Vec<[Card; 2]>,
    community: [Card; 5],
) -> (Vec<usize>, Vec<PokerHandRank>) {
    // Get the best hands for each hand
    let best_hands = hands
        .iter()
        .map(|hand| {
            get_best_hand([
                hand[0],
                hand[1],
                community[0],
                community[1],
                community[2],
                community[3],
                community[4],
            ])
        })
        .collect::<Vec<_>>();

    // Get the hightest ranking hand of the best hands
    let winning_hand = best_hands.iter().max().unwrap();

    // Find the players that match the winning hand
    // (Allows for ties)
    (
        best_hands
            .iter()
            .enumerate()
            .filter_map(
                |(i, hand)| {
                    if hand == winning_hand {
                        Some(i)
                    } else {
                        None
                    }
                },
            )
            .collect(),
        best_hands,
    )
}

/// Get a deck of cards but remove the given cards
pub fn deck_without_cards(deck: Vec<Card>, cards: Vec<Card>) -> Vec<Card> {
    let mut deck = deck;
    for card in cards {
        deck.retain(|c| *c != card);
    }
    deck
}

#[cfg(test)]
mod test {
    use super::*;
    use PokerHandRank::*;
    use Rank::*;
    use Suit::*;

    #[test]
    fn test_get_best_hand() {
        // Test high card
        assert_eq!(
            get_best_hand([
                Card::new(Ace, Spades),
                Card::new(Queen, Clubs),
                Card::new(Nine, Hearts),
                Card::new(Eight, Clubs),
                Card::new(Seven, Spades),
                Card::new(Three, Diamonds),
                Card::new(Two, Hearts),
            ]),
            HighCard(Ace, Queen, Nine, Eight, Seven)
        );
        // Test pair
        assert_eq!(
            get_best_hand([
                Card::new(Ace, Spades),
                Card::new(Queen, Clubs),
                Card::new(Nine, Hearts),
                Card::new(Nine, Clubs),
                Card::new(Seven, Spades),
                Card::new(Three, Diamonds),
                Card::new(Two, Hearts),
            ]),
            Pair(Nine, Ace, Queen, Seven)
        );
        // Test two pair
        assert_eq!(
            get_best_hand([
                Card::new(Ace, Spades),
                Card::new(Queen, Clubs),
                Card::new(Nine, Hearts),
                Card::new(Nine, Clubs),
                Card::new(Seven, Spades),
                Card::new(Seven, Diamonds),
                Card::new(Two, Hearts),
            ]),
            TwoPair(Nine, Seven, Ace)
        );
        // Test three of a kind
        assert_eq!(
            get_best_hand([
                Card::new(Ace, Spades),
                Card::new(Queen, Clubs),
                Card::new(Nine, Hearts),
                Card::new(Nine, Clubs),
                Card::new(Nine, Spades),
                Card::new(Seven, Diamonds),
                Card::new(Two, Hearts),
            ]),
            ThreeOfAKind(Nine, Ace, Queen)
        );
        // Test straight
        assert_eq!(
            get_best_hand([
                Card::new(Ace, Hearts),
                Card::new(King, Hearts),
                Card::new(Queen, Clubs),
                Card::new(Jack, Diamonds),
                Card::new(Ten, Spades),
                Card::new(Eight, Clubs),
                Card::new(Two, Spades),
            ]),
            Straight(Ace)
        );
        // Test flush
        assert_eq!(
            get_best_hand([
                Card::new(Ace, Spades),
                Card::new(Queen, Spades),
                Card::new(Nine, Spades),
                Card::new(Eight, Clubs),
                Card::new(Seven, Spades),
                Card::new(Three, Diamonds),
                Card::new(Two, Spades),
            ]),
            Flush(Ace, Queen, Nine, Seven, Two)
        );
        // Test Full House
        assert_eq!(
            get_best_hand([
                Card::new(Ace, Spades),
                Card::new(Ace, Clubs),
                Card::new(Ace, Hearts),
                Card::new(Eight, Spades),
                Card::new(Seven, Spades),
                Card::new(Two, Diamonds),
                Card::new(Two, Spades),
            ]),
            FullHouse(Ace, Two)
        );

        // Test Four of a kind
        assert_eq!(
            get_best_hand([
                Card::new(Ace, Spades),
                Card::new(Ace, Clubs),
                Card::new(Ace, Hearts),
                Card::new(Ace, Diamonds),
                Card::new(Seven, Spades),
                Card::new(Two, Diamonds),
                Card::new(Two, Spades),
            ]),
            FourOfAKind(Ace, Seven)
        );

        // Test Straight Flush
        assert_eq!(
            get_best_hand([
                Card::new(Ace, Spades),
                Card::new(King, Spades),
                Card::new(Queen, Spades),
                Card::new(Jack, Spades),
                Card::new(Ten, Spades),
                Card::new(Two, Hearts),
                Card::new(Three, Clubs),
            ]),
            StraightFlush(Ace)
        );
    }

    #[test]
    fn test_hand_order() {
        // Make 20 hands to test compairing
        let mut hands = vec![
            HighCard(Ace, King, Queen, Jack, Ten),
            Pair(Ace, King, Four, Three),
            TwoPair(Ace, King, Queen),
            ThreeOfAKind(Ace, King, Queen),
            Straight(Ace),
            Flush(Ace, King, Queen, Jack, Ten),
            FullHouse(Ace, King),
            FourOfAKind(Ace, King),
            StraightFlush(Ace),
            HighCard(Ace, Queen, Jack, Ten, Nine),
            Pair(Ace, Queen, Four, Five),
            TwoPair(Ace, Queen, Jack),
            ThreeOfAKind(Ace, Queen, Two),
        ];

        // Sort the hands
        hands.sort_unstable();

        // Check that the hands are in the correct order
        assert_eq!(
            hands,
            vec![
                HighCard(Ace, Queen, Jack, Ten, Nine),
                HighCard(Ace, King, Queen, Jack, Ten),
                Pair(Ace, Queen, Four, Five),
                Pair(Ace, King, Four, Three),
                TwoPair(Ace, Queen, Jack),
                TwoPair(Ace, King, Queen),
                ThreeOfAKind(Ace, Queen, Two),
                ThreeOfAKind(Ace, King, Queen),
                Straight(Ace),
                Flush(Ace, King, Queen, Jack, Ten),
                FullHouse(Ace, King),
                FourOfAKind(Ace, King),
                StraightFlush(Ace),
            ]
        );
    }

    #[test]
    fn test_get_combinations() {
        let cards = [
            Card::new(Ace, Spades),
            Card::new(King, Spades),
            Card::new(Queen, Spades),
            Card::new(Jack, Spades),
            Card::new(Ten, Spades),
            Card::new(Nine, Spades),
            Card::new(Eight, Spades),
        ];
        let combinations = get_combinations(cards);
        assert_eq!(combinations.len(), 21);
    }
}
