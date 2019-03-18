use std::fmt;
use std::cmp::Ordering;
use serde::{Serialize, Deserialize};

use crate::cards::Hand;

#[derive (Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CombinationType {
 Point, Sequence, Set 
}

impl fmt::Display for CombinationType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
           CombinationType::Point => write!(f, "Point"),
           CombinationType::Sequence => write!(f, "Sequence"),
           CombinationType::Set => write!(f, "Set"),
       }
    }
}

#[derive (Debug, Serialize, Deserialize)]
struct Combination { 
    combination_type: CombinationType,
    cards: Hand
} 

impl fmt::Display for Combination {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} : {}", self.combination_type, self.cards)
    }
}

impl PartialEq for Combination {
    fn eq(&self, other: &Combination) -> bool {
        self.combination_type != other.combination_type || ( self.cards == other.cards )
    }
}
impl Eq for Combination {}

impl Ord for Combination {
    fn cmp(&self, other: &Combination) -> Ordering {
        if (self.combination_type != other.combination_type) {
            return Ordering::Equal
        } else if (self.cards.len() != other.cards.len()){
            return self.cards.len().cmp(&other.cards.len())
        }
        self.height.cmp(&other.height)
    }
}

impl PartialOrd for Combination {
    fn partial_cmp(&self, other: &Combination) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// instance Ord Combination where
//   compare (Combination ta ha ) (Combination tb hb )
//     | ta /= tb               = EQ
//     | length ha /= length hb = compare (length ha) (length hb) -- by card count, valid for all combination types
//     | ta == Sequence = compare (maximum (toList ha)) (maximum (toList hb))
//     | ta == Set      = compare (maximum (toList ha)) (maximum (toList hb))
//     | ta == Point    = if length ha == 0
//                           then EQ
//                           else compare (sum $ pointValue <$> toList ha) (sum $ pointValue <$> toList hb)
