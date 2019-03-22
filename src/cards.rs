use std::fmt;
use std::cmp::Ordering;
use std::slice::Iter;
use serde::{Serialize, Deserialize};
use rand::{Rng};

// -----------  Suit -----------
#[derive (Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
           Suit::Heart => write!(f, "♥"),
           Suit::Diamond => write!(f, "♦"),
           Suit::Spade => write!(f, "♠"),
           Suit::Club => write!(f, "♣"),
       }
    }
}

impl Suit {
   pub fn iter() -> Iter<'static, Suit> {
        static SUITS: [Suit;  4] = [
            Suit::Heart,
            Suit::Diamond, 
            Suit::Spade,
            Suit::Club
        ];
        SUITS.into_iter()
    }
}

// -----------  Rank -----------
#[derive (Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Rank {
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace
}

impl Rank {
    pub fn point_value(&self) -> u32 {
       match self {
           Rank::Seven => 7,
           Rank::Eight => 8,
           Rank::Nine => 9,
           Rank::Ace => 11,
           _ => 10,
       }
    }

   pub fn iter() -> Iter<'static, Rank> {
        static RANKS: [Rank;  8] = [
            Rank::Seven,
            Rank::Eight,
            Rank::Nine,
            Rank::Ten,
            Rank::Jack,  
            Rank::Queen, 
            Rank::King,
            Rank::Ace,
        ];
        RANKS.into_iter()
    }

    pub fn succ(&self) -> Option<Self> {
        match self {
            Rank::Seven => Some(Rank::Eight),
            Rank::Eight => Some(Rank::Nine),
            Rank::Nine  => Some(Rank::Ten),
            Rank::Ten   => Some(Rank::Jack),
            Rank::Jack  => Some(Rank::Queen),
            Rank::Queen => Some(Rank::King),
            Rank::King  => Some(Rank::Ace),
            Rank::Ace   => None
        }
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
           Rank::Seven => write!(f, "7"),
           Rank::Eight => write!(f, "8"),
           Rank::Nine => write!(f, "9"),
           Rank::Ten => write!(f, "10"),
           Rank::Jack => write!(f, "J"),
           Rank::Queen => write!(f, "Q"),
           Rank::King => write!(f, "K"),
           Rank::Ace => write!(f, "A"),
       }
    }
}

// -----------  Card -----------
#[derive (Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Card { 
    pub rank: Rank,
    pub suit: Suit
    }

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.rank, self.suit)
    }
}

impl Card {
    pub fn new(rank:Rank, suit:Suit) -> Self {
        Card { rank, suit }
    }
    pub fn point_value(&self) -> u32 {
        self.rank.point_value()
    }
}

// -----------  Deck -----------
#[derive (Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Deck(Vec<Card>);

impl Deck {
    pub fn empty_deck() -> Self {
        Deck (vec![])
    }

    pub fn new() -> Self {
        let mut dcards: Vec<Card> = Vec::new();
        for r in Rank::iter(){
            for s in Suit::iter(){
                dcards.push(Card::new(r.clone(), s.clone()));
            }
        }
        Deck(dcards)
    }

    pub fn get_cards(&self) -> &Vec<Card>{
        let Deck(cards) = self;
        cards
    }

    pub fn shuffle<RNG:Rng>(&mut self, mut rng: RNG) {
        rng.shuffle(&mut self.0);
    }
}

// -----------  Hand -----------
#[derive (Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Hand(Vec<Card>);

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Hand(cards) = self;
        for (count, c) in cards.iter().enumerate() {
            if count != 0 { write!(f, " "); }
            write!(f, "{}", c);
        }
        write!(f, " ")
    }
}

impl Hand {
    pub fn empty_hand() -> Self {
        Hand (vec![])
    }

    pub fn new(cards:Vec<Card>) -> Self {
        Hand (cards)
    }

    pub fn cards(self) -> Vec<Card>{
        let Hand(cards) = self;
        cards
    }

    pub fn iter(&self) -> Iter<Card> {
        let Hand(cards) = self;
        cards.iter()
    }

    pub fn len(&self) -> usize {
        let Hand(cards) = self;
        cards.len()
    }

    pub fn max(&self) -> Option<&Card> {
        let Hand(cards) = self;
        cards.iter().max()
    }

    // used for declaration
    pub fn point_value(&self) -> u32 {
        let Hand(cards) = self;
        cards.iter().map(|c| c.point_value()).sum()
    }

    pub fn sort_by_suit(&mut self) {
        let Hand(cards) = self;
        cards.sort_by(|a, b| {
            if a == b {
                Ordering::Equal
            } else if a.suit == b.suit {
                a.rank.cmp(&b.rank)
            } else {
                a.suit.cmp(&b.suit)
            }
        });
    }

    pub fn sort_by_rank(&mut self)  {
        let Hand(cards) = self;
        cards.sort_by(|a, b| {
            if a == b {
                Ordering::Equal
            } else if a.rank == b.rank {
                a.suit.cmp(&b.suit)
            } else {
                a.rank.cmp(&b.rank)
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deck() {
        let Deck(cards) = Deck::new();
        assert_eq!(cards.len(), 32);
    }

    #[test]
    fn test_sort_hand() {
        let mut hand = Hand::new(vec![ 
            Card::new(Rank::Seven, Suit::Diamond), 
            Card::new(Rank::King, Suit::Diamond), 
            Card::new(Rank::Seven, Suit::Heart), 
            Card::new(Rank::Eight, Suit::Spade), 
        ]);
        hand.sort_by_rank();
        assert_eq!(
            hand,
            Hand::new(vec![
                Card::new(Rank::Seven, Suit::Heart), 
                Card::new(Rank::Seven, Suit::Diamond), 
                Card::new(Rank::Eight, Suit::Spade), 
                Card::new(Rank::King, Suit::Diamond), 
            ])
        );
        hand.sort_by_suit();
        assert_eq!(
            hand,
            Hand::new(vec![
                Card::new(Rank::Seven, Suit::Heart), 
                Card::new(Rank::Seven, Suit::Diamond), 
                Card::new(Rank::King, Suit::Diamond), 
                Card::new(Rank::Eight, Suit::Spade), 
            ])
        );
    }
}

// TODO
//
// instance (Ord a, Binary a) => Binary (OSet a) where
//     put s = Bin.put (size s) <> mapM_ Bin.put (toList s)
//     get   = liftM fromList Bin.get
//
// instance (Ord a, ToJSON a) => ToJSON (OSet a) where
//     toJSON = toJSON . toList
//
// instance (Ord a, FromJSON a) => FromJSON (OSet a) where
//     parseJSON = fmap fromList . parseJSON
//
