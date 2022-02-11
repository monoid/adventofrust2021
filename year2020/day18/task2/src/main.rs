use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    println!(
        "{}",
        stdin
            .lines()
            .map(|line| lib::calc_expression2(&line.unwrap()).unwrap().1 as i64)
            .sum::<i64>()
    );
}
