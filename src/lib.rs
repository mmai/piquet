use rand_core::SeedableRng;

mod cards;
mod combinations;
mod game;

pub fn run() {
    println!("PIQUET");
    let (seed, rng) = make_rng();
    let mut game = game::Game::new(rng);
    println!("Game for seed {:?}: {:?}", seed, game);
}   

fn make_rng() -> ([u8; 16], rand_xorshift::XorShiftRng) {
    let intseeds: [u8; 16] = rand::random();
    ( intseeds, rand_xorshift::XorShiftRng::from_seed(intseeds))
}
