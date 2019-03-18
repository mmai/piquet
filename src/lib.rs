mod cards;
mod combinations;

pub fn display() {
    println!("PIQUET");
    println!("{}", cards::Rank::Eight > cards::Rank::Seven);


        let mut hand = cards::Hand::new(vec![ 
            cards::Card::new(cards::Rank::Seven, cards::Suit::Diamond), 
            cards::Card::new(cards::Rank::Eight, cards::Suit::Diamond), 
            cards::Card::new(cards::Rank::Seven, cards::Suit::Heart), 
            cards::Card::new(cards::Rank::King, cards::Suit::Spade), 
        ]);
        hand.sort_by_rank();

        println!("{}", hand);
}
