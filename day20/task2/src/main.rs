use lib::{enchance, read_data};

fn main() {
    let (algo, mut map) = read_data();
    let neg = algo.as_bytes()[0] == b'#';

    for _ in 0..25 {
        map = enchance(enchance(map, &algo, false, neg), &algo, neg, false);
    }
    println!("{}", map.len());
}
