fn main() {
    let mut state = 0;
    let mut game = lib::Game::new(2020);
    for seed in [9,19,1,6,0,5,4] {
        state = game.turn(seed);
    }
    for _ in 8..2020 {
        state = game.turn(state);
    }
    println!("{}", state);
}
