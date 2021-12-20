use lib::{enchance, read_data};

fn main() {
    let (algo, map) = read_data();
    let neg = algo.as_bytes()[0] == b'#';
    let enchanced2 = enchance(enchance(map, &algo, false, neg), &algo, neg, false);
    println!("{}", enchanced2.len());
}
