use std::{collections::HashSet, io};

fn group_size(answers: &str) -> usize {
    answers
        .split_ascii_whitespace()
        .map(|ans| ans.chars().collect::<HashSet<_>>())
        .reduce(|a, b| a.intersection(&b).cloned().collect::<HashSet<_>>())
        .unwrap()
        .len()
}

fn main() {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    let result = lib::records(&mut stdin)
        .map(|s| group_size(&s))
        .sum::<usize>();
    println!("{}", result);
}
