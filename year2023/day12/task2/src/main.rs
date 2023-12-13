fn main() {
    // We need a dynamic-programming approach to handle it in reasonable time.
    let lines = lib::read_data5x();
    let possibilities = lines
        .iter()
        .map(lib::Line::count_possibilities)
        .sum::<u64>();

    println!("{}", possibilities);
}
