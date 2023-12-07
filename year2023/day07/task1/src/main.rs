fn main() {
    let mut hands_with_values: Vec<_> = std::io::stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            lib::parse_line_v1(&line)
        })
        .collect();
    hands_with_values.sort_unstable();

    let score = hands_with_values
        .iter()
        .enumerate()
        .map(|(i, (_hand, score))| (i + 1) * (*score as usize))
        .sum::<usize>();

    println!("{}", score);
}
