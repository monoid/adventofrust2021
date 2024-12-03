use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let pairs = lib::extract1(&buf);
    let sum: u32 = pairs.into_iter().map(|(a, b)| a * b).sum();
    println!("{sum}");
}
