pub fn read_data() -> Vec<Vec<i32>> {
    std::io::stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let line = line.trim();
            line.split_ascii_whitespace()
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect()
}
