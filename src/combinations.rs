use std::fmt;
use std::collections::HashMap;
use std::cmp::Ordering;

use serde::{Serialize, Deserialize};
use crate::cards::{Hand, Card, Rank, Suit };

#[derive (Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
pub struct Combination { 
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
        if self.combination_type != other.combination_type {
            return Ordering::Equal
        } else if self.cards.len() != other.cards.len(){
            // by card count, valid for all combination types
            return self.cards.len().cmp(&other.cards.len())
        }
        return match self.combination_type {
            CombinationType::Sequence => self.cards.max().cmp(&other.cards.max()),
            CombinationType::Set      => self.cards.max().cmp(&other.cards.max()),
            CombinationType::Point    => if self.cards.len() == 0 {
                Ordering::Equal
            } else {
                self.cards.point_value().cmp(&other.cards.point_value())
            }
        }
    }
}

impl PartialOrd for Combination {
    fn partial_cmp(&self, other: &Combination) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Combination {
    // const set:      CombinationType = CombinationType::Set;
    // const point:    CombinationType = CombinationType::Point;
    // const sequence: CombinationType = CombinationType::Sequence;

    // This is used in the first part of the declaration
    pub fn compare_length(&self, other:Combination) -> Ordering {
        self.cards.len().cmp(&other.cards.len())
    }

    pub fn show_declaration(&self) -> String {
        let Combination { combination_type:ctype, cards } = self;
        return match ctype {
            CombinationType::Point => format!("Point of {}", cards.len()),
            CombinationType::Set => match cards.len() {
                3 => String::from("Trio"),
                4 => String::from("Quatorze"),
                _ => panic!("not a set")
            },
            CombinationType::Sequence => match cards.len() {
                3 => String::from("Tierce"),
                4 => String::from("Quart"),
                5 => String::from("Cinquième"),
                6 => String::from("Sixième"  ),
                7 => String::from("Septième" ),
                8 => String::from("Huitième" ),
                _ => panic!("not a sequence")
            }
        }
    }

    pub fn show_declaration_complete(&self) -> String {
        let Combination { combination_type:ctype, cards:chand } = self;
        return match ctype {
            CombinationType::Point => format!("{} totaling {}", self.show_declaration(), chand.point_value()),
            CombinationType::Sequence => format!("{} to {}", self.show_declaration(), chand.max().unwrap()),
            CombinationType::Set => format!("{} of {}", self.show_declaration(), chand.max().unwrap()),
        }
    }

    pub fn points(&self) -> usize {
        let Combination { combination_type:ctype, cards } = self;
        let size = cards.len();
        match ctype {
            CombinationType::Point    => size,
            CombinationType::Set      => if size == 4 { 14 } else { 3 },
            CombinationType::Sequence => if size > 4 { size + 10 } else { size },
        }

    }

    pub fn new(combination_type: CombinationType, cards: Hand) -> Self {
        Combination {combination_type, cards}
    }
}

pub fn get_combinations(ctype: &CombinationType, hand: &Hand) -> Vec<Combination> {
    let mut chand = hand.clone();
    match ctype {
        CombinationType::Point => {
            chand.sort_by_suit();
            let combs_hashmap = chand.iter().fold(HashMap::new(), |mut m, c| { 
                let key = format!("{}", c.suit);
                m.entry(key).or_insert(Vec::new()).push(c.clone()); 
                m 
            });
            combs_hashmap.values()
                .map(|c| Combination::new(ctype.clone(), Hand::new(c.clone()))) 
                .collect()
        },
        CombinationType::Set => {
            chand.sort_by_rank();
            let combs_hashmap = chand.iter().fold(HashMap::new(), |mut m, c| { 
                let key = format!("{}", c.rank);
                if c.rank > Rank::Nine {
                    m.entry(key).or_insert(Vec::new()).push(c.clone()); 
                }
                m 
            });
            combs_hashmap.values()
                .filter(|&c| c.len() > 2)
                .map(|c| Combination::new(ctype.clone(), Hand::new(c.clone()))) 
                .collect()
        },
        CombinationType::Sequence => {
            chand.sort_by_suit();
            let combs_vec = chand.iter().fold(Vec::new(), |mut acc, c| { 
                if acc.len() == 0 {
                    acc.push(vec![c.clone()])
                } else {
                    let seqIdx = acc.len() - 1;
                    let prec = &acc[seqIdx][acc[seqIdx].len() - 1];
                    if prec.suit == c.suit && prec.rank.succ() == Some(c.rank.clone()) {
                        acc[seqIdx].push(c.clone());
                    } else {
                        acc.push(vec![c.clone()])
                    }
                }
                acc 
            });
            combs_vec.iter()
                .filter(|&c| c.len() > 2)
                .map(|c| Combination::new(ctype.clone(), Hand::new(c.clone()))) 
                .collect()
        },
    }
}

pub fn get_smaller_combinations(mcomb: Option<Combination>, combs: Vec<Combination>) -> Vec<Combination> {
    match mcomb {
        None => Vec::new(),
        Some(comb) => combs.into_iter()
            .filter(|item| item < &comb)
            .collect()
    }
}

pub fn is_carte_blanche(hand: Hand) -> bool {
    let heads = vec![
        crate::cards::Rank::King,
        crate::cards::Rank::Queen,
        crate::cards::Rank::Jack,
    ];
    !hand.cards().iter().any(|card| heads.contains(&card.rank))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_combinations() {
        let hand = Hand::new(vec![ 
            Card::new(Rank::Seven, Suit::Diamond), 
            Card::new(Rank::King, Suit::Diamond), 
            Card::new(Rank::King, Suit::Heart), 
            Card::new(Rank::King, Suit::Spade), 
            Card::new(Rank::Seven, Suit::Heart), 
            Card::new(Rank::Eight, Suit::Spade), 
            Card::new(Rank::Nine, Suit::Spade), 
            Card::new(Rank::Ten, Suit::Spade), 
            Card::new(Rank::Jack, Suit::Spade), 
        ]);
        let ctype = CombinationType::Set;
        let combs_set = get_combinations(&ctype, &hand);
        let mut expected_hand = Hand::new(
                vec![
                    Card::new(Rank::King, Suit::Diamond), 
                    Card::new(Rank::King, Suit::Heart), 
                    Card::new(Rank::King, Suit::Spade), 
                ]
            );
        expected_hand.sort_by_suit();
        assert_eq!(
            combs_set,
            vec![
            Combination::new(ctype.clone(), expected_hand)
            ]
        );

        let ctype = CombinationType::Sequence;
        let combs_set = get_combinations(&ctype, &hand);
        let expected_hand = Hand::new(
                vec![
                Card::new(Rank::Eight, Suit::Spade), 
                Card::new(Rank::Nine, Suit::Spade), 
                Card::new(Rank::Ten, Suit::Spade), 
                Card::new(Rank::Jack, Suit::Spade), 
                ]
            );
        assert_eq!(
            combs_set,
            vec![
            Combination::new(ctype.clone(), expected_hand)
            ]
        );
    }

    #[test]
    fn test_carteblanche() {
        let hand = Hand::new(vec![ 
            Card::new(Rank::Seven, Suit::Diamond), 
            Card::new(Rank::King, Suit::Diamond), 
            Card::new(Rank::King, Suit::Heart), 
            Card::new(Rank::King, Suit::Spade), 
            Card::new(Rank::Seven, Suit::Heart), 
            Card::new(Rank::Eight, Suit::Spade), 
            Card::new(Rank::Nine, Suit::Spade), 
            Card::new(Rank::Ten, Suit::Spade), 
            Card::new(Rank::Jack, Suit::Spade), 
        ]);
        assert!(!is_carte_blanche(hand));
        let hand = Hand::new(vec![ 
            Card::new(Rank::Seven, Suit::Diamond), 
            Card::new(Rank::Seven, Suit::Heart), 
            Card::new(Rank::Eight, Suit::Spade), 
            Card::new(Rank::Nine, Suit::Spade), 
            Card::new(Rank::Ten, Suit::Spade), 
        ]);
        assert!(is_carte_blanche(hand));
    }
}

