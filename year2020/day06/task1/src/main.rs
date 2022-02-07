use std::{collections::HashSet, io};

fn group_size(answers: &str) -> usize {
    let mut bag = HashSet::new();
    for c in answers.chars() {
        if !c.is_ascii_whitespace() {
            bag.insert(c);
        }
    }
    bag.len()
}

fn main() {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    let result = lib::records(&mut stdin)
        .map(|s| group_size(&s))
        .sum::<usize>();
    println!("{}", result);
}
