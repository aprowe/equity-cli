use crate::card::{Rank, Card};

/// Enumeration of all Poker Hands
#[derive(Debug, Clone, Copy, Ord, Eq)]
pub enum PokerHandRank {
    // All five cards of different ranks
    HighCard(Rank, Rank, Rank, Rank, Rank),

    // Pair, then three kickers
    Pair(Rank, Rank, Rank, Rank),

    // Two pairs, then kicker
    TwoPair(Rank, Rank, Rank),

    // Three of a kind, then two kickers
    ThreeOfAKind(Rank, Rank, Rank),

    // High card of the straight
    Straight(Rank),

    // All five cards of the same suit
    Flush(Rank, Rank, Rank, Rank, Rank),

    // Rank 1 full of rank 2
    FullHouse(Rank, Rank),

    // Four of rank 1 plus kicker
    FourOfAKind(Rank, Rank),

    // Straight flush, high card of the straight
    StraightFlush(Rank),
}

// Decide if two poker hands are equivalent
impl PartialEq for PokerHandRank {
    fn eq(&self, other: &Self) -> bool {
        use PokerHandRank::*;
        match (self, other) {
            (HighCard(a1, b1, c1, d1, e1), HighCard(a2, b2, c2, d2, e2)) => {
                a1 == a2 && b1 == b2 && c1 == c2 && d1 == d2 && e1 == e2
            }
            (Pair(a1, b1, c1, d1), Pair(a2, b2, c2, d2)) => {
                a1 == a2 && b1 == b2 && c1 == c2 && d1 == d2
            }
            (TwoPair(a1, b1, c1), TwoPair(a2, b2, c2)) => a1 == a2 && b1 == b2 && c1 == c2,
            (ThreeOfAKind(a1, b1, c1), ThreeOfAKind(a2, b2, c2)) => {
                a1 == a2 && b1 == b2 && c1 == c2
            }
            (Straight(a1), Straight(a2)) => a1 == a2,
            (Flush(a1, b1, c1, d1, e1), Flush(a2, b2, c2, d2, e2)) => {
                a1 == a2 && b1 == b2 && c1 == c2 && d1 == d2 && e1 == e2
            }
            (FullHouse(a1, b1), FullHouse(a2, b2)) => a1 == a2 && b1 == b2,
            (FourOfAKind(a1, b1), FourOfAKind(a2, b2)) => a1 == a2 && b1 == b2,
            (StraightFlush(a1), StraightFlush(b2)) => a1 == b2,

            // If they arent the same type, they arent equal
            _ => false
        }
    }
}

impl PartialOrd for PokerHandRank {
    #[allow(unused_assignments)]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use std::cmp::Ordering;
        use PokerHandRank::*;

        // Convert the rank into a number for comparing
        fn rank_to_value(rank: &PokerHandRank) -> u8 {
            match rank {
                HighCard(..) => 1,
                Pair(..) => 2,
                TwoPair(..) => 3,
                ThreeOfAKind(..) => 4,
                Straight(..) => 5,
                Flush(..) => 6,
                FullHouse(..) => 7,
                FourOfAKind(..) => 8,
                StraightFlush(..) => 9,
            }
        }

        let self_value = rank_to_value(self);
        let other_value = rank_to_value(other);

        if self_value < other_value {
            Some(Ordering::Less)
        } else if self_value > other_value {
            Some(Ordering::Greater)
        } else {
            // If hand ranks are the same, comparison logic needed for each hand type
            match (self, other) {
                // Compare based on the highest card or cards in the hand
                (HighCard(a1, b1, c1, d1, e1), HighCard(a2, b2, c2, d2, e2)) => {
                    compare_ranks(&[a1, b1, c1, d1, e1], &[a2, b2, c2, d2, e2])
                }
                (Pair(a1, b1, c1, d1), Pair(a2, b2, c2, d2)) => {
                    compare_ranks(&[a1, b1, c1, d1], &[a2, b2, c2, d2])
                }
                (TwoPair(a1, b1, c1), TwoPair(a2, b2, c2)) => {
                    compare_ranks(&[a1, b1, c1], &[a2, b2, c2])
                }
                (ThreeOfAKind(a1, b1, c1), ThreeOfAKind(a2, b2, c2)) => {
                    compare_ranks(&[a1, b1, c1], &[a2, b2, c2])
                }
                (Straight(r1), Straight(r2)) => r1.partial_cmp(r2),
                (Flush(a1, b1, c1, d1, e1), Flush(a2, b2, c2, d2, e2)) => {
                    compare_ranks(&[a1, b1, c1, d1, e1], &[a2, b2, c2, d2, e2])
                }
                (FullHouse(r1, t1), FullHouse(r2, t2)) => compare_ranks(&[r1, t1], &[r2, t2]),
                (FourOfAKind(r1, k1), FourOfAKind(r2, k2)) => compare_ranks(&[r1, k1], &[r2, k2]),
                (StraightFlush(r1), StraightFlush(r2)) => r1.partial_cmp(r2),
                _ => unreachable!("All matches should be covered"),
            }
        }
    }
}

