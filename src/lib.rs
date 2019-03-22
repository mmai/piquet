mod cards;
mod combinations;
mod game;

pub fn run() {
    println!("PIQUET");
    let seed: [u8; 16] = rand::random();
    let mut game = game::Game::new(seed.clone());
    println!("Game for seed {:?}: {:?}", seed, game);
}   
