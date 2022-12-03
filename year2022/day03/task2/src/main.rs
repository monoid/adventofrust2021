use std::collections::HashSet;

use itertools::Itertools;

fn main() {
    let sum = lib::read_items()
        .chunks(3)
        .into_iter()
        .map(|group| {
            lib::priority(
                group
                    .map(|(c1, c2)| HashSet::<_>::from_iter(c1.chars().chain(c2.chars())))
                    .reduce(|a, e| HashSet::from_iter(a.intersection(&e).cloned()))
                    .unwrap()
                    .into_iter()
                    .next()
                    .unwrap(),
            ) as u32
        })
        .sum::<u32>();
    println!("{}", sum);
}
