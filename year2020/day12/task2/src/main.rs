use std::io;

fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock();

    let mut ship = lib::Ship::new();
    for cmd in lib::read_commands(stdin) {
        ship.apply_wp(cmd);
    }

    println!("{}", ship.distance());
}
