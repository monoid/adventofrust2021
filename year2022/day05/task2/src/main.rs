use std::io;

fn main() {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    let mut ship = lib::read_initial_state(&mut stdin);
    let moves = lib::read_moves(&mut stdin);

    for move_ in &moves {
        lib::apply_move2(&mut ship, move_);
    }

    // We assume that labels are sorted just to simplify the code.
    for stack in ship.values() {
        print!("{}", stack.last().unwrap());
    }
    println!();
}
