use std::io;

fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let data = lib::read_tiles(stdin);
    let composed = lib::compose_tiles(&data);
    let id11 = composed[0][0].id() as u64;
    let id12 = composed[0].last().unwrap().id() as u64;
    let last = composed.last().unwrap();
    let id21 = last[0].id() as u64;
    let id22 = last.last().unwrap().id() as u64;

    println!("{}", id11 * id12 * id21 * id22);
}
