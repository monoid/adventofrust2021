use std::{collections::HashMap, io};

fn count_nested(data: &HashMap<String, Vec<(usize, String)>>, start: &str) -> usize {
    let nested = data.get(start).unwrap();
    nested
        .iter()
        .map(|(mul, child)| mul * (1 + count_nested(data, child)))
        .sum()
}

fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let data = lib::read_rules(stdin)
        .into_iter()
        .map(|rec| (rec.color, rec.nested))
        .collect();

    println!("{}", count_nested(&data, "shiny gold"))
}
