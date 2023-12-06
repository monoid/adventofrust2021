fn main() {
    let data = &lib::RACES_REAL[..];
    let result: u64 = data.iter().map(lib::Race::beats).product();
    println!("{}", result);
}