/// 
/// Important function that takes 5 cards and creates a poker hand out of it
/// 
pub fn cards_to_hand(cards: [Card; 5]) -> PokerHandRank {
    // sort the cards
    let mut cards = cards;
    cards.sort_unstable_by_key(|card| card.rank);

    // Reverse the cards so the highest card is first
    cards.reverse();

    // Check for straight flush
    if cards
        .windows(2)
        .all(|pair| pair[1].rank.next() == pair[0].rank)
        && cards.iter().all(|card| card.suit == cards[0].suit)
    {
        return PokerHandRank::StraightFlush(cards[0].rank);
    }

    // Check for four of a kind
    // needs to check the first 4 cards and the last 4 cards
    if cards[0].rank == cards[1].rank
        && cards[1].rank == cards[2].rank
        && cards[2].rank == cards[3].rank
    {
        return PokerHandRank::FourOfAKind(cards[0].rank, cards[4].rank);
    }

    if cards[1].rank == cards[2].rank
        && cards[2].rank == cards[3].rank
        && cards[3].rank == cards[4].rank
    {
        return PokerHandRank::FourOfAKind(cards[1].rank, cards[0].rank);
    }

    // Check for full house
    // needs to check the first 3 cards and the last 3 cards
    if cards[0].rank == cards[1].rank
        && cards[1].rank == cards[2].rank
        && cards[3].rank == cards[4].rank
    {
        return PokerHandRank::FullHouse(cards[0].rank, cards[3].rank);
    }
    if cards[0].rank == cards[1].rank
        && cards[2].rank == cards[3].rank
        && cards[3].rank == cards[4].rank
    {
        return PokerHandRank::FullHouse(cards[2].rank, cards[0].rank);
    }

    // Check for flush
    if cards.iter().all(|card| card.suit == cards[0].suit) {
        return PokerHandRank::Flush(
            cards[0].rank,
            cards[1].rank,
            cards[2].rank,
            cards[3].rank,
            cards[4].rank,
        );
    }

    // Check for striaght
    if cards
        .windows(2)
        .all(|pair| pair[1].rank.next() == pair[0].rank)
    {
        return PokerHandRank::Straight(cards[0].rank);
    }

    // Check for three of a kind
    // needs to check the first 3 cards, the middle 3 cards and the last 3 cards
    if cards[0].rank == cards[1].rank && cards[1].rank == cards[2].rank {
        return PokerHandRank::ThreeOfAKind(cards[0].rank, cards[3].rank, cards[4].rank);
    }
    if cards[1].rank == cards[2].rank && cards[2].rank == cards[3].rank {
        return PokerHandRank::ThreeOfAKind(cards[1].rank, cards[0].rank, cards[4].rank);
    }
    if cards[2].rank == cards[3].rank && cards[3].rank == cards[4].rank {
        return PokerHandRank::ThreeOfAKind(cards[2].rank, cards[0].rank, cards[1].rank);
    }

    // Check for two pair
    // needs to check the first 2 cards, the middle 2 cards and the last 2 cards
    if cards[0].rank == cards[1].rank && cards[2].rank == cards[3].rank {
        return PokerHandRank::TwoPair(cards[0].rank, cards[2].rank, cards[4].rank);
    }
    if cards[0].rank == cards[1].rank && cards[3].rank == cards[4].rank {
        return PokerHandRank::TwoPair(cards[0].rank, cards[3].rank, cards[2].rank);
    }
    if cards[1].rank == cards[2].rank && cards[3].rank == cards[4].rank {
        return PokerHandRank::TwoPair(cards[1].rank, cards[3].rank, cards[0].rank);
    }

    // Check for pair
    // needs to check the first 2 cards, the middle 2 cards and the last 2 cards
    if cards[0].rank == cards[1].rank {
        return PokerHandRank::Pair(cards[0].rank, cards[2].rank, cards[3].rank, cards[4].rank);
    }
    if cards[1].rank == cards[2].rank {
        return PokerHandRank::Pair(cards[1].rank, cards[0].rank, cards[3].rank, cards[4].rank);
    }
    if cards[2].rank == cards[3].rank {
        return PokerHandRank::Pair(cards[2].rank, cards[0].rank, cards[1].rank, cards[4].rank);
    }
    if cards[3].rank == cards[4].rank {
        return PokerHandRank::Pair(cards[3].rank, cards[0].rank, cards[1].rank, cards[2].rank);
    }

    // If none of the above, the hand is a high card
    PokerHandRank::HighCard(
        cards[0].rank,
        cards[1].rank,
        cards[2].rank,
        cards[3].rank,
        cards[4].rank,
    )
}

/// Function for comparing two lists of numbers for determining which hand is greater
/// Compares first two numbers. If they are the same,
/// continues comparing the next two numbers
fn compare_ranks(n_list: &[&Rank], m_list: &[&Rank]) -> Option<std::cmp::Ordering> {
    let mut n_sum: i32 = 0;
    let mut n_multiplier = 15 * 5;

    for n in n_list.into_iter() {
        n_sum += **n as i32 * n_multiplier;
        n_multiplier /= 15;
    }

    let mut m_sum: i32 = 0;
    let mut m_multiplier = 15 * 5;
    for m in m_list.into_iter() {
        m_sum += **m as i32 * m_multiplier;
        m_multiplier /= 15;
    }

    n_sum.partial_cmp(&m_sum)
}
