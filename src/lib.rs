use rand::{random, XorShiftRng, Rng, SeedableRng};

mod cards;
mod combinations;

pub fn run() {
    println!("PIQUET");
    let seed = makeRndSeed();

        let mut hand = cards::Hand::new(vec![ 
            cards::Card::new(cards::Rank::Seven, cards::Suit::Diamond), 
            cards::Card::new(cards::Rank::Eight, cards::Suit::Diamond), 
            cards::Card::new(cards::Rank::Seven, cards::Suit::Heart), 
            cards::Card::new(cards::Rank::King, cards::Suit::Spade), 
        ]);
        hand.sort_by_rank();

        println!("{}", hand);
}   

fn makeRndSeed() -> [u32; 4]{
    let x: u32 = random();
    println!("random: {}", x);
    [x, 0, 0, 0]
}
