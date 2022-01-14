use std::{
    collections::HashMap,
    io::{self, BufRead as _},
};

fn main() {
    let mut lengths = HashMap::<usize, usize>::new();
    for line in io::stdin().lock().lines() {
        if let Some((_, tail)) = line.unwrap().split_once('|') {
            let tail = tail.trim();
            for digit in tail.split_ascii_whitespace() {
                *lengths.entry(digit.len()).or_default() += 1;
            }
        }
    }
    println!(
        "{}",
        lengths.get(&2).unwrap_or(&0)
            + lengths.get(&4).unwrap_or(&0)
            + lengths.get(&3).unwrap_or(&0)
            + lengths.get(&7).unwrap_or(&0)
    );
}
