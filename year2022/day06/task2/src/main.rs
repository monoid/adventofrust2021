use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin().lock();
    let line = stdin.lines().next().unwrap().unwrap();
    println!("{}", lib::marker_position(&line, 14).unwrap());
}
