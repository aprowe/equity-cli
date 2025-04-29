use std::{
    cmp::Ordering,
    fmt::{Display, Formatter},
};

/// Defines card structures such as Rank, Card, Suit
/// And the helper functions that they need

/// Standard 2-A card rankings
#[derive(Clone, Copy, Ord, Eq)]
pub enum Rank {
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}

/// Suit Enumeration
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Suit {
    Spades = 0,
    Hearts = 1,
    Clubs = 2,
    Diamonds = 3,
}

/// Combination of a rank and suit
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

///////////////////////////////////////////////
/// Rank Implementations
///////////////////////////////////////////////
impl Rank {
    /// Numeric Value of a rank
    pub fn value(&self) -> u32 {
        *self as u32
    }

    /// Next card in rankings for finding straights
    pub fn next(&self) -> Rank {
        match self {
            Rank::Two => Rank::Three,
            Rank::Three => Rank::Four,
            Rank::Four => Rank::Five,
            Rank::Five => Rank::Six,
            Rank::Six => Rank::Seven,
            Rank::Seven => Rank::Eight,
            Rank::Eight => Rank::Nine,
            Rank::Nine => Rank::Ten,
            Rank::Ten => Rank::Jack,
            Rank::Jack => Rank::Queen,
            Rank::Queen => Rank::King,
            Rank::King => Rank::Ace,
            Rank::Ace => Rank::Two,
        }
    }

    /// Convert rank to string for display
    pub fn to_string(&self) -> String {
        match self {
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "T",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
            Rank::Ace => "A",
        }
        .to_string()
    }
}

impl Display for Rank {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_string().as_str())
    }
}

impl std::fmt::Debug for Rank {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_string().as_str())
    }
}

impl PartialEq for Rank {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}

impl PartialOrd for Rank {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value().partial_cmp(&other.value())
    }
}

///////////////////////////////////////////////
/// Suit Implementations
///////////////////////////////////////////////
impl std::fmt::Debug for Suit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Suit::Spades => "s",
            Suit::Hearts => "h",
            Suit::Clubs => "c",
            Suit::Diamonds => "d",
        })
    }
}

impl std::fmt::Debug for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}{:?}", self.rank, self.suit))
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct ParseCardError {
    msg: String,
}

impl<'a> Into<ParseCardError> for &'a str {
    fn into(self: &'a str) -> ParseCardError {
        ParseCardError {
            msg: self.to_string(),
        }
    }
}

///////////////////////////////////////////////
/// Card Implementations
///////////////////////////////////////////////
impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Self {
        Self { rank, suit }
    }

    pub fn from_string(s: &str) -> Result<Self, ParseCardError> {
        let mut chars = s.chars().map(|c| c.to_lowercase().next().unwrap());

        let rank = match chars.next().ok_or("Empty String".into())? {
            '2' => Rank::Two,
            '3' => Rank::Three,
            '4' => Rank::Four,
            '5' => Rank::Five,
            '6' => Rank::Six,
            '7' => Rank::Seven,
            '8' => Rank::Eight,
            '9' => Rank::Nine,
            'T' => Rank::Ten,
            't' => Rank::Ten,
            'j' => Rank::Jack,
            'q' => Rank::Queen,
            'k' => Rank::King,
            'a' => Rank::Ace,
            _ => {
                return Err("Unmatched Rank".into());
            }
        };

        let suit = match chars.next().ok_or("No Suit".into())? {
            's' => Suit::Spades,
            'h' => Suit::Hearts,
            'c' => Suit::Clubs,
            'd' => Suit::Diamonds,
            _ => {
                return Err("Unmatched Suit".into());
            }
        };

        Ok(Self::new(rank, suit))
    }
}

///
/// Get a vector of cards from a string
/// such as "AhKsQh2c"
///
pub fn cards_from_str(s: &str) -> Vec<Card> {
    if s.is_empty() {
        return vec![];
    }

    // Group into two characters
    // First character is rank, second is suit
    s.chars()
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|chunk| chunk.iter().collect::<String>())
        .map(|card| Card::from_string(&card).expect(&format!("Error Parsing String: {}", s)))
        .collect()
}
