fn main() {
    let lines = lib::read_data();
    let possibilities = lines
        .iter()
        .map(lib::Line::count_possibilities)
        .sum::<u64>();

    println!("{}", possibilities);
}
