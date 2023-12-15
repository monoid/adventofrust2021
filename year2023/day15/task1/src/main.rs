fn main() {
    let data = lib::read_data_v1();

    let val: u32 = data.iter().map(|s| lib::hash(s.as_bytes()) as u32).sum();
    println!("{val}");
}
