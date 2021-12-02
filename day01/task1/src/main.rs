use std::io::{self, BufRead as _};
use itertools::Itertools;

fn main() {
    let count: i32 = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().parse::<i32>().expect("Malformed int"))
        .tuple_windows()
        .map(|(s1, s2)| (s1 < s2) as i32)
        .sum();
    println!("{}", count);
}
