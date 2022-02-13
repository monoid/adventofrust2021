use std::io;

fn main() {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    let rules = lib::read_rules(&mut stdin).unwrap();
    println!("{}", lib::count_matches(&mut stdin, &rules));
}
