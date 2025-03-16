fn main() {
    let mut stones = lib::Stones::read(std::io::stdin().lock());
    for _ in 0..75 {
        stones.blink();
    }
    let res = stones.len();
    println!("{res}");
}
