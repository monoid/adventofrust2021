use std::io;

fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let cmd = lib::read_input(stdin);
    println!("{}", lib::execute(&cmd));
}
