use std::collections::HashSet;

fn main() {
    let sum = lib::read_items()
        .map(|(c1, c2)| {
            let h1 = HashSet::<_>::from_iter(c1.chars());
            let h2 = HashSet::from_iter(c2.chars());
            let common = h1.intersection(&h2).cloned().next().unwrap();
            lib::priority(common) as u32
        })
        .sum::<u32>();
    println!("{}", sum);
}
