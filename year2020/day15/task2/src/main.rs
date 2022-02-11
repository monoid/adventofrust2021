fn main() {
    let size = 30000000;
    let mut state = 0;
    let mut game = lib::Game::new(size);
    for seed in [9,19,1,6,0,5,4] {
        state = game.turn(seed);
    }
    for _ in 8..size {
        state = game.turn(state);
    }
    println!("{}", state);
}
