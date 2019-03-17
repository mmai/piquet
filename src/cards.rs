use std::fmt;
use std::cmp::Ordering;
use serde::{Serialize, Deserialize};

#[derive (Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
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

#[derive (Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
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

#[derive (Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Card { 
    rank: Rank,
    suit: Suit
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
}


pub struct Hand {
    cards: Vec<Card>,
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.cards)
    }
}

impl Hand {
    pub fn new(cards:Vec<Card>) -> Self {
        Hand {cards}
    }

    pub fn sort_by_suit(&mut self) {
        self.cards.sort_by(|a, b| {
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
        self.cards.sort_by(|a, b| {
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

// /// A deck of cards.
// pub struct Deck {
//     cards: Vec<Card>,
// }



#[cfg(test)]
mod tests {
    use super::*;

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
            hand.cards,
            vec![
                Card::new(Rank::Seven, Suit::Heart), 
                Card::new(Rank::Seven, Suit::Diamond), 
                Card::new(Rank::Eight, Suit::Spade), 
                Card::new(Rank::King, Suit::Diamond), 
            ]
        );
        hand.sort_by_suit();
        assert_eq!(
            hand.cards,
            vec![
                Card::new(Rank::Seven, Suit::Heart), 
                Card::new(Rank::Seven, Suit::Diamond), 
                Card::new(Rank::King, Suit::Diamond), 
                Card::new(Rank::Eight, Suit::Spade), 
            ]
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
// type Deck = OSet Card
// type Hand = OSet Card
//
// sortedDeck :: Deck
// sortedDeck = fromList [Card rank suit | rank <- [Seven .. Ace],  suit <- [Clubs .. Spades]]
//
// noCards :: OSet Card
// noCards = fromList []
