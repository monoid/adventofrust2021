fn main() {
    let data = lib::read_data();

    let sum = data.iter().map(|line| predict(line)).sum::<i32>();
    println!("{}", sum);
}

fn predict(line: &[i32]) -> i32 {
    use itertools::Itertools;
    let mut last_values = vec![*line.last().unwrap()];
    let mut prev_line = line;
    let mut diff;

    while prev_line.iter().any(|n| *n != 0) {
        diff = prev_line
            .iter()
            .cloned()
            .tuple_windows::<(_, _)>()
            .map(|(a, b)| b - a)
            .collect_vec();
        last_values.push(*diff.last().unwrap());

        prev_line = &diff;
        assert!(prev_line.len() > 1);
    }

    last_values.iter().sum()
}
